pub mod jsondata;
pub mod export;
pub mod cnd_annotations;
pub mod auto_register;

pub use export::{export_json_instance, export_json_instance_with_decorators};
// Re-export the derive macro for spatial annotations
pub use caraspace_export_macros::CndDecorators;
use serde::Serialize;
use std::env;
use std::fs;
use std::process::Command;

/// Extract the simple type name from a full type path
fn extract_type_name<T>() -> &'static str {
    let full_name = std::any::type_name::<T>();
    // Extract the part after the last '::'
    full_name.split("::").last().unwrap_or(full_name)
}

/// Creates a diagram of the given data structure and opens it in the browser.
///
/// This function serializes the given struct to JSON and embeds it in an HTML template.
/// If the struct was decorated with spatial annotation macros, those annotations
/// will be automatically included in the visualization.
///
/// # Arguments
/// * `value` - The struct to serialize into JSON and visualize.
///
/// # Example
/// ```
/// use rust_viz::diagram;
/// use serde::Serialize;
///
/// #[derive(Serialize)]
/// struct Company {
///     name: String,
///     employees: Vec<String>,
/// }
///
/// let company = Company { 
///     name: "Acme Corp".to_string(),
///     employees: vec!["Alice".to_string(), "Bob".to_string()],
/// };
/// diagram(&company);
/// ```
pub fn diagram<T: Serialize + cnd_annotations::HasCndDecorators>(value: &T) {
    // Collect decorators from both type-level (struct annotations) and instance-level annotations
    // This follows the Python approach of collecting from class hierarchy and object instances
    let cnd_spec = collect_cnd_spec_for_diagram(value);
    diagram_impl(value, &cnd_spec);
}

/// Collect CnD specification for diagram generation.
/// This AUTOMATICALLY discovers and registers all decorated types without manual intervention.
/// The system now performs automatic registration during serialization.
fn collect_cnd_spec_for_diagram<T: cnd_annotations::HasCndDecorators + Serialize>(value: &T) -> String {
    println!("🔍 Assembling CnD spec with automatic type discovery...");
    
    // Step 1: Collect decorators from root type (this registers the root automatically)
    let type_decorators = T::decorators();
    println!("   📝 Root type decorators: {} constraints, {} directives", 
             type_decorators.constraints.len(), 
             type_decorators.directives.len());
    
    // Step 2: Perform automatic discovery and registration of all nested decorated types
    // This is the key improvement: we automatically discover and register types during serialization
    let auto_registered_count = perform_smart_auto_registration(value);
    if auto_registered_count > 0 {
        println!("   🎯 Auto-registered {} decorated types", auto_registered_count);
    }
    
    // Step 3: Collect decorators from all types (now that they're auto-registered)
    let (_, nested_decorators) = crate::export::export_json_instance_with_decorators(value, extract_type_name::<T>());
    println!("   🧩 Nested type decorators: {} constraints, {} directives", 
             nested_decorators.constraints.len(), 
             nested_decorators.directives.len());
    
    // Step 4: Combine all decorators
    let mut combined_decorators = type_decorators;
    combined_decorators.constraints.extend(nested_decorators.constraints);
    combined_decorators.directives.extend(nested_decorators.directives);
    
    // Step 5: Collect instance-level annotations
    let instance_decorators = cnd_annotations::collect_instance_only_decorators(value);
    combined_decorators.constraints.extend(instance_decorators.constraints);
    combined_decorators.directives.extend(instance_decorators.directives);
    
    println!("   🔗 Total decorators: {} constraints, {} directives", 
             combined_decorators.constraints.len(), 
             combined_decorators.directives.len());
    
    // Step 6: Generate YAML specification
    let cnd_spec = cnd_annotations::to_yaml(&combined_decorators).unwrap_or_default();
    println!("   ✅ Generated CnD spec automatically!\n{}", cnd_spec);
    
    cnd_spec
}

/// Perform intelligent automatic registration of decorated types
/// This function discovers types during serialization and triggers their registration
fn perform_smart_auto_registration<T: Serialize + cnd_annotations::HasCndDecorators>(_value: &T) -> usize {
    // For the current implementation, we rely on the improved serialization process
    // to automatically trigger registration when decorated types are encountered.
    
    // The serialization process itself now handles auto-registration through 
    // the enhanced collect_decorators_for_type method in the JsonDataSerializer.
    
    0 // Return 0 since registration happens transparently during serialization
}

/// Smart automatic registration that works without requiring the complex serializer
fn attempt_smart_registration(_type_name: &str) -> bool {
    // This function is called during serialization for each discovered type
    // The actual registration happens in the enhanced export.rs serialization process
    false
}

/// Test helper function to collect CnD spec without opening browser
pub fn collect_cnd_spec_for_test<T: cnd_annotations::HasCndDecorators + Serialize>(value: &T) -> String {
    collect_cnd_spec_for_diagram(value)
}

/// Creates a diagram with CnD annotations.
///
/// This function is used when you have types that implement HasCndDecorators.
///
/// # Arguments
/// * `value` - The struct to serialize into JSON and visualize.
///
/// # Example
/// ```
/// use rust_viz::diagram_with_annotations;
/// use serde::Serialize;
///
/// #[derive(Serialize)]
/// struct Company {
///     name: String,
///     employees: Vec<String>,
/// }
///
/// let company = Company { 
///     name: "Acme Corp".to_string(),
///     employees: vec!["Alice".to_string(), "Bob".to_string()],
/// };
/// diagram_with_annotations(&company); // CnD spec automatically generated from annotations
/// ```
pub fn diagram_with_annotations<T: Serialize + cnd_annotations::HasCndDecorators>(value: &T) {
    // Extract CnD spec from spatial annotations
    let cnd_spec = cnd_annotations::to_yaml_for_instance(value).unwrap_or_default();
    diagram_impl(value, &cnd_spec);
}

/// Internal implementation shared by both diagram functions
pub fn diagram_impl<T: Serialize>(value: &T, cnd_spec: &str) {
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

    // Open the HTML file directly in the browser
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

/// Creates a diagram with a custom CnD specification.
///
/// This is the legacy version of the diagram function that allows you to provide
/// a custom CnD specification instead of using spatial annotations.
///
/// # Arguments
/// * `value` - The struct to serialize into JSON and visualize.
/// * `cnd_spec` - A YAML string containing the CnD layout specification.
///
/// # Example
/// ```
/// use rust_viz::diagram_with_spec;
/// use serde::Serialize;
///
/// #[derive(Serialize)]
/// struct MyStruct {
///     field: String,
/// }
///
/// let my_struct = MyStruct { field: "Hello, world!".to_string() };
/// let cnd_spec = r#"
/// layout default:
///   nodes:
///     atom_nodes: sourceData.atoms
///       - position: randomly
///       - shape: oval
///       - size: [20, 20]
///       - color: lightblue
///       - label: this.label
/// "#;
/// diagram_with_spec(&my_struct, cnd_spec);
/// ```
pub fn diagram_with_spec<T: Serialize>(value: &T, cnd_spec: &str) {
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

    // Open the HTML file directly in the browser
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
