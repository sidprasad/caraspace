# Install

Add caraspace and serde to your `Cargo.toml`:

```toml
[dependencies]
caraspace = "0.1"
serde = { version = "1", features = ["derive"] }
```

That's the whole installation. No system packages, no extra fetching at
runtime — the HTML template and the rendering JavaScript are baked into
the crate as `include_str!` payloads, so the diagrams work offline and on
machines with locked-down package managers.

## MSRV

Caraspace's minimum supported Rust version is **1.80**. Newer toolchains
are recommended; we exercise the latest stable in CI and pin the MSRV
floor only where it doesn't cost ergonomics.

If you need to compile on an older toolchain, please open an issue rather
than working around it locally — there are usually cheap fixes.

## Optional: install `mdbook` to build these docs locally

If you want to read the docs offline:

```sh
cargo install mdbook
mdbook serve docs
```

This is only needed if you're contributing changes to the documentation
site itself.
