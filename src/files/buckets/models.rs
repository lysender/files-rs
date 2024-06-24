use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Queryable, Selectable, Insertable, Serialize)]
#[diesel(table_name = crate::schema::buckets)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Bucket {
    pub id: String,
    pub client_id: String,
    pub name: String,
    pub label: String,
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct NewBucket {
    #[validate(length(min = 1, max = 50))]
    #[validate(custom(function = "crate::validators::sluggable"))]
    pub name: String,

    #[validate(length(min = 1, max = 100))]
    pub label: String,
}

#[derive(Debug, Clone, Deserialize, Validate, AsChangeset)]
#[diesel(table_name = crate::schema::buckets)]
pub struct UpdateBucket {
    #[validate(length(min = 1, max = 100))]
    pub label: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_bucket() {
        let data = NewBucket {
            name: "hello-world".to_string(),
            label: "Hello World".to_string(),
        };
        assert!(data.validate().is_ok());

        let data = NewBucket {
            name: "hello_world".to_string(),
            label: "Hello World".to_string(),
        };
        assert!(data.validate().is_err());

        let data = NewBucket {
            name: "".to_string(),
            label: "Hello World".to_string(),
        };
        assert!(data.validate().is_err());
    }
}
