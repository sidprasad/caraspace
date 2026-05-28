# Supported field types

## Type categories

CaraSpace categorizes Rust types into semantic groups that determine how
they're relationalized.

## 1. Primitive types

**Types**: `i8`, `i16`, `i32`, `i64`, `i128`, `u8`, `u16`, `u32`, `u64`,
`u128`, `f32`, `f64`, `bool`, `char`, `String`.

**Relationalization**: Become leaf atoms with their value as the label.

```rust
struct Data {
    count: i32,    // → Atom { type: "i32", label: "42" }
    name: String,  // → Atom { type: "string", label: "Alice" }
}
```

**Decorator collection**: Primitives don't have decorators (no custom
types).

## 2. Struct types (named fields)

**Pattern**: Each field becomes its own relation named after the field.

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

- Field names have semantic meaning (not just "field_0", "field_1").
- Enables field-specific SpyTial rules: `name: position: as_label`.
- Supports querying by relationship: "find all names", "find all ages".

**Decorator collection**: Triggered when
`serialize_struct("Person", ...)` is called.

## 3. Sequences (Vec, arrays, slices)

**Types**: `Vec<T>`, `[T; N]`, `&[T]`, `VecDeque<T>`.

**Pattern**: Positional indexing via `idx(container, index, element)`.

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

- Preserves O(1) positional access semantics.
- Index is meaningful: first employee, second employee, etc.
- Enables grid layouts, ordinal positioning in SpyTial.

**Decorator collection**: For `Vec<Person>`, the macro extracts `Person`
and generates a call to collect its decorators.

## 4. Tuples (heterogeneous fixed-size)

**Types**: `(T1, T2, ...)`, `(T1, T2, T3, T4, ...)`.

**Pattern**: Same as sequences, but type is "tuple".

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

- Both have positional semantics.
- Fixed positions have meaning: (x, y), (red, green, blue).
- Tuples are heterogeneous, but indexing is still the access pattern.

## 5. Maps (associative collections)

**Types**: `HashMap<K, V>`, `BTreeMap<K, V>`, `IndexMap<K, V>`.

**Pattern**: Ternary relations `map_entry(map, key, value)`.

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

- Keys are **data** (computed at runtime), not metadata.
- Dynamic key sets vs fixed field names.
- Associative lookup semantics: `map[key] → value`.

**Decorator collection**: Map types themselves don't have decorators,
but key/value types might.

## 6. Option types

**Types**: `Option<T>`.

**Pattern**: Transparent — serializes the inner value or "None".

```rust
struct Person {
    nickname: Option<String>,
}
```

**Relationalization**:

- `Some("Bob")` → serializes as if it were `"Bob"` directly.
- `None` → creates an atom with type "option" and label "None".

**Decorator collection**: For `Option<Person>`, the macro extracts
`Person`.

## 7. Box and smart pointers

**Types**: `Box<T>`, `Rc<T>`, `Arc<T>`.

**Pattern**: Transparent — serializes the inner value.

```rust
struct Node {
    next: Option<Box<Node>>,
}
```

**Decorator collection**: For `Box<Node>`, the macro extracts `Node`.

## 8. Tuple structs (positional named types)

**Types**: `struct Point(f64, f64)`, `struct Color(u8, u8, u8, u8)`.

**Pattern**: Positional indexing like tuples.

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

- Access is by position: `point.0`, `point.1`.
- Named type provides semantic context.
- Common for coordinate types, RGB colors.

## 9. Enums

**Types**: Rust enums with variants.

**Patterns**: Depends on variant type.

```rust
#[derive(Serialize)]
enum Shape {
    Circle { radius: f64 },           // Struct variant
    Rectangle(f64, f64),               // Tuple variant
    Point,                             // Unit variant
}
```

**Relationalization**:

- **Struct variants**: Like structs, field names become relations.
- **Tuple variants**: Like tuples, positional indexing.
- **Unit variants**: Single atom with variant name.

## Complex nested types

The system handles arbitrarily nested types:

```rust
struct Organization {
    departments: Vec<Department>,
    executives: HashMap<String, Box<Person>>,
    archive: Option<Vec<Box<Department>>>,
}
```

**Type walking**:

1. `Vec<Department>` → analyzes `Department`.
2. `HashMap<String, Box<Person>>` → analyzes `Box<Person>` → analyzes
   `Person`.
3. `Option<Vec<Box<Department>>>` → analyzes `Vec<Box<Department>>` →
   analyzes `Department`.

**Deduplication**: Each type is only analyzed once (tracked by
`seen_types: HashSet<String>`).

## Summary table

| Type pattern | Relation pattern | Example | Why? |
|--------------|------------------|---------|------|
| Primitives | Leaf atoms | `i32` → `Atom{type: "i32"}` | No internal structure |
| Structs | Field-named relations | `name(person, str)` | Semantic field names |
| Vec/Arrays | `idx(container, pos, elem)` | `idx(vec, "0", item)` | Positional access |
| Tuples | `idx(container, pos, elem)` | `idx(tuple, "1", val)` | Fixed positions |
| Maps | `map_entry(map, key, val)` | `map_entry(m, k, v)` | Associative lookup |
| Options | Transparent or None | `Some(x)` → like `x` | Wrapper type |
| Box | Transparent | `Box<T>` → like `T` | Pointer indirection |
| Enums | Variant-dependent | Struct/Tuple/Unit | Multiple representations |

## Conclusion

The Rust integration with SpyTial demonstrates how static typing and
lack of reflection require fundamentally different approaches:

1. **Compile-time analysis.** Procedural macros walk the type tree at
   compile time, generating decorator collection code.
2. **Serialization as structure discovery.** Serde provides the hook
   into data structure traversal.
3. **Explicit opt-in.** Types must derive `SpytialDecorators` — no
   global reflection.
4. **Registration pattern.** Thread-safe one-time registration via
   `std::sync::Once`.
5. **Semantic preservation.** Different collection types use different
   relationalization patterns.
6. **Type safety.** The compiler ensures all references are valid.

This design trades runtime flexibility for compile-time guarantees,
zero-cost abstractions, and type safety — perfectly aligned with Rust's
philosophy.
