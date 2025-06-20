use mystia_core::{Compiler, type_to_json};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct Mystia {
    bytecode: Vec<u8>,
    return_type: String,
}

#[wasm_bindgen]
impl Mystia {
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
pub fn mystia(source: &str) -> Result<Mystia, String> {
    let mut compiler = Compiler::new();
    if let Some(wat_code) = compiler.build(source) {
        let bytes = wat::parse_str(wat_code.clone()).unwrap();
        Ok(Mystia {
            bytecode: bytes,
            return_type: type_to_json(&compiler.program_return),
        })
    } else {
        let error_message = "failed to parse, compile or check type consistency";
        Err(compiler.occurred_error.unwrap_or(error_message.to_string()))
    }
}
