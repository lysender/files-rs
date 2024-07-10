use uuid::Uuid;

// Can't be too long
const MAX_SLUG_LEN: usize = 30;

pub fn slugify(s: &str) -> String {
    let slug: String = s
        .trim()
        .chars()
        .filter_map(|c| match c {
            'A'..='Z' => Some(c.to_ascii_lowercase()),
            'a'..='z' | '0'..='9' | '.' | '-' | '_' => Some(c),
            ' ' => Some('-'),
            _ => None,
        })
        .collect();

    // Ensure there are no consecutive hyphens
    let mut items: Vec<char> = Vec::new();
    let mut prev_hyphen = false;

    for ch in slug.chars() {
        if ch == '-' {
            if prev_hyphen {
                continue;
            }
            prev_hyphen = true;
        } else {
            prev_hyphen = false;
        }
        items.push(ch);
    }

    let slug: String = items.iter().collect();
    if slug.len() > MAX_SLUG_LEN {
        return slug.as_str()[slug.len() - MAX_SLUG_LEN..].to_string();
    }
    slug
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
    fn test_slugify_too_long() {
        let s = "The quick brown fox jumps over - Copy(1).jpg";
        assert_eq!(slugify(s), "brown-fox-jumps-over-copy1.jpg");
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
