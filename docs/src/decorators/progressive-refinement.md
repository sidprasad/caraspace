# Progressive refinement: a red-black tree

The default layout will get you most of the way for trees and lists, but
real diagrams want a few hints. This walkthrough adds one decorator at a
time and watches a red-black tree clarify.

Each stage is independently runnable — drop the snippet into a struct
definition, render a tree, observe the change. The full progressive demo
lives in [`examples/rbt.rs`](https://github.com/sidprasad/caraspace/blob/main/examples/rbt.rs).

## Stage 1: bare derive

Three derives, no decorators. The diagram is a graph — every node and
edge is correct, but it's flat.

```rust
#[derive(Debug, Serialize, SpytialDecorators)]
struct RBNode {
    key: u32,
    color: Color,
    left: Option<Box<RBNode>>,
    right: Option<Box<RBNode>>,
}
```

The browser shows nodes connected by `left` / `right` / `color` edges,
but you can't read the tree as a tree yet. Atoms have no labels and the
layout doesn't reflect BST order.

## Stage 2: show the key

Without this, nodes are just anonymous atoms.

```rust
#[attribute(field = "key")]
```

Every `RBNode` atom now carries its key as a label. That alone makes the
graph navigable — you can see what's where.

## Stage 3: make it a tree

Left children go down-and-left, right children go down-and-right. Now
the layout encodes BST order visually.

```rust
#[orientation(selector = "{x, y : RBNode | x->y in left}",  directions = ["left",  "below"])]
#[orientation(selector = "{x, y : RBNode | x->y in right}", directions = ["right", "below"])]
```

The selector syntax says: "for any pair `(x, y)` of `RBNode`s where
`x -> y` is in the `left` relation, place `y` to the left of and below
`x`." Same idea for `right`. The solver now arranges nodes the way you'd
sketch a tree on paper.

## Stage 4: make it a *red-black* tree

Color the nodes by their `Color` field. The selector pattern matches
*any* `RBNode` whose `color` is `Red` — rules are over structure, not
specific instances.

```rust
#[atom_color(selector = "{x : RBNode | @:(x.color) = Red}",   value = "red")]
#[atom_color(selector = "{x : RBNode | @:(x.color) = Black}", value = "black")]
```

This is the property that makes the layout principled rather than
ad-hoc: the rules apply to every node that matches, no matter how the
tree was built or how deep the node is.

## Stage 5: hide the scaffolding

The `Color` enum atoms and the `None` sentinels aren't interesting —
their effect is already visible in node color and absent edges. Drop
them from the canvas.

```rust
#[hide_atom(selector = "Color + u32 + None")]
```

The selector `Color + u32 + None` is a set expression: match any atom
whose type is `Color`, `u32`, or the literal `None`. Those atoms vanish;
the structural information they carried (color, key value, missing
child) is preserved by the other rules.

## What just happened

Each rule refined an existing structure rather than imposing an external
aesthetic. None of them say where any *specific* node goes; they say
which *relationships* should hold across every node of the matching
shape. Add a sixth constraint and the diagram stays consistent. Remove
one and the diagram still parses your value — it just looks less
specific.

This is the design principle the [overview](./overview.md) page sums up
as "decorators are specs, not draw calls." If you want to see what the
solver does with the negative form ("these atoms must *not* line up"),
read [Negated constraints](./negated.md) next.
