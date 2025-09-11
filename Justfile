build:
    cargo build --target wasm32-unknown-unknown  --no-default-features
    cp ./target/wasm32-unknown-unknown/debug/archivist.wasm web/archivist.wasm

build-release:
    cargo build --release --target wasm32-unknown-unknown --no-default-features
    cp ./target/wasm32-unknown-unknown/release/archivist.wasm web/archivist.wasm


serve:
    basic-http-server web/