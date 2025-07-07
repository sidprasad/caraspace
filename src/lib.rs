pub mod jsondata;
pub mod export;

pub use export::export_json_instance;

use serde::Serialize;
use std::env;
use std::fs;
use std::process::Command;

/// Prints the given JSON and opens it in the browser.
///
/// # Arguments
/// * `value` - The struct to serialize into JSON.
/// * `extra` - A string argument (currently unused, pass an empty string).
///
/// # Example
/// ```
/// use json_data_instance_export::printcnd;
///
/// #[derive(Serialize)]
/// struct MyStruct {
///     field: String,
/// }
///
/// let my_struct = MyStruct { field: "Hello, world!".to_string() };
/// printcnd(&my_struct, "");
/// ```
pub fn printcnd<T: Serialize>(value: &T, _extra: &str) {
    // Serialize the struct to JSON
    let json_data = serde_json::to_string_pretty(value).unwrap();

    // Create a temporary file path
    let temp_dir = env::temp_dir();
    let temp_file_path = temp_dir.join("struct_data.json");

    // Write JSON data to the temporary file
    fs::write(&temp_file_path, json_data).expect("Failed to write JSON to temporary file");

    // Debug: Print the file path
    println!("Opening file at: {:?}", temp_file_path);

    // Open the file in the default browser
    Command::new("open")
        .arg(temp_file_path)
        .spawn()
        .expect("Failed to open browser");

    // Currently, `extra` is unused, but it can be extended for future functionality.
}
