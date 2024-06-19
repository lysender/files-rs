use derive_more::From;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    #[from]
    AnyError(String),

    ValidationError(String),
}

// Allow string slices to be converted to Error
impl From<&str> for Error {
    fn from(val: &str) -> Self {
        Self::AnyError(val.to_string())
    }
}

// Allow AnyError to be displayed
impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::AnyError(val) => write!(f, "{}", val),
            Self::ValidationError(val) => write!(f, "{}", val),
        }
    }
}
