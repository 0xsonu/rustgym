/// Takes ownership of a String and returns a new one with a prefix.
pub fn move_and_return(input: String) -> String {
    format!("Moved: {}", input)
}
