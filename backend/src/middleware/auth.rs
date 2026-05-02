use axum::{
    extract::{FromRef, FromRequestParts, Request},
    http::{header, request::Parts, StatusCode},
    middleware::Next,
    response::Response,
};

use crate::{services::auth_service::Claims, AppState};

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
/// then attaches the CurrentUser to request extensions.
pub async fn auth_middleware(
    state: axum::extract::State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .map(|s| s.to_string());

    let token = match auth_header.as_deref() {
        Some(value) if value.starts_with("Bearer ") => &value[7..],
        _ => return Err(StatusCode::UNAUTHORIZED),
    };

    let claims =
        crate::services::auth_service::decode_access_token(token, &state.config.jwt_secret)
            .map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Attach claims to request extensions
    request.extensions_mut().insert(CurrentUser { claims });

    Ok(next.run(request).await)
}

/// Extractor for handlers that require authentication.
/// Pulls the `CurrentUser` from request extensions (set by auth_middleware).
///
/// If used without the auth middleware layer, returns 401.
impl<S> FromRequestParts<S> for CurrentUser
where
    S: Send + Sync,
    AppState: FromRef<S>,
{
    type Rejection = StatusCode;

    fn from_request_parts<'life0, 'life1, 'async_trait>(
        parts: &'life0 mut Parts,
        _state: &'life1 S,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<Self, Self::Rejection>> + Send + 'async_trait>,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            parts
                .extensions
                .get::<CurrentUser>()
                .cloned()
                .ok_or(StatusCode::UNAUTHORIZED)
        })
    }
}
