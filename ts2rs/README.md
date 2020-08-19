# ts2rs

A Typescript definition to `wasm-bindgen` converter.

This tool takes a typescript definition file (".d.ts") and tries to create the corresponding `wasm-bindgen` definitions.
It's specifically written for this project and doesn't try to be a complete solution.
To give you an idea of how sinful this tool really is I just need to mention that the "parsing" of the typescript file mostly uses regular expressions.
This means that the parser only works for decently formatted input files.

There are also a ton of other quirks and I'm not gonna list all of them because I don't expect anyone to read this.

If you're actually thinking about using this for your own project, please contact me. I'd love to collaborate on a proper solution instead!

The tool will output a Rust module for each namespace.
The result will most likely not work out of the box.
Things that need to be done include:

- import types from other modules and web-sys
- function overloads need to be given different names
- type unions need to be broken up
- idents that collide with Rust keywords need to be changed
- types need to be corrected

## Details

- Input file is hard-coded in `__main__.py` to "monaco.d.ts"
- Input file must contain one or more namespaces.
- Type aliases are only supported for string unions
- object types that span over multiple lines and contain a semicolon break the parser => rewrite as single line

## Usage

Needs Python 3.8 to run.

```bash
python -m ts2rs
```

## Unsupported things

- Generics
- Type aliases other than string enums
- Interface function syntax
- Optional method syntax => rewrite as an optional property instead
