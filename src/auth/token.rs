use jwt_simple::prelude::*;
use serde::{Deserialize, Serialize};

use super::models::Actor;
use crate::{Error, Result};

#[derive(Debug, Deserialize, Serialize)]
struct ActorClaims {
    scope: String,
}

pub fn create_auth_token(actor: &Actor, secret: &str) -> Result<String> {
    // For some reason, there are some miliseconds of drift between the server and the client
    // Let's just add a 10 second delay to fix the issue
    let now = Clock::now_since_epoch();
    let drift = Duration::from_secs(10);

    let key = HS256Key::from_bytes(secret.as_bytes());
    let data = ActorClaims {
        scope: actor.scope.clone(),
    };

    let claims = Claims::with_custom_claims(data, Duration::from_days(7))
        .invalid_before(now - drift)
        .with_subject(&actor.id);

    match key.authenticate(claims) {
        Ok(token) => Ok(token),
        Err(e) => Err(format!("Error creating token: {}", e).into()),
    }
}

pub fn verify_auth_token(token: &str, secret: &str) -> Result<Actor> {
    let key = HS256Key::from_bytes(secret.as_bytes());
    let Ok(claims) = key.verify_token::<ActorClaims>(&token, None) else {
        return Err(Error::InvalidAuthToken);
    };

    let Some(subject) = claims.subject else {
        return Err(Error::InvalidAuthToken);
    };

    if claims.custom.scope.len() == 0 {
        return Err(Error::InvalidAuthToken);
    }

    Ok(Actor {
        id: subject,
        name: "client".to_string(),
        scope: claims.custom.scope,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jwt_token() {
        // Generate token
        let actor = Actor {
            id: "thor01".to_string(),
            name: "client".to_string(),
            scope: "auth files".to_string(),
        };
        let token = create_auth_token(&actor, "secret").unwrap();
        assert!(token.len() > 0);

        // Validate it back
        let actor = verify_auth_token(&token, "secret").unwrap();
        assert_eq!(actor.id, "thor01".to_string());
        assert_eq!(actor.name, "client".to_string());
        assert_eq!(actor.scope, "auth files".to_string());
    }

    #[test]
    fn test_expired_token() {
        let token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWJqZWN0IjoidGhvcjAxIiwic2NvcGUiOiJhdXRoIGZpbGVzIiwiZXhwIjoxNzE5MDc2MTI2fQ.ep8nXWWHS75MxoOY_yB4m0uoWgxCz1bPNvTPIgourcQ".to_string();
        let result = verify_auth_token(&token, "secret");
        assert!(result.is_err());
    }
}
