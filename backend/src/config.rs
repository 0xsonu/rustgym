use std::env;

/// Application configuration loaded from environment variables.
#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub redis_url: String,
    pub jwt_secret: String,
    pub frontend_url: String,
    pub s3_bucket: String,
    pub s3_region: String,
    pub s3_endpoint: Option<String>,
    pub runner_url: String,
    pub server_port: u16,
}

impl Config {
    /// Load configuration from environment variables.
    ///
    /// # Panics
    /// Panics if required environment variables are not set.
    pub fn from_env() -> Self {
        Self {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            redis_url: env::var("REDIS_URL").expect("REDIS_URL must be set"),
            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
            frontend_url: env::var("FRONTEND_URL").expect("FRONTEND_URL must be set"),
            s3_bucket: env::var("S3_BUCKET").unwrap_or_else(|_| "rustgym".to_string()),
            s3_region: env::var("S3_REGION").unwrap_or_else(|_| "us-east-1".to_string()),
            s3_endpoint: env::var("S3_ENDPOINT").ok(),
            runner_url: env::var("RUNNER_URL")
                .unwrap_or_else(|_| "http://localhost:3001".to_string()),
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .expect("SERVER_PORT must be a valid u16"),
        }
    }
}
