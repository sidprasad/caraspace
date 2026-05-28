# Architecture overview

This section is about the design rationale behind caraspace —
specifically, the unique problems Rust's static type system raises for a
visualization tool that's used to runtime reflection, and the choices
caraspace makes to solve them.

If you just want to render diagrams, skip this. The
[getting started](../getting-started/quickstart.md) and
[decorators](../decorators/overview.md) sections cover the user-facing
surface. This section is for people contributing to caraspace, debugging
unexpected layout behavior, or porting the same ideas to another static
language.

## The pipeline

```text
            compile time             |               runtime
                                     |
  #[derive(SpytialDecorators)]       |   diagram(&value)
              |                      |          |
              v                      |          v
   HasSpytialDecorators::decorators  |   T::decorators() ─────┐
              |                      |                        |
              v                      |                        v
   SpytialDecorators registry        |    YAML spec assembled
              |                      |                        |
              |                      |                        v
              |                      |   serde-driven export ─┤
              |                      |          |             |
              |                      |          v             v
              └─────────────────────────► JsonDataInstance + spec
                                     |          |
                                     |          v
                                     |   render HTML template
                                     |          |
                                     |          v
                                     |   write file + browser
```

The five stages, top to bottom:

1. **Compile-time decorator collection.** The `SpytialDecorators` derive
   macro reads attributes on each type, walks the type tree to find
   nested decorated types, and generates a `decorators()` implementation
   that returns a complete spec — its own attributes plus its nested
   types' attributes. See
   [Compile-time decorator collection](./compile-time.md).

2. **Runtime registry.** The first call to `T::decorators()` registers
   `T`'s decorators in a global `HashMap<String, SpytialDecorators>`
   keyed by type name. Subsequent calls are O(1) lookups. Registration
   is gated by `std::sync::Once` for thread-safety.

3. **Serde-driven traversal.** When `diagram()` runs, the value is
   serialized through a custom `JsonDataSerializer`. Serde's `serialize_struct`,
   `serialize_seq`, `serialize_map`, etc. methods tell the serializer
   what *kind* of structure it's looking at — that semantic distinction
   is preserved in the output. See
   [Serialization as structure discovery](./serialization.md).

4. **Relational JSON.** Each struct becomes an atom; each field becomes
   a relation between atoms. Sequences and tuples get a synthetic
   `idx(container, index, element)` relation. Maps get a ternary
   `map_entry(map, key, value)`. The output is a `JsonDataInstance`
   with `atoms` and `relations` arrays. See
   [Supported field types](./field-types.md) for the per-type rules.

5. **Browser rendering.** The bundled HTML template is filled in with
   the relational JSON and the assembled YAML spec, written to a file,
   and opened. Layout happens entirely in the browser via spytial-core,
   which is bundled inside the template.

## Why this shape

Each stage corresponds to a constraint Rust's type system imposes:

- **No runtime reflection** ⇒ we need compile-time annotation collection
  to know what decorators a type has, and serde-driven traversal to
  walk its structure without runtime introspection.
- **Monomorphization** ⇒ generic wrappers like `Vec<T>` are concrete at
  the call site; the derive macro extracts `T` from them at compile
  time.
- **Orphan rules** ⇒ caraspace can't add a `HasSpytialDecorators` impl
  to `Vec<T>` or `String`; users opt in by deriving the trait on their
  own types.
- **Zero-cost abstractions** ⇒ everything resolves to a flat pipeline
  with no runtime type discovery; the only dynamic dispatch is in the
  YAML serializer.

The rest of this section unpacks each stage in detail.
