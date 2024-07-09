use deadpool_diesel::sqlite::Pool;

use diesel::dsl::count_star;
use diesel::prelude::*;
use diesel::{QueryDsl, SelectableHelper};
use tracing::error;
use validator::Validate;

use crate::dirs::{Dir, NewDir, UpdateDir};
use crate::schema::dirs::{self, dsl};
use crate::util::generate_id;
use crate::validators::flatten_errors;
use crate::web::pagination::Paginated;
use crate::{Error, Result};

use super::{FileDtox, FilePayload};

const MAX_FILES: i32 = 1000;
const MAX_PER_PAGE: i32 = 50;

pub async fn list_files() -> Result<Vec<FileDtox>> {
    Ok(vec![])
}

pub async fn create_file(db_pool: &Pool, dir_id: &str, data: &FilePayload) -> Result<FileDtox> {
    let Ok(db) = db_pool.get().await else {
        return Err("Error getting db connection".into());
    };

    // Validate actual file
    // Create versions if an image
    // Upload to storage
    // Save to database
    // Cleanup created files
    // Profit?

    let Ok(kind) = infer::get_from_path(&data.path) else {
        return Err("Unable to read uploaded file".into());
    };
    let Some(kind) = kind else {
        return Err("Uploaded file type unknown".into());
    };

    let content_type = kind.mime_type().to_string();
    if content_type != data.content_type {
        return Err("Uploaded file type mismatch".into());
    }
    println!("Content type: {}", content_type);

    let today = chrono::Utc::now().timestamp();
    let mut file = FileDtox {
        id: generate_id(),
        dir_id: dir_id.to_string(),
        name: data.name.clone(),
        filename: data.filename.clone(),
        content_type,
        size: data.size,
        is_image: data.is_image,
        img_dimension: None,
        img_versions: None,
        created_at: today,
        updated_at: today,
    };

    if data.is_image {
        // Create image versions
        // Save to storage
        // Update file with versions
    }

    Ok(file)
}
