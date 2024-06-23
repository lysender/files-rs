use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct Actor {
    pub id: String,
    pub name: String,
    pub scope: String,
}

#[derive(Deserialize, Serialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthToken {
    pub token: String,
}
