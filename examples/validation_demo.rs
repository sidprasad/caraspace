// This file contains examples of validation functionality
// It's not a traditional test file but shows how validation works

use rust_viz::spytial_annotations::validation::{get_constraint_params, get_directive_params, validate_params};

fn main() {
    // Example of successful validation
    let constraint_params = get_constraint_params();
    let orientation_def = constraint_params.get("orientation").unwrap();
    let provided = vec!["selector".to_string(), "directions".to_string()];
    
    match validate_params("orientation", &provided, orientation_def) {
        Ok(()) => println!("✓ Orientation validation passed"),
        Err(e) => println!("✗ Orientation validation failed: {}", e),
    }

    // Example of validation failure - missing parameter
    let provided_missing = vec!["selector".to_string()]; // missing "directions"
    match validate_params("orientation", &provided_missing, orientation_def) {
        Ok(()) => println!("✗ Should have failed validation"),
        Err(e) => println!("✓ Expected validation error: {}", e),
    }

    // Example of group validation with multiple parameter sets
    let group_def = constraint_params.get("group").unwrap();
    
    // Test field-based group
    let field_based = vec![
        "field".to_string(),
        "groupOn".to_string(),
        "addToGroup".to_string(),
    ];
    match validate_params("group", &field_based, group_def) {
        Ok(()) => println!("✓ Group field-based validation passed"),
        Err(e) => println!("✗ Group field-based validation failed: {}", e),
    }

    // Test selector-based group
    let selector_based = vec!["selector".to_string(), "name".to_string()];
    match validate_params("group", &selector_based, group_def) {
        Ok(()) => println!("✓ Group selector-based validation passed"),
        Err(e) => println!("✗ Group selector-based validation failed: {}", e),
    }

    // Test invalid group parameters
    let invalid_group = vec!["invalid".to_string(), "params".to_string()];
    match validate_params("group", &invalid_group, group_def) {
        Ok(()) => println!("✗ Should have failed group validation"),
        Err(e) => println!("✓ Expected group validation error: {}", e),
    }

    // Test directive validation
    let directive_params = get_directive_params();
    let flag_def = directive_params.get("flag").unwrap();
    
    let valid_flag = vec!["name".to_string()];
    match validate_params("flag", &valid_flag, flag_def) {
        Ok(()) => println!("✓ Flag validation passed"),
        Err(e) => println!("✗ Flag validation failed: {}", e),
    }

    // Test unknown parameter in directive
    let invalid_flag = vec!["name".to_string(), "unknown".to_string()];
    match validate_params("flag", &invalid_flag, flag_def) {
        Ok(()) => println!("✗ Should have failed flag validation"),
        Err(e) => println!("✓ Expected flag validation error: {}", e),
    }
}