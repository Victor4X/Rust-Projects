use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn alert(string: &str);
}

pub fn greet(name: &str) {
    alert(&format!("Hello {}", name));
}