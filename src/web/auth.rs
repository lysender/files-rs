use axum::{extract::State, Json};

use crate::{
    auth::{create_auth_token, extract_admin_hash, verify_password, Actor, AuthToken, Credentials},
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

    // Validate usename/password against the admin hash
    let admin_hash = extract_admin_hash()?;
    let combined = format!("{}:{}", credentials.username, credentials.password);
    let _ = verify_password(&combined, &admin_hash)?;

    // Generate a token
    let actor = Actor {
        id: state.config.client_id.clone(),
        name: "client".to_string(),
        scope: "auth files".to_string(),
    };
    let token = create_auth_token(&actor, &state.config.jwt_secret)?;
    let token = AuthToken { token };

    Ok(JsonResponse::new(serde_json::to_string(&token).unwrap()))
}
