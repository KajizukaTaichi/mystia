use crate::*;

#[derive(Debug, Clone)]
pub enum Value {
    Integer(i32),
    Bool(bool),
    Float(f64),
    Pointer(i32),
    String(String),
}

impl Node for Value {
    fn parse(source: &str) -> Option<Self> {
        Some(
            // Integer literal
            if let Ok(n) = source.parse::<i32>() {
                Value::Integer(n)
            // Float number literal
            } else if let Ok(n) = source.parse::<f64>() {
                Value::Float(n)
            // Boolean literal
            } else if let Ok(n) = source.parse::<bool>() {
                Value::Bool(n)
            // Pointer: memory address
            } else if let Some(source) = source.strip_prefix("0x") {
                let Ok(addr) = i32::from_str_radix(source, 16) else {
                    return None;
                };
                Value::Pointer(addr)
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
            Value::Integer(n) | Value::Pointer(n) => format!("(i32.const {n})"),
            Value::Float(n) => format!("(f64.const {n})"),
            Value::Bool(n) => Value::Integer(if *n { 1 } else { 0 }).compile(ctx)?,
            Value::String(str) => {
                let result = Expr::Literal(Value::Pointer(ctx.alloc_index.clone())).compile(ctx)?;
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
            Value::String(_) | Value::Pointer(_) => Type::Pointer,
            Value::Float(_) => Type::Float,
            Value::Bool(_) => Type::Bool,
        })
    }

    fn addr_infer(&self, _: &mut Compiler) -> Option<i32> {
        Some(match self {
            Value::Pointer(to) => *to,
            _ => 0,
        })
    }
}

#[derive(Clone, Debug)]
pub enum Type {
    Integer,
    Float,
    Bool,
    Pointer,
    Void,
}

impl Node for Type {
    fn parse(source: &str) -> Option<Self> {
        match source.trim() {
            "int" => Some(Self::Integer),
            "float" => Some(Self::Float),
            "ptr" => Some(Self::Pointer),
            "void" => Some(Self::Void),
            _ => None,
        }
    }

    fn compile(&self, _: &mut Compiler) -> Option<String> {
        Some(
            match self {
                Self::Integer | Self::Pointer | Self::Bool => "i32",
                Self::Float => "f64",
                Self::Void => return None,
            }
            .to_string(),
        )
    }

    fn type_infer(&self, _: &mut Compiler) -> Option<Type> {
        Some(self.clone())
    }

    fn addr_infer(&self, _: &mut Compiler) -> Option<i32> {
        None
    }
}
