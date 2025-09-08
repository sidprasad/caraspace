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

#[test]
fn test_comprehensive_serialization_fixes() {
    // Test a complex structure that demonstrates both fixes
    #[derive(serde::Serialize)]
    struct ComplexData {
        numbers: Vec<i32>,
        items: Vec<String>,
        metadata: std::collections::HashMap<String, String>,
    }
    
    let mut metadata = std::collections::HashMap::new();
    metadata.insert("version".to_string(), "1.0".to_string());
    metadata.insert("author".to_string(), "test".to_string());
    
    let complex_data = ComplexData {
        numbers: vec![10, 20, 30],
        items: vec!["first".to_string(), "second".to_string()],
        metadata,
    };
    
    let json_instance = export_json_instance(&complex_data);
    
    // Verify the root is treated as a struct (has valid field names)
    let root_struct = json_instance.atoms.iter()
        .find(|atom| atom.label == "struct")
        .expect("Should have root struct");
    assert_eq!(root_struct.r#type, "struct");
    
    // Verify we have field relations for the struct
    let field_relation = json_instance.relations.iter()
        .find(|r| r.name == "field")
        .expect("Should have field relations");
    assert_eq!(field_relation.types, vec!["struct", "atom", "atom"]);
    
    // Verify lists have positional information
    let items_relation = json_instance.relations.iter()
        .find(|r| r.name == "items")
        .expect("Should have items relations with positioning");
    assert_eq!(items_relation.types, vec!["list", "int", "atom"]);
    
    // Verify we have index atoms for positioning
    let index_atoms: Vec<_> = json_instance.atoms.iter()
        .filter(|atom| atom.r#type == "int" && 
                atom.label.chars().all(|c| c.is_ascii_digit()))
        .collect();
    assert!(!index_atoms.is_empty(), "Should have index atoms for list positioning");
    
    // Verify nested HashMap is also treated as struct (has valid field names)
    let struct_atoms: Vec<_> = json_instance.atoms.iter()
        .filter(|atom| atom.r#type == "struct")
        .collect();
    assert_eq!(struct_atoms.len(), 2, "Should have root struct and nested struct");
    
    println!("✅ Comprehensive serialization fixes test passed!");
    println!("   - Root struct properly detected");
    println!("   - List positioning preserved with ternary relations"); 
    println!("   - Nested struct/map heuristics working");
}