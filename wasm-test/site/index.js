import("./node_modules/wasm-test/wasm_test.js").then((js) => {
    let result = js.greet("Anders");
    console.log(result);

    let array1 = new Float32Array([1,2,3,4]);
    let array2 = new Float32Array([1,20,3,4]);

    let compareArrays = js.compare_arrays(array1, array2);
    console.log(compareArrays);
});