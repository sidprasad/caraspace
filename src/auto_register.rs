//! Auto-registration utilities for CnD decorated types
//! 
//! This module provides utilities to automatically register decorated types
//! without requiring manual enumeration.

/// Register CnD decorators for multiple types with a simple syntax
/// This is much more ergonomic than manual registration
#[macro_export]
macro_rules! register_cnd_types {
    ($($type:ty),+ $(,)?) => {
        // Register the specified types by calling their decorators() method
        // This is much simpler than the old register_types2::<T1, T2>() pattern
        $(
            let _ = <$type as $crate::spytial_annotations::HasSpytialDecorators>::decorators();
        )+
    };
}