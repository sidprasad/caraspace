//! Integration test for recursive CnD decorator collection
//! 
//! This test validates that the system correctly collects decorators from
//! nested types, not just the root type.

use json_data_instance_export::{CndDecorators, cnd_annotations::HasCndDecorators, register_cnd_types};
use serde::Serialize;

#[derive(Serialize, CndDecorators)]
#[attribute(field = "name")]
#[flag(name="root_flag")]
struct RootType {
    name: String,
    nested: NestedType,
}

#[derive(Serialize, CndDecorators)]
#[attribute(field = "value")]
#[flag(name="nested_flag")]
struct NestedType {
    value: i32,
}

#[test]
fn test_recursive_decorator_collection() {
    // Register both types using the new cleaner API
    register_cnd_types!(RootType, NestedType);
    
    // Create a test instance
    let root = RootType {
        name: "test".to_string(),
        nested: NestedType { value: 42 },
    };
    
    // Collect decorators using the diagram system (without actually opening browser)
    let cnd_spec = json_data_instance_export::collect_cnd_spec_for_test(&root);
    
    // Verify that we have decorators from both types
    assert!(cnd_spec.contains("root_flag"), "Should contain root type flag");
    assert!(cnd_spec.contains("nested_flag"), "Should contain nested type flag");
    assert!(cnd_spec.contains("name"), "Should contain root type attribute field");
    assert!(cnd_spec.contains("value"), "Should contain nested type attribute field");
    
    // Count directives (should be 4: 2 from root + 2 from nested)
    let directive_count = cnd_spec.matches("- ").count();
    assert_eq!(directive_count, 4, "Should have 4 directives total");
}

#[test]
fn test_individual_type_decorators() {
    // Test that individual types have their decorators
    let root_decorators = RootType::decorators();
    assert_eq!(root_decorators.directives.len(), 2);
    
    let nested_decorators = NestedType::decorators();
    assert_eq!(nested_decorators.directives.len(), 2);
}