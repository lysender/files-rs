use core::result::Result;
use validator::{ValidationError, ValidationErrors};

pub fn sluggable_string(value: &str) -> Result<(), ValidationError> {
    if value.len() == 0 {
        return Err(ValidationError::new("sluggable_string"));
    }
    let valid = value.chars().all(|c| c.is_alphanumeric() || c == '-');
    match valid {
        true => Ok(()),
        false => Err(ValidationError::new("sluggable_string")),
    }
}

pub fn flatten_errors(errors: &ValidationErrors) -> String {
    // Collect field keys first
    let mut fields: Vec<String> = errors
        .field_errors()
        .keys()
        .map(|k| k.to_string())
        .collect();

    // Ensure error fields are sorted ascending
    fields.sort();

    let field_errors = errors.field_errors();
    let messages: Vec<String> = fields
        .into_iter()
        .map(|k| {
            let Some(item) = field_errors.get(k.as_str()) else {
                return format!("{}: invalid", k);
            };
            let msgs: Vec<String> = item.iter().map(|i| error_to_string(i)).collect();
            format!("{}: {}", k, msgs.join(", "))
        })
        .collect();

    messages.join(", ")
}

fn error_to_string(error: &ValidationError) -> String {
    // Provide partial error code conversion
    match error.code.as_ref() {
        "email" => "invalid email".to_string(),
        "url" => "invalid url".to_string(),
        "length" => match (error.params.get("min"), error.params.get("max")) {
            (Some(min), Some(max)) => format!("must be between {} and {} characters", min, max),
            (Some(min), None) => format!("must be at least {} characters", min),
            (None, Some(max)) => format!("must be at most {} characters", max),
            _ => "invalid length".to_string(),
        },
        "required" => "required".to_string(),
        "sluggable_string" => "must be composed of alpha-numeric characters or dashes".to_string(),
        _ => "invalid".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use validator::Validate;

    #[derive(Debug, Clone, Validate)]
    pub struct TestStruct {
        #[validate(length(min = 1, max = 50))]
        pub name: String,

        #[validate(length(min = 1, max = 100))]
        pub label: String,
    }

    #[test]
    fn test_sluggable_string() {
        assert!(sluggable_string("hello-world").is_ok());
        assert!(sluggable_string("Hello-World-123").is_ok());
        assert!(sluggable_string("hello_world").is_err());
        assert!(sluggable_string("hello world").is_err());
        assert!(sluggable_string("").is_err());
    }

    #[test]
    fn test_flatten_errors() {
        let data = TestStruct {
            name: "".to_string(),
            label: "".to_string(),
        };
        let errors = data.validate().unwrap_err();
        let flattened = flatten_errors(&errors);
        assert_eq!(flattened, "label: must be between 1 and 100 characters, name: must be between 1 and 50 characters");
    }
}
