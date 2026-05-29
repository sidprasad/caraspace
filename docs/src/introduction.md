# Introduction

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

## Where next

- [Install](./getting-started/install.md) and run your [first diagram](./getting-started/first-diagram.md).
- [Decorators are specs, not draw calls](./decorators/overview.md) — the design principle behind the layout system.
- The longer version of this argument — including the BDD example that
  motivates the design and the cross-language story — is on Brown PLT's
  blog: [Diagramming Program Values by Spatial Refinement](https://blog.brownplt.org/2026/05/22/spytial.html).
