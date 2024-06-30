use core::result::Result;
use validator::ValidationError;

use super::sluggable;

pub fn csvname(value: &str) -> Result<(), ValidationError> {
    if value.len() == 0 {
        return Err(ValidationError::new("csvname"));
    }

    let valid = value.split(",").all(|chunk| {
        if chunk.len() == 0 {
            return false;
        }
        sluggable(chunk).is_ok()
    });

    match valid {
        true => Ok(()),
        false => Err(ValidationError::new("csvname")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csvname() {
        assert!(csvname("hello-world").is_ok());
        assert!(csvname("Hello-World-123").is_ok());
        assert!(csvname("hello-world,other-world-bruh").is_ok());
        assert!(csvname("Hello-World-123,this,that").is_ok());
        assert!(csvname(",").is_err());
        assert!(csvname(",,").is_err());
        assert!(csvname("").is_err());
    }
}
