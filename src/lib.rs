pub mod jsondata;
pub mod export;
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
/// ```rust
/// use serde::Serialize;
/// use json_data_instance_export::{diagram, SpytialDecorators};
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
/// let company = Company { /* ... */ };
/// diagram(&company);  // Shows decorators from both Company AND Person
/// ```
pub fn diagram<T: spytial_annotations::HasSpytialDecorators + Serialize>(value: &T) {
    let cnd_spec = collect_cnd_spec_for_diagram(value);
    diagram_impl(value, &cnd_spec);
}

/// Collect CnD specification using compile-time decorator collection.
/// 
/// With the new compile-time system, calling `T::decorators()` returns decorators
/// from the type itself AND all nested types that have decorators. This eliminates
/// the need for complex runtime type discovery and registration.
fn collect_cnd_spec_for_diagram<T: spytial_annotations::HasSpytialDecorators + Serialize>(_value: &T) -> String {
    println!("🔍 Assembling CnD spec with compile-time decorator collection...");
    
    // The magic happens here: T::decorators() includes ALL decorators 
    // from this type AND all nested decorated types (analyzed at compile time)
    let all_decorators = T::decorators();
    println!("   ✨ Compile-time collected decorators: {} constraints, {} directives", 
             all_decorators.constraints.len(), 
             all_decorators.directives.len());
    
    // Serialize to YAML
    let cnd_spec = spytial_annotations::to_yaml(&all_decorators).unwrap_or_default();
    println!("   📋 Generated CnD spec:\n{}", cnd_spec);
    
    cnd_spec
}

/// Creates a diagram with a custom CnD specification (legacy function).
///
/// This allows you to provide a custom CnD specification instead of using 
/// the automatic compile-time decorator collection.
pub fn diagram_with_spec<T: Serialize>(value: &T, cnd_spec: &str) {
    diagram_impl(value, cnd_spec);
}

/// Internal implementation shared by diagram functions.
fn diagram_impl<T: Serialize>(value: &T, cnd_spec: &str) {
    // Export the struct to our custom JSON format with type information
    let json_instance = export_json_instance(value);
    let json_data = serde_json::to_string_pretty(&json_instance).unwrap();

    // Load the HTML template and replace the placeholders
    let template = include_str!("../templates/template.html");
    let rendered_html = template
        .replace("{{ json_data }}", &json_data)
        .replace("{{ cnd_spec }}", cnd_spec);

    // Save the rendered HTML to a temporary file
    let temp_dir = env::temp_dir();
    let temp_file_path = temp_dir.join("rust_viz_data.html");
    fs::write(&temp_file_path, rendered_html).expect("Failed to write HTML to file");

    // Open the HTML file in the browser
    let file_url = format!("file://{}", temp_file_path.display());
    println!("Opening visualization at: {}", file_url);

    #[cfg(target_os = "macos")]
    let open_cmd = "open";
    #[cfg(target_os = "windows")]
    let open_cmd = "start";
    #[cfg(target_os = "linux")]
    let open_cmd = "xdg-open";

    Command::new(open_cmd)
        .arg(&temp_file_path)
        .spawn()
        .expect("Failed to open browser");
}
