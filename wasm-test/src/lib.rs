use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    String::from(format!("Hello {}", name))
}

#[wasm_bindgen]
pub fn compare_arrays(array1: &[f32], array2: &[f32]) -> bool {
    if array1.len() != array2.len() { return false; }

    for i in 0..array1.len() {
        if array1[i] != array2[i] { return false; }
    }

    return true;
}