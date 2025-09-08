//! # Spytial Annotations
//!
//! A runtime annotation system equivalent to Python sPyTial decorators.
//! Provides runtime instance annotations, builder patterns for type-level 
//! annotations, and YAML serialization.

pub mod runtime;
pub mod validation;

#[cfg(test)]
mod tests;

// Re-export the main types and functions
pub use runtime::{
    SpytialDecorators, Constraint, Directive, HasSpytialDecorators,
    annotate_instance, collect_decorators_for_instance, to_yaml, 
    to_yaml_for_type, to_yaml_for_instance, Annotation,
    // Builder types for creating annotations
    AnnotationBuilder, SpytialDecoratorsBuilder,
};

// Re-export validation for examples and advanced usage
pub use validation::{get_constraint_params, get_directive_params, validate_params};