pub mod jsondata;
pub mod export;

pub use export::export_json_instance;
use serde::Serialize;
use std::env;
use std::fs;
use std::process::Command;

/// Prints the given JSON data and opens it in the browser as a visualization.
///
/// This function serializes the given struct to JSON, embeds it in an HTML template,
/// writes it to a temporary file, and opens it in the default browser.
///
/// # Arguments
/// * `value` - The struct to serialize into JSON and visualize.
/// * `cnd_spec` - A YAML string containing the CnD layout specification.
///
/// # Example
/// ```
/// use rust_viz::visualize;
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
///   edges:
///     relation_edges: sourceData.relations ->* tuples
///       - source: this.atoms[0]
///       - target: this.atoms[1]
///       - color: gray
///       - label: parent.name
/// "#;
/// visualize(&my_struct, cnd_spec);
/// ```
pub fn visualize<T: Serialize>(value: &T, cnd_spec: &str) {
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

/// Legacy alias for `visualize` function to maintain backward compatibility.
/// 
/// # Deprecated
/// Use `visualize` instead. This function will be removed in a future version.
pub fn printcnd<T: Serialize>(value: &T, cnd_spec: &str) {
    visualize(value, cnd_spec);
}
