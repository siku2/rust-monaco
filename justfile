@chores:
    just fmt
    just clippy

clippy:
    cargo clippy --all-targets

fmt:
    cargo +nightly fmt

doc *args:
    cargo +nightly doc {{args}}

@build_js:
    just build/build
    rm js/*
    cp build/dist/*.js js
