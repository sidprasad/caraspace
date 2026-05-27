# CaraSpace

You can prove memory safety at compile time. You can derive `Hash`, `Ord`,
and `Serialize` from a single line. You can run a million-row benchmark
while the GC-language people are still writing config. And yet, when you
want to know the shape of the `BTreeMap<NodeId, Vec<Edge>>` your program
just built, you reach for `dbg!` and start counting braces.

```text
[src/main.rs:42:17] tree = RBTree {
    root: Some(
        RBNode { key: 38, color: Black, left: Some(RBNode { key: 19,
        color: Black, left: Some(RBNode { key: 12, color: Black, left:
        Some(RBNode { key: 8, color: Red, left: None, right: None }),
        right: None }), right: Some(RBNode { key: 31, color: Red, ...
```

You know what this *is*. You'd sketch it on paper in five seconds. The
terminal won't.

## The tree was already in the `derive(Debug)`

`#[derive(Debug)]` is proof that Rust already knows how to walk your value.
Structs become records. Fields become edges. Enum variants become labels.
`Box`, `Option`, `Vec` are indirection. The type system has named every
node and every reference long before any visualization tool sees it.

Caraspace refines that structure into a faithful picture instead of nested
text, and exposes it through the macro you already reach for:

```diff
- std::dbg!(tree)
+ caraspace::dbg!(tree)
```

stderr stays byte-identical. The browser opens an interactive diagram of
the same value. `caraspace::dbg!` is a strict superset of `std::dbg!` —
same calling convention, same return semantics, plus the picture.

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

let tree = build_tree();
let tree = dbg!(tree);
```

The longer version of this argument — including the BDD example that
motivates the design and the cross-language story — is on Brown PLT's
blog: [Diagramming Program Values by Spatial Refinement](https://blog.brownplt.org/2026/05/22/spytial.html).

## Decorators are specs, not draw calls

The default layout will get you most of the way for trees and lists, but
real diagrams want a few hints. Caraspace decorators describe *what should
hold* about the layout — not *how to render* it. They're declarative
constraints, attached to the type once, applied everywhere a value of that
type appears.

Watch a red-black tree clarify as constraints accumulate.

**Stage 1: bare derive.** Three derives, no decorators. The diagram is a
graph — every node and edge is correct, but it's flat.

```rust
#[derive(Debug, Serialize, SpytialDecorators)]
struct RBNode {
    key: u32,
    color: Color,
    left: Option<Box<RBNode>>,
    right: Option<Box<RBNode>>,
}
```

**Stage 2: show the key.** Without this, nodes are just anonymous atoms.

```rust
#[attribute(field = "key")]
```

**Stage 3: make it a tree.** Left children go down-and-left, right
children go down-and-right. Now the layout encodes BST order visually.

```rust
#[orientation(selector = "{x, y : RBNode | x->y in left}",  directions = ["left",  "below"])]
#[orientation(selector = "{x, y : RBNode | x->y in right}", directions = ["right", "below"])]
```

**Stage 4: make it a *red-black* tree.** Color the nodes by their `Color`
field. The selector pattern matches *any* `RBNode` whose `color` is `Red`
— rules are over structure, not specific instances.

```rust
#[atom_color(selector = "{x : RBNode | @:(x.color) = Red}",   value = "red")]
#[atom_color(selector = "{x : RBNode | @:(x.color) = Black}", value = "black")]
```

**Stage 5: hide the scaffolding.** The `Color` enum atoms and the `None`
sentinels aren't interesting — their effect is already visible in node
color and absent edges. Drop them from the canvas.

```rust
#[hide_atom(selector = "Color + u32 + None")]
```

Each rule refines an existing structure rather than imposing an external
aesthetic. None of them say where any specific node goes; they say which
*relationships* should hold across every node of the matching shape. Add
a sixth constraint and the diagram stays consistent. Remove one and the
diagram still parses your value — it just looks less specific.

Decorators are collected transitively. Decorating `Person` once is enough
for those decorators to apply wherever `Person` appears inside another
decorated type — `Vec<T>`, `Option<T>`, `Box<T>`, and their nested
combinations all unwrap during the compile-time walk. No central registry,
no runtime registration.

The full progressive demo lives in [`examples/rbt.rs`](./examples/rbt.rs).

## Install

```toml
[dependencies]
caraspace = "0.0"
serde = { version = "1", features = ["derive"] }
```

## Reference

`dbg!` matches the `std::dbg!` calling convention:

| Form         | Behavior                                                          |
|--------------|-------------------------------------------------------------------|
| `dbg!()`     | Prints location to stderr (same as `std::dbg!()`)                 |
| `dbg!(x)`    | Prints `{:#?}` + opens diagram, returns `x` through               |
| `dbg!(&x)`   | Same, borrows                                                     |
| `dbg!(a, b)` | Returns `(a, b)`; one diagram tab per argument                    |

Type requirements: `Debug` (already required by `std::dbg!`), plus
`Serialize` and `SpytialDecorators`. `SPYTIAL_NO_OPEN=1` suppresses browser
launch — stderr is unaffected, so `cargo test` capture behaves exactly
like it does for `std::dbg!`.

For library code, or anywhere you don't want stderr noise:

```rust
use caraspace::diagram;
diagram(&tree); // no stderr, no source location, doesn't move
```

Full decorator attribute reference: [USER_GUIDE.md](./USER_GUIDE.md).

## Examples

| Example       | What it shows                                              |
|---------------|------------------------------------------------------------|
| `dbg_basic`   | The smallest `dbg!` swap                                   |
| `demo`        | Decorator collection across nested structs                 |
| `rbt`         | Red-black tree progressive refinement (Stages 1–5 above)   |

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

Open `http://localhost:8080/rust_viz_data.html`. Browser launch is
disabled inside the container (`SPYTIAL_NO_OPEN=1`).

## Docs

- [USER_GUIDE.md](./USER_GUIDE.md) — full decorator reference, common workflows
- [doc.md](./doc.md) — compile-time analysis internals
- [Spytial blog post](https://blog.brownplt.org/2026/05/22/spytial.html) — design philosophy and motivating examples

## Development

```bash
cargo test --lib --tests
cargo test --doc
cargo run --example rbt
```

## License

MIT or Apache-2.0, at your option.
