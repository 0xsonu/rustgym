/// Send a verification email to the user (stub — logs with tracing).
pub async fn send_verification_email(email: &str, token: &str) {
    tracing::info!(
        email = %email,
        token = %token,
        "Sending verification email (stub)"
    );
}

/// Send a password reset email to the user (stub — logs with tracing).
pub async fn send_password_reset_email(email: &str, token: &str) {
    tracing::info!(
        email = %email,
        token = %token,
        "Sending password reset email (stub)"
    );
}
