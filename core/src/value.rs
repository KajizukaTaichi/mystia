use crate::*;

#[derive(Debug, Clone)]
pub enum Value {
    Integer(i32),
    Number(f64),
    Bool(bool),
    Array(i32, Type),
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
            Value::Number(n) => format!("(f64.const {n})"),
            Value::Array(n, _) | Value::Integer(n) => format!("(i32.const {n})"),
            Value::Bool(n) => Value::Integer(if *n { 1 } else { 0 }).compile(ctx)?,
            Value::String(str) => {
                let result = Value::Array(ctx.alloc_index.clone(), Type::String).compile(ctx)?;
                ctx.static_data
                    .push(format!(r#"(data {} "{str}")"#, result));
                ctx.alloc_index += str.len() as i32;
                result
            }
        })
    }

    fn type_infer(&self, _: &mut Compiler) -> Option<Type> {
        Some(match self {
            Value::Number(_) => Type::Number,
            Value::Integer(_) => Type::Integer,
            Value::Bool(_) => Type::Bool,
            Value::String(_) => Type::String,
            Value::Array(_, t) => Type::Array(Box::new(t.clone())),
        })
    }
}

#[derive(Clone, Debug)]
pub enum Type {
    Integer,
    Number,
    Bool,
    Array(Box<Type>),
    String,
    Void,
}

impl Node for Type {
    fn parse(source: &str) -> Option<Self> {
        match source.trim() {
            "num" => Some(Self::Number),
            "str" => Some(Self::String),
            "nil" => Some(Self::Void),
            _ => {
                if let Some(source) = source.strip_prefix("[") {
                    if let Some(source) = source.strip_suffix("]") {
                        Some(Type::Array(Box::new(Type::parse(source)?)))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
        }
    }

    fn compile(&self, _: &mut Compiler) -> Option<String> {
        Some(
            match self {
                Self::Number => "f64",
                Self::Array(_) | Self::String | Self::Bool | Type::Integer => "i32",
                Self::Void => return None,
            }
            .to_string(),
        )
    }

    fn type_infer(&self, _: &mut Compiler) -> Option<Type> {
        Some(self.clone())
    }
}

impl Type {
    pub fn bytes_length(&self) -> i32 {
        match self {
            Type::Array(_) | Type::String | Type::Bool | Type::Integer => 4,
            Type::Number => 8,
            Type::Void => 0,
        }
    }
}
