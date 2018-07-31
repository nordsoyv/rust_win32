cd wasm-lib
cargo +nightly build --target wasm32-unknown-unknown
cd ..
wasm-bindgen target/wasm32-unknown-unknown/debug/wasm_lib.wasm --out-dir ./web-frontend/src/wasm