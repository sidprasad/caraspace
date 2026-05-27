# CaraSpace

**`dbg!` for shapes, not just text.**

`caraspace::dbg!` is a drop-in replacement for `std::dbg!` that opens an
interactive diagram of the value alongside the usual stderr trail. Same
calling convention, same `{:#?}` output, plus the picture.

```rust
use caraspace::dbg;

let tree = build_red_black_tree();
let tree = dbg!(tree);
```

stderr:

```text
[src/main.rs:42:17] tree = RBTree {
    root: Some(RBNode { key: 38, color: Black, left: Some(RBNode { ... }), ... }),
}
```

Browser: an interactive diagram of the same tree.

The structure was already in your types — `#[derive(Debug)]` is proof that
Rust knows how to walk your value. Caraspace refines that into a faithful
picture instead of nested text. For the longer argument, see
[Spytial on Brown PLT's blog](https://blog.brownplt.org/2026/05/22/spytial.html).

## Install

```toml
[dependencies]
caraspace = "0.1"
serde = { version = "1", features = ["derive"] }
```

## The swap

```diff
- std::dbg!(tree)
+ caraspace::dbg!(tree)
```

Or shadow it for the whole module:

```rust
use caraspace::dbg;
```

The calling convention matches `std::dbg!` exactly:

| Form         | Behavior                                                          |
|--------------|-------------------------------------------------------------------|
| `dbg!()`     | Prints location to stderr (same as `std::dbg!()`)                 |
| `dbg!(x)`    | Prints `{:#?}` + opens diagram, returns `x` through               |
| `dbg!(&x)`   | Same, borrows                                                     |
| `dbg!(a, b)` | Returns `(a, b)`; one diagram tab per argument                    |

Type requirements: `Debug` (already required by `std::dbg!`), plus `Serialize`
and `SpytialDecorators`:

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
```

That's the whole setup. `SPYTIAL_NO_OPEN=1` suppresses browser launch (CI,
tests, headless runs) — stderr is unaffected, so `cargo test` capture behaves
exactly like it does for `std::dbg!`.

## When you want more than the default layout

Caraspace decorators (`#[attribute]`, `#[align]`, `#[orientation]`,
`#[atom_color]`, …) tell the diagrammer what to emphasise — relative
position, color, alignment, grouping. They're declarative constraints, not
imperative rendering code:

```rust
#[derive(Debug, Serialize, SpytialDecorators)]
#[attribute(field = "key")]
#[attribute(field = "color")]
#[orientation(selector = "{x, y : RBNode | x->y in left}",  directions = ["left",  "below"])]
#[orientation(selector = "{x, y : RBNode | x->y in right}", directions = ["right", "below"])]
#[atom_color(selector = "{x : RBNode | @:(x.color) = Red}",   value = "red")]
#[atom_color(selector = "{x : RBNode | @:(x.color) = Black}", value = "black")]
struct RBNode { /* ... */ }
```

Decorators are collected from nested types automatically — decorating
`Person` once is enough for those decorators to apply wherever `Person`
appears inside another decorated type. `Vec<T>`, `Option<T>`, `Box<T>`, and
their nested combinations all unwrap during the compile-time walk.

Full attribute reference: [USER_GUIDE.md](./USER_GUIDE.md).

## Without `dbg!`

For library code, or anywhere you don't want stderr noise, call `diagram`
directly:

```rust
use caraspace::diagram;
diagram(&tree); // no stderr line, no source location, doesn't move
```

## Examples

| Example       | What it shows                                          |
|---------------|--------------------------------------------------------|
| `dbg_basic`   | The smallest `dbg!` swap                               |
| `demo`        | Decorator collection across nested structs             |
| `rbt`         | Insertion-balanced red-black tree with layout + colors |

```bash
cargo run --example dbg_basic
cargo run --example rbt
```

## Headless / Docker

```bash
docker build -t caraspace .
docker run --rm -p 8080:8080 caraspace          # default: rbt
docker run --rm -p 8080:8080 caraspace demo
```

Open `http://localhost:8080/rust_viz_data.html`. Browser launch is disabled
inside the container (`SPYTIAL_NO_OPEN=1`).

## Docs

- [USER_GUIDE.md](./USER_GUIDE.md) — full decorator reference, common workflows
- [doc.md](./doc.md) — compile-time analysis internals
- [Spytial blog post](https://blog.brownplt.org/2026/05/22/spytial.html) — design philosophy

## Development

```bash
cargo test --lib --tests
cargo test --doc
cargo run --example rbt
```

## License

MIT or Apache-2.0, at your option.
