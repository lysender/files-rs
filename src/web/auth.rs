use axum::{extract::State, Json};

use crate::{
    auth::{create_auth_token, verify_password, Actor, AuthResponse, Credentials},
    clients::get_client,
    users::find_user_by_username,
    Error, Result,
};

use super::{response::JsonResponse, server::AppState};

pub async fn authenticate_handler(
    State(state): State<AppState>,
    payload: Option<Json<Credentials>>,
) -> Result<JsonResponse> {
    let Some(credentials) = payload else {
        return Err(Error::InvalidPassword);
    };

    let db_pool = state.db_pool.clone();

    // Validate client first
    let client = get_client(&db_pool, &credentials.client_id).await?;
    let Some(client) = client else {
        return Err(Error::InvalidClient);
    };

    if &client.status != "active" {
        return Err(Error::InvalidClient);
    }

    // Validate user
    let user =
        find_user_by_username(&db_pool, &credentials.client_id, &credentials.username).await?;
    let Some(user) = user else {
        return Err(Error::InvalidPassword);
    };

    if &user.status != "active" {
        return Err(Error::InactiveUser);
    }

    // Validate password
    let _ = verify_password(&credentials.password, &user.password)?;

    // Generate a token
    let actor = Actor {
        id: user.id.clone(),
        client_id: state.config.client_id.clone(),
        scope: "auth files".to_string(),
    };
    let token = create_auth_token(&actor, &state.config.jwt_secret)?;
    let token = AuthResponse { user, token };

    Ok(JsonResponse::new(serde_json::to_string(&token).unwrap()))
}
