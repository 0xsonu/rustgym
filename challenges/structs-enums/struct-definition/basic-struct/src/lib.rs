/// A user with a username, email, and active status.
pub struct User {
    pub username: String,
    pub email: String,
    pub active: bool,
}

/// Creates a new User with the given username and email. Active defaults to true.
pub fn create_user(username: &str, email: &str) -> User {
    User {
        username: String::from(username),
        email: String::from(email),
        active: true,
    }
}
