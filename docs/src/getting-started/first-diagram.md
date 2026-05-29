# Your first diagram

Once you've got caraspace in `Cargo.toml` (see [Install](./install.md)),
this is the smallest interesting program you can run:

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

    let _ = dbg!(tree);
}
```

Run it:

```sh
cargo run
```

Two things happen:

1. Your terminal prints `[src/main.rs:LINE:COL] tree = Node { … }` —
   byte-identical to what `std::dbg!` would have done.
2. A browser tab opens with the rendered tree.

The diagram is a self-contained HTML file. The page does its layout in the
browser using a bundled copy of spytial-core; nothing phones home, and
the file keeps working after you close caraspace.

## Where the file lives

By default caraspace writes the rendered HTML to a per-call file in your
OS temp directory, with a name like `caraspace-{pid}-{counter}-{nanos}.html`.
Concurrent `dbg!` calls don't trample each other.

If you'd rather pin the output to a known path — useful for serving from
a static file server, or for `scp`ing the file off a remote machine —
set `SPYTIAL_OUTPUT_PATH`:

```sh
SPYTIAL_OUTPUT_PATH=/tmp/my-diagram.html cargo run
```

That path is taken verbatim. Each `dbg!` call overwrites it, so this is a
one-diagram-at-a-time setup.

## Skipping the browser launch

In CI, over SSH, inside a container — anywhere there's no display —
disable the browser launch with `SPYTIAL_NO_OPEN`:

```sh
SPYTIAL_NO_OPEN=1 cargo run
```

The values accepted are `1`, `true`, and `yes` (case-insensitive). With
the browser launch suppressed, caraspace prints the path of the rendered
HTML to stderr so you can open it manually:

```text
caraspace: diagram written to /tmp/caraspace-12345-0-987654321.html
```

This is exactly the configuration the Docker setup uses — see the
[headless mode](../workflows/headless.md) and [Docker](../workflows/docker.md)
pages for the full picture.

## What to do next

The default layout will get you most of the way for trees and small
graphs. Once you have a non-trivial structure, you'll start wanting to
say things like "left children go down-and-left" or "color these nodes
red." That's what decorators are for — read
[Specs, not draw calls](../decorators/overview.md) next.
