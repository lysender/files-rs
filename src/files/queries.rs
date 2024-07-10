use deadpool_diesel::sqlite::Pool;
use exif::{In, Tag};
use image::imageops;
use image::io::Reader as ImageReader;
use std::fs::File as FsFile;
use std::path::PathBuf;

use diesel::dsl::count_star;
use diesel::prelude::*;
use diesel::{QueryDsl, SelectableHelper};
use image::DynamicImage;
use tracing::error;
use validator::Validate;

use crate::dirs::{Dir, NewDir, UpdateDir};
use crate::schema::dirs::{self, dsl};
use crate::util::generate_id;
use crate::validators::flatten_errors;
use crate::web::pagination::Paginated;
use crate::{Error, Result};

use super::{FileDtox, FilePayload, ImgDimension, ImgVersion, ImgVersionDto, ALLOWED_IMAGE_TYPES};

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

    let mut file = init_file(dir_id, data)?;

    if file.is_image {
        let orientation = match parse_exif_orientation(&data.path) {
            Ok(v) => v,
            Err(_) => 1,
        };

        let img = read_image(&data.path)?;

        // Rotate based on exif orientation before creating versions
        let rotated_img = match orientation {
            8 => img.rotate90(),
            3 => img.rotate180(),
            6 => img.rotate90(),
            _ => img,
        };

        let source_width = rotated_img.width();
        let source_height = rotated_img.height();

        let orig_version = ImgVersionDto {
            version: ImgVersion::Original,
            dimension: ImgDimension {
                width: source_width,
                height: source_height,
            },
            url: None,
        };

        file.img_versions = Some(vec![orig_version]);

        // Create image versions
        // Save to storage
        // Update file with versions
    }

    Ok(file)
}

fn init_file(dir_id: &str, data: &FilePayload) -> Result<FileDtox> {
    let Ok(kind) = infer::get_from_path(&data.path) else {
        return Err("Unable to read uploaded file".into());
    };
    let Some(kind) = kind else {
        return Err("Uploaded file type unknown".into());
    };

    let mut is_image = false;
    let content_type = kind.mime_type().to_string();
    if content_type.starts_with("image/") {
        if !ALLOWED_IMAGE_TYPES.contains(&content_type.as_str()) {
            return Err("Uploaded image type not allowed".into());
        }
        is_image = true;
    }

    // May be a few second delayed due to image processing
    let today = chrono::Utc::now().timestamp();

    let file = FileDtox {
        id: generate_id(),
        dir_id: dir_id.to_string(),
        name: data.name.clone(),
        filename: data.filename.clone(),
        content_type,
        size: data.size,
        url: None,
        is_image,
        img_versions: None,
        created_at: today,
        updated_at: today,
    };

    Ok(file)
}

fn read_image(path: &PathBuf) -> Result<DynamicImage> {
    match ImageReader::open(path) {
        Ok(read_img) => match read_img.with_guessed_format() {
            Ok(format_img) => match format_img.decode() {
                Ok(img) => Ok(img),
                Err(e) => {
                    let msg = format!("Unable to decode image: {}", e.to_string());
                    error!("{}", msg);
                    Err(msg.as_str().into())
                }
            },
            Err(e) => {
                let msg = format!("Unable to guess image format: {}", e.to_string());
                error!("{}", msg);
                Err(msg.as_str().into())
            }
        },
        Err(e) => {
            let msg = format!("Unable to read image: {}", e.to_string());
            error!("{}", msg);
            Err(msg.as_str().into())
        }
    }
}

fn parse_exif_orientation(path: &PathBuf) -> Result<u32> {
    let Ok(file) = FsFile::open(path) else {
        return Err("Unable to open file".into());
    };

    let mut buf_reader = std::io::BufReader::new(&file);
    let exit_reader = exif::Reader::new();
    let Ok(exif) = exit_reader.read_from_container(&mut buf_reader) else {
        return Err("Unable to read exif data".into());
    };

    // Default to 1 if cannot identify orientation
    let result = match exif.get_field(Tag::Orientation, In::PRIMARY) {
        Some(orientation) => match orientation.value.get_uint(0) {
            Some(v @ 1..=8) => v,
            _ => 1,
        },
        None => 1,
    };

    Ok(result)
}
