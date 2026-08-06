# real-blog

Kyle's self-introduction rendered by libcosmic + iced on WebAssembly. Dioxus is
deliberately retained only for its `dx` development server and asset bundling;
the UI, renderer, and browser event loop are owned by iced.

The web-compatible libcosmic/iced stack is a recursive git submodule at
`vendor/libcosmic`. Its upstream revisions and compatibility patches are
documented in `vendor/libcosmic/WEB_SUPPORT.md`.

Traditional Chinese glyphs use justfont's `jf-openhuninn` 2.1. The release font
and its license are stored in `assets/fonts` and loaded into iced at startup.

`vendor/atomicwrites` preserves libcosmic's native config API while returning a
clear unsupported error for browser filesystem writes. Browsers have no COSMIC
config directory, so the vendored config fallback keeps the default theme.

## Develop

Initialize the dependency stack after cloning the project:

```sh
git submodule update --init --recursive
```

Then start the Dioxus asset server:

```sh
dx serve --platform web
```

## Release bundle

```sh
dx build --platform web --release --debug-symbols false
```

The Cloudflare Pages configuration serves the generated bundle from
`target/dx/real-blog/release/web/public`.

`dx` 0.7.7 and `wasm-bindgen` 0.2.118 are intentionally pinned together.
Disabling deployment DWARF is important: Binaryen 129 aborts on the large
debug-enabled iced/WGPU module, while the command above completes bundling and
optimization normally.

Do not patch only `iced_winit` to crates.io iced: all iced crates must remain on
the same revision to avoid incompatible duplicate `iced_core` and
`iced_program` types.
