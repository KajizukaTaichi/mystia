use mystia_core::Compiler;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn mystia(source: &str) -> Option<Vec<u8>> {
    if let Some(wat_code) = Compiler::new().build(source) {
        if let Ok(bytes) = wat::parse_str(wat_code) {
            Some(bytes)
        } else {
            None
        }
    } else {
        None
    }
}
