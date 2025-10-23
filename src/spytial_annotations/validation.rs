/// Validation utilities for macro parameter checking
use std::collections::HashMap;

/// Definition of valid parameters for each constraint type
pub fn get_constraint_params() -> HashMap<&'static str, ConstraintParamDef> {
    let mut params = HashMap::new();
    
    params.insert("orientation", ConstraintParamDef::Single {
        required: vec!["selector", "directions"],
        optional: vec![],
    });
    
    params.insert("cyclic", ConstraintParamDef::Single {
        required: vec!["selector", "direction"],
        optional: vec![],
    });
    
    params.insert("group", ConstraintParamDef::Multiple(vec![
        ParamSet {
            required: vec!["field", "groupOn", "addToGroup"],
            optional: vec!["selector"],
        },
        ParamSet {
            required: vec!["selector", "name"],
            optional: vec![],
        },
    ]));
    
    params
}

/// Definition of valid parameters for each directive type
pub fn get_directive_params() -> HashMap<&'static str, ConstraintParamDef> {
    let mut params = HashMap::new();
    
    params.insert("atomColor", ConstraintParamDef::Single {
        required: vec!["selector", "value"],
        optional: vec![],
    });
    
    params.insert("size", ConstraintParamDef::Single {
        required: vec!["selector", "height", "width"],
        optional: vec![],
    });
    
    params.insert("icon", ConstraintParamDef::Single {
        required: vec!["selector", "path", "showLabels"],
        optional: vec![],
    });
    
    params.insert("edgeColor", ConstraintParamDef::Single {
        required: vec!["field", "value"],
        optional: vec!["selector"],
    });
    
    params.insert("projection", ConstraintParamDef::Single {
        required: vec!["sig"],
        optional: vec![],
    });
    
    params.insert("attribute", ConstraintParamDef::Single {
        required: vec!["field"],
        optional: vec!["selector"],
    });
    
    params.insert("hideField", ConstraintParamDef::Single {
        required: vec!["field"],
        optional: vec!["selector"],
    });
    
    params.insert("hideAtom", ConstraintParamDef::Single {
        required: vec!["selector"],
        optional: vec![],
    });
    
    params.insert("inferredEdge", ConstraintParamDef::Single {
        required: vec!["name", "selector"],
        optional: vec![],
    });
    
    params.insert("flag", ConstraintParamDef::Single {
        required: vec!["name"],
        optional: vec![],
    });
    
    params
}

/// Parameter definition for constraint/directive validation
#[derive(Debug, Clone)]
pub enum ConstraintParamDef {
    Single {
        required: Vec<&'static str>,
        optional: Vec<&'static str>,
    },
    Multiple(Vec<ParamSet>),
}

/// A set of parameters (for constraints that support multiple forms)
#[derive(Debug, Clone)]
pub struct ParamSet {
    pub required: Vec<&'static str>,
    pub optional: Vec<&'static str>,
}

/// Validate that provided parameters match the expected schema
pub fn validate_params(
    annotation_type: &str,
    provided_params: &[String],
    param_def: &ConstraintParamDef,
) -> Result<(), String> {
    match param_def {
        ConstraintParamDef::Single { required, optional } => {
            validate_single_param_set(annotation_type, provided_params, required, optional)
        }
        ConstraintParamDef::Multiple(param_sets) => {
            // Try each parameter set until one matches
            let mut errors = Vec::new();
            
            for (i, param_set) in param_sets.iter().enumerate() {
                match validate_single_param_set(
                    annotation_type,
                    provided_params,
                    &param_set.required,
                    &param_set.optional,
                ) {
                    Ok(()) => return Ok(()), // Found a matching set
                    Err(err) => errors.push(format!("Set {}: {}", i + 1, err)),
                }
            }
            
            // None matched, create comprehensive error
            let set_descriptions: Vec<String> = param_sets
                .iter()
                .enumerate()
                .map(|(i, set)| {
                    format!(
                        "Set {}: required: [{}], optional: [{}]",
                        i + 1,
                        set.required.join(", "),
                        set.optional.join(", ")
                    )
                })
                .collect();
            
            Err(format!(
                "No valid parameter set found for '{}'. Expected one of: {}. Provided: [{}]",
                annotation_type,
                set_descriptions.join(" OR "),
                provided_params.join(", ")
            ))
        }
    }
}

/// Validate a single parameter set
fn validate_single_param_set(
    annotation_type: &str,
    provided_params: &[String],
    required: &[&str],
    optional: &[&str],
) -> Result<(), String> {
    // Check for missing required parameters
    let missing: Vec<&str> = required
        .iter()
        .filter(|&&param| !provided_params.contains(&param.to_string()))
        .copied()
        .collect();
    
    if !missing.is_empty() {
        return Err(format!(
            "Missing required parameters for '{}': [{}]",
            annotation_type,
            missing.join(", ")
        ));
    }
    
    // Check for unknown parameters
    let all_valid: Vec<String> = required
        .iter()
        .chain(optional.iter())
        .map(|s| s.to_string())
        .collect();
    
    let unknown: Vec<&String> = provided_params
        .iter()
        .filter(|param| !all_valid.contains(param))
        .collect();
    
    if !unknown.is_empty() {
        return Err(format!(
            "Unknown parameters for '{}': [{}]. Valid parameters: [{}]",
            annotation_type,
            unknown.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", "),
            all_valid.join(", ")
        ));
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orientation_validation_success() {
        let params = get_constraint_params();
        let orientation_def = params.get("orientation").unwrap();
        
        let provided = vec!["selector".to_string(), "directions".to_string()];
        assert!(validate_params("orientation", &provided, orientation_def).is_ok());
    }

    #[test]
    fn test_orientation_validation_missing_param() {
        let params = get_constraint_params();
        let orientation_def = params.get("orientation").unwrap();
        
        let provided = vec!["selector".to_string()]; // missing "directions"
        let result = validate_params("orientation", &provided, orientation_def);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Missing required parameters"));
    }

    #[test]
    fn test_group_validation_field_based() {
        let params = get_constraint_params();
        let group_def = params.get("group").unwrap();
        
        let provided = vec![
            "field".to_string(),
            "groupOn".to_string(),
            "addToGroup".to_string(),
        ];
        assert!(validate_params("group", &provided, group_def).is_ok());
    }

    #[test]
    fn test_group_validation_selector_based() {
        let params = get_constraint_params();
        let group_def = params.get("group").unwrap();
        
        let provided = vec!["selector".to_string(), "name".to_string()];
        assert!(validate_params("group", &provided, group_def).is_ok());
    }

    #[test]
    fn test_unknown_parameter() {
        let params = get_directive_params();
        let flag_def = params.get("flag").unwrap();
        
        let provided = vec!["name".to_string(), "unknown".to_string()];
        let result = validate_params("flag", &provided, flag_def);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unknown parameters"));
    }
}