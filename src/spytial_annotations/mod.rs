//! # SpyTial Annotations
//!
//! A runtime annotation system for SpyTial spatial layout and visualization.
//! Provides runtime instance annotations, builder patterns for type-level 
//! annotations, and YAML serialization.

pub mod runtime;
pub mod validation;

// Re-export the main types and functions
pub use runtime::{
    SpytialDecorators, Constraint, Directive, HasSpytialDecorators,
    annotate_instance, collect_decorators_for_instance, collect_instance_only_decorators, to_yaml, 
    to_yaml_for_type, to_yaml_for_instance, Annotation,
    register_type_decorators, get_type_decorators,
    auto_register_related_types, auto_register_types,
    ensure_types_registered, register_types, register_types2, register_types3,
    // Builder types for creating annotations
    AnnotationBuilder, SpytialDecoratorsBuilder,
};

// Re-export validation for examples and advanced usage
pub use validation::{get_constraint_params, get_directive_params, validate_params};