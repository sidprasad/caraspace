# Docker

The repository ships a `Dockerfile` and a small entrypoint script that
together give you a one-command demo of caraspace without installing a
Rust toolchain on your host.

## Build the image

```sh
docker build -t caraspace .
```

## Run the default example

The default `CMD` runs the red-black tree example:

```sh
docker run --rm -p 8080:8080 caraspace
```

The container builds the example, runs it once, and then serves the
rendered HTML over a small HTTP server on port 8080. Open
<http://localhost:8080/rust_viz_data.html> in a browser on your host to
see the diagram.

## Run a specific example

Pass the example name as an argument:

```sh
docker run --rm -p 8080:8080 caraspace demo
```

Available examples mirror the ones in `examples/` — `dbg_basic`, `demo`,
`rbt`, and any others present in the source tree.

## What the entrypoint does

The container's entrypoint sets two environment variables before
running your example:

- `SPYTIAL_NO_OPEN=1` disables the browser-launch step, since there's
  no display inside the container.
- `SPYTIAL_OUTPUT_PATH=/tmp/rust_viz_data.html` pins the rendered HTML
  to a known location. That path is what the in-container HTTP server
  serves at `/rust_viz_data.html`.

Together they convert `diagram()`'s default "write to a random temp
file, open a browser" behavior into "write to a fixed file, expose
over HTTP." See the [headless mode](./headless.md) page for the same
two knobs in non-Docker setups.

## Volume-mounting for development

If you want to iterate on examples without rebuilding the image, mount
the source tree into the container:

```sh
docker run --rm -p 8080:8080 \
  -v "$PWD:/work" \
  -w /work \
  caraspace cargo run --example rbt
```

The compiled binary lives in the container, but the `.html` it
generates ends up in the mounted directory's tree, so you can `cat` or
post-process it from the host.

## Caveats

- Browser launch is always disabled inside the container; you must open
  the served URL on your host.
- The HTTP server is single-threaded and intended for local viewing
  only, not production hosting.
- Each container run regenerates the diagram on top of itself
  (`SPYTIAL_OUTPUT_PATH` is fixed). If your example calls `dbg!`
  multiple times, only the last call's output is preserved.
