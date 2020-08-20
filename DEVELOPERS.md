# Information for Developers

## General Requirements

More specific requirements will be listed in the subdirectories.

This project is built for the stable Rust toolchain but it requires the nightly versions of rustfmt and clippy.

Run the following command to install the nightly toolchain:

```bash
rustup toolchain install nightly
```

To make your life easier you should also install [`just`](https://github.com/casey/just) which is used to perform various tasks.

You can run the following command to install it:

```bash
cargo install just
```

## Structure

- `build/`: javascript source files
- `js/`: built Javascript code which is used by the code
- `src/`: library source code
- `ts2rs/`: tool for converting Monaco's Typescript definition to `wasm-bindgen`
