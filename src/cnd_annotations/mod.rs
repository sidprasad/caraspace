//! # CnD Annotations
//!
//! A runtime annotation system for CnD spatial layout and visualization.
//! Provides runtime instance annotations, builder patterns for type-level 
//! annotations, and YAML serialization.

pub mod runtime;
pub mod validation;

// Re-export the main types and functions
pub use runtime::{
    CndDecorators, Constraint, Directive, HasCndDecorators,
    annotate_instance, collect_decorators_for_instance, to_yaml, 
    to_yaml_for_type, to_yaml_for_instance, Annotation,
    register_type_decorators, get_type_decorators,
    // Builder types for creating annotations
    AnnotationBuilder, CndDecoratorsBuilder,
};

// Re-export validation for examples and advanced usage
pub use validation::{get_constraint_params, get_directive_params, validate_params};