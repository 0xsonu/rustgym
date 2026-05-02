use axum::{
    extract::{FromRequestParts, Request},
    http::{header, request::Parts, StatusCode},
    middleware::Next,
    response::Response,
};
use std::sync::Arc;

use crate::{config::Config, services::auth_service::Claims};

/// Authenticated user extracted from a valid JWT Bearer token.
/// Use this as an extractor in handlers that require authentication:
///
/// ```rust
/// async fn handler(user: CurrentUser) -> impl IntoResponse { ... }
/// ```
#[derive(Debug, Clone)]
pub struct CurrentUser {
    pub claims: Claims,
}

impl CurrentUser {
    /// The user's UUID (as string).
    pub fn user_id(&self) -> &str {
        &self.claims.sub
    }

    /// The user's username.
    pub fn username(&self) -> &str {
        &self.claims.username
    }

    /// The user's role.
    pub fn role(&self) -> &str {
        &self.claims.role
    }
}

/// Axum middleware that extracts and validates the JWT Bearer token,
/// then attaches the Claims to request extensions.
pub async fn auth_middleware(
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Extract config from extensions (set by the state layer)
    let config = request
        .extensions()
        .get::<Arc<Config>>()
        .cloned()
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

    let auth_header = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok());

    let token = match auth_header {
        Some(value) if value.starts_with("Bearer ") => &value[7..],
        _ => return Err(StatusCode::UNAUTHORIZED),
    };

    let claims = crate::services::auth_service::decode_access_token(token, &config.jwt_secret)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Attach claims to request extensions
    let mut request = request;
    request.extensions_mut().insert(CurrentUser {
        claims,
    });

    Ok(next.run(request).await)
}

/// Extractor for handlers that require authentication.
/// Pulls the `CurrentUser` from request extensions (set by auth_middleware).
///
/// If used without the auth middleware layer, returns 401.
#[axum::async_trait]
impl<S> FromRequestParts<S> for CurrentUser
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<CurrentUser>()
            .cloned()
            .ok_or(StatusCode::UNAUTHORIZED)
    }
}
