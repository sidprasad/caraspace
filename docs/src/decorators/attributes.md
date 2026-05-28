# Attribute reference

Every caraspace decorator is a Rust attribute on a type that derives
`SpytialDecorators`. They map onto the same builder and YAML
serialization layer used by spytial-core, so anything you can express
here is also expressible as a hand-written spec passed to
[`diagram_with_spec`](../workflows/library.md).

The attributes group naturally into three families:

- **Display attributes** — what shows up on each node (`attribute`, `flag`).
- **Layout constraints** — where nodes go relative to each other
  (`orientation`, `align`, `cyclic`, `group`).
- **Styling** — colors, sizes, icons, and edge appearance
  (`atom_color`, `size`, `icon`, `edge_style`).
- **Filtering and overrides** — what's visible or computed
  (`projection`, `hide_field`, `hide_atom`, `inferred_edge`, `tag`).

## `#[attribute(field = "...")]`

Promote a field's value into the node's label.

```rust
#[derive(Serialize, SpytialDecorators)]
#[attribute(field = "name")]
struct Person {
    name: String,
    age: u32,
}
```

Without this, the node renders as an anonymous atom of type `Person`.
With it, the node carries the person's name. **Use this when** a struct
has an obvious identity field — `name`, `id`, `key`, `label`.

## `#[flag(name = "...")]`

Set a global display flag, e.g. `hideDisconnected`.

```rust
#[derive(Serialize, SpytialDecorators)]
#[flag(name = "hideDisconnected")]
struct Graph { /* ... */ }
```

**Use this when** you want a coarse-grained behavior switch that affects
the whole rendering rather than a specific selector. Flags are layout
hints, not constraints.

## `#[orientation(selector = "...", directions = [...], negated = false)]`

Position related atoms relative to each other.

```rust
#[derive(Serialize, SpytialDecorators)]
#[orientation(selector = "{x, y : RBNode | x->y in left}",  directions = ["left",  "below"])]
#[orientation(selector = "{x, y : RBNode | x->y in right}", directions = ["right", "below"])]
struct RBNode { /* ... */ }
```

`directions` can include `"left"`, `"right"`, `"above"`, `"below"`. The
selector picks pairs of atoms; the constraint says the second goes in
those directions from the first. **Use this when** you have an asymmetric
relation that benefits from a consistent direction — tree edges,
linked-list `next` pointers, parent/child arrows.

Set `negated = true` to flip the constraint into `hold: never` form,
which the solver reads as "these pairs must *not* line up this way."
See [Negated constraints](./negated.md).

## `#[align(selector = "...", direction = "horizontal" | "vertical", negated = false)]`

Force atoms matched by the selector to share an axis.

```rust
#[derive(Serialize, SpytialDecorators)]
#[align(selector = "reports_to", direction = "horizontal")]
struct Person {
    name: String,
    reports_to: Option<Box<Person>>,
}
```

**Use this when** a relation produces a chain that should read as a row
or a column — org-chart reporting lines, doubly-linked lists, sequence
indices.

`negated = true` says the matched atoms must *not* be aligned.

## `#[cyclic(selector = "...", direction = "...", negated = false)]`

Arrange matched atoms around a circle.

```rust
#[derive(Serialize, SpytialDecorators)]
#[cyclic(selector = "next", direction = "clockwise")]
struct RingNode { /* ... */ }
```

`direction` is `"clockwise"` or `"counterclockwise"`. **Use this when**
you have a ring structure — circular linked lists, scheduling rounds,
state machines with a clear cycle.

`negated = true` says the atoms must not be arranged cyclically.

## `#[group(..., negated = false)]`

Cluster related atoms into a visual group.

```rust
#[derive(Serialize, SpytialDecorators)]
#[group(field = "department")]
struct Employee { /* ... */ }
```

The argument shape depends on what you're grouping by — `field`,
`selector`, or both. **Use this when** atoms share a logical category
that should read as a region (a department, a partition, a connected
component).

`negated = true` forbids the grouping.

## `#[atom_color(selector = "...", value = "...")]`

Color the matched atoms.

```rust
#[derive(Serialize, SpytialDecorators)]
#[atom_color(selector = "{x : RBNode | @:(x.color) = Red}",   value = "red")]
#[atom_color(selector = "{x : RBNode | @:(x.color) = Black}", value = "black")]
struct RBNode { /* ... */ }
```

