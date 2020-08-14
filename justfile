@chores:
    just fmt
    just clippy

clippy:
    cargo clippy --bins --examples --tests --benches --all-targets

fmt:
    cargo +nightly fmt

@build_js:
    just build/build
    rm js/*
    cp build/dist/*.js js
