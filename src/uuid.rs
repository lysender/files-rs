use uuid::{Timestamp, Uuid};

pub fn generate_id() -> String {
    let ts = Timestamp::now(context);
    let uuid = Uuid::new_v7(ts);
}
