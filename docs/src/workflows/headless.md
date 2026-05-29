# Headless mode

Caraspace's default behavior — open a browser tab on every `diagram()`
or `dbg!()` call — is the right thing for an interactive shell, but it's
the wrong thing for CI, remote shells, containers, or test suites. Two
environment variables cover the headless cases.

## `SPYTIAL_NO_OPEN` — suppress the browser launch

Set to `1`, `true`, or `yes` (case-insensitive) and caraspace will skip
the platform-specific browser-open command:

```sh
SPYTIAL_NO_OPEN=1 cargo run --example rbt
```

The rendered HTML is still written to disk, and the path is printed to
stderr:

```text
caraspace: diagram written to /tmp/caraspace-12345-0-987654321.html
```

stderr is unaffected by `SPYTIAL_NO_OPEN` — `dbg!`'s pretty-printed
output still appears, so `cargo test` capture behaves exactly as it does
for `std::dbg!`.

This is the right setting for:

- CI jobs that run examples or tests
- `cargo run` over SSH
- Docker containers (the bundled image sets this automatically)
- Any other environment where there's no display

## `SPYTIAL_OUTPUT_PATH` — pin the output filename

By default each diagram gets a unique path under the OS temp dir, with a
name like `caraspace-{pid}-{counter}-{nanos}.html`. The randomness
prevents concurrent `dbg!` calls from trampling each other.

If you want a stable, predictable path — for serving the file with a
static HTTP server, or for round-tripping it off a remote machine — set
`SPYTIAL_OUTPUT_PATH`:

```sh
SPYTIAL_OUTPUT_PATH=/var/www/diagram.html cargo run
```

The value is used verbatim. Caveats:

- **Each `diagram()` call overwrites the file.** If your program calls
  `dbg!` more than once, only the last result is preserved. Pair this
  with `SPYTIAL_NO_OPEN=1` if you're doing one rendering at a time.
- **The directory must exist.** Caraspace doesn't create parent
  directories.
- **Concurrent calls race.** If two threads emit diagrams at the same
  time, the file content is undefined.

## Combining the two

The typical headless setup uses both:

```sh
SPYTIAL_NO_OPEN=1 \
SPYTIAL_OUTPUT_PATH=/tmp/diagram.html \
  cargo run --example rbt
```

That's the configuration the [Docker workflow](./docker.md) wires up by
default — the entrypoint pins the output path to a known location and
disables the browser launch, so the container can serve the rendered
HTML over HTTP.
