use mystia_core::Compiler;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn mystia_compile(source: &str) -> Option<String> {
    Compiler::new().build(source)
}
