import("./node_modules/wasm-test/wasm_test.js").then((js) => {
    let result = js.greet("Anders");
    console.log(result);
});