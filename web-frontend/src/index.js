const js = import("./wasm/wasm_lib.js");

js.then(js => {
    js.greet("World!");
});
