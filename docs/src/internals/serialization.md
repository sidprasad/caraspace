# Serialization as structure discovery

## The core challenge

**Rust doesn't have runtime reflection** like Java or Python. We can't
iterate over an object's fields at runtime or discover its type
dynamically. Instead, Rust exposes structure through **serialization**.

## Serde: the hook into structure

CaraSpace uses [Serde](https://serde.rs/), Rust's serialization
framework, as the mechanism to traverse and understand data structures.

### Why Serde?

Serde provides trait-based serialization that:

1. **Preserves semantic information.** Different entry points for
   structs vs maps vs sequences.
2. **Type-driven.** The type system guides serialization decisions.
3. **Zero-copy.** Can inspect structure without allocating.
4. **Widely adopted.** Most Rust types already implement `Serialize`.

## Custom serializer implementation

CaraSpace implements a custom Serde serializer (`JsonDataSerializer`)
that:

```rust
pub struct JsonDataSerializer {
    counter: usize,
    atoms: Vec<IAtom>,              // Collected atoms
    relations: HashMap<String, IRelation>,  // Collected relations
    collected_decorators: SpytialDecorators,    // Decorators from visited types
    visited_types: HashSet<String>,         // Prevent duplicate collection
}
```

### How the serializer works

When you call `diagram(&company)`:

1. **Serialization starts.** `company.serialize(&mut serializer)`.
2. **Struct handling.** Serde calls `serialize_struct("Company", 2)`:
   - Creates an atom with type "Company".
   - **Triggers decorator collection** for type "Company".
3. **Field serialization.** For each field, Serde calls
   `serialize_field("name", &value)`:
   - Recursively serializes the value.
   - Creates a relation named after the field.
4. **Nested types.** When encountering `Vec<Person>`:
   - Calls `serialize_seq` for the vector.
   - For each element, calls `serialize_struct("Person", 2)`.
   - **Triggers decorator collection** for type "Person".
5. **Relation building.** Each structural relationship becomes a
   relation in the output.

## Hooking into the serializer

The key insight is that **Serde's trait methods tell us about
structure**:

| Serde Method | Structural Meaning | SpyTial Output |
|--------------|-------------------|------------|
| `serialize_struct(name, len)` | Named struct with fields | Atom of type `name`, field relations |
| `serialize_seq(len)` | Ordered collection | Atom of type "sequence", `idx` relations |
| `serialize_map(len)` | Key-value pairs | Atom of type "map", `map_entry` relations |
| `serialize_tuple(len)` | Fixed-size heterogeneous | Atom of type "tuple", `idx` relations |

## Decorator collection during serialization

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

1. Checks if decorators for this type are already in the registry.
2. If found, merges them into the collected set.
3. Prevents duplicate collection using `visited_types` tracking.

## The complete flow

```text
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
