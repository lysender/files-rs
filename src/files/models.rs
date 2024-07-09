use diesel::prelude::*;
use serde::Serialize;

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
    pub img_dimension: Option<String>,
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
    pub is_image: bool,
    pub img_dimension: Option<ImgDimension>,
    pub img_versions: Option<Vec<ImgVersionDto>>,
    pub created_at: i64,
    pub updated_at: i64,
}

impl From<File> for FileDtox {
    fn from(file: File) -> Self {
        let img_dimension = match file.img_dimension {
            Some(dim) => {
                // Parse dimension string like 200x400 into width: 200, height: 400
                let values: Vec<i32> = dim
                    .split('x')
                    .filter_map(|s| s.parse::<i32>().ok())
                    .collect();

                if values.len() == 2 {
                    Some(ImgDimension {
                        width: values[0],
                        height: values[1],
                    })
                } else {
                    None
                }
            }
            None => None,
        };
        let img_versions = match file.img_versions {
            Some(versions) => {
                // Parse versions string like: orig,thumb into its equivalent struct
                let versions: Vec<ImgVersion> = versions
                    .split(',')
                    .filter_map(|s| s.try_into().ok())
                    .collect();

                if versions.len() > 0 {
                    Some(
                        versions
                            .into_iter()
                            .map(|v| ImgVersionDto {
                                version: v,
                                url: None,
                            })
                            .collect::<Vec<ImgVersionDto>>(),
                    )
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
            img_dimension,
            img_versions,
            created_at: file.created_at,
            updated_at: file.updated_at,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ImgDimension {
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum ImgVersion {
    Original,
    Thumbnail,
}

#[derive(Debug, Clone, Serialize)]
pub struct ImgVersionDto {
    pub version: ImgVersion,
    pub url: Option<String>,
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
