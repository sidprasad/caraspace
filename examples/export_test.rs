use json_data_instance_export::{export_json_instance};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
struct Company {
    name: String,
    employees: Vec<Person>,
    metadata: HashMap<String, String>,
}

#[derive(Serialize)]
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let mut metadata = HashMap::new();
    metadata.insert("industry".to_string(), "Tech".to_string());
    metadata.insert("founded".to_string(), "2020".to_string());

    let company = Company {
        name: "Acme Corp".to_string(),
        employees: vec![
            Person { name: "Alice".to_string(), age: 30 },
            Person { name: "Bob".to_string(), age: 25 },
        ],
        metadata,
    };

    let json_instance = export_json_instance(&company);
    println!("=== ATOMS ===");
    for atom in &json_instance.atoms {
        println!("{}: {} ({})", atom.id, atom.label, atom.r#type);
    }
    
    println!("\n=== RELATIONS ===");
    for relation in &json_instance.relations {
        println!("{}:", relation.name);
        for tuple in &relation.tuples {
            println!("  {:?} -> {:?}", tuple.atoms, tuple.types);
        }
    }
}
