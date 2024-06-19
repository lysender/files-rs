use core::result::Result;

use validator::ValidationError;

pub fn sluggable_string(value: &str) -> Result<(), ValidationError> {
    if value.len() == 0 {
        return Err(ValidationError::new("empty_sluggable_string"));
    }
    let valid = value.chars().all(|c| c.is_alphanumeric() || c == '-');
    match valid {
        true => Ok(()),
        false => Err(ValidationError::new("invalid_sluggable_string")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sluggable_string() {
        assert!(sluggable_string("hello-world").is_ok());
        assert!(sluggable_string("Hello-World-123").is_ok());
        assert!(sluggable_string("hello_world").is_err());
        assert!(sluggable_string("hello world").is_err());
        assert!(sluggable_string("").is_err());
    }
}
