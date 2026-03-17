//! # SpyTial Annotations
//!
//! Compile-time decorator system for SpyTial spatial layout and visualization.
//! Provides type-level decorator collection via derive macros and YAML serialization.

pub mod runtime;
#[cfg(test)]
mod validation;

// Re-export the main types and functions
pub use runtime::{
    get_type_decorators,
    register_type_decorators,
    to_yaml,
    Constraint,
    DecoProbe,
    DefaultDecorators,
    Directive,
    HasSpytialDecorators,
    SpytialDecorators,
    SpytialDecoratorsBuilder,
};
