use mystia_core::{Compiler, Type};
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
                return_type: type_to_json(&compiler.program_return),
            })
        } else {
            None
        }
    } else {
        None
    }
}

pub fn type_to_json(typ: &Type) -> String {
    match typ {
        Type::Integer => "\"int\"".to_string(),
        Type::Number => "\"num\"".to_string(),
        Type::Bool => "\"bool\"".to_string(),
        Type::String => "\"str\"".to_string(),
        Type::Void => "null".to_string(),
        Type::Dict(dict) => format!(
            "{{ type: \"dict\", fields: {{ {} }} }}",
            dict.iter()
                .map(|(k, (offset, typ))| format!(
                    "{k}: {{ type: {}, offset: {offset} }}",
                    typ.ffi_json()
                ))
                .collect::<Vec<_>>()
                .join(", ")
        ),
        Type::Array(typ, len) => format!(
            "{{ type: \"array\", element: {}, length: {len} }}",
            typ.ffi_json()
        ),
        Type::Enum(e) => format!(
            "{{ type: \"enum\", enum: [{}] }}",
            e.iter()
                .map(|x| format!("\"{x}\""))
                .collect::<Vec<_>>()
                .join(", ")
        ),
        Type::Alias(name) => format!("{{ type: \"alias\", name: {name} }}"),
    }
}
