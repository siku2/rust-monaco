# `rust-monaco`

[![Crate Info](https://img.shields.io/crates/v/monaco.svg)][crate-info]
[![API Documentation](https://docs.rs/monaco/badge.svg)][api-documentation]

Rust WASM bindings for the [Monaco Editor](https://microsoft.github.io/monaco-editor/) using `wasm-bindgen`.

## Cargo Features

- "api" (default feature) - Activate a more ergonomic Rust API. See [`monaco::api`](https://docs.rs/monaco/latest/monaco/api/)
- "workers" (default feature) - Include the language web workers. If not set you will have to provide them manually or accept a heavy performance penalty.
- "yew-components" - Enable Yew components. See [`monaco::yew`](https://docs.rs/monaco/latest/monaco/yew/)

## Examples

See the [examples](examples) directory.

[crate-info]: https://crates.io/crates/monaco
[api-documentation]: https://docs.rs/monaco
