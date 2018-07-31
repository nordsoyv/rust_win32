 ### Setting up wasm build
 
1. Add nightly 

`rustup install nightly`
2. Set nightly as default

`rustup default nightly`
3. Add wasm target

`rustup target add wasm32-unknown-unknown` 
4. Add wasm-bindgen cli tool

`cargo +nightly install wasm-bindgen-cli`
5. Add build wasm configuration

`cargo +nightly build --target wasm32-unknown-unknown`
6. Run wasm-buildgen on generated wasm file

`wasm-bindgen target/wasm32-unknown-unknown/debug/js_hello_world.wasm --out-dir .`
Can run `convert_wasm.bat` file for this