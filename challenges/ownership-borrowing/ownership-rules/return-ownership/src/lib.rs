/// Creates a greeting string and returns ownership to the caller.
pub fn give_ownership() -> String {
    String::from("yours now")
}

/// Takes a string, processes it, and gives back both the original and a new value.
pub fn take_and_give_back(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
