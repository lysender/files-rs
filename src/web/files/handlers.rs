use axum::{
    extract::{Multipart, State},
    Extension,
};
use tokio::{
    fs::{create_dir_all, File},
    io::AsyncWriteExt,
};

use crate::{
    auth::Actor,
    buckets::Bucket,
    dirs::Dir,
    files::{FilePayload, ALLOWED_IMAGE_TYPES},
    roles::Permission,
    storage::list_objects,
    util::slugify_prefixed,
    web::{response::JsonResponse, server::AppState},
    Error, Result,
};

pub async fn list_files_handler(
    Extension(actor): Extension<Actor>,
    Extension(bucket): Extension<Bucket>,
    Extension(dir): Extension<Dir>,
) -> Result<JsonResponse> {
    let permissions = vec![Permission::FilesList, Permission::FilesView];
    if !actor.has_permissions(&permissions) {
        return Err(Error::Forbidden("Insufficient permissions".to_string()));
    }

    let files = list_objects(&bucket.name, &dir.name).await?;
    Ok(JsonResponse::new(serde_json::to_string(&files).unwrap()))
}

pub async fn create_file_handler(
    State(state): State<AppState>,
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
        let content_type = field.content_type().unwrap().to_string();
        let is_image = content_type.starts_with("image/");

        if is_image {
            // Initial validation from mime-type whitelist
            if !ALLOWED_IMAGE_TYPES.contains(&content_type.as_str()) {
                return Err(Error::FileTypeNotAllowed);
            }
        }

        // Ensure upload dir exists
        let upload_dir = state.config.upload_dir.clone().join("orig");
        let dir_res = create_dir_all(upload_dir.clone()).await;
        if let Err(_) = dir_res {
            return Err("Unable to create upload dir".into());
        }

        // Prepare to save to file
        let file_path = upload_dir.as_path().join(&filename);
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
                name: original_filename,
                filename: filename.clone(),
                path: upload_dir.clone().join(&filename),
                content_type,
                size: size as i64,
                is_image,
            }
        })
    }

    let Some(payload) = payload else {
        return Err(Error::MissingUploadFile("Missing upload file".to_string()));
    };

    println!("payload: {:?}", payload);
    Ok(JsonResponse::new("uploaded file...".to_string()))
}
