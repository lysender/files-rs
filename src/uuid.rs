use uuid::Uuid;

pub fn generate_id() -> String {
    Uuid::now_v7().as_simple().to_string()
}
