use json_data_instance_export::{diagram, CndDecorators, register_cnd_types};
use serde::Serialize;

#[derive(Serialize, CndDecorators)]
#[attribute(field = "name")]
#[flag(name="hideDisconnected")]
struct Company {
    name: String,
    employees: Vec<Person>,
}

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

    // Much nicer API - just list the types that have CnD decorators
    register_cnd_types!(Company, Person);

    diagram(&company);
}