use mystia_core::Compiler;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct Result(Vec<u8>, String);

#[wasm_bindgen]
impl Result {
    #[wasm_bindgen]
    pub fn get_bytecode(&self) -> Vec<u8> {
        self.0.clone()
    }

    #[wasm_bindgen]
    pub fn get_return_type(&self) -> String {
        self.1.clone()
    }
}

#[wasm_bindgen]
pub fn mystia(source: &str) -> Option<Result> {
    let mut compiler = Compiler::new();
    if let Some(wat_code) = compiler.build(source) {
        if let Ok(bytes) = wat::parse_str(wat_code) {
            Some(Result(bytes, compiler.program_return.ffi_json()))
        } else {
            None
        }
    } else {
        None
    }
}
