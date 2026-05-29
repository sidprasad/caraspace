# Contributing

Contributions are welcome via pull requests. Bug reports, feature
requests, and questions are also welcome on the
[issue tracker](https://github.com/sidprasad/caraspace/issues).

The canonical version of this page lives in
[CONTRIBUTING.md](https://github.com/sidprasad/caraspace/blob/main/CONTRIBUTING.md)
in the repository root. The notes below summarize the most common
workflows.

## Local setup

```sh
cargo build
cargo test --lib --tests
cargo test --doc
```

## Running examples

```sh
cargo run --example rbt
```

Examples may open a browser window by default. To run headless (e.g. in
CI), set `SPYTIAL_NO_OPEN=1`:

```sh
SPYTIAL_NO_OPEN=1 cargo run --example rbt
```

See the [headless mode](../workflows/headless.md) page for the full set
of environment variables.

## Code style

- `cargo fmt` clean (rustfmt with the repo's default config).
- `cargo clippy --all-targets -- -D warnings` clean.

## Pull requests

- Add an entry to the `Unreleased` section of
  [CHANGELOG.md](https://github.com/sidprasad/caraspace/blob/main/CHANGELOG.md).
- Changes to decorators must include test coverage in
  `tests/decorators.rs`.
- Keep PRs focused; unrelated cleanups belong in separate PRs.

## License

By contributing, you agree that your contributions will be dual-licensed
under the [MIT](https://github.com/sidprasad/caraspace/blob/main/LICENSE-MIT)
and [Apache 2.0](https://github.com/sidprasad/caraspace/blob/main/LICENSE-APACHE)
licenses, matching the project's own licensing.

## Releases

Maintainers: see [Releasing](./releasing.md) for the release process.