`value` can be any CSS color string. **Use this when** a field is more
useful as visual hue than as a separate node — enum tags, status flags,
classification labels.

## `#[size(selector = "...", height = 40, width = 60)]`

Override node dimensions in pixels.

```rust
#[derive(Serialize, SpytialDecorators)]
#[size(selector = "Root", height = 60, width = 80)]
struct Tree { /* ... */ }
```

**Use this when** specific atoms need extra room (large labels, root
nodes you want to emphasize). Most diagrams don't need this — the
default sizing is reasonable.

## `#[icon(selector = "...", path = "...", show_labels = true)]`

Replace matched atoms with an SVG icon.

```rust
#[derive(Serialize, SpytialDecorators)]
#[icon(selector = "User", path = "/icons/user.svg", show_labels = true)]
struct App { /* ... */ }
```

The `path` resolves against the page hosting the diagram. **Use this
when** atoms represent real-world entities with conventional iconography
— users, files, databases.

## `#[edge_style(...)]`

Style the relation arrows.

```rust
#[derive(Serialize, SpytialDecorators)]
#[edge_style(
    field = "reports_to",
    value = "blue",
    style = "dashed",
    weight = 2.0,
    show_label = true,
    hidden = false,
)]
struct Person { /* ... */ }
```

Supported keys: `field`, `value`, `style` (`"solid"`, `"dashed"`, etc.),
`weight`, `show_label`, `hidden`, `filter`, `selector`. **Use this when**
the edges themselves carry meaning — different relation kinds should
look different, optional edges should dash, redundant edges should hide.

## `#[projection(sig = "...")]`

Render atoms of `sig` as projections instead of standalone nodes.

```rust
#[derive(Serialize, SpytialDecorators)]
#[projection(sig = "Timestamp")]
struct Event { /* ... */ }
```

**Use this when** a value should appear *on* other nodes rather than as
a node of its own — timestamps annotating events, weights labeling
edges.

## `#[hide_field(field = "...")]`

Suppress a relation from the rendering.

```rust
#[derive(Serialize, SpytialDecorators)]
#[hide_field(field = "internal_id")]
struct Record { /* ... */ }
```

**Use this when** a field is required by the data model but is noise in
the diagram — backing IDs, denormalized caches, debug flags.

## `#[hide_atom(selector = "...")]`

Suppress matched atoms entirely.

```rust
#[derive(Serialize, SpytialDecorators)]
#[hide_atom(selector = "Color + u32 + None")]
struct RBNode { /* ... */ }
```

The selector is a set expression — `Color + u32 + None` matches any atom
whose type is `Color`, `u32`, or the sentinel `None`. **Use this when**
the structure is implicit in the rest of the diagram — colors that show
up as atom_color, missing children that show up as absent edges.

## `#[inferred_edge(name = "...", selector = "...")]`

Define a synthetic edge that isn't in the original data.

```rust
#[derive(Serialize, SpytialDecorators)]
#[inferred_edge(name = "transitive_reports", selector = "reports_to.^reports_to")]
struct Person { /* ... */ }
```

**Use this when** an interesting relationship is derivable but not
explicitly stored — transitive closures, computed adjacencies, joins.

## `#[tag(to_tag = "...", name = "...", value = "...")]`

Attach a computed attribute to matched atoms.

```rust
#[derive(Serialize, SpytialDecorators)]
#[tag(to_tag = "RBNode", name = "depth", value = "...")]
struct RBNode { /* ... */ }
```

**Use this when** atoms need an extra label or property that isn't in
the source data — depth annotations, computed roles, classification
tags.

## Compile-time traversal

The derive macro walks common container types and automatically pulls in
decorators from the inner types. Supported wrappers:

- `Vec<T>`
- `Option<T>`
- `Box<T>`
- Nested combinations such as `Vec<Option<Box<T>>>`

This means decorating `Person` is usually enough for those decorators to
apply when `Company` contains `Vec<Person>`. There's no central
registry; everything resolves at compile time.

## Runtime annotations

If you need imperative control — building decorators in a function,
parameterizing them by config — use the `spytial_annotations` module
directly:

```rust
use caraspace::spytial_annotations::{annotate_instance, AnnotationBuilder};
```

This path is better suited to advanced integrations than to the normal
"derive and render" workflow.
