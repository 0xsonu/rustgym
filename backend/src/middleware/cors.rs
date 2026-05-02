use axum::http::{HeaderValue, Method};
use tower_http::cors::CorsLayer;

use crate::config::Config;

/// Build a CORS layer configured to allow only the FRONTEND_URL origin.
pub fn cors_layer(config: &Config) -> CorsLayer {
    let origin = config
        .frontend_url
        .parse::<HeaderValue>()
        .expect("FRONTEND_URL must be a valid header value");

    CorsLayer::new()
        .allow_origin(origin)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([
            axum::http::header::CONTENT_TYPE,
            axum::http::header::AUTHORIZATION,
        ])
        .allow_credentials(true)
}
