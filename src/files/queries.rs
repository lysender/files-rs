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

use crate::buckets::BucketDto;
use crate::dirs::{update_dir_timestamp, Dir};
use crate::schema::files::{self, dsl};
use crate::storage::upload_object;
use crate::util::generate_id;
use crate::validators::flatten_errors;
use crate::web::pagination::Paginated;
use crate::{Error, Result};

use super::{
    FileDto, FileObject, FilePayload, ImgDimension, ImgVersion, ImgVersionDto, ListFilesParams,
    ALLOWED_IMAGE_TYPES, MAX_DIMENSION, MAX_PREVIEW_DIMENSION, MAX_THUMB_DIMENSION, ORIGINAL_PATH,
};

const MAX_PER_PAGE: i32 = 50;

pub async fn list_files(
    db_pool: &Pool,
    dir: &Dir,
    params: &ListFilesParams,
) -> Result<Paginated<FileObject>> {
    if let Err(errors) = params.validate() {
        return Err(Error::ValidationError(flatten_errors(&errors)));
    }
    let Ok(db) = db_pool.get().await else {
        return Err("Error getting db connection".into());
    };

    let did = dir.id.clone();

    let total_records = list_files_count(db_pool, &dir.id, params).await?;
    let mut page: i32 = 1;
    let mut per_page: i32 = MAX_PER_PAGE;
    let mut offset: i64 = 0;

    if let Some(per_page_param) = params.per_page {
        if per_page_param > 0 && per_page_param <= MAX_PER_PAGE {
            per_page = per_page_param;
        }
    }

    let total_pages: i64 = (total_records as f64 / per_page as f64).ceil() as i64;

    if let Some(p) = params.page {
        let p64 = p as i64;
        if p64 > 0 && p64 <= total_pages {
            page = p;
            offset = (p64 - 1) * per_page as i64;
        }
    }

    // Do not query if we already know there are no records
    if total_pages == 0 {
        return Ok(Paginated::new(Vec::new(), page, per_page, total_records));
    }

    let params_copy = params.clone();
    let conn_result = db
        .interact(move |conn| {
            let mut query = dsl::files.into_boxed();
            query = query.filter(dsl::dir_id.eq(did.as_str()));

            if let Some(keyword) = params_copy.keyword {
                if keyword.len() > 0 {
                    let pattern = format!("%{}%", keyword);
                    query = query.filter(dsl::name.like(pattern));
                }
            }
            query
                .limit(per_page as i64)
                .offset(offset)
                .select(FileObject::as_select())
                .order(dsl::created_at.desc())
                .load::<FileObject>(conn)
        })
        .await;

    match conn_result {
        Ok(select_res) => match select_res {
            Ok(items) => Ok(Paginated::new(items, page, per_page, total_records)),
            Err(e) => {
                error!("{e}");
                Err("Error reading files".into())
            }
        },
        Err(e) => {
            error!("{e}");
            Err("Error using the db connection".into())
        }
    }
}

async fn list_files_count(db_pool: &Pool, dir_id: &str, params: &ListFilesParams) -> Result<i64> {
    let Ok(db) = db_pool.get().await else {
        return Err("Error getting db connection".into());
    };

    let did = dir_id.to_string();
    let params_copy = params.clone();

    let conn_result = db
        .interact(move |conn| {
            let mut query = dsl::files.into_boxed();
            query = query.filter(dsl::dir_id.eq(did.as_str()));
            if let Some(keyword) = params_copy.keyword {
                if keyword.len() > 0 {
                    let pattern = format!("%{}%", keyword);
                    query = query.filter(dsl::name.like(pattern));
                }
            }
            query.select(count_star()).get_result::<i64>(conn)
        })
        .await;

    match conn_result {
        Ok(count_res) => match count_res {
            Ok(count) => Ok(count),
            Err(e) => {
                error!("{}", e);
                Err("Error counting files".into())
            }
        },
        Err(e) => {
            error!("{}", e);
            Err("Error using the db connection".into())
        }
    }
}

