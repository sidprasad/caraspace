# rust-viz

A lightweight Rust crate for data visualization using Serde serialization and CnD (Constraint-based Network Diagrams).

## Features

- **Lightweight**: No heavy server dependencies - just opens HTML files directly in the browser
- **Serde Integration**: Automatically visualize any serializable Rust data structure
- **No CORS Issues**: Generated HTML files can be opened locally without a web server
- **Standard Rust Patterns**: Follows Rust naming conventions and idiomatic design
- **Spytial Annotations**: Runtime annotation system equivalent to Python sPyTial decorators

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
rust-viz = "0.1.0"
```

### Basic Visualization

```rust
use rust_viz::diagram;
use serde::Serialize;

#[derive(Serialize)]
struct Person {
    name: String,
    age: u32,
}

#[derive(Serialize)]
struct Company {
    name: String,
    employees: Vec<Person>,
}

fn main() {
    let company = Company {
        name: "Acme Corp".into(),
        employees: vec![
            Person { name: "Alice".into(), age: 30 },
            Person { name: "Bob".into(), age: 25 },
        ],
    };

    // This will generate an HTML file and open it in your default browser
    diagram(&company, "");
}
```

### Spytial Annotations

The spytial annotations system provides a runtime annotation system equivalent to Python sPyTial decorators, allowing you to add visualization constraints and directives to your data structures.

```rust
use rust_viz::spytial_annotations::{
    SpytialDecoratorsBuilder, AnnotationBuilder, HasSpytialDecorators,
    annotate_instance, to_yaml_for_type, to_yaml_for_instance,
};

#[derive(Debug)]
struct Node {
    value: String,
    children: Vec<Node>,
}

impl HasSpytialDecorators for Node {
    fn decorators() -> rust_viz::spytial_annotations::SpytialDecorators {
        SpytialDecoratorsBuilder::new()
            .orientation("value", vec!["above"])
            .group_field_based("children", 0, 1, None)
            .atom_color("value", "lightblue")
            .flag("example_node")
            .build()
    }
}

fn main() {
    // Type-level decorators
    let yaml = to_yaml_for_type::<Node>().unwrap();
    println!("Type decorators:\n{}", yaml);

    // Instance-level annotations
    let mut node = Node {
        value: "Root".to_string(),
        children: vec![],
    };

    // Add runtime annotations
    annotate_instance(&mut node, AnnotationBuilder::orientation("self.children", vec!["horizontal"]));
    annotate_instance(&mut node, AnnotationBuilder::atom_color("self.value", "red"));

    // Get combined decorators (type + instance)
    let combined_yaml = to_yaml_for_instance(&node).unwrap();
    println!("Combined decorators:\n{}", combined_yaml);
}
```

#### Available Annotations

**Constraints** (layout/structural):
- `orientation(selector, directions)` - Specify spatial orientation
- `cyclic(selector, direction)` - Define cyclic layout
- `group_field_based(field, groupOn, addToGroup, selector?)` - Group by field
- `group_selector_based(selector, name)` - Group by selector

**Directives** (visual/behavioral):
- `atom_color(selector, value)` - Set atom colors
- `size(selector, height, width)` - Set element size
- `icon(selector, path, showLabels)` - Add icons
- `edge_color(field, value, selector?)` - Set edge colors
- `projection(sig)` - Define projections
- `attribute(field, selector?)` - Add attributes
- `hide_field(field, selector?)` - Hide fields
- `hide_atom(selector)` - Hide atoms
- `inferred_edge(name, selector)` - Add inferred edges
- `flag(name)` - Add flags

#### Builder Pattern

Use `SpytialDecoratorsBuilder` to create type-level decorators:

```rust
let decorators = SpytialDecoratorsBuilder::new()
    .orientation("items", vec!["vertical", "stack"])
    .group_selector_based("self.elements", "main_group")
    .atom_color("nodes", "#ff0000")
    .size("labels", 100, 50)
    .flag("custom_layout")
    .build();
```

#### Runtime Annotations

Use `AnnotationBuilder` to create instance-level annotations:

```rust
// Add annotations to instances at runtime
annotate_instance(&mut obj, AnnotationBuilder::orientation("self", vec!["horizontal"]));
annotate_instance(&mut obj, AnnotationBuilder::flag("runtime_modified"));

// Collect all decorators (type + instance)
let all_decorators = collect_decorators_for_instance(&obj);
```

#### YAML Serialization

All decorators serialize to YAML format compatible with Python sPyTial:

```yaml
constraints:
- orientation:
    selector: value
    directions:
    - above
- group:
    field: children
    groupOn: 0
    addToGroup: 1
directives:
- atomColor:
    selector: value
    value: lightblue
- flag: example_node
```

## How it Works

1. **Serialization**: Your Rust data structure is serialized using Serde
2. **JSON Transformation**: The data is converted to a graph-compatible JSON format
3. **HTML Generation**: An HTML file with embedded visualization is created
4. **Browser Display**: The file is opened directly in your default browser

## Migration from Previous Versions

If you were using the old `printcnd` function, you can either:
- Use the new `diagram` function (recommended)
- Continue using `printcnd` as a legacy alias (will be deprecated)

## Requirements

- Rust 2021 Edition or later
- A default web browser for displaying visualizations


## THoughts:

CaraSpace: “the shell that reveals spatial structure”

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.