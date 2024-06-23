pub fn valid_username_format(username: &str) -> bool {
    if username.len() < 4 {
        return false;
    }
    username.chars().all(|ch| ch.is_alphanumeric())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_username_format1() {
        let username = "Admin123";
        assert_eq!(valid_username_format(username), true);
    }

    #[test]
    fn test_valid_username_format2() {
        let username = "luffy";
        assert_eq!(valid_username_format(username), true);
    }

    #[test]
    fn test_invalid_username_format() {
        let username = "Awesome-Admin";
        assert_eq!(valid_username_format(username), false);
    }

    #[test]
    fn test_invalid_username_format_special_chars() {
        let username = "admin@123";
        assert_eq!(valid_username_format(username), false);
    }

    #[test]
    fn test_invalid_username_format_length() {
        let username = "Adm";
        assert_eq!(valid_username_format(username), false);
    }
}
