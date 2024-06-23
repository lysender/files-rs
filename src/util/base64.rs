use base64::prelude::*;

use crate::{Error, Result};

pub fn base64_encode(val: &str) -> String {
    BASE64_STANDARD.encode(val.as_bytes())
}

pub fn base64_decode(val: &str) -> Result<String> {
    let decoded = BASE64_STANDARD.decode(val.as_bytes());
    match decoded {
        Ok(val) => Ok(String::from_utf8(val).unwrap()),
        Err(_) => Err(Error::Base64DecodeError(
            "Invalid base64 string".to_string(),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base64_encode() {
        let val = "hello";
        let encoded = base64_encode(val);
        assert_eq!(encoded, "aGVsbG8=");
    }

    #[test]
    fn test_base64_decode() {
        let val = "aGVsbG8=";
        let decoded = base64_decode(val).unwrap();
        assert_eq!(decoded, "hello");
    }

    #[test]
    fn test_base64_decode_invalid() {
        let val = "aGVsbG8";
        let decoded = base64_decode(val);
        assert!(decoded.is_err());
    }
}
