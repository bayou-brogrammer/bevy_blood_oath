#/bin/bash

cargo build --target wasm32-unknown-unknown --release

wasm-bindgen ./target/wasm32-unknown-unknown/release/bload_oath.wasm --out-dir ./wasm_help/staging --no-modules --no-typescript
cp ./wasm_help/index.html ./wasm_help/staging/index.html

# ./webglbuild2.bat