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
        let versions = create_versions(data)?;
        if versions.len() > 0 {
            file.img_versions = Some(versions);
        }

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

fn create_versions(data: &FilePayload) -> Result<Vec<ImgVersionDto>> {
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
        filename: data.filename.clone(),
        url: None,
    };

    // Create thumbnail
    let thumb = create_thumbnail(data, rotated_img)?;

    Ok(vec![orig_version, thumb])
}

fn create_thumbnail(data: &FilePayload, mut img: DynamicImage) -> Result<ImgVersionDto> {
    // Prepare dir
    let thumb_dir = data
        .upload_dir
        .clone()
        .join(ImgVersion::Thumbnail.to_string());

    if let Err(err) = std::fs::create_dir_all(&thumb_dir) {
        return Err(format!("Unable to create thumbnail dir: {}", err).into());
    }
    println!("thumb_dir: {:?}", thumb_dir);

    let Ok(dim) = ImgDimension::try_from(ImgVersion::Thumbnail) else {
        return Err("Unable to identify thumbnail dimension settings".into());
    };

    let source_width = img.width();
    let source_height = img.height();

    // This one is brought to you by chad jipitty
    let aspect_ratio = dim.width as f32 / dim.height as f32;
    let current_aspect_ratio = source_width as f32 / source_height as f32;

    let (crop_width, crop_height, x_offset, y_offset) = if current_aspect_ratio > aspect_ratio {
        // Crop horizontally (landscape mode)
        let crop_width = (source_height as f32 * aspect_ratio) as u32;
        let x_offset = (source_width - crop_width) / 2;
        (crop_width, source_height, x_offset, 0)
    } else {
        // Crop vertically (portrait mode)
        let crop_height = (source_width as f32 / aspect_ratio) as u32;
        let y_offset = (source_height - crop_height) / 2;
        (source_width, crop_height, 0, y_offset)
    };

    // Crop the image using scaled dimensions, cutting off some parts
    let cropped = img.crop(x_offset, y_offset, crop_width, crop_height);

    // Resize the cropped image to the desired dimensions
    let resized_img = cropped.resize_exact(dim.width, dim.height, imageops::FilterType::Lanczos3);

    // Save the resized image
    let version = ImgVersionDto {
        version: ImgVersion::Thumbnail,
        dimension: ImgDimension {
            width: cropped.width(),
            height: cropped.height(),
        },
        filename: data.filename.clone(),
        url: None,
    };

    let dest_file = version.to_path(&data.upload_dir);

    // All non-original versions will be saved as JPEG
    if let Err(err) = resized_img.save_with_format(dest_file, image::ImageFormat::Jpeg) {
        return Err(format!("Unable to save thumbnail: {}", err).into());
    }

    Ok(version)
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
