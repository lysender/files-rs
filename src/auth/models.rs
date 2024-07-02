use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::users::UserDto;

#[derive(Clone)]
pub struct ActorPayload {
    pub id: String,
    pub client_id: String,
    pub scope: String,
}

#[derive(Clone)]
pub struct Actor {
    pub id: String,
    pub client_id: String,
    pub scope: String,
    pub user: UserDto,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct Credentials {
    #[validate(length(equal = 32))]
    pub client_id: String,

    #[validate(length(min = 1, max = 30))]
    pub username: String,

    #[validate(length(min = 8, max = 100))]
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthToken {
    pub token: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub user: UserDto,
    pub token: String,
}
