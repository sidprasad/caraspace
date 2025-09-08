use json_data_instance_export::{diagram, CndDecorators, collect_cnd_spec_for_test};
use json_data_instance_export::cnd_annotations::HasCndDecorators;
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

    println!("Before any registration:");
    let spec1 = collect_cnd_spec_for_test(&company);
    println!("Spec 1 (Company only):\n{}", spec1);
    
    println!("\nAfter forcing Person registration:");
    let _ = Person::decorators(); // Force registration
    let spec2 = collect_cnd_spec_for_test(&company);
    println!("Spec 2 (with Person):\n{}", spec2);
}