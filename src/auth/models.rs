use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    roles::{has_permissions, Permission, Role},
    users::UserDto,
};

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

impl Actor {
    pub fn has_auth_scope(&self) -> bool {
        self.scope.contains("auth")
    }

    pub fn has_files_scope(&self) -> bool {
        self.scope.contains("files")
    }

    pub fn has_scope(&self, scope: &str) -> bool {
        self.scope.contains(scope)
    }

    pub fn has_roles(&self, roles: &Vec<Role>) -> bool {
        roles.iter().all(|role| self.user.roles.contains(role))
    }

    pub fn has_permissions(&self, permissions: &Vec<Permission>) -> bool {
        has_permissions(&self.user.roles, &permissions)
    }
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
