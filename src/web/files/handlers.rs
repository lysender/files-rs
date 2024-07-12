use axum::{
    extract::{Multipart, Query, State},
    http::StatusCode,
    Extension,
};
use tokio::{
    fs::{create_dir_all, File},
    io::AsyncWriteExt,
};

use crate::{
    auth::Actor,
    buckets::BucketDto,
    dirs::Dir,
    files::{create_file, list_files, FilePayload, ImgVersion, ListFilesParams},
    roles::Permission,
    util::slugify_prefixed,
    web::{response::JsonResponse, server::AppState},
    Error, Result,
};

pub async fn list_files_handler(
    State(state): State<AppState>,
    Extension(actor): Extension<Actor>,
    Extension(bucket): Extension<BucketDto>,
    Extension(dir): Extension<Dir>,
    query: Option<Query<ListFilesParams>>,
) -> Result<JsonResponse> {
    let permissions = vec![Permission::FilesList, Permission::FilesView];
    if !actor.has_permissions(&permissions) {
        return Err(Error::Forbidden("Insufficient permissions".to_string()));
    }

    let Some(params) = query else {
        return Err(Error::BadRequest("Invalid query parameters".to_string()));
    };
    let files = list_files(&state.db_pool, &bucket.name, &dir, &params).await?;
    Ok(JsonResponse::new(serde_json::to_string(&files).unwrap()))
}

pub async fn create_file_handler(
    State(state): State<AppState>,
    Extension(bucket): Extension<BucketDto>,
    Extension(dir): Extension<Dir>,
    mut multipart: Multipart,
) -> Result<JsonResponse> {
    let mut payload: Option<FilePayload> = None;

    while let Some(mut field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        if name != "file" {
            continue;
        }

        let original_filename = field.file_name().unwrap().to_string();

        // Low chance of collision but higher than the full uuid v7 string
        // Prefer a shorter filename for better readability
        let filename = slugify_prefixed(&original_filename);

        // Ensure upload dir exists
        let orig_dir = state
            .config
            .upload_dir
            .clone()
            .join(ImgVersion::Original.to_string());
        let dir_res = create_dir_all(orig_dir.clone()).await;
        if let Err(_) = dir_res {
            return Err("Unable to create upload dir".into());
        }

        // Prepare to save to file
        let file_path = orig_dir.as_path().join(&filename);
        let Ok(mut file) = File::create(&file_path).await else {
            return Err("Unable to create file".into());
        };

        // Stream contents to file
        let mut size: usize = 0;
        while let Some(chunk) = field.chunk().await.unwrap() {
            size += chunk.len();
            file.write_all(&chunk).await.unwrap();
        }

        payload = Some({
            FilePayload {
                upload_dir: state.config.upload_dir.clone(),
                name: original_filename,
                filename: filename.clone(),
                path: orig_dir.clone().join(&filename),
                size: size as i64,
            }
        })
    }

    let Some(payload) = payload else {
        return Err(Error::MissingUploadFile("Missing upload file".to_string()));
    };

    let db_pool = state.db_pool.clone();
    let res = create_file(&db_pool, &bucket, &dir, &payload).await;
    match res {
        Ok(file) => Ok(JsonResponse::with_status(
            StatusCode::CREATED,
            serde_json::to_string(&file).unwrap(),
        )),
        Err(e) => Err(e),
    }
}
