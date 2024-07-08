use axum::{
    extract::{Json, Multipart, Path, Query, State},
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
    roles::Permission,
    storage::list_objects,
    util::{generate_id, slugify_prefixed},
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
    while let Some(mut field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap();
        let original_filename = field.file_name().unwrap();
        let filename = slugify_prefixed(&original_filename);
        let content_type = field.content_type().unwrap();

        // Ensure upload dir exists
        let upload_dir = state.config.upload_dir.clone();
        let dir_res = create_dir_all(upload_dir.clone()).await;
        if let Err(_) = dir_res {
            return Err("Unable to create upload dir".into());
        }

        println!("name: {}", name);
        println!("original filename: {}", original_filename);
        println!("filename: {}", filename);
        println!("content_type: {}", content_type);

        // Prepare to save to file
        let file_path = upload_dir.as_path().join(&filename);
        let Ok(mut file) = File::create(&file_path).await else {
            return Err("Unable to create file".into());
        };

        // Stream contents to file
        while let Some(chunk) = field.chunk().await.unwrap() {
            file.write_all(&chunk).await.unwrap();
        }

        println!("uploaded file: {:?}", file_path);
    }
    Ok(JsonResponse::new("uploaded file...".to_string()))
}
