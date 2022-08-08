#/bin/bash

while getopts r flag
do
    case "${flag}" in
        r) release=true;;
    esac
done

echo $release

if [ "$release" = true ]
then
    echo "Building release"
    cargo build --target wasm32-unknown-unknown --release --no-default-features
    wasm-bindgen ./target/wasm32-unknown-unknown/release/bload_oath.wasm --out-dir ./dist/wasm --no-modules --no-typescript
else
    echo "Building debug"
    cargo build --target wasm32-unknown-unknown --no-default-features
    wasm-bindgen ./target/wasm32-unknown-unknown/debug/bload_oath.wasm --out-dir ./dist/wasm --no-modules --no-typescript
fi
