use mystia_core::{Compiler, Type};
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
                    type_to_json(typ)
                ))
                .collect::<Vec<_>>()
                .join(", ")
        ),
        Type::Array(typ) => format!("{{ type: \"array\", element: {} }}", type_to_json(typ)),
        Type::Enum(e) => format!(
            "{{ type: \"enum\", enum: [{}] }}",
            e.iter()
                .map(|x| format!("\"{x}\""))
                .collect::<Vec<_>>()
                .join(", ")
        ),
        Type::Alias(name) => format!("{{ type: \"alias\", name: {name} }}"),
        Type::Any => format!("\"any\""),
    }
}
