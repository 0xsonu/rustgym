/// Extracts the domain from an email address.
pub fn extract_domain(email: &str) -> Option<&str> {
    let at_pos = email.find('@')?;
    let domain = &email[at_pos + 1..];
    if domain.is_empty() {
        None
    } else {
        Some(domain)
    }
}
