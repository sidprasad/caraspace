//! CaraSpace is the Rust-facing integration layer for Spytial.
//!
//! Start with `README.md` for the project overview and `USER_GUIDE.md` for the
//! end-user workflow.

pub mod export;
pub mod jsondata;
pub mod spytial_annotations;

pub use export::export_json_instance;
// Re-export the derive macro for spatial annotations
pub use caraspace_export_macros::SpytialDecorators;
use serde::Serialize;
use std::env;
use std::fs;
use std::process::Command;

/// Creates a diagram of the given data structure and opens it in the browser.
///
/// This function uses **compile-time decorator collection** to automatically include
/// decorators from all nested types without requiring manual registration.
///
/// ## How it works:
/// 1. **Compile-time analysis**: The `#[derive(SpytialDecorators)]` macro analyzes the type tree
/// 2. **Automatic inclusion**: Decorators from nested types are automatically included
/// 3. **Single call**: Just call `diagram(&your_struct)` - no registration needed
///
/// ## Example:
/// ```no_run
/// use serde::Serialize;
/// use caraspace::{diagram, SpytialDecorators};
///
/// #[derive(Serialize, SpytialDecorators)]
/// #[attribute(field = "name")]
/// struct Company {
///     name: String,
///     employees: Vec<Person>,  // Person's decorators automatically included
/// }
///
/// #[derive(Serialize, SpytialDecorators)]
/// #[attribute(field = "age")]
/// struct Person {
///     name: String,
///     age: u32,
/// }
///
/// let company = Company {
///     name: "Acme Corp".to_string(),
///     employees: vec![Person {
///         name: "Alice".to_string(),
///         age: 30,
///     }],
/// };
/// diagram(&company);  // Shows decorators from both Company AND Person
/// ```
pub fn diagram<T: spytial_annotations::HasSpytialDecorators + Serialize>(value: &T) {
    let spytial_spec = collect_spytial_spec_for_diagram(value);
    diagram_impl(value, &spytial_spec);
}

/// Collect SpyTial specification using compile-time decorator collection.
///
/// With the new compile-time system, calling `T::decorators()` returns decorators
/// from the type itself AND all nested types that have decorators. This eliminates
/// the need for complex runtime type discovery and registration.
fn collect_spytial_spec_for_diagram<T: spytial_annotations::HasSpytialDecorators + Serialize>(
    _value: &T,
) -> String {
    // The magic happens here: T::decorators() includes ALL decorators
    // from this type AND all nested decorated types (analyzed at compile time)
    let all_decorators = T::decorators();

    // Serialize to YAML
    spytial_annotations::to_yaml(&all_decorators).unwrap_or_default()
}

/// Creates a diagram with a custom SpyTial specification (legacy function).
///
/// This allows you to provide a custom SpyTial specification instead of using
/// the automatic compile-time decorator collection.
pub fn diagram_with_spec<T: Serialize>(value: &T, spec: &str) {
    diagram_impl(value, spec);
}

/// Internal implementation shared by diagram functions.
fn diagram_impl<T: Serialize>(value: &T, spec: &str) {
    // Export the struct to our custom JSON format with type information
    let json_instance = export_json_instance(value);
    let json_data = serde_json::to_string_pretty(&json_instance).unwrap();

    // Load the HTML template and replace the placeholders
    let template = include_str!("../templates/template.html");
    let rendered_html = template
        .replace("{{ json_data }}", &json_data)
        .replace("{{ spytial_spec }}", spec);

    // Save the rendered HTML to a temporary file
    let temp_dir = env::temp_dir();
    let temp_file_path = temp_dir.join("rust_viz_data.html");
    fs::write(&temp_file_path, rendered_html).expect("Failed to write HTML to file");

    let skip_browser_open = env::var("SPYTIAL_NO_OPEN")
        .map(|raw| matches!(raw.to_ascii_lowercase().as_str(), "1" | "true" | "yes"))
        .unwrap_or(false);

    if skip_browser_open {
        return;
    }

    #[cfg(target_os = "macos")]
    let open_cmd = "open";
    #[cfg(target_os = "windows")]
    let open_cmd = "start";
    #[cfg(target_os = "linux")]
    let open_cmd = "xdg-open";

    if let Err(error) = Command::new(open_cmd).arg(&temp_file_path).spawn() {
        eprintln!(
            "Failed to open browser: {}. Open this file manually: {}",
            error,
            temp_file_path.display()
        );
    }
}
