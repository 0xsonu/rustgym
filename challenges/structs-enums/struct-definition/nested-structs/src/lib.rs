/// A street address.
pub struct Address {
    pub street: String,
    pub city: String,
    pub zip: String,
}

/// A person with a name and address.
pub struct Person {
    pub name: String,
    pub age: u32,
    pub address: Address,
}

/// Creates a new Person with the given details.
pub fn create_person(name: &str, age: u32, street: &str, city: &str, zip: &str) -> Person {
    Person {
        name: String::from(name),
        age,
        address: Address {
            street: String::from(street),
            city: String::from(city),
            zip: String::from(zip),
        },
    }
}

/// Returns a formatted string: "Name lives in City".
pub fn describe_person(person: &Person) -> String {
    format!("{} lives in {}", person.name, person.address.city)
}
