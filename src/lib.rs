pub mod jsondata;
pub mod export;

pub use export::export_json_instance;

use serde::Serialize;
use std::env;
use std::fs;
use std::process::Command;
use tera::{Context, Tera};
use warp::Filter;

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
    let json_instance = export_json_instance(value);
    let json_data = serde_json::to_string_pretty(&json_instance).unwrap();

    // Load the HTML template
    let tera = Tera::new("templates/*.html").expect("Failed to load templates");
    let mut context = Context::new();
    context.insert("json_data", &json_data);

    // Render the template
    let rendered_html = tera.render("template.html", &context).expect("Failed to render template");

    // Save the rendered HTML to a temporary file
    let temp_dir = env::temp_dir();
    let temp_file_path = temp_dir.join("struct_data.html");
    fs::write(&temp_file_path, rendered_html).expect("Failed to write HTML to file");

    // Serve the specific file using a lightweight web server
    let route = warp::fs::file(temp_file_path.clone());
    let server = warp::serve(route);

    // Start the server and open the URL in the browser
    let port = 3030;
    let url = format!("http://localhost:{}/struct_data.html", port);
    println!("Serving at: {}", url);

    Command::new("open")
        .arg(url)
        .spawn()
        .expect("Failed to open browser");

    tokio::runtime::Runtime::new().unwrap().block_on(server.run(([127, 0, 0, 1], port)));
}
