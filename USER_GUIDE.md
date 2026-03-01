# CaraSpace User Guide

CaraSpace is the Rust-facing way to generate Spytial diagrams from Rust data.

This repository is named `caraspace`, while the current Cargo package name is `json_data_instance_export`.

## What You Get

- Serde-based export from Rust values into the relational JSON shape expected by Spytial
- A derive macro, `SpytialDecorators`, for attaching layout and display metadata to Rust types
- `diagram(&value)` for quickly rendering a local HTML visualization in the browser
- `diagram_with_spec(&value, spec)` when you want to hand-write the Spytial YAML

## Install

Add the crate to your project:

```toml
[dependencies]
json_data_instance_export = { path = "/path/to/caraspace" }
serde = { version = "1", features = ["derive"] }
```

If you are consuming a published version later, replace the `path` dependency with the crate version.

## Quick Start

```rust
use json_data_instance_export::{diagram, SpytialDecorators};
use serde::Serialize;

#[derive(Serialize, SpytialDecorators)]
#[attribute(field = "name")]
struct Company {
    name: String,
    employees: Vec<Person>,
}

#[derive(Serialize, SpytialDecorators)]
#[attribute(field = "name")]
#[align(selector = "reports_to", direction = "horizontal")]
struct Person {
    name: String,
    reports_to: Option<Box<Person>>,
}

fn main() {
    let company = Company {
        name: "Acme".to_string(),
        employees: vec![Person {
            name: "Alice".to_string(),
            reports_to: None,
        }],
    };

    diagram(&company);
}
```

## Core Workflow

1. Derive `Serialize` and `SpytialDecorators` on the Rust types you want to visualize.
2. Attach layout or styling attributes to those types.
3. Build a normal Rust value.
4. Call `diagram(&value)` for the generated spec, or `diagram_with_spec(&value, spec)` for a custom one.

## Main API

### `diagram`

```rust
use json_data_instance_export::diagram;
```

Uses compile-time decorator collection and opens a generated HTML file in the default browser.

### `diagram_with_spec`

```rust
use json_data_instance_export::diagram_with_spec;
```

Useful when you want to bypass derive-generated constraints and provide explicit YAML:

```rust
let spec = r#"
constraints:
  - align:
      selector: reports_to
      direction: horizontal
directives:
  - flag: hideDisconnected
"#;
```

### `export_json_instance`

```rust
use json_data_instance_export::export_json_instance;
```

Exports your Rust value without opening a browser. This is the right entry point if you want to integrate CaraSpace into another tool.

## Supported Decorator Attributes

These map onto the Rust builder and YAML serialization layer in `spytial_annotations`.

| Attribute | Purpose |
|----------|---------|
| `#[attribute(field = "...")]` | Show a field as an attribute |
| `#[flag(name = "...")]` | Set a global display flag |
| `#[orientation(selector = "...", directions = [...])]` | Relative layout constraint |
| `#[align(selector = "...", direction = "horizontal" \| "vertical")]` | Force alignment |
| `#[cyclic(selector = "...", direction = "...")]` | Cyclic ordering |
| `#[group(...)]` | Group by selector or field |
| `#[atom_color(selector = "...", value = "...")]` | Color nodes |
| `#[size(selector = "...", height = 40, width = 60)]` | Set node size |
| `#[icon(selector = "...", path = "...", show_labels = true)]` | Set node icon |
| `#[edge_color(field = "...", value = "...")]` | Color edges |
| `#[projection(sig = "...")]` | Projection directive |
| `#[hide_field(field = "...")]` | Hide a relation |
| `#[hide_atom(selector = "...")]` | Hide selected atoms |
| `#[inferred_edge(name = "...", selector = "...")]` | Add inferred edges |

## Compile-Time Behavior

The derive macro walks common container types and includes decorators from nested types automatically.

Supported traversal patterns:

- `Vec<T>`
- `Option<T>`
- `Box<T>`
- nested combinations such as `Vec<Option<Box<T>>>`

This means decorating `Person` is usually enough for those decorators to appear when `Company` contains `Vec<Person>`.

## Runtime Annotations

If you need imperative control, use the `spytial_annotations` module directly:

```rust
use json_data_instance_export::spytial_annotations::{annotate_instance, AnnotationBuilder};
```

That path is better suited to advanced integrations than to the normal “derive and render” workflow.

## Recommended Development Commands

```bash
cargo test --lib --tests
cargo test --doc
cargo run --example demo
```

## Limitations and Expectations

- The browser rendering path relies on the embedded HTML template and CDN-hosted `spytial-core`.
- Compile-time type walking is intentionally conservative; it handles common wrappers, not arbitrary type-level programming.
- The crate name and repository name do not currently match. In Cargo code samples, use `json_data_instance_export`.

## Where To Read Next

- [README.md](./README.md) for the project overview
- [doc.md](./doc.md) for the design rationale and implementation details
