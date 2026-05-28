# Decorators are specs, not draw calls

Caraspace decorators describe *what should hold* about a layout — not
*how to render* it. They're declarative constraints, attached to a type
once, applied everywhere a value of that type appears.

This is the load-bearing design choice. A decorator like
`#[align(selector = "reports_to", direction = "horizontal")]` does not
say "draw an arrow from A to B." It says "in any rendering of this value,
atoms related by `reports_to` should line up horizontally." The layout
engine is free to pick the actual coordinates, as long as the constraint
holds.

Three properties fall out of that:

**Decorators compose.** Add an orientation rule, then a coloring rule,
then a hide rule — the diagram refines step by step. No single rule has
to know about any other; the solver reconciles them.

**Decorators are per-type, not per-instance.** You decorate `RBNode`
once. Every `RBNode` in the value — whether it's the root, a leaf, or
nested ten levels deep in a `Vec<Option<Box<RBNode>>>` — picks up the
same constraints. There's no central registry to keep in sync, no
runtime registration step.

**Decorators are collected transitively.** Decorating `Person` once is
enough for those decorators to apply wherever `Person` appears inside
another decorated type — `Vec<T>`, `Option<T>`, `Box<T>`, and their
nested combinations all unwrap during the compile-time walk.

If you've used CSS selectors or query-based UI styling, this will feel
familiar: the decorator is the rule, the selector is the match, the
solver picks the rendering. If you haven't, the
[progressive refinement walkthrough](./progressive-refinement.md) shows
how five rules accumulate into a polished red-black tree picture.

The full attribute reference lives at
[Attribute reference](./attributes.md). Skip there if you want to know
exactly what each attribute does.
