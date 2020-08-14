@build_js:
    just build/build
    rm js/*
    cp build/dist/*.js js
