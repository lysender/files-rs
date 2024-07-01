use axum::{extract::State, Json};

use crate::{
    auth::{authenticate, Credentials},
    Error, Result,
};

use super::{response::JsonResponse, server::AppState};

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
