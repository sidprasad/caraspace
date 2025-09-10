use json_data_instance_export::{diagram, CndDecorators};
use serde::Serialize;

/// Company type with compile-time decorator collection.
/// When you call `Company::decorators()`, it automatically includes
/// decorators from nested Person types too!
#[derive(Serialize, CndDecorators)]
#[attribute(field = "name")]
#[flag(name="hideDisconnected")]
struct Company {
    name: String,
    employees: Vec<Person>,
}

/// Person type with decorators that will be automatically
/// included when processing any type that contains Person fields.
#[derive(Serialize, CndDecorators)]
#[attribute(field = "age")]
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let company = Company {
        name: "Acme Corp".to_string(),
        employees: vec![
            Person { name: "Alice".to_string(), age: 30 },
            Person { name: "Bob".to_string(), age: 25 },
        ],
    };

    
    // This call to diagram() will automatically collect decorators from:
    // 1. Company type (name attribute, hideDisconnected flag)
    // 2. Person type (age attribute) - discovered automatically at compile time!
    diagram(&company);
}
