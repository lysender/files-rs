use uuid::Uuid;

pub fn generate_id() -> String {
    Uuid::now_v7().as_simple().to_string()
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_generate_id() {
        // Should be a 32-character uuid string
        let id = generate_id();
        assert_eq!(id.len(), 32);

        // Can be parsed back as uuid
        let parsed = Uuid::from_str(id.as_str());
        assert!(parsed.is_ok());
    }
}
