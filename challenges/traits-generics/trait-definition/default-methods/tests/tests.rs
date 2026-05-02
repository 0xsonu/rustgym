use default_methods::{Introduce, Student, Teacher};

#[test]
fn test_student_name() {
    let s = Student { name: String::from("Alice") };
    assert_eq!(s.name(), "Alice");
}

#[test]
fn test_student_default_introduce() {
    let s = Student { name: String::from("Bob") };
    assert_eq!(s.introduce(), "Hi, my name is Bob.");
}

#[test]
fn test_teacher_override() {
    let t = Teacher {
        name: String::from("Dr. Smith"),
        subject: String::from("Rust"),
    };
    assert_eq!(t.introduce(), "I'm Dr. Smith, and I teach Rust.");
}

#[test]
fn test_teacher_name() {
    let t = Teacher {
        name: String::from("Prof. Jones"),
        subject: String::from("Math"),
    };
    assert_eq!(t.name(), "Prof. Jones");
}
