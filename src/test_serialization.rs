use crate::export::export_json_instance;
use serde::Serialize;

#[derive(Serialize)]
struct TestStruct {
    name: String,
    items: Vec<String>,
}

#[test]
fn test_current_serialization() {
    let test_data = TestStruct {
        name: "Test".to_string(),
        items: vec!["item1".to_string(), "item2".to_string(), "item3".to_string()],
    };

    let json_instance = export_json_instance(&test_data);
    println!("Atoms:");
    for atom in &json_instance.atoms {
        println!("  {:?}", atom);
    }
    
    println!("\nRelations:");
    for relation in &json_instance.relations {
        println!("  {:?}", relation);
    }
}