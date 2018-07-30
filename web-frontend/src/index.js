const js = import("./wasm/wasm.js");

js.then(js => {
    js.greet("World!");
});
