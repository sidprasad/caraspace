use json_data_instance_export::export_json_instance;
use serde::Serialize;

#[derive(Serialize)]
enum Color {
    Red,
    Black,
    Blue,
}

#[derive(Serialize)]
struct TestStruct {
    color: Color,
    value: u32,
}

fn main() {
    let test = TestStruct {
        color: Color::Red,
        value: 42,
    };
    
    let instance = export_json_instance(&test);
    
    println!("Atoms:");
    for atom in &instance.atoms {
        println!("  - id: {}, type: {}, label: {}", atom.id, atom.r#type, atom.label);
    }
    
    println!("\nRelations:");
    for relation in &instance.relations {
        println!("  - {}: {} tuples", relation.name, relation.tuples.len());
        for tuple in &relation.tuples {
            println!("    types: {:?}", tuple.types);
        }
    }
}
