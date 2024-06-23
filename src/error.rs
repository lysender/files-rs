use axum::{
    body::Body,
    response::{IntoResponse, Response},
};
use derive_more::From;

use crate::web::response::to_error_response;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    #[from]
    AnyError(String),
    BadRequest(String),
    ValidationError(String),
    NotFound(String),
    InvalidAuthToken,
    NoAuthToken,
    InvalidClient,
    RequiresAuth,
    HashPasswordError(String),
    VerifyPasswordHashError(String),
    InvalidPassword,
    Base64DecodeError(String),
}

// Allow string slices to be converted to Error
impl From<&str> for Error {
    fn from(val: &str) -> Self {
        Self::AnyError(val.to_string())
    }
}

// Allow errors to be displayed as string
impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::AnyError(val) => write!(f, "{}", val),
            Self::BadRequest(val) => write!(f, "{}", val),
            Self::ValidationError(val) => write!(f, "{}", val),
            Self::NotFound(val) => write!(f, "{}", val),
            Self::InvalidAuthToken => write!(f, "Invalid auth token"),
            Self::NoAuthToken => write!(f, "No auth token"),
            Self::InvalidClient => write!(f, "Invalid client"),
            Self::RequiresAuth => write!(f, "Requires authentication"),
            Self::HashPasswordError(val) => write!(f, "{}", val),
            Self::VerifyPasswordHashError(val) => write!(f, "{}", val),
            Self::InvalidPassword => write!(f, "Invalid password"),
            Self::Base64DecodeError(val) => write!(f, "{}", val),
        }
    }
}

// Allow errors to be rendered as response
impl IntoResponse for Error {
    fn into_response(self) -> Response<Body> {
        to_error_response(self)
    }
}
