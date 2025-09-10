# JSON Data Instance Export with Compile-Time Decorator Collection

A Rust library for exporting data structures to JSON with automatic visualization capabilities.

## How Compile-Time Collection Works

Our system uses **procedural macro analysis** to walk the entire type tree at compile time:

1. **Field Analysis**: The `CndDecorators` derive macro analyzes all fields in your struct
2. **Type Tree Walking**: For each field, it recursively analyzes the type:
   - `Vec<T>` → analyzes `T`
   - `Option<T>` → analyzes `T` 
   - `Box<T>` → analyzes `T`
   - Direct types → analyzes the type itself
3. **Decorator Collection**: For each discovered type that implements `HasCndDecorators`, it generates code to include those decorators
4. **Deduplication**: Uses a `HashSet` to prevent including the same type multiple times

### Generated Code Example

When you derive `CndDecorators` on `Company`, the macro generates:

```rust
impl HasCndDecorators for Company {
    fn decorators() -> CndDecorators {
        let mut builder = CndDecoratorsBuilder::new();
        
        // Add Company's own decorators
        builder.add_attribute_directive("name");
        builder.add_flag_directive("hideDisconnected");
        
        // Automatically include Person decorators (compile-time discovered!)
        builder.include_decorators_from_type::<Person>();
        
        builder.build()
    }
}
```

## Library Structure

```
src/
├── lib.rs                    # Main API with cleaned up functions
├── export.rs                 # JSON serialization with type information
├── jsondata.rs               # Custom JSON data structures
└── cnd_annotations/
    ├── mod.rs                # CnD decorator system
    └── runtime.rs            # Runtime builder with compile-time support

macros/
└── src/
    └── lib.rs                # Compile-time type analysis procedural macro
```

## 🔧 API Reference

### Core Functions

- `diagram<T>(value: &T)` - Create visualization with automatic decorator collection
- `diagram_with_spec<T>(value: &T, cnd_spec: &str)` - Create visualization with custom CnD spec

### Decorator Attributes

- `#[attribute(field = "field_name")]` - Mark a field as an attribute
- `#[flag(name = "flag_name")]` - Add a boolean flag
- `#[constraint(...)]` - Add layout constraints (coming soon)

### Supported Field Types

The compile-time analysis supports:
- Direct types: `Person`, `MyStruct`
- Collections: `Vec<T>`, `Vec<Box<T>>`
- Options: `Option<T>`, `Option<Box<T>>`
- Boxed types: `Box<T>`
- Nested combinations: `Vec<Option<Box<Person>>>`



## 🛠 Development

Run the example:
```bash
cargo run --example demo
```

Check for issues:
```bash
cargo check
```

The example demonstrates a `Company` with `Vec<Person>` where both types have decorators, and shows how all decorators are automatically collected without manual registration.


## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.