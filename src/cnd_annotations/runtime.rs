use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
use once_cell::sync::Lazy;

/// Main structure containing all CnD decorators for a type or instance
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CndDecorators {
    pub constraints: Vec<Constraint>,
    pub directives: Vec<Directive>,
}

impl Default for CndDecorators {
    fn default() -> Self {
        Self {
            constraints: Vec::new(),
            directives: Vec::new(),
        }
    }
}

/// Constraint types (layout/structural constraints)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Constraint {
    Orientation(OrientationConstraint),
    Cyclic(CyclicConstraint),
    Group(GroupConstraint),
}

/// Directive types (visual/behavioral directives)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Directive {
    AtomColor(AtomColorDirective),
    Size(SizeDirective),
    Icon(IconDirective),
    EdgeColor(EdgeColorDirective),
    Projection(ProjectionDirective),
    Attribute(AttributeDirective),
    HideField(HideFieldDirective),
    HideAtom(HideAtomDirective),
    InferredEdge(InferredEdgeDirective),
    Flag(FlagDirective),
}

// Constraint implementations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OrientationConstraint {
    pub orientation: OrientationParams,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OrientationParams {
    pub selector: String,
    pub directions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CyclicConstraint {
    pub cyclic: CyclicParams,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CyclicParams {
    pub selector: String,
    pub direction: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GroupConstraint {
    pub group: GroupParams,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum GroupParams {
    FieldBased {
        field: String,
        #[serde(rename = "groupOn")]
        group_on: u32,
        #[serde(rename = "addToGroup")]
        add_to_group: u32,
        #[serde(skip_serializing_if = "Option::is_none")]
        selector: Option<String>,
    },
    SelectorBased {
        selector: String,
        name: String,
    },
}

// Directive implementations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AtomColorDirective {
    #[serde(rename = "atomColor")]
    pub atom_color: AtomColorParams,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AtomColorParams {
    pub selector: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SizeDirective {
    pub size: SizeParams,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SizeParams {
    pub selector: String,
    pub height: u32,
    pub width: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IconDirective {
    pub icon: IconParams,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IconParams {
    pub selector: String,
    pub path: String,
    #[serde(rename = "showLabels")]
    pub show_labels: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EdgeColorDirective {
    #[serde(rename = "edgeColor")]
    pub edge_color: EdgeColorParams,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EdgeColorParams {
    pub field: String,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProjectionDirective {
    pub projection: ProjectionParams,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProjectionParams {
    pub sig: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AttributeDirective {
    pub attribute: AttributeParams,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AttributeParams {
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HideFieldDirective {
    #[serde(rename = "hideField")]
    pub hide_field: HideFieldParams,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HideFieldParams {
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HideAtomDirective {
    #[serde(rename = "hideAtom")]
    pub hide_atom: HideAtomParams,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HideAtomParams {
    pub selector: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InferredEdgeDirective {
    #[serde(rename = "inferredEdge")]
    pub inferred_edge: InferredEdgeParams,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InferredEdgeParams {
    pub name: String,
    pub selector: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FlagDirective {
    pub flag: String,
}

/// Trait implemented by structs with CnD decorators
/// All types have a default implementation that returns empty decorators
pub trait HasCndDecorators {
    fn decorators() -> CndDecorators {
        CndDecorators::default()
    }
    
    /// Collect decorators from this type and all nested types that also implement HasCndDecorators
    /// Default implementation just returns self decorators
    fn recursive_decorators() -> CndDecorators {
        Self::decorators()
    }
    
    /// Trigger registration of this type in the global registry
    /// This should be called to ensure the type is available for serialization-time lookup
    fn ensure_registered() {
        // Calling decorators() triggers registration in the macro-generated code
        let _ = Self::decorators();
    }
}

/// Global registry for instance-level annotations
static INSTANCE_REGISTRY: Lazy<Mutex<HashMap<usize, CndDecorators>>> = 
    Lazy::new(|| Mutex::new(HashMap::new()));

/// Counter for generating unique instance IDs
static INSTANCE_ID_COUNTER: Lazy<Mutex<usize>> = Lazy::new(|| Mutex::new(0));

/// Global registry for type-level decorators keyed by type name
static TYPE_REGISTRY: Lazy<Mutex<HashMap<String, CndDecorators>>> = 
    Lazy::new(|| Mutex::new(HashMap::new()));

/// Register CnD decorators for a type (used by procedural macros)
pub fn register_type_decorators(type_name: &str, decorators: CndDecorators) {
    let mut registry = TYPE_REGISTRY.lock().unwrap();
    registry.insert(type_name.to_string(), decorators);
}

/// Get CnD decorators for a type by name
pub fn get_type_decorators(type_name: &str) -> Option<CndDecorators> {
    let registry = TYPE_REGISTRY.lock().unwrap();
    registry.get(type_name).cloned()
}

/// Annotation to apply to an instance at runtime
#[derive(Debug, Clone)]
pub struct Annotation {
    pub annotation_type: String,
    pub params: HashMap<String, serde_json::Value>,
}

/// Apply an annotation to an instance at runtime
pub fn annotate_instance<T>(instance: &mut T, annotation: Annotation) {
    let instance_addr = instance as *const T as usize;
    let mut registry = INSTANCE_REGISTRY.lock().unwrap();
    
    let decorators = registry.entry(instance_addr).or_default();
    
    // Convert annotation to appropriate constraint or directive
    match annotation.annotation_type.as_str() {
        "orientation" => {
            if let (Some(selector), Some(directions)) = (
                annotation.params.get("selector").and_then(|v| v.as_str()),
                annotation.params.get("directions").and_then(|v| v.as_array())
            ) {
                let dirs: Vec<String> = directions.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect();
                
                decorators.constraints.push(Constraint::Orientation(OrientationConstraint {
                    orientation: OrientationParams {
                        selector: substitute_self_reference(selector, instance_addr),
                        directions: dirs,
                    }
                }));
            }
        }
        "cyclic" => {
            if let (Some(selector), Some(direction)) = (
                annotation.params.get("selector").and_then(|v| v.as_str()),
                annotation.params.get("direction").and_then(|v| v.as_str())
            ) {
                decorators.constraints.push(Constraint::Cyclic(CyclicConstraint {
                    cyclic: CyclicParams {
                        selector: substitute_self_reference(selector, instance_addr),
                        direction: direction.to_string(),
                    }
                }));
            }
        }
        "atomColor" => {
            if let (Some(selector), Some(value)) = (
                annotation.params.get("selector").and_then(|v| v.as_str()),
                annotation.params.get("value").and_then(|v| v.as_str())
            ) {
                decorators.directives.push(Directive::AtomColor(AtomColorDirective {
                    atom_color: AtomColorParams {
                        selector: substitute_self_reference(selector, instance_addr),
                        value: value.to_string(),
                    }
                }));
            }
        }
        "flag" => {
            if let Some(name) = annotation.params.get("name").and_then(|v| v.as_str()) {
                decorators.directives.push(Directive::Flag(FlagDirective {
                    flag: name.to_string(),
                }));
            }
        }
        // Add other annotation types as needed
        _ => {} // Unknown annotation type, ignore or handle error
    }
}

/// Collect decorators for both type and instance
pub fn collect_decorators_for_instance<T: HasCndDecorators>(instance: &T) -> CndDecorators {
    let mut combined = T::decorators();
    
    let instance_addr = instance as *const T as usize;
    let registry = INSTANCE_REGISTRY.lock().unwrap();
    
    if let Some(instance_decorators) = registry.get(&instance_addr) {
        combined.constraints.extend(instance_decorators.constraints.clone());
        combined.directives.extend(instance_decorators.directives.clone());
    }
    
    combined
}

/// Collect only instance-level decorators (without type decorators)
pub fn collect_instance_only_decorators<T>(instance: &T) -> CndDecorators {
    let instance_addr = instance as *const T as usize;
    let registry = INSTANCE_REGISTRY.lock().unwrap();
    
    if let Some(instance_decorators) = registry.get(&instance_addr) {
        instance_decorators.clone()
    } else {
        CndDecorators::default()
    }
}

/// Serialize decorators to YAML string
pub fn to_yaml(decorators: &CndDecorators) -> Result<String, serde_yaml::Error> {
    serde_yaml::to_string(decorators)
}

/// Helper function to get decorators for a type as YAML
pub fn to_yaml_for_type<T: HasCndDecorators>() -> Result<String, serde_yaml::Error> {
    to_yaml(&T::decorators())
}

/// Helper function to get decorators for an instance as YAML
pub fn to_yaml_for_instance<T: HasCndDecorators>(instance: &T) -> Result<String, serde_yaml::Error> {
    to_yaml(&collect_decorators_for_instance(instance))
}

/// A helper function to ensure all types in a data structure are registered
/// Users should call this for their root data type to enable recursive decorator collection
pub fn ensure_types_registered<T: HasCndDecorators>() {
    T::ensure_registered();
}

/// Register a type and its common field types
/// This is a convenience function for common patterns
pub fn register_types<T1: HasCndDecorators>() {
    T1::ensure_registered();
}

/// Register two types and their decorators
pub fn register_types2<T1: HasCndDecorators, T2: HasCndDecorators>() {
    T1::ensure_registered();
    T2::ensure_registered();
}

/// Register three types and their decorators
pub fn register_types3<T1: HasCndDecorators, T2: HasCndDecorators, T3: HasCndDecorators>() {
    T1::ensure_registered();
    T2::ensure_registered();
    T3::ensure_registered();
}

/// Replace "self" in selector with instance-specific identifier
fn substitute_self_reference(selector: &str, _instance_addr: usize) -> String {
    let mut counter = INSTANCE_ID_COUNTER.lock().unwrap();
    *counter += 1;
    selector.replace("self", &format!("obj_{}", *counter))
}

/// Builder for creating CnD decorators
#[derive(Debug, Default)]
pub struct CndDecoratorsBuilder {
    constraints: Vec<Constraint>,
    directives: Vec<Directive>,
}

impl CndDecoratorsBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn orientation(mut self, selector: &str, directions: Vec<&str>) -> Self {
        self.constraints.push(Constraint::Orientation(OrientationConstraint {
            orientation: OrientationParams {
                selector: selector.to_string(),
                directions: directions.iter().map(|s| s.to_string()).collect(),
            }
        }));
        self
    }

    pub fn cyclic(mut self, selector: &str, direction: &str) -> Self {
        self.constraints.push(Constraint::Cyclic(CyclicConstraint {
            cyclic: CyclicParams {
                selector: selector.to_string(),
                direction: direction.to_string(),
            }
        }));
        self
    }

    pub fn group_field_based(mut self, field: &str, group_on: u32, add_to_group: u32, selector: Option<&str>) -> Self {
        self.constraints.push(Constraint::Group(GroupConstraint {
            group: GroupParams::FieldBased {
                field: field.to_string(),
                group_on,
                add_to_group,
                selector: selector.map(|s| s.to_string()),
            }
        }));
        self
    }

    pub fn group_selector_based(mut self, selector: &str, name: &str) -> Self {
        self.constraints.push(Constraint::Group(GroupConstraint {
            group: GroupParams::SelectorBased {
                selector: selector.to_string(),
                name: name.to_string(),
            }
        }));
        self
    }

    pub fn atom_color(mut self, selector: &str, value: &str) -> Self {
        self.directives.push(Directive::AtomColor(AtomColorDirective {
            atom_color: AtomColorParams {
                selector: selector.to_string(),
                value: value.to_string(),
            }
        }));
        self
    }

    pub fn size(mut self, selector: &str, height: u32, width: u32) -> Self {
        self.directives.push(Directive::Size(SizeDirective {
            size: SizeParams {
                selector: selector.to_string(),
                height,
                width,
            }
        }));
        self
    }

    pub fn icon(mut self, selector: &str, path: &str, show_labels: bool) -> Self {
        self.directives.push(Directive::Icon(IconDirective {
            icon: IconParams {
                selector: selector.to_string(),
                path: path.to_string(),
                show_labels,
            }
        }));
        self
    }

    pub fn edge_color(mut self, field: &str, value: &str, selector: Option<&str>) -> Self {
        self.directives.push(Directive::EdgeColor(EdgeColorDirective {
            edge_color: EdgeColorParams {
                field: field.to_string(),
                value: value.to_string(),
                selector: selector.map(|s| s.to_string()),
            }
        }));
        self
    }

    pub fn projection(mut self, sig: &str) -> Self {
        self.directives.push(Directive::Projection(ProjectionDirective {
            projection: ProjectionParams {
                sig: sig.to_string(),
            }
        }));
        self
    }

    pub fn attribute(mut self, field: &str, selector: Option<&str>) -> Self {
        self.directives.push(Directive::Attribute(AttributeDirective {
            attribute: AttributeParams {
                field: field.to_string(),
                selector: selector.map(|s| s.to_string()),
            }
        }));
        self
    }

    pub fn hide_field(mut self, field: &str, selector: Option<&str>) -> Self {
        self.directives.push(Directive::HideField(HideFieldDirective {
            hide_field: HideFieldParams {
                field: field.to_string(),
                selector: selector.map(|s| s.to_string()),
            }
        }));
        self
    }

    pub fn hide_atom(mut self, selector: &str) -> Self {
        self.directives.push(Directive::HideAtom(HideAtomDirective {
            hide_atom: HideAtomParams {
                selector: selector.to_string(),
            }
        }));
        self
    }

    pub fn inferred_edge(mut self, name: &str, selector: &str) -> Self {
        self.directives.push(Directive::InferredEdge(InferredEdgeDirective {
            inferred_edge: InferredEdgeParams {
                name: name.to_string(),
                selector: selector.to_string(),
            }
        }));
        self
    }

    pub fn flag(mut self, name: &str) -> Self {
        self.directives.push(Directive::Flag(FlagDirective {
            flag: name.to_string(),
        }));
        self
    }

    pub fn build(self) -> CndDecorators {
        CndDecorators {
            constraints: self.constraints,
            directives: self.directives,
        }
    }
}

/// Builder for creating individual annotations
#[derive(Debug)]
pub struct AnnotationBuilder;

impl AnnotationBuilder {
    pub fn orientation(selector: &str, directions: Vec<&str>) -> Annotation {
        let mut params = std::collections::HashMap::new();
        params.insert("selector".to_string(), serde_json::Value::String(selector.to_string()));
        params.insert("directions".to_string(), serde_json::Value::Array(
            directions.iter().map(|s| serde_json::Value::String(s.to_string())).collect()
        ));
        
        Annotation {
            annotation_type: "orientation".to_string(),
            params,
        }
    }

    pub fn cyclic(selector: &str, direction: &str) -> Annotation {
        let mut params = std::collections::HashMap::new();
        params.insert("selector".to_string(), serde_json::Value::String(selector.to_string()));
        params.insert("direction".to_string(), serde_json::Value::String(direction.to_string()));
        
        Annotation {
            annotation_type: "cyclic".to_string(),
            params,
        }
    }

    pub fn atom_color(selector: &str, value: &str) -> Annotation {
        let mut params = std::collections::HashMap::new();
        params.insert("selector".to_string(), serde_json::Value::String(selector.to_string()));
        params.insert("value".to_string(), serde_json::Value::String(value.to_string()));
        
        Annotation {
            annotation_type: "atomColor".to_string(),
            params,
        }
    }

    pub fn flag(name: &str) -> Annotation {
        let mut params = std::collections::HashMap::new();
        params.insert("name".to_string(), serde_json::Value::String(name.to_string()));
        
        Annotation {
            annotation_type: "flag".to_string(),
            params,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spytial_decorators_default() {
        let decorators = CndDecorators::default();
        assert!(decorators.constraints.is_empty());
        assert!(decorators.directives.is_empty());
    }

    #[test]
    fn test_yaml_serialization() {
        let decorators = CndDecorators {
            constraints: vec![
                Constraint::Orientation(OrientationConstraint {
                    orientation: OrientationParams {
                        selector: "value".to_string(),
                        directions: vec!["above".to_string()],
                    }
                })
            ],
            directives: vec![
                Directive::Flag(FlagDirective {
                    flag: "test_flag".to_string()
                })
            ],
        };

        let yaml = to_yaml(&decorators).unwrap();
        assert!(yaml.contains("orientation"));
        assert!(yaml.contains("flag"));
    }

    #[test]
    fn test_self_reference_substitution() {
        let result = substitute_self_reference("self.field", 12345);
        assert!(result.starts_with("obj_"));
        assert!(result.contains(".field"));
    }
}