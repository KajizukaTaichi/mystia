use mystia_core::Compiler;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct Mystia;

impl Mystia {
    pub fn compile(source: &str) -> Option<String> {
        Compiler::new().build(source)
    }
}
