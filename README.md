# rust-viz

A lightweight Rust crate for data visualization using Serde serialization and CnD (Constraint-based Network Diagrams).

## Features

- **Lightweight**: No heavy server dependencies - just opens HTML files directly in the browser
- **Serde Integration**: Automatically visualize any serializable Rust data structure
- **No CORS Issues**: Generated HTML files can be opened locally without a web server
- **Standard Rust Patterns**: Follows Rust naming conventions and idiomatic design

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
rust-viz = "0.1.0"
```

Then use it in your code:

```rust
use rust_viz::visualize;
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
    visualize(&company, "");
}
```

## How it Works

1. **Serialization**: Your Rust data structure is serialized using Serde
2. **JSON Transformation**: The data is converted to a graph-compatible JSON format
3. **HTML Generation**: An HTML file with embedded visualization is created
4. **Browser Display**: The file is opened directly in your default browser

## Migration from Previous Versions

If you were using the old `printcnd` function, you can either:
- Use the new `visualize` function (recommended)
- Continue using `printcnd` as a legacy alias (will be deprecated)

## Requirements

- Rust 2021 Edition or later
- A default web browser for displaying visualizations

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.