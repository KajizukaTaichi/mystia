use crate::*;

#[derive(Debug, Clone)]
pub enum Value {
    Nimber(f64),
    Bool(bool),
    Array(i32, usize),
    String(String),
}

impl Node for Value {
    fn parse(source: &str) -> Option<Self> {
        Some(
            // Number literal
            if let Ok(n) = source.parse::<f64>() {
                Value::Number(n)
            // Boolean literal
            } else if let Ok(n) = source.parse::<bool>() {
                Value::Bool(n)
            // String litera;
            } else if source.starts_with("\"") && source.ends_with("\"") {
                let source = source.get(1..source.len() - 1)?.trim();
                Value::String(str_escape(source))
            } else {
                return None;
            },
        )
    }

    fn compile(&self, ctx: &mut Compiler) -> Option<String> {
        Some(match self {
            Value::Integer(n) | Value::Array(n, _) => format!("(i32.const {n})"),
            Value::Float(n) => format!("(f64.const {n})"),
            Value::Bool(n) => Value::Integer(if *n { 1 } else { 0 }).compile(ctx)?,
            Value::String(str) => {
                let result = Value::Array(ctx.alloc_index.clone(), str.len()).compile(ctx)?;
                ctx.static_data
                    .push(format!(r#"(data {} "{str}")"#, result));
                ctx.alloc_index += str.len() as i32;
                result
            }
        })
    }

    fn type_infer(&self, _: &mut Compiler) -> Option<Type> {
        Some(match self {
            Value::Integer(_) => Type::Integer,
            Value::String(_) => Type::Pointer,
            Value::Float(_) => Type::Float,
            Value::Bool(_) => Type::Bool,
        })
    }
}

#[derive(Clone, Debug)]
pub enum Type {
    Number,
    Bool,
    Array,
    String,
    Void,
}

impl Node for Type {
    fn parse(source: &str) -> Option<Self> {
        match source.trim() {
            "num" => Some(Self::Number),
            "str" => Some(Self::String),
            "arr" => Some(Self::Array),
            "nil" => Some(Self::Void),
            _ => None,
        }
    }

    fn compile(&self, _: &mut Compiler) -> Option<String> {
        Some(
            match self {
                Self::Array | Self::String => "i32",
                Self::Number => "f64",
                Self::Void => return None,
            }
            .to_string(),
        )
    }

    fn type_infer(&self, _: &mut Compiler) -> Option<Type> {
        Some(self.clone())
    }
}
