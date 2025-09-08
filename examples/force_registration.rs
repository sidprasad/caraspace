use json_data_instance_export::{diagram, CndDecorators};
use json_data_instance_export::cnd_annotations::HasCndDecorators;
use serde::Serialize;

#[derive(Serialize, CndDecorators)]
#[attribute(field = "name")]
#[flag(name="hideDisconnected")]
struct Company {
    name: String,
    employees: Vec<Person>,
}

impl Company {
    fn new(name: String, employees: Vec<Person>) -> Self {
        // Registration happens automatically when the type is constructed
        let _ = Self::decorators();
        Self { name, employees }
    }
}

#[derive(Serialize, CndDecorators)]
#[attribute(field = "name")]
#[flag(name="hideDisconnected")]
struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: String, age: u32) -> Self {
        // Registration happens automatically when the type is constructed
        let _ = Self::decorators();
        Self { name, age }
    }
}

fn main() {
    let company = Company::new(
        "Acme Corp".to_string(),
        vec![
            Person::new("Alice".to_string(), 30),
            Person::new("Bob".to_string(), 25),
        ],
    );

    // Clean - no manual registration required!
    diagram(&company);
}