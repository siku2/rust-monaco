# Information for Developers

## General Requirements

More specific requirements will be listed in the subdirectories

- `just`: task runner
- `wasm-pack`: builds the examples

Install both of them:

```bash
cargo install just wasm-pack
```

## Structure

- `build/`: javascript source files
- `js/`: built Javascript code which is used by the code
- `src/`: library source code
- `ts2rs/`: tool for converting Monaco's Typescript definition to `wasm-bindgen`
