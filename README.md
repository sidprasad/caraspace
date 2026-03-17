# CaraSpace

Spytial for Rust.

The crate is published as `caraspace`.

## Documentation

- [User Guide](./USER_GUIDE.md) for installation, quick start, and common workflows
- [Design Notes](./doc.md) for the compile-time and serialization internals

## How Compile-Time Annotation Collection Works

Uses **procedural macro analysis** to walk the entire type tree at compile time:

1. **Field Analysis**: The `SpytialDecorators` derive macro analyzes all fields in your struct
2. **Type Tree Walking**: For each field, it recursively analyzes the type:
   - `Vec<T>` → analyzes `T`
   - `Option<T>` → analyzes `T` 
   - `Box<T>` → analyzes `T`
   - Direct types → analyzes the type itself
3. **Decorator Collection**: For each discovered type that implements `HasSpytialDecorators`, it generates code to include those decorators

## Library Structure

```
src/
├── lib.rs                    # Main API with cleaned up functions
├── export.rs                 # JSON serialization with type information
├── jsondata.rs               # Custom JSON data structures
└── spytial_annotations/
    ├── mod.rs                # SpyTial decorator system
    └── runtime.rs            # Runtime builder with compile-time support

macros/
└── src/
    └── lib.rs                # Compile-time type analysis procedural macro
```

## API Reference

### Core Functions

- `diagram<T>(value: &T)` - Create visualization with automatic decorator collection
- `diagram_with_spec<T>(value: &T, spec: &str)` - Create visualization with a custom SpyTial spec (escape hatch)
- `export_json_instance<T>(value: &T)` - Export Rust data to the relational JSON format used by Spytial

### Decorator Attributes

- `#[attribute(field = "field_name")]` - Mark a field as an attribute
- `#[flag(name = "flag_name")]` - Add a boolean flag
- `#[orientation(selector = "...", directions = [...])]` - Relative positioning constraint
- `#[align(selector = "...", direction = "...")]` - Horizontal or vertical alignment
- `#[cyclic(selector = "...", direction = "...")]` - Cyclic ordering constraint
- `#[group(...)]` - Grouping constraint
- Visual directives such as `#[atom_color(...)]`, `#[size(...)]`, `#[icon(...)]`, `#[hide_atom(...)]`

### Supported Field Types

The compile-time analysis supports:
- Direct types: `Person`, `MyStruct`
- Collections: `Vec<T>`, `Vec<Box<T>>`
- Options: `Option<T>`, `Option<Box<T>>`
- Boxed types: `Box<T>`
- Nested combinations: `Vec<Option<Box<Person>>>`



## Representative Examples

- `cargo run --example demo` shows decorator collection on nested business-domain structs.
- `cargo run --example rbt` builds an insertion-balanced red-black tree (LLRB style) and renders it with node color/layout decorators.

## Development

```bash
cargo test --lib --tests
cargo test --doc
cargo run --example demo
cargo run --example rbt
```

## Docker

Build the image:

```bash
docker build -t caraspace .
```

Run the default red-black tree example:

```bash
docker run --rm -p 8080:8080 caraspace
```

Run a different example:

```bash
docker run --rm -p 8080:8080 caraspace demo
```

When the example finishes, the container starts a small HTTP server and logs:

`Visualization server ready at http://localhost:8080/rust_viz_data.html`

Open that URL in your host browser. Browser launch inside the container stays disabled (`SPYTIAL_NO_OPEN=1`).


## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.


## Data Dup

The Rust mental model we're following:

Zero-sized types → Singletons (they have no data, only type/variant identity)
Types with data → Multiple atoms (each occurrence is a distinct value)
