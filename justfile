# Format the code and run clippy
@chores:
    just fmt
    just clippy

# Run clippy on the code
clippy *args:
    cargo +nightly clippy --all-targets --all-features {{args}}

# Format the code.
# Requires the nightly Rustfmt because it uses unstable features.
fmt *args:
    cargo +nightly fmt {{args}}

# Generate the documentation
doc *args:
    cargo +nightly doc --all-features {{args}}

# Build the Javascript code
@build_js:
    just build/build
    rm js/*
    cp build/dist/*.js js
