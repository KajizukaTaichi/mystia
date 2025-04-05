use crate::*;

#[derive(Debug, Clone)]
pub enum Value {
    Integer(i32),
    Number(f64),
    Bool(bool),
    Array(i32, usize, Type),
    Dict(i32, Dict),
    String(String),
}

impl Node for Value {
    fn parse(source: &str) -> Option<Self> {
        Some(
            // Integer literal
            if let Ok(n) = source.parse::<i32>() {
                Value::Integer(n)
            // Number literal
            } else if let Ok(n) = source.parse::<f64>() {
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
            Value::Integer(n) | Value::Array(n, _, _) | Value::Dict(n, _) => {
                format!("(i32.const {n})")
            }
            Value::Bool(n) => Value::Integer(if *n { 1 } else { 0 }).compile(ctx)?,
            Value::String(str) => {
                let len = str.len() + 1;
                let result = Value::Array(ctx.alloc_index, len, Type::String).compile(ctx)?;
                ctx.static_data
                    .push(format!(r#"(data {} "{str}\00")"#, result));
                ctx.alloc_index += len as i32;
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
            Value::Array(_, len, typ) => Type::Array(Box::new(typ.clone()), *len),
            Value::Dict(_, typ) => Type::Dict(typ.clone()),
        })
    }
}
