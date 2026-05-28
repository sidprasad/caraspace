# Compile-time decorator collection

## Overview

Unlike dynamically-typed languages that can discover types and metadata
at runtime, Rust's static type system requires a different approach.
CaraSpace uses **procedural macros** to perform type analysis and
annotation collection at compile time.

## How it works

### 1. The `SpytialDecorators` derive macro

The core mechanism is the `#[derive(SpytialDecorators)]` procedural
macro, which:

```rust
#[derive(Serialize, SpytialDecorators)]
#[attribute(field = "name")]
#[flag(name = "important")]
struct Company {
    name: String,
    employees: Vec<Person>,
}
```

When the compiler processes this derive macro:

1. **Attribute parsing.** The macro scans all `#[attribute(...)]`,
   `#[flag(...)]`, and other SpyTial-specific attributes on the struct.
2. **Field type walking.** It recursively analyzes all field types to
   discover nested structures.
3. **Code generation.** It generates an implementation of the
   `HasSpytialDecorators` trait that returns all decorators.

### 2. Type tree walking algorithm

The macro performs a compile-time analysis of the type structure:

```rust
// Given this structure:
struct Company {
    name: String,
    employees: Vec<Person>,  // ← The macro analyzes this
}

struct Person {
    name: String,
    age: u32,
}
```

The macro's type-walking algorithm:

1. **Identifies collection types**: `Vec<T>`, `Option<T>`, `Box<T>`.
2. **Extracts inner types**: For `Vec<Person>`, extracts `Person`.
3. **Filters out primitives**: Skips `String`, `u32`, `i32`, etc.
4. **Generates decorator calls**: Creates code to call
   `Person::decorators()`.
5. **Avoids duplicates**: Uses a `HashSet` to track already-seen types.

```rust
// Generated code conceptually looks like:
impl HasSpytialDecorators for Company {
    fn decorators() -> SpytialDecorators {
        SpytialDecoratorsBuilder::new()
            .attribute("name", None)
            .include_decorators_from_type::<Person>()  // ← Auto-generated
            .build()
    }
}
```

### 3. Automatic registration

When `decorators()` is called on any type, it:

1. Uses `std::sync::Once` to ensure one-time registration.
2. Stores the decorators in a global registry keyed by type name.
3. Makes decorators available for later lookup during serialization.

## Key benefits for static languages

1. **Zero runtime reflection.** All type information is resolved at
   compile time.
2. **Type safety.** The compiler ensures all referenced types exist and
   implement required traits.
3. **Performance.** No runtime type discovery overhead.
4. **Explicit dependencies.** The macro makes type relationships
   visible in generated code.

## Statically-typed considerations

A handful of Rust-specific constraints shape how the macro is written.

### No runtime type information (RTTI)

You can't call `object.getClass().getName()` like in Java. Instead, the
macro stamps type names into the generated code at compile time, and
serde provides type names through its `serialize_struct(name, ...)` call
during traversal. Those captured names become atom types in the
relational model.

### No reflection API

You can't iterate `object.fields()` at runtime. Serde traverses
structure through trait methods: each field causes a
`serialize_field(key, value)` call, the `key` parameter provides the
field name, and field values are recursively serialized.

### Monomorphization

Generic types like `Vec<Person>` are monomorphized at compile time —
each instantiation is a separate type. The procedural macro analyzes
generic parameters: it extracts `T` from `Vec<T>`, `Option<T>`,
`Box<T>`, generates calls for the concrete type `T`, and works with
nested generics like `Vec<Option<Box<Person>>>`.

### Trait bounds and orphan rules

You can't implement external traits for external types. The
`HasSpytialDecorators` trait is local, and user types opt in via
`#[derive(SpytialDecorators)]`. This means caraspace only works for
types in the same crate or types that derive the macro — you can't add
decorators to `String` or `Vec<T>` directly.

### Lifetime and ownership

Serialization needs to borrow data without taking ownership. Serde uses
`&self` references throughout: `fn serialize<S: Serializer>(&self, serializer: S)`.
No copying or cloning is required, and the structure-walking is a
zero-cost abstraction over the data.

## Compile-time vs runtime trade-offs

| Aspect | Dynamic Languages (Python) | Static Languages (Rust) |
|--------|---------------------------|-------------------------|
| Type discovery | Runtime inspection | Compile-time macro analysis |
| Decorator collection | Walk object at runtime | Generate calls at compile time |
| Type registry | Build during runtime traversal | Register via `Once` on first call |
| Performance | Runtime overhead for inspection | Zero-cost after compilation |
| Flexibility | Can handle any type | Only works with opt-in types |

## The registration pattern

In statically-typed languages, we need explicit registration:

```rust
impl HasSpytialDecorators for Company {
    fn decorators() -> SpytialDecorators {
        // Register on first call
        static REGISTRATION: std::sync::Once = std::sync::Once::new();
        REGISTRATION.call_once(|| {
            let decorators = /* ... build decorators ... */;
            register_type_decorators("Company", decorators.clone());
        });

        // Return decorators (possibly re-building)
        /* ... */
    }
}
```

This pattern:

- Uses `std::sync::Once` for thread-safe one-time initialization.
- Registers decorators in a global `HashMap<String, SpytialDecorators>`.
- Allows later lookup by type name during serialization.
