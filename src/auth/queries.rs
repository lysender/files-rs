use validator::Validate;

use super::{create_auth_token, verify_password, Actor, AuthResponse, Credentials};

use crate::{
    clients::get_client, users::find_user_by_username, validators::flatten_errors,
    web::server::AppState, Error, Result,
};

pub async fn authenticate(state: &AppState, credentials: &Credentials) -> Result<AuthResponse> {
    if let Err(errors) = credentials.validate() {
        return Err(Error::ValidationError(flatten_errors(&errors)));
    }

    // Validate client first
    let db_pool = state.db_pool.clone();
    let client = get_client(&db_pool, &credentials.client_id).await?;
    let Some(client) = client else {
        return Err(Error::InvalidClient);
    };

    if &client.status != "active" {
        return Err(Error::InvalidClient);
    }

    // Validate user
    let user = find_user_by_username(&db_pool, &credentials.username).await?;
    let Some(user) = user else {
        return Err(Error::InvalidPassword);
    };

    if &user.client_id != &client.id {
        return Err(Error::UserNotFound);
    }

    if &user.status != "active" {
        return Err(Error::InactiveUser);
    }

    // Validate password
    let _ = verify_password(&credentials.password, &user.password)?;

    // Generate a token
    let actor = Actor {
        id: user.id.clone(),
        client_id: client.id.clone(),
        scope: "auth files".to_string(),
    };
    let token = create_auth_token(&actor, &state.config.jwt_secret)?;
    Ok(AuthResponse {
        user: user.into(),
        token,
    })
}