pub async fn count_dir_files(db_pool: &Pool, dir_id: &str) -> Result<i64> {
    let Ok(db) = db_pool.get().await else {
        return Err("Error getting db connection".into());
    };

    let did = dir_id.to_string();
    let conn_result = db
        .interact(move |conn| {
            dsl::files
                .filter(dsl::dir_id.eq(did.as_str()))
                .select(count_star())
                .get_result::<i64>(conn)
        })
        .await;

    match conn_result {
        Ok(count_res) => match count_res {
            Ok(count) => Ok(count),
            Err(e) => {
                error!("{}", e);
                Err("Error counting files".into())
            }
        },
        Err(e) => {
            error!("{}", e);
            Err("Error using the db connection".into())
        }
    }
}

pub async fn create_file(
    db_pool: &Pool,
    bucket: &BucketDto,
    dir: &Dir,
    data: &FilePayload,
) -> Result<FileObject> {
    let mut file_dto = init_file(dir, data)?;

    if bucket.images_only && !file_dto.is_image {
        return Err(Error::ValidationError("Bucket only accepts images".into()));
    }

    if file_dto.is_image {
        let versions = create_versions(data)?;
        if versions.len() > 0 {
            file_dto.img_versions = Some(versions);
        }
    }

    let _ = upload_object(bucket, dir, &data.upload_dir, &file_dto).await?;

    // Save to database
    let file_db_pool = db_pool.clone();
    let Ok(db) = file_db_pool.get().await else {
        return Err("Error getting db connection".into());
    };

    let file: FileObject = file_dto.clone().into();
    let file_copy = file.clone();

    let conn_result = db
        .interact(move |conn| {
            diesel::insert_into(files::table)
                .values(&file_copy)
                .execute(conn)
        })
        .await;

    match conn_result {
        Ok(insert_res) => match insert_res {
            Ok(_) => {
                // Cleanup files before returning...
                if let Err(e) = cleanup_temp_uploads(data, &file_dto) {
                    // Can't afford to fail here, we will just log the error...
                    error!("{}", e);
                }

                // Also update dir
                let today = chrono::Utc::now().timestamp();
                let dir_result = update_dir_timestamp(db_pool, &dir.id, today).await;
                if let Err(e) = dir_result {
                    // Can't afford to fail here, we will just log the error...
                    error!("{}", e);
                }

                Ok(file)
            }
            Err(e) => {
                error!("{}", e);
                Err("Error creating file".into())
            }
        },
        Err(e) => {
            error!("{}", e);
            Err("Error using the db connection".into())
        }
    }
}

pub async fn get_file(pool: &Pool, id: &str) -> Result<Option<FileObject>> {
    let Ok(db) = pool.get().await else {
        return Err("Error getting db connection".into());
    };

    let fid = id.to_string();
    let conn_result = db
        .interact(move |conn| {
            dsl::files
                .find(fid)
                .select(FileObject::as_select())
                .first::<FileObject>(conn)
                .optional()
        })
        .await;

    match conn_result {
        Ok(select_res) => match select_res {
            Ok(item) => Ok(item),
            Err(e) => {
                error!("{e}");
                Err("Error reading files".into())
            }
        },
        Err(e) => {
            error!("{e}");
            Err("Error using the db connection".into())
        }
    }
}

pub async fn delete_file(pool: &Pool, id: &str) -> Result<()> {
    let Ok(db) = pool.get().await else {
        return Err("Error getting db connection".into());
    };

    let fid = id.to_string();
    let conn_result = db
        .interact(move |conn| diesel::delete(dsl::files.filter(dsl::id.eq(fid))).execute(conn))
        .await;

    match conn_result {
        Ok(delete_res) => match delete_res {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("{e}");
                Err("Error deleting file".into())
            }
        },
        Err(e) => {
            error!("{e}");
            Err("Error using the db connection".into())
        }
    }
}

fn cleanup_temp_uploads(data: &FilePayload, file: &FileDto) -> Result<()> {
    if file.is_image {
        // Cleanup versions
        if let Some(versions) = &file.img_versions {
            let mut errors: Vec<String> = Vec::new();
            for version in versions.iter() {
                let source_file = version.to_path(&data.upload_dir, &file.filename);
                if let Err(err) = std::fs::remove_file(&source_file) {
                    errors.push(format!("Unable to remove file after upload: {}", err));
                }
            }

            if errors.len() > 0 {
                return Err(errors.join(", ").as_str().into());
            }
        }
    } else {
        // Cleanup original file
        let upload_dir = data.upload_dir.clone();
        let source_file = upload_dir.join(ORIGINAL_PATH).join(&file.filename);
        if let Err(err) = std::fs::remove_file(&source_file) {
            return Err(format!("Unable to remove file after upload: {}", err).into());
        }
    }

    Ok(())
}

