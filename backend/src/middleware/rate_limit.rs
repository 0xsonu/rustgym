use axum::{
    extract::{ConnectInfo, Request},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use deadpool_redis::Pool;
use std::net::SocketAddr;

/// Rate limit scope determines the request budget.
#[derive(Debug, Clone, Copy)]
pub enum RateLimitScope {
    /// Auth routes: 20 requests per minute per IP.
    Auth,
    /// Global routes: 200 requests per minute per IP.
    Global,
}

impl RateLimitScope {
    fn max_requests(&self) -> u64 {
        match self {
            RateLimitScope::Auth => 20,
            RateLimitScope::Global => 200,
        }
    }

    fn key_prefix(&self) -> &'static str {
        match self {
            RateLimitScope::Auth => "rate:auth",
            RateLimitScope::Global => "rate:ip",
        }
    }
}

/// Create a rate limiting middleware layer for the given scope.
/// Uses Redis sliding window counters with graceful degradation.
pub fn rate_limit_middleware(
    scope: RateLimitScope,
) -> impl Fn(Request, Next) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Response, StatusCode>> + Send>> + Clone
{
    move |request: Request, next: Next| {
        let scope = scope;
        Box::pin(async move {
            rate_limit_inner(scope, request, next).await
        })
    }
}

async fn rate_limit_inner(
    scope: RateLimitScope,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Try to get the Redis pool from extensions
    let redis_pool = request.extensions().get::<Option<Pool>>().cloned().flatten();

    // Extract client IP
    let ip = extract_client_ip(&request);

    if let Some(pool) = redis_pool {
        match check_rate_limit(&pool, scope, &ip).await {
            Ok(allowed) => {
                if !allowed {
                    return Err(StatusCode::TOO_MANY_REQUESTS);
                }
            }
            Err(e) => {
                // Graceful degradation: log warning and allow request if Redis is unavailable
                tracing::warn!("Rate limiting unavailable (Redis error): {}. Allowing request.", e);
            }
        }
    } else {
        tracing::warn!("Rate limiting unavailable (no Redis pool). Allowing request.");
    }

    Ok(next.run(request).await)
}

/// Extract client IP from the request.
/// Checks X-Forwarded-For header first, then falls back to ConnectInfo.
fn extract_client_ip(request: &Request) -> String {
    // Check X-Forwarded-For header
    if let Some(forwarded_for) = request
        .headers()
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
    {
        if let Some(first_ip) = forwarded_for.split(',').next() {
            return first_ip.trim().to_string();
        }
    }

    // Fall back to ConnectInfo
    if let Some(connect_info) = request.extensions().get::<ConnectInfo<SocketAddr>>() {
        return connect_info.0.ip().to_string();
    }

    "unknown".to_string()
}

/// Check and increment the rate limit counter using Redis sliding window.
/// Returns Ok(true) if the request is allowed, Ok(false) if rate limited.
async fn check_rate_limit(
    pool: &Pool,
    scope: RateLimitScope,
    ip: &str,
) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
    use deadpool_redis::redis::AsyncCommands;

    let now = chrono::Utc::now();
    let window_minute = now.timestamp() / 60;
    let key = format!("{}:{}:{}", scope.key_prefix(), ip, window_minute);

    let mut conn = pool.get().await?;

    // Increment counter and set expiry
    let count: u64 = conn.incr(&key, 1u64).await?;

    // Set TTL on first request in this window (when count == 1)
    if count == 1 {
        let _: () = conn.expire(&key, 60).await?;
    }

    Ok(count <= scope.max_requests())
}
