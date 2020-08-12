@build_js:
    just js/build
    cp js/dist/*.js src/sources/embedded
