fn alphanumeric(password: &str) -> bool {
    if password.is_empty() {
        return false;
    } else {
        password.chars().all(|c| c.is_alphanumeric())
    }
}