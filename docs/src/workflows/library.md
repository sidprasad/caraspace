# Library integration

The default `diagram()` and `dbg!()` entry points are tuned for
interactive use — they pretty-print, they open a browser, they
swallow errors with an `eprintln!`. That's right for a debugging tool;
it's wrong for a library that wants to be a clean dependency.

This page covers the entry points you'd use when caraspace is buried
inside something else: a CLI subcommand, a test harness, a build script,
or a web service that hands clients a rendered diagram.

## `diagram(&value)` — fire and forget

```rust
use caraspace::diagram;
diagram(&tree);
```

The friendly entry point. Renders the value, writes to a temp file,
opens a browser tab. Errors at any step (serialization failure, missing
write permission on the temp dir, no browser-open command) are printed
to stderr and swallowed; the function returns `()` regardless.

Use it when you don't care to handle failure differently from "user
notices the browser didn't open."

## `diagram_with_spec(&value, spec)` — hand-written constraints

```rust
use caraspace::diagram_with_spec;

let spec = r#"
constraints:
  - align:
      selector: reports_to
      direction: horizontal
directives:
  - flag: hideDisconnected
"#;

diagram_with_spec(&tree, spec);
```

Same diagram-and-browser flow as `diagram()`, but with a YAML spec
string you've assembled yourself. This bypasses the derive-generated
decorators on `T`. Useful when:

- You want to render a type you can't add a derive to.
- You want to override the derive output for one call.
- You're generating the spec from configuration or another data source.

The YAML schema is the same one the `SpytialDecorators` derive emits;
see the [attribute reference](../decorators/attributes.md) for the keys.

## `export_json_instance(&value)` — render without opening

```rust
use caraspace::export_json_instance;
let instance = export_json_instance(&tree);
// instance: JsonDataInstance { atoms, relations }
```

Returns the relational JSON representation of the value without writing
any HTML or touching the browser. This is the right entry point for:

- Persisting the diagram data to disk for later rendering.
- Sending it across a network to a remote renderer.
- Integrating caraspace into a tool that has its own UI.

`export_json_instance` is infallible at the call boundary — if
serialization fails, it logs to stderr and returns an empty
`JsonDataInstance`. That's convenient for the "best effort" case but
hides errors you might want to handle.

## `try_export_json_instance(&value) -> Result<…>` — fallible export

```rust
use caraspace::export::try_export_json_instance;

match try_export_json_instance(&tree) {
    Ok(instance) => persist(instance),
    Err(err) => log::error!("caraspace export failed: {err}"),
}
```

The fallible variant. If `Serialize` returns an error, you get the error
back instead of a silent empty instance. **This is the right choice for
library code** that wants to surface failure to its caller rather than
degrade silently.

For symmetry, the `try_` variant is also the right choice in tests that
want to assert "serialization succeeded" rather than "serialization
returned something."

## Choosing between them

| Use case                          | Entry point                       |
|-----------------------------------|-----------------------------------|
| `println!`-style ad-hoc debugging | `caraspace::dbg!`                 |
| One-call render with auto layout  | `diagram(&value)`                 |
| Render with a custom YAML spec    | `diagram_with_spec(&value, spec)` |
| Capture relational JSON only      | `export_json_instance(&value)`    |
| Same, but surface errors          | `try_export_json_instance(&value)`|
