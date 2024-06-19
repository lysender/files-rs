use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize)]
pub struct File {
    pub name: String,
    pub url: String,
}
