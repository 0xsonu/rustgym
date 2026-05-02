/// Returns personal info as a tuple of (name, age, is_student).
pub fn personal_info() -> (String, u32, bool) {
    let name = String::from("Alice");
    let age = 30;
    let is_student = true;
    (name, age, is_student)
}
