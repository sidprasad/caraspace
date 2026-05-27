# CaraSpace

**Spytial for Rust — diagrams instead of `{:#?}` output.**

You already write `dbg!(tree)` and squint at 200 lines of nested debug
output. Swap one line and you also get a real diagram in your browser.
`caraspace::dbg!` is a strict superset of `std::dbg!` — same stderr
output, same return semantics, plus the picture.

```rust
use caraspace::dbg; // shadows std::dbg!

let tree = build_red_black_tree();
let tree = dbg!(tree); // stderr trail + browser tab; `tree` flows through
```

The structure was already in your types. Rust's `derive` macros, the
borrow checker, even your `Debug` impl all know what nodes and edges
live inside your value — caraspace refines that into a faithful picture.
For the longer version of the argument, see
[Spytial on Brown PLT's blog](https://blog.brownplt.org/2026/05/22/spytial.html).

## Install

```toml
[dependencies]
caraspace = "0.1"
serde = { version = "1", features = ["derive"] }
```

## 30-second example

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

fn main() {
    let tree = Node {
        key: 5,
        left: Some(Box::new(Node { key: 3, left: None, right: None })),
        right: Some(Box::new(Node { key: 7, left: None, right: None })),
    };
    dbg!(tree);
}
```

Derive `Debug`, `Serialize`, `SpytialDecorators`, drop in `dbg!`, run.
That's the whole story for the simple case.

## Drop-in for `std::dbg!`

`caraspace::dbg!` is a strict superset of `std::dbg!` — same calling
convention, same return semantics, plus a diagram. The migration is
literally one line:

```diff
- std::dbg!(tree)
+ caraspace::dbg!(tree)
```

or, with a `use`:

```rust
use caraspace::dbg; // shadows std::dbg! for the rest of the module
```

| Form               | Behavior                                          |
|--------------------|---------------------------------------------------|
| `dbg!()`           | Prints location to stderr (same as `std::dbg!()`) |
| `dbg!(x)`          | Prints `[file:line] x = {:#?}` + opens diagram, returns `x` |
| `dbg!(&x)`         | Same, borrows                                     |
| `dbg!(a, b)`       | Returns `(a, b)`, opens one tab per argument      |

Requirements on the value's type: `Debug` (same as `std::dbg!`), plus
`Serialize` and `SpytialDecorators` for caraspace.

Coexists fine with `std::dbg!` (different namespace). Honor
`SPYTIAL_NO_OPEN=1` to suppress browser launch in tests, CI, or any
headless context — stderr behavior is unchanged so `assert!`-style test
flows still work.

## The two ways in

- **`dbg!(value)`** — printf-style, one-character swap from `std::dbg!`,
  prints to stderr *and* opens a browser tab. Returns the value through
  so it composes inside larger expressions.
- **`diagram(&value)`** — the explicit function form. No stderr output,
  no source location, doesn't take ownership.

Both pick up decorators attached to the type and its nested types.

## Telling the diagrammer what you want

Caraspace's decorators (`#[attribute]`, `#[align]`, `#[orientation]`,
`#[atom_color]`, …) specify layout *declaratively* on the type, not the
value. The diagrammer treats them as constraints:

```rust
#[derive(Serialize, SpytialDecorators)]
#[attribute(field = "key")]
#[attribute(field = "color")]
#[orientation(selector = "{x, y : RBNode | x->y in left}",  directions = ["left",  "below"])]
#[orientation(selector = "{x, y : RBNode | x->y in right}", directions = ["right", "below"])]
#[atom_color(selector = "{x : RBNode | @:(x.color) = Red}",   value = "red")]
#[atom_color(selector = "{x : RBNode | @:(x.color) = Black}", value = "black")]
struct RBNode { /* ... */ }
```

The full red-black-tree demo is in [`examples/rbt.rs`](./examples/rbt.rs):

```bash
cargo run --example rbt
```

See [USER_GUIDE.md](./USER_GUIDE.md) for the complete decorator reference.

## How the derive walks your types

`#[derive(SpytialDecorators)]` analyses the type tree at compile time and
includes decorators from nested types automatically:

- `Vec<T>`, `Option<T>`, `Box<T>` → unwraps to `T`
- Nested combinations like `Vec<Option<Box<T>>>` work the same way
- Direct types are analysed in place

Decorating `Person` once is enough for those decorators to show up
wherever `Person` appears inside another decorated type. No manual
registration.

## Examples

- `cargo run --example dbg_basic` — the smallest `dbg!` swap, runnable
- `cargo run --example demo` — decorator collection on nested structs
- `cargo run --example rbt` — insertion-balanced red-black tree with
  color and layout decorators

## Headless / Docker

```bash
docker build -t caraspace .
docker run --rm -p 8080:8080 caraspace          # default: rbt
docker run --rm -p 8080:8080 caraspace demo     # any other example
```

Open `http://localhost:8080/rust_viz_data.html`. Browser launch is
disabled inside the container (`SPYTIAL_NO_OPEN=1`).

## Documentation

- [USER_GUIDE.md](./USER_GUIDE.md) — installation, decorator reference,
  common workflows
- [doc.md](./doc.md) — compile-time analysis internals
- [Spytial blog post](https://blog.brownplt.org/2026/05/22/spytial.html)
  — the design philosophy

## Development

```bash
cargo test --lib --tests
cargo test --doc
cargo run --example rbt
```

## License

MIT or Apache-2.0, at your option.
