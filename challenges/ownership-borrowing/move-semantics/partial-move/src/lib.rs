/// A person with name and email.
pub struct Person {
    pub name: String,
    pub email: String,
}

/// Extracts the name from a Person, consuming the struct.
pub fn extract_name(person: Person) -> String {
    person.name
}

/// Splits a person into their name and email.
pub fn split_person(person: Person) -> (String, String) {
    (person.name, person.email)
}
