//! # SpyTial Annotations
//!
//! A runtime annotation system for SpyTial spatial layout and visualization.
//! Provides runtime instance annotations, builder patterns for type-level
//! annotations, and YAML serialization.

pub mod runtime;
pub mod validation;

// Re-export the main types and functions
pub use runtime::{
    annotate_instance,
    auto_register_related_types,
    auto_register_types,
    collect_decorators_for_instance,
    collect_instance_only_decorators,
    ensure_types_registered,
    get_type_decorators,
    register_type_decorators,
    register_types,
    register_types2,
    register_types3,
    to_yaml,
    to_yaml_for_instance,
    to_yaml_for_type,
    Annotation,
    // Builder types for creating annotations
    AnnotationBuilder,
    Constraint,
    Directive,
    HasSpytialDecorators,
    SpytialDecorators,
    SpytialDecoratorsBuilder,
};

// Re-export validation for examples and advanced usage
pub use validation::{get_constraint_params, get_directive_params, validate_params};
