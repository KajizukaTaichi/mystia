use mystia_core::Compiler;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct Result {
    bytecode: Vec<u8>,
    return_type: String,
}

#[wasm_bindgen]
impl Result {
    #[wasm_bindgen]
    pub fn get_bytecode(&self) -> Vec<u8> {
        self.bytecode.clone()
    }

    #[wasm_bindgen]
    pub fn get_return_type(&self) -> String {
        self.return_type.clone()
    }
}

#[wasm_bindgen]
pub fn mystia(source: &str) -> Option<Result> {
    let mut compiler = Compiler::new();
    if let Some(wat_code) = compiler.build(source) {
        if let Ok(bytes) = wat::parse_str(wat_code.clone()) {
            Some(Result {
                bytecode: bytes,
                return_type: compiler.program_return.ffi_json(),
            })
        } else {
            None
        }
    } else {
        None
    }
}
