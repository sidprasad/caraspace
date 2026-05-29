# Quick start

The shortest path from `Cargo.toml` to a diagram: derive `Serialize` and
`SpytialDecorators` on the types you care about, build a value, hand it
to `diagram(&value)`.

```rust
use caraspace::{diagram, SpytialDecorators};
use serde::Serialize;

#[derive(Serialize, SpytialDecorators)]
#[attribute(field = "name")]
struct Company {
    name: String,
    employees: Vec<Person>,
}

#[derive(Serialize, SpytialDecorators)]
#[attribute(field = "name")]
#[align(selector = "reports_to", direction = "horizontal")]
struct Person {
    name: String,
    reports_to: Option<Box<Person>>,
}

fn main() {
    let company = Company {
        name: "Acme".to_string(),
        employees: vec![Person {
            name: "Alice".to_string(),
            reports_to: None,
        }],
    };

    diagram(&company);
}
```

A browser tab opens with the rendered diagram. Decorators on `Person`
apply automatically wherever a `Person` appears inside `Company`, because
the derive walks `Vec<Person>` at compile time. You don't have to register
nested types anywhere.

## The `dbg!` variant

If you're used to scattering `std::dbg!` through your code, the
`caraspace::dbg!` macro is a strict superset — same stderr trail, plus
the diagram:

```rust
use caraspace::{dbg, SpytialDecorators};
use serde::Serialize;

#[derive(Debug, Serialize, SpytialDecorators)]
#[attribute(field = "key")]
struct Node {
    key: u32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

fn build_tree() -> Node {
    Node {
        key: 5,
        left: Some(Box::new(Node { key: 3, left: None, right: None })),
        right: Some(Box::new(Node { key: 7, left: None, right: None })),
    }
}

fn main() {
    let tree = build_tree();
    let tree = dbg!(tree);
    let _ = tree;
}
```

`dbg!(x)` prints `[file:line:col] x = {:#?}` to stderr (using the same
pretty-printing as `std::dbg!`), opens a diagram, and returns `x` through
the expression. `dbg!(&x)` borrows, `dbg!(a, b)` returns `(a, b)` and
opens one diagram per argument.

Type requirements: `Debug` (already required by `std::dbg!`), plus
`Serialize` and `SpytialDecorators`.

## Core workflow

1. Derive `Serialize` and `SpytialDecorators` on the Rust types you want to visualize.
2. Attach layout or styling attributes to those types (see the [attribute reference](../decorators/attributes.md)).
3. Build a normal Rust value.
4. Call `diagram(&value)` for the derive-generated spec, or
   `diagram_with_spec(&value, spec)` for a hand-written one.

Once you have a value rendering, the next thing to learn is what the
attributes actually do — start with [Specs, not draw calls](../decorators/overview.md).
