# Caraspace

[![crates.io](https://img.shields.io/crates/v/caraspace.svg)](https://crates.io/crates/caraspace)
[![docs.rs](https://img.shields.io/docsrs/caraspace)](https://docs.rs/caraspace)
[![PR checks](https://github.com/sidprasad/caraspace/actions/workflows/pr.yml/badge.svg)](https://github.com/sidprasad/caraspace/actions/workflows/pr.yml)
[![License](https://img.shields.io/crates/l/caraspace.svg)](#license)

You can prove memory safety at compile time. You can derive `Hash`, `Ord`,
and `Serialize` from a single line. And yet, when you want to know the
shape of the `BTreeMap<NodeId, Vec<Edge>>` your program just built, you
reach for `dbg!` and start counting braces.

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

`#[derive(Debug)]` is proof that Rust already knows how to walk your
value. Caraspace refines that walk into a faithful diagram instead of
nested text, and exposes it through the macro you already reach for:

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
#[orientation(selector = "{x, y : Node | x->y in left}",  directions = ["left",  "below"])]
#[orientation(selector = "{x, y : Node | x->y in right}", directions = ["right", "below"])]
struct Node {
    key: u32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

let tree = build_tree();
let tree = dbg!(tree); // returns `tree` through; diagram opens in browser
```

Decorators describe *what should hold* about the layout, not *how to
render* it. They're declarative constraints, attached to the type once,
applied everywhere a value of that type appears. The
[progressive-refinement walkthrough](https://sidprasad.github.io/caraspace/decorators/progressive-refinement.html)
shows how a flat graph clarifies into a red-black tree as constraints
accumulate.

For the design philosophy and motivating examples, see Brown PLT's blog
post: [Diagramming Program Values by Spatial Refinement](https://blog.brownplt.org/2026/05/22/spytial.html).

## Install

```toml
[dependencies]
caraspace = "0.1"
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
`Serialize` and `SpytialDecorators`.

Environment variables:

| Variable               | Effect                                                          |
|------------------------|-----------------------------------------------------------------|
| `SPYTIAL_NO_OPEN=1`    | Skip browser launch; useful for `cargo test` and CI             |
| `SPYTIAL_OUTPUT_PATH`  | Pin the HTML output to a specific path (default: random tempfile) |

For library code, or anywhere you don't want stderr noise:

```rust
use caraspace::diagram;
diagram(&tree); // no stderr, no source location, doesn't move
```

Failures (serialization, file write, missing browser) never panic — they
log a one-line warning to stderr and return. Use `try_export_json_instance`
for the fallible Result-returning path.

## Examples

```bash
cargo run --example dbg_basic     # smallest dbg! swap
cargo run --example demo          # decorator collection across nested structs
cargo run --example rbt           # progressive refinement of a red-black tree
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

- **Guide:** <https://sidprasad.github.io/caraspace/> — install, decorators, workflows, internals
- **API:** <https://docs.rs/caraspace> — generated rustdoc
- **Design:** [Diagramming Program Values by Spatial Refinement](https://blog.brownplt.org/2026/05/22/spytial.html)

## Status

| | |
|---|---|
| Version | 0.1.0 |
| MSRV | Rust 1.80 |
| OS | macOS, Linux, Windows |
| License | MIT or Apache-2.0 |

## Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md). Bug reports and PRs welcome.

## License

Dual-licensed under [MIT](./LICENSE-MIT) or [Apache 2.0](./LICENSE-APACHE),
at your option.
