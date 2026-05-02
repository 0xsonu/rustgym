/// A street address.
/// TODO: Define with pub fields: street (String), city (String), zip (String).
pub struct Address {
    // TODO: Add fields
}

/// A person with a name and address.
/// TODO: Define with pub fields: name (String), age (u32), address (Address).
pub struct Person {
    // TODO: Add fields
}

/// Creates a new Person with the given details.
/// TODO: Create a Person with a nested Address.
pub fn create_person(name: &str, age: u32, street: &str, city: &str, zip: &str) -> Person {
    todo!()
}

/// Returns a formatted string: "Name lives in City".
/// TODO: Access the nested address field to get the city.
pub fn describe_person(person: &Person) -> String {
    todo!()
}
