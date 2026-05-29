# Negated constraints

Every layout constraint that takes a selector — `orientation`, `align`,
`cyclic`, `group` — accepts an optional `negated = true` argument.
Negation flips a constraint from "this *must* hold" to "this *must not*
hold."

Under the hood, `negated = true` emits the `hold: never` form that
spytial-core uses for disjunctive solving. That's the same machinery the
solver uses to reason about "either A or B" — by negating one branch
you eliminate a configuration the layout engine would otherwise consider.

## When you'd use it

**Forbidding a layout the data would suggest.** A directed graph might
have a `predecessor` edge that the default solver tries to lay out
top-to-bottom, but you've already aligned `successor` and the two
together would produce a tangle. Negate the orientation on
`predecessor` and the solver picks something cleaner.

```rust
#[orientation(
    selector = "{x, y : Node | x->y in predecessor}",
    directions = ["above"],
    negated = true,
)]
```

**Mutually exclusive alignment.** Two relations whose endpoints should
*not* line up horizontally — say, members of two different teams who
shouldn't visually merge.

```rust
#[align(
    selector = "{x, y : Person | x.team != y.team}",
    direction = "horizontal",
    negated = true,
)]
```

**Breaking a cycle.** When the data has a near-cycle but you want the
diagram to read as a sequence, negate the cyclic constraint on the
candidate ring and the solver will pick an open chain.

```rust
#[cyclic(selector = "next", direction = "clockwise", negated = true)]
```

**Forbidding grouping.** Two atoms that the structure suggests share a
category but that you want visually separated:

```rust
#[group(field = "category", negated = true)]
```

## What `hold: never` means

When the solver encounters a normal constraint, it adds the matched
atoms' configuration to the set of things that must be true in any valid
layout. With `negated = true`, the matched configuration is added to the
set of things that must *not* be true — the solver is allowed any
arrangement that doesn't satisfy the constraint pattern.

This is most useful in combination with other (positive) constraints:
the positive ones say what shape the layout should take, and the negated
ones rule out a degenerate case the solver might otherwise pick. A
diagram with only negated constraints is under-specified and the solver
will fall back to defaults.

## Composing with positive constraints

Negated and positive constraints can target overlapping selectors. A
common pattern is to assert a strong layout for the typical case and
negate the exception:

```rust
#[align(selector = "{x, y : Node | x->y in default_edge}", direction = "vertical")]
#[align(selector = "{x, y : Node | x->y in exception_edge}", direction = "vertical", negated = true)]
```

This reads as "default edges align vertically; exception edges must
not." The solver reconciles both, producing a layout where defaults
stack neatly and exceptions break to the side.
