pub fn replace_extension(filename: &str, ext: &str) -> String {
    let mut parts: Vec<&str> = filename.split('.').collect();
    if parts.len() == 1 {
        return format!("{}.{}", filename, ext);
    }
    parts.pop();
    parts.push(ext);
    parts.join(".")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extension() {
        let filename = "hello.jpg";
        let ext = "png";
        assert_eq!(replace_extension(filename, ext), "hello.png");
    }

    #[test]
    fn test_extension_multi_dot() {
        let filename = "hi.hello.jpg";
        let ext = "png";
        assert_eq!(replace_extension(filename, ext), "hi.hello.png");
    }

    #[test]
    fn test_no_extension() {
        let filename = "hello";
        let ext = "png";
        assert_eq!(replace_extension(filename, ext), "hello.png");
    }
}
