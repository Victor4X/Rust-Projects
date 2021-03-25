import("./node_modules/wasm-test/wasm_test.js").then((js) => {
    js.greet("from Rust!");
});