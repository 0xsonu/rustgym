use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::error::AppError;

// ─── Password Hashing ────────────────────────────────────────────────────────

/// Hash a password using Argon2 with default parameters.
pub fn hash_password(password: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| AppError::Internal(format!("Password hashing failed: {}", e)))?;
    Ok(hash.to_string())
}

/// Verify a password against a stored Argon2 hash.
pub fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
    let parsed_hash = PasswordHash::new(hash)
        .map_err(|e| AppError::Internal(format!("Invalid password hash format: {}", e)))?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

// ─── JWT Access Tokens ───────────────────────────────────────────────────────

/// JWT claims for access tokens.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    /// Subject — user UUID as string
    pub sub: String,
    /// Username
    pub username: String,
    /// User role (student, mentor, admin)
    pub role: String,
    /// Expiration time (Unix timestamp)
    pub exp: i64,
    /// Issued at (Unix timestamp)
    pub iat: i64,
}

/// Generate a JWT access token with 15-minute expiry.
pub fn generate_access_token(
    user_id: Uuid,
    username: &str,
    role: &str,
    secret: &str,
) -> Result<String, AppError> {
    let now = Utc::now();
    let exp = now + chrono::Duration::minutes(15);

    let claims = Claims {
        sub: user_id.to_string(),
        username: username.to_string(),
        role: role.to_string(),
        exp: exp.timestamp(),
        iat: now.timestamp(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| AppError::Internal(format!("Token generation failed: {}", e)))
}

/// Decode and validate a JWT access token.
pub fn decode_access_token(token: &str, secret: &str) -> Result<Claims, AppError> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|e| AppError::Unauthorized(format!("Invalid token: {}", e)))?;

    Ok(token_data.claims)
}

// ─── Refresh Tokens ──────────────────────────────────────────────────────────

/// Generate a random 32-byte refresh token, returned as base64.
pub fn generate_refresh_token() -> String {
    let mut bytes = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut bytes);
    BASE64.encode(bytes)
}

/// Hash a refresh token using SHA-256 for database storage.
pub fn hash_refresh_token(token: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    hex::encode(hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_and_verify_password() {
        let password = "secure_password_123";
        let hash = hash_password(password).unwrap();

        assert!(verify_password(password, &hash).unwrap());
        assert!(!verify_password("wrong_password", &hash).unwrap());
    }

    #[test]
    fn test_generate_and_decode_access_token() {
        let user_id = Uuid::new_v4();
        let username = "testuser";
        let role = "student";
        let secret = "test_secret_key_for_jwt";

        let token = generate_access_token(user_id, username, role, secret).unwrap();
        let claims = decode_access_token(&token, secret).unwrap();

        assert_eq!(claims.sub, user_id.to_string());
        assert_eq!(claims.username, username);
        assert_eq!(claims.role, role);
    }

    #[test]
    fn test_decode_invalid_token() {
        let result = decode_access_token("invalid.token.here", "secret");
        assert!(result.is_err());
    }

    #[test]
    fn test_generate_refresh_token() {
        let token1 = generate_refresh_token();
        let token2 = generate_refresh_token();

        // Tokens should be non-empty and unique
        assert!(!token1.is_empty());
        assert_ne!(token1, token2);

        // Should be valid base64
        assert!(BASE64.decode(&token1).is_ok());
    }

    #[test]
    fn test_hash_refresh_token() {
        let token = "some_refresh_token_value";
        let hash1 = hash_refresh_token(token);
        let hash2 = hash_refresh_token(token);

        // Same input should produce same hash
        assert_eq!(hash1, hash2);

        // Hash should be 64 hex chars (SHA-256 = 32 bytes = 64 hex)
        assert_eq!(hash1.len(), 64);

        // Different input should produce different hash
        let hash3 = hash_refresh_token("different_token");
        assert_ne!(hash1, hash3);
    }
}
