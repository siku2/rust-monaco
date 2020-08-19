# Format the code and run clippy
@chores:
    just fmt
    just clippy

# Run clippy on the code
clippy:
    cargo +nightly clippy --all-targets --all-features

# Format the code.
# Requires the nightly Rustfmt because it uses unstable features.
fmt:
    cargo +nightly fmt

# Generate the documentation
doc *args:
    cargo +nightly doc {{args}}

# Build the Javascript code
@build_js:
    just build/build
    rm js/*
    cp build/dist/*.js js
