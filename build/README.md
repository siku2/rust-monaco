# Javascript source

This directory contains the source for building the monaco Javascript code files.

## Requirements

- `Node.js` + `npm`: <https://nodejs.org/>

Run the following command to install the node dependencies:

```bash
just install
```

## Build

```bash
just build
```

> **NOTE:** This will output the files to the `dist/` directory.
> Use `just build_js` in the root directory to update the files used by the library.
