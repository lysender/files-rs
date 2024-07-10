use std::{path::PathBuf, str::FromStr};

use diesel::prelude::*;
use serde::Serialize;

pub const ALLOWED_IMAGE_TYPES: [&str; 4] = ["image/jpeg", "image/pjpeg", "image/png", "image/gif"];

#[derive(Debug, Clone, Queryable, Selectable, Insertable, Serialize)]
#[diesel(table_name = crate::schema::files)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct File {
    pub id: String,
    pub dir_id: String,
    pub name: String,
    pub filename: String,
    pub content_type: String,
    pub size: i64,
    pub is_image: i32,
    pub img_versions: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct FileDtox {
    pub id: String,
    pub dir_id: String,
    pub name: String,
    pub filename: String,
    pub content_type: String,
    pub size: i64,

    // Only available on non-image files
    pub url: Option<String>,

    pub is_image: bool,

    // Only available for image files, main url is in orig version
    pub img_versions: Option<Vec<ImgVersionDto>>,

    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone)]
pub struct FilePayload {
    pub name: String,
    pub filename: String,
    pub path: PathBuf,
    pub content_type: String,
    pub size: i64,
    pub is_image: bool,
}

impl From<File> for FileDtox {
    fn from(file: File) -> Self {
        let img_versions = match file.img_versions {
            Some(versions_str) => {
                let versions: Vec<ImgVersionDto> = versions_str
                    .split(',')
                    .filter_map(|s| s.parse::<ImgVersionDto>().ok())
                    .collect();

                if versions.len() > 0 {
                    Some(versions)
                } else {
                    None
                }
            }
            None => None,
        };

        Self {
            id: file.id,
            dir_id: file.dir_id,
            name: file.name,
            filename: file.filename,
            content_type: file.content_type,
            size: file.size,
            is_image: file.is_image == 1,
            img_versions,
            url: None,
            created_at: file.created_at,
            updated_at: file.updated_at,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ImgDimension {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum ImgVersion {
    #[serde(rename = "orig")]
    Original,

    #[serde(rename = "thumb")]
    Thumbnail,
}

#[derive(Debug, Clone, Serialize)]
pub struct ImgVersionDto {
    pub version: ImgVersion,
    pub dimension: ImgDimension,
    pub url: Option<String>,
}

impl FromStr for ImgVersionDto {
    type Err = String;

    /// Parse string like "orig:200x400" into ImgVersionDto without the url part
    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 2 {
            return Err("Invalid image version dto".to_string());
        }

        let version = ImgVersion::try_from(parts[0])?;
        let dimension = parts[1]
            .split('x')
            .filter_map(|s| s.parse::<u32>().ok())
            .collect::<Vec<u32>>();

        if dimension.len() != 2 {
            return Err("Invalid image dimension".to_string());
        }

        Ok(Self {
            version,
            dimension: ImgDimension {
                width: dimension[0],
                height: dimension[1],
            },
            url: None,
        })
    }
}

impl core::fmt::Display for ImgVersion {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Original => write!(f, "{}", "orig"),
            Self::Thumbnail => write!(f, "{}", "thumb"),
        }
    }
}

impl TryFrom<&str> for ImgVersion {
    type Error = String;

    fn try_from(value: &str) -> core::result::Result<Self, Self::Error> {
        match value {
            "orig" => Ok(Self::Original),
            "thumb" => Ok(Self::Thumbnail),
            _ => Err(format!("Invalid image version: {}", value)),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct FileDto {
    pub name: String,
    pub urls: FileUrls,
}

#[derive(Debug, Clone, Serialize)]
pub struct FileUrls {
    pub o: String,
    pub s: String,
}

impl FileUrls {
    pub fn new() -> Self {
        Self {
            o: "".to_string(),
            s: "".to_string(),
        }
    }
}
