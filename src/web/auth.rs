use axum::{extract::State, middleware, routing::get, Extension, Json, Router};

use crate::{
    auth::{authenticate, Actor, Credentials},
    Error, Result,
};

use super::{middlewares::require_auth_middleware, response::JsonResponse, server::AppState};

pub async fn authenticate_handler(
    State(state): State<AppState>,
    payload: Option<Json<Credentials>>,
) -> Result<JsonResponse> {
    let Some(credentials) = payload else {
        return Err(Error::BadRequest("Invalid credentials payload".into()));
    };

    let res = authenticate(&state, &credentials).await?;
    Ok(JsonResponse::new(serde_json::to_string(&res).unwrap()))
}

pub fn user_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(profile_handler))
        .route("/permissions", get(user_permissions))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            require_auth_middleware,
        ))
        .with_state(state)
}

pub async fn profile_handler(Extension(actor): Extension<Actor>) -> Result<JsonResponse> {
    Ok(JsonResponse::new(
        serde_json::to_string(&actor.user).unwrap(),
    ))
}

pub async fn user_permissions(Extension(actor): Extension<Actor>) -> Result<JsonResponse> {
    let mut items: Vec<String> = actor
        .get_permissions()
        .iter()
        .map(|p| p.to_string())
        .collect();
    items.sort();
    Ok(JsonResponse::new(serde_json::to_string(&items).unwrap()))
}