fn init_file(dir: &Dir, data: &FilePayload) -> Result<FileDto> {
    let mut is_image = false;
    let content_type = get_content_type(&data.path)?;
    if content_type.starts_with("image/") {
        if !ALLOWED_IMAGE_TYPES.contains(&content_type.as_str()) {
            return Err("Uploaded image type not allowed".into());
        }
        is_image = true;
    }

    // May be a few second delayed due to image processing
    let today = chrono::Utc::now().timestamp();

    let file = FileDto {
        id: generate_id(),
        dir_id: dir.id.clone(),
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
    let orientation = parse_exif_orientation(&data.path).unwrap_or(1);
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

    let mut versions: Vec<ImgVersionDto> = vec![orig_version];

    // // Only create preview if original image has side longer than max
    if source_width > MAX_DIMENSION || source_height > MAX_DIMENSION {
        let preview = create_preview(data, &rotated_img)?;
        versions.push(preview);
    }

    // Create thumbnail
    let thumb = create_thumbnail(data, &rotated_img)?;
    versions.push(thumb);

    Ok(versions)
}

fn create_preview(data: &FilePayload, img: &DynamicImage) -> Result<ImgVersionDto> {
    // Prepare dir
    let prev_dir = data
        .upload_dir
        .clone()
        .join(ImgVersion::Preview.to_string());

    if let Err(err) = std::fs::create_dir_all(&prev_dir) {
        return Err(format!("Unable to create preview dir: {}", err).into());
    }

    // Either resize to max dimension or original dimension
    // whichever is smaller
    let mut max_width = MAX_PREVIEW_DIMENSION;
    if img.width() < MAX_PREVIEW_DIMENSION {
        max_width = img.width();
    }
    let mut max_height = MAX_PREVIEW_DIMENSION;
    if img.height() < MAX_PREVIEW_DIMENSION {
        max_height = img.height();
    }

    let resized_img = img.resize(max_width, max_height, imageops::FilterType::Lanczos3);

    // Save the resized image
    let version = ImgVersionDto {
        version: ImgVersion::Preview,
        dimension: ImgDimension {
            width: resized_img.width(),
            height: resized_img.height(),
        },
        url: None,
    };

    let dest_file = version.to_path(&data.upload_dir, &data.filename);

    if let Err(err) = resized_img.save(dest_file) {
        return Err(format!("Unable to save preview: {}", err).into());
    }

    Ok(version)
}

fn create_thumbnail(data: &FilePayload, img: &DynamicImage) -> Result<ImgVersionDto> {
    // Prepare dir
    let prev_dir = data
        .upload_dir
        .clone()
        .join(ImgVersion::Thumbnail.to_string());

    if let Err(err) = std::fs::create_dir_all(&prev_dir) {
        return Err(format!("Unable to create preview dir: {}", err).into());
    }

    // Either resize to max dimension or original dimension
    // whichever is smaller
    let mut max_width = MAX_THUMB_DIMENSION;
    if img.width() < MAX_THUMB_DIMENSION {
        max_width = img.width();
    }
    let mut max_height = MAX_THUMB_DIMENSION;
    if img.height() < MAX_THUMB_DIMENSION {
        max_height = img.height();
    }

    let resized_img = img.resize(max_width, max_height, imageops::FilterType::Lanczos3);

    // Save the resized image
    let version = ImgVersionDto {
        version: ImgVersion::Thumbnail,
        dimension: ImgDimension {
            width: resized_img.width(),
            height: resized_img.height(),
        },
        url: None,
    };

    let dest_file = version.to_path(&data.upload_dir, &data.filename);

    if let Err(err) = resized_img.save(dest_file) {
        return Err(format!("Unable to save preview: {}", err).into());
    }

    Ok(version)
}

fn get_content_type(path: &PathBuf) -> Result<String> {
    match infer::get_from_path(path) {
        Ok(Some(kind)) => Ok(kind.mime_type().to_string()),
        Ok(None) => Err("Uploaded file type unknown".into()),
        Err(_) => Err("Unable to read uploaded file".into()),
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
