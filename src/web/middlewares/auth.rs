use axum::{
    body::Body,
    extract::{Request, State},
    http::header,
    middleware::Next,
    response::Response,
    Extension,
};

use crate::{
    auth::{models::Actor, token::verify_auth_token},
    web::{response::to_error_response, server::AppState},
    Error,
};

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Response<Body> {
    // Middleware to extract actor information from the request
    // Do not enforce authentication here, just extract the actor information
    let auth_header = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let mut actor: Option<Actor> = None;

    if let Some(auth_header) = auth_header {
        // At this point, authentication must be verified
        if !auth_header.starts_with("Bearer ") {
            return to_error_response(Error::InvalidAuthToken);
        }
        let token = auth_header.replace("Bearer ", "");
        let Ok(data) = verify_auth_token(&token, &state.config.jwt_secret) else {
            return to_error_response(Error::InvalidAuthToken);
        };
        if &data.id != &state.config.client_id {
            return to_error_response(Error::InvalidClient);
        }
        actor = Some(data);
    }

    if let Some(actor) = actor {
        // Forward to the next middleware/handler passing the actor information
        request.extensions_mut().insert(actor);
    }

    let response = next.run(request).await;
    response
}

pub async fn require_auth_middleware(
    actor: Option<Extension<Actor>>,
    request: Request,
    next: Next,
) -> Response<Body> {
    if actor.is_none() {
        return to_error_response(Error::NoAuthToken);
    }

    next.run(request).await
}
