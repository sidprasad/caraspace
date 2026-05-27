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

/// Strict superset of [`std::dbg!`]: prints the `Debug` representation to
/// stderr *and* opens an interactive diagram of the value in your browser.
///
/// The calling convention matches `std::dbg!` exactly, so swapping
/// `std::dbg!` for `caraspace::dbg!` (or `use caraspace::dbg;`) is purely
/// additive — you keep the stderr trail you already rely on and get the
/// diagram on top.
///
/// - `dbg!()` — prints the source location, opens nothing.
/// - `dbg!(expr)` — evaluates `expr`, prints `[file:line:col] expr = …` to
///   stderr (using `{:#?}`), opens a diagram in the browser, and returns
///   the value through.
/// - `dbg!(a, b, …)` — returns a tuple `(a, b, …)`. Each argument is
///   diagrammed (opens one tab per argument).
///
/// The expression's type must derive [`std::fmt::Debug`],
/// [`serde::Serialize`], and [`SpytialDecorators`]. Both owned
/// (`dbg!(x)`) and borrowed (`dbg!(&x)`) forms work.
///
/// # Examples
///
/// ```no_run
/// use caraspace::{dbg, SpytialDecorators};
/// use serde::Serialize;
///
/// #[derive(Debug, Serialize, SpytialDecorators)]
/// #[attribute(field = "key")]
/// struct Node {
///     key: u32,
///     left: Option<Box<Node>>,
///     right: Option<Box<Node>>,
/// }
///
/// let tree = Node {
///     key: 5,
///     left: Some(Box::new(Node { key: 3, left: None, right: None })),
///     right: Some(Box::new(Node { key: 7, left: None, right: None })),
/// };
///
/// // Drop in for `std::dbg!`: prints Debug + opens a diagram,
/// // returns `tree` through for further use.
/// let tree = dbg!(tree);
/// ```
///
/// To suppress browser launch (CI, tests, headless runs), set
/// `SPYTIAL_NO_OPEN=1`. Stderr output is unaffected, so `cargo test`
/// captures still behave exactly like they would for `std::dbg!`.
#[macro_export]
macro_rules! dbg {
    () => {
        ::std::eprintln!(
            "[{}:{}:{}]",
            ::std::file!(),
            ::std::line!(),
            ::std::column!(),
        )
    };
    ($val:expr $(,)?) => {
        match $val {
            tmp => {
                ::std::eprintln!(
                    "[{}:{}:{}] {} = {:#?}",
                    ::std::file!(),
                    ::std::line!(),
                    ::std::column!(),
                    ::std::stringify!($val),
                    &tmp,
                );
                $crate::diagram(&tmp);
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($($crate::dbg!($val)),+,)
    };
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
