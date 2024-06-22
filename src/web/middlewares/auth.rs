use axum::{
    body::Body,
    extract::{Request, State},
    http::header,
    middleware::Next,
    response::Response,
};

use crate::{
    auth::token::verify_auth_token,
    web::{response::to_error_response, server::AppState},
    Error,
};

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Response<Body> {
    let auth_header = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let Some(auth_header) = auth_header else {
        return to_error_response(Error::NoAuthToken);
    };

    if !auth_header.starts_with("Bearer ") {
        return to_error_response(Error::InvalidAuthToken);
    }

    // Validate the token
    let token = auth_header.replace("Bearer ", "");
    let Ok(actor) = verify_auth_token(&token, &state.config.jwt_secret) else {
        return to_error_response(Error::InvalidAuthToken);
    };

    // Actor id must be valid
    if &actor.id != &state.config.client_id {
        return to_error_response(Error::InvalidClient);
    }

    // Forward to the next middleware/handler passing the actor information
    request.extensions_mut().insert(actor);
    let response = next.run(request).await;
    response
}
