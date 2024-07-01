use serde::{Deserialize, Serialize};

use crate::users::User;

#[derive(Clone)]
pub struct Actor {
    pub id: String,
    pub client_id: String,
    pub scope: String,
}

#[derive(Deserialize, Serialize)]
pub struct Credentials {
    pub client_id: String,
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthToken {
    pub token: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub user: User,
    pub token: String,
}
