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

#[test]
fn test_list_positioning() {
    let test_data = vec!["first".to_string(), "second".to_string(), "third".to_string()];
    let json_instance = export_json_instance(&test_data);
    
    // Find the items relation
    let items_relation = json_instance.relations.iter()
        .find(|r| r.name == "items")
        .expect("Should have items relation");
    
    // Verify we have ternary relations with proper types
    assert_eq!(items_relation.types, vec!["list", "int", "atom"]);
    assert_eq!(items_relation.tuples.len(), 3);
    
    // Find atoms by label to verify positioning
    let mut index_atoms: Vec<_> = json_instance.atoms.iter()
        .filter(|atom| atom.r#type == "int")
        .collect();
    index_atoms.sort_by_key(|atom| atom.label.parse::<i32>().unwrap());
    
    assert_eq!(index_atoms[0].label, "0");
    assert_eq!(index_atoms[1].label, "1");
    assert_eq!(index_atoms[2].label, "2");
    
    println!("✅ List positioning test passed!");
}

#[test]
fn test_struct_vs_map() {
    // Test struct serialization
    let test_struct = TestStruct {
        name: "Test".to_string(),
        items: vec!["item1".to_string()],
    };
    let struct_instance = export_json_instance(&test_struct);
    
    // Should have struct type and field relations
    let root_atom = &struct_instance.atoms[0];
    assert_eq!(root_atom.r#type, "struct");
    
    let field_relation = struct_instance.relations.iter()
        .find(|r| r.name == "field")
        .expect("Should have field relation");
    assert_eq!(field_relation.types, vec!["struct", "atom", "atom"]);
    
    // Test generic map serialization  
    use std::collections::HashMap;
    let mut test_map = HashMap::new();
    test_map.insert("123".to_string(), "value1".to_string());
    test_map.insert("key with spaces".to_string(), "value2".to_string());
    
    let map_instance = export_json_instance(&test_map);
    
    // Should have map type and entry relations (due to non-field-like keys)
    let root_atom = &map_instance.atoms[0];
    assert_eq!(root_atom.r#type, "map");
    
    let entry_relation = map_instance.relations.iter()
        .find(|r| r.name == "entry")
        .expect("Should have entry relation");
    assert_eq!(entry_relation.types, vec!["map", "atom", "atom"]);
    
    println!("✅ Struct vs Map test passed!");
}