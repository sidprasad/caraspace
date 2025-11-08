# SpyTial Integration with Rust: A Comprehensive Guide

This document provides a detailed explanation of how SpyTial integrates with Rust, highlighting the unique approaches required for statically-typed languages.

## Table of Contents
1. [Compile-Time Annotation Collection](#compile-time-annotation-collection)
2. [Serialization-Based Structure Exposure](#serialization-based-structure-exposure)
3. [Statically-Typed Language Considerations](#statically-typed-language-considerations)
4. [Supported Field Types and Relationalization](#supported-field-types-and-relationalization)

---

## Compile-Time Annotation Collection

### Overview

Unlike dynamically-typed languages that can discover types and metadata at runtime, Rust's static type system requires a different approach. CaraSpace uses **procedural macros** to perform type analysis and annotation collection at compile time.

### How It Works

#### 1. The `SpytialDecorators` Derive Macro

The core mechanism is the `#[derive(SpytialDecorators)]` procedural macro, which:

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

1. **Attribute Parsing**: The macro scans all `#[attribute(...)]`, `#[flag(...)]`, and other SpyTial-specific attributes on the struct
2. **Field Type Walking**: It recursively analyzes all field types to discover nested structures
3. **Code Generation**: It generates an implementation of the `HasSpytialDecorators` trait that returns all decorators

#### 2. Type Tree Walking Algorithm

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

1. **Identifies collection types**: `Vec<T>`, `Option<T>`, `Box<T>`
2. **Extracts inner types**: For `Vec<Person>`, extracts `Person`
3. **Filters out primitives**: Skips `String`, `u32`, `i32`, etc.
4. **Generates decorator calls**: Creates code to call `Person::decorators()`
5. **Avoids duplicates**: Uses a `HashSet` to track already-seen types

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

#### 3. Automatic Registration

When `decorators()` is called on any type, it:

1. Uses `std::sync::Once` to ensure one-time registration
2. Stores the decorators in a global registry keyed by type name
3. Makes decorators available for later lookup during serialization

### Key Benefits for Static Languages

1. **Zero Runtime Reflection**: All type information is resolved at compile time
2. **Type Safety**: The compiler ensures all referenced types exist and implement required traits
3. **Performance**: No runtime type discovery overhead
4. **Explicit Dependencies**: The macro makes type relationships visible in generated code

---

## Serialization-Based Structure Exposure

### The Core Challenge

**Rust doesn't have runtime reflection** like Java or Python. We can't iterate over an object's fields at runtime or discover its type dynamically. Instead, Rust exposes structure through **serialization**.

### Serde: The Hook Into Structure

CaraSpace uses [Serde](https://serde.rs/), Rust's serialization framework, as the mechanism to traverse and understand data structures.

#### Why Serde?

Serde provides trait-based serialization that:

1. **Preserves Semantic Information**: Different entry points for structs vs maps vs sequences
2. **Type-Driven**: The type system guides serialization decisions
3. **Zero-Copy**: Can inspect structure without allocating
4. **Widely Adopted**: Most Rust types already implement `Serialize`

### Custom Serializer Implementation

CaraSpace implements a custom Serde serializer (`JsonDataSerializer`) that:

```rust
pub struct JsonDataSerializer {
    counter: usize,
    atoms: Vec<IAtom>,              // Collected atoms
    relations: HashMap<String, IRelation>,  // Collected relations
    collected_decorators: SpytialDecorators,    // Decorators from visited types
    visited_types: HashSet<String>,         // Prevent duplicate collection
}
```

#### How the Serializer Works

When you call `diagram(&company)`:

1. **Serialization Starts**: `company.serialize(&mut serializer)`
2. **Struct Handling**: Serde calls `serialize_struct("Company", 2)`
   - Creates an atom with type "Company"
   - **Triggers decorator collection** for type "Company"
3. **Field Serialization**: For each field, Serde calls `serialize_field("name", &value)`
   - Recursively serializes the value
   - Creates a relation named after the field
4. **Nested Types**: When encountering `Vec<Person>`:
   - Calls `serialize_seq` for the vector
   - For each element, calls `serialize_struct("Person", 2)`
   - **Triggers decorator collection** for type "Person"
5. **Relation Building**: Each structural relationship becomes a relation in the output

### Hooking Into the Serializer

The key insight is that **Serde's trait methods tell us about structure**:

| Serde Method | Structural Meaning | SpyTial Output |
|--------------|-------------------|------------|
| `serialize_struct(name, len)` | Named struct with fields | Atom of type `name`, field relations |
| `serialize_seq(len)` | Ordered collection | Atom of type "sequence", `idx` relations |
| `serialize_map(len)` | Key-value pairs | Atom of type "map", `map_entry` relations |
| `serialize_tuple(len)` | Fixed-size heterogeneous | Atom of type "tuple", `idx` relations |

### Decorator Collection During Serialization

When the serializer encounters a struct:

```rust
fn serialize_struct(&mut self, name: &str, _len: usize) -> Result<...> {
    let struct_id = self.emit_atom(name, name);
    
    // ← This is where we collect decorators!
    self.collect_decorators_for_type(name);
    
    Ok(StructSerializer { ... })
}
```

The `collect_decorators_for_type` method:

1. Checks if decorators for this type are already in the registry
2. If found, merges them into the collected set
3. Prevents duplicate collection using `visited_types` tracking

### The Complete Flow

```
User Code:
    diagram(&company)
        ↓
Lib.rs:
    1. T::decorators()        ← Compile-time generated, returns Company decorators
    2. export_json_instance(value)
        ↓
Export.rs (Custom Serializer):
    company.serialize(serializer)
        ↓
    serialize_struct("Company", ...)
        → collect_decorators_for_type("Company")  ← Already registered
        → Create atom + relations for Company
        ↓
    serialize_field("employees", &vec)
        ↓
    serialize_seq(2)
        → Create sequence atom
        ↓
    For each Person:
        serialize_struct("Person", ...)
            → collect_decorators_for_type("Person")  ← Discovered during serialization!
            → Create atom + relations for Person
```

---

## Statically-Typed Language Considerations

### Unique Challenges in Rust

#### 1. No Runtime Type Information (RTTI)

**Problem**: Can't call `object.getClass().getName()` like in Java

**Solution**: Serde provides type names through serialization methods
- `serialize_struct(name, ...)` gives us the struct name
- Type names are captured during serialization
- Stored as atom types in the relational model

#### 2. No Reflection API

**Problem**: Can't iterate `object.fields()` at runtime

**Solution**: Serde traverses structure through trait methods
- Each field causes a `serialize_field(key, value)` call
- The `key` parameter provides the field name
- Field values are recursively serialized

#### 3. Monomorphization

**Problem**: Generic types like `Vec<Person>` are monomorphized at compile time. Each instantiation is a separate type.

**Solution**: The procedural macro analyzes generic parameters
- Extracts `T` from `Vec<T>`, `Option<T>`, `Box<T>`
- Generates calls for the concrete type `T`
- Works with nested generics: `Vec<Option<Box<Person>>>`

#### 4. Trait Bounds and Orphan Rules

**Problem**: Can't implement external traits for external types

**Solution**: The `HasSpytialDecorators` trait is local
- User types opt-in via `#[derive(SpytialDecorators)]`
- Only works for types in the same crate or types that derive the macro
- Can't add decorators to `String` or `Vec<T>` directly

#### 5. Lifetime and Ownership

**Problem**: Serialization needs to borrow data without taking ownership

**Solution**: Serde uses `&self` references throughout
- `fn serialize<S: Serializer>(&self, serializer: S)`
- No copying or cloning required
- Zero-cost abstraction over the structure

### Design Decisions Specific to Static Typing

#### Compile-Time vs Runtime Trade-offs

| Aspect | Dynamic Languages (Python) | Static Languages (Rust) |
|--------|---------------------------|-------------------------|
| Type Discovery | Runtime inspection | Compile-time macro analysis |
| Decorator Collection | Walk object at runtime | Generate calls at compile time |
| Type Registry | Build during runtime traversal | Register via `Once` on first call |
| Performance | Runtime overhead for inspection | Zero-cost after compilation |
| Flexibility | Can handle any type | Only works with opt-in types |

#### The Registration Pattern

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
- Uses `std::sync::Once` for thread-safe one-time initialization
- Registers decorators in a global `HashMap<String, SpytialDecorators>`
- Allows later lookup by type name during serialization

---

## Supported Field Types and Relationalization

### Type Categories

CaraSpace categorizes Rust types into semantic groups that determine how they're relationalized.

### 1. Primitive Types

**Types**: `i8`, `i16`, `i32`, `i64`, `i128`, `u8`, `u16`, `u32`, `u64`, `u128`, `f32`, `f64`, `bool`, `char`, `String`

**Relationalization**: Become leaf atoms with their value as the label

```rust
struct Data {
    count: i32,    // → Atom { type: "i32", label: "42" }
    name: String,  // → Atom { type: "string", label: "Alice" }
}
```

**Decorator Collection**: Primitives don't have decorators (no custom types)

### 2. Struct Types (Named Fields)

**Pattern**: Each field becomes its own relation named after the field

```rust
#[derive(Serialize, SpytialDecorators)]
#[attribute(field = "name")]
struct Person {
    name: String,
    age: u32,
}
```

**Relationalization**:
```json
{
  "atoms": [
    {"id": "person_id", "type": "Person", "label": "Person"},
    {"id": "name_id", "type": "string", "label": "Alice"},
    {"id": "age_id", "type": "i32", "label": "30"}
  ],
  "relations": [
    {
      "name": "name",
      "types": ["Person", "string"],
      "tuples": [{"atoms": ["person_id", "name_id"]}]
    },
    {
      "name": "age",
      "types": ["Person", "i32"],
      "tuples": [{"atoms": ["person_id", "age_id"]}]
    }
  ]
}
```

**Why this pattern?**
- Field names have semantic meaning (not just "field_0", "field_1")
- Enables field-specific SpyTial rules: `name: position: as_label`
- Supports querying by relationship: "find all names", "find all ages"

**Decorator Collection**: Triggered when `serialize_struct("Person", ...)` is called

### 3. Sequences (Vec, Arrays, Slices)

**Types**: `Vec<T>`, `[T; N]`, `&[T]`, `VecDeque<T>`

**Pattern**: Positional indexing via `idx(container, index, element)`

```rust
struct Company {
    employees: Vec<Person>,
}
```

**Relationalization**:
```json
{
  "relations": [
    {
      "name": "idx",
      "types": ["sequence", "index", "Person"],
      "tuples": [
        {"atoms": ["vec_id", "0", "person1_id"]},
        {"atoms": ["vec_id", "1", "person2_id"]},
        {"atoms": ["vec_id", "2", "person3_id"]}
      ]
    }
  ]
}
```

**Why this pattern?**
- Preserves O(1) positional access semantics
- Index is meaningful: first employee, second employee, etc.
- Enables grid layouts, ordinal positioning in SpyTial

**Decorator Collection**: For `Vec<Person>`, the macro extracts `Person` and generates a call to collect its decorators

### 4. Tuples (Heterogeneous Fixed-Size)

**Types**: `(T1, T2, ...)`, `(T1, T2, T3, T4, ...)`

**Pattern**: Same as sequences, but type is "tuple"

```rust
struct Point {
    coordinates: (f64, f64),
    metadata: (String, i32, bool),
}
```

**Relationalization**:
```json
{
  "relations": [
    {
      "name": "idx",
      "types": ["tuple", "index", "f64"],
      "tuples": [
        {"atoms": ["tuple_id", "0", "x_coord"]},
        {"atoms": ["tuple_id", "1", "y_coord"]}
      ]
    }
  ]
}
```

**Why same as sequences?**
- Both have positional semantics
- Fixed positions have meaning: (x, y), (red, green, blue)
- Tuples are heterogeneous, but indexing is still the access pattern

### 5. Maps (Associative Collections)

**Types**: `HashMap<K, V>`, `BTreeMap<K, V>`, `IndexMap<K, V>`

**Pattern**: Ternary relations `map_entry(map, key, value)`

```rust
struct Config {
    settings: HashMap<String, i32>,
}
```

**Relationalization**:
```json
{
  "relations": [
    {
      "name": "map_entry",
      "types": ["map", "string", "i32"],
      "tuples": [
        {"atoms": ["map_id", "key1_id", "value1_id"]},
        {"atoms": ["map_id", "key2_id", "value2_id"]}
      ]
    }
  ]
}
```

**Why different from structs?**
- Keys are **data** (computed at runtime), not metadata
- Dynamic key sets vs fixed field names
- Associative lookup semantics: `map[key] → value`

**Decorator Collection**: Map types themselves don't have decorators, but key/value types might

### 6. Option Types

**Types**: `Option<T>`

**Pattern**: Transparent - serializes the inner value or "None"

```rust
struct Person {
    nickname: Option<String>,
}
```

**Relationalization**:
- `Some("Bob")` → serializes as if it were `"Bob"` directly
- `None` → creates an atom with type "option" and label "None"

**Decorator Collection**: For `Option<Person>`, the macro extracts `Person`

### 7. Box and Smart Pointers

**Types**: `Box<T>`, `Rc<T>`, `Arc<T>`

**Pattern**: Transparent - serializes the inner value

```rust
struct Node {
    next: Option<Box<Node>>,
}
```

**Decorator Collection**: For `Box<Node>`, the macro extracts `Node`

### 8. Tuple Structs (Positional Named Types)

**Types**: `struct Point(f64, f64)`, `struct Color(u8, u8, u8, u8)`

**Pattern**: Positional indexing like tuples

```rust
#[derive(Serialize)]
struct Point(f64, f64);
```

**Relationalization**:
```json
{
  "relations": [
    {
      "name": "idx",
      "types": ["Point", "index", "f64"],
      "tuples": [
        {"atoms": ["point_id", "0", "x_val"]},
        {"atoms": ["point_id", "1", "y_val"]}
      ]
    }
  ]
}
```

**Why positional?**
- Access is by position: `point.0`, `point.1`
- Named type provides semantic context
- Common for coordinate types, RGB colors

### 9. Enums

**Types**: Rust enums with variants

**Patterns**: Depends on variant type

```rust
#[derive(Serialize)]
enum Shape {
    Circle { radius: f64 },           // Struct variant
    Rectangle(f64, f64),               // Tuple variant
    Point,                             // Unit variant
}
```

**Relationalization**:
- **Struct variants**: Like structs, field names become relations
- **Tuple variants**: Like tuples, positional indexing
- **Unit variants**: Single atom with variant name

### Complex Nested Types

The system handles arbitrarily nested types:

```rust
struct Organization {
    departments: Vec<Department>,
    executives: HashMap<String, Box<Person>>,
    archive: Option<Vec<Box<Department>>>,
}
```

**Type Walking**:
1. `Vec<Department>` → analyzes `Department`
2. `HashMap<String, Box<Person>>` → analyzes `Box<Person>` → analyzes `Person`
3. `Option<Vec<Box<Department>>>` → analyzes `Vec<Box<Department>>` → analyzes `Department`

**Deduplication**: Each type is only analyzed once (tracked by `seen_types: HashSet<String>`)

### Summary Table

| Type Pattern | Relation Pattern | Example | Why? |
|--------------|------------------|---------|------|
| Primitives | Leaf atoms | `i32` → `Atom{type: "i32"}` | No internal structure |
| Structs | Field-named relations | `name(person, str)` | Semantic field names |
| Vec/Arrays | `idx(container, pos, elem)` | `idx(vec, "0", item)` | Positional access |
| Tuples | `idx(container, pos, elem)` | `idx(tuple, "1", val)` | Fixed positions |
| Maps | `map_entry(map, key, val)` | `map_entry(m, k, v)` | Associative lookup |
| Options | Transparent or None | `Some(x)` → like `x` | Wrapper type |
| Box | Transparent | `Box<T>` → like `T` | Pointer indirection |
| Enums | Variant-dependent | Struct/Tuple/Unit | Multiple representations |

---

## Conclusion

The Rust integration with SpyTial demonstrates how static typing and lack of reflection require fundamentally different approaches:

1. **Compile-Time Analysis**: Procedural macros walk the type tree at compile time, generating decorator collection code
2. **Serialization as Structure Discovery**: Serde provides the hook into data structure traversal
3. **Explicit Opt-In**: Types must derive `SpytialDecorators` - no global reflection
4. **Registration Pattern**: Thread-safe one-time registration via `std::sync::Once`
5. **Semantic Preservation**: Different collection types use different relationalization patterns
6. **Type Safety**: The compiler ensures all references are valid

This design trades runtime flexibility for compile-time guarantees, zero-cost abstractions, and type safety - perfectly aligned with Rust's philosophy.
