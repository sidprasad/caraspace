# Vendored spytial-core assets

These files are vendored from [spytial-core](https://github.com/sidprasad/spytial-core)
at the version recorded in `VERSION.txt`. They are bundled into the rendered HTML by
`src/lib.rs` at compile time via `include_str!`, so `dbg!`/`diagram` works offline and
without network access.

To update: rebuild spytial-core (`npm run build:all`), then copy the four files in
this directory from `spytial-core/dist/`. Update `VERSION.txt` to match.

| File in this directory | Source in spytial-core |
|---|---|
| `spytial-core.global.js` | `dist/browser/spytial-core-complete.global.js` |
| `spytial-core.css` | `dist/browser/spytial-core-complete.css` |
| `react-component-integration.global.js` | `dist/components/react-component-integration.global.js` |
| `react-component-integration.css` | `dist/components/react-component-integration.css` |

`.map` files are intentionally not vendored to keep the published crate small.
