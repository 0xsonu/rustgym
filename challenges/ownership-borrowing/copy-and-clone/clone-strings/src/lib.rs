/// Creates a clone of the input string and returns both the original and the clone.
pub fn clone_and_modify(original: String) -> (String, String) {
    let cloned = original.clone();
    (original, cloned)
}

/// Creates a greeting by cloning the name for use in two different messages.
pub fn double_greeting(name: String) -> (String, String) {
    let formal = format!("Hello, {}!", name.clone());
    let casual = format!("Hey, {}!", name);
    (formal, casual)
}
