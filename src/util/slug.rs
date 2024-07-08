use uuid::Uuid;

pub fn slugify(s: &str) -> String {
    s.trim()
        .chars()
        .filter_map(|c| match c {
            'A'..='Z' => Some(c.to_ascii_lowercase()),
            'a'..='z' | '0'..='9' | '.' | '-' | '_' => Some(c),
            ' ' => Some('-'),
            _ => None,
        })
        .collect()
}

pub fn slugify_prefixed(s: &str) -> String {
    let id = Uuid::now_v7().to_string();
    let prefix = id
        .split('-')
        .last()
        .expect("Expected the last part of uuid string");
    let slug = slugify(s);
    format!("{}-{}", prefix, slug)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slugify() {
        let s = "Hello, World!";
        assert_eq!(slugify(s), "hello-world");
    }

    #[test]
    fn test_slugify_prefixed() {
        let s = "Hello, World!";
        let slug = slugify_prefixed(s);
        let parts: Vec<&str> = slug.split('-').collect();
        assert_eq!(parts.len(), 3);
        assert_eq!(parts[0].len(), 12);
        assert_eq!(parts[1], "hello");
        assert_eq!(parts[2], "world");
    }
}
