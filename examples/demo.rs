use json_data_instance_export_macros::attribute;
use json_data_instance_export::diagram;
use serde::Serialize;

#[derive(Serialize)]
#[attribute(field = "name")]
struct Company {
    name: String,
    employees: Vec<Person>,
}

#[derive(Serialize)]
#[attribute(field = "name")]
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

    // Much more Rust-like functional style!
    diagram(&company);
}
