use crate::*;

#[derive(Clone, Debug)]
pub enum Type {
    Integer,
    Number,
    Bool,
    Array(Box<Type>, usize),
    String,
    Void,
}

impl Node for Type {
    fn parse(source: &str) -> Option<Self> {
        match source.trim() {
            "int" => Some(Self::Integer),
            "num" => Some(Self::Number),
            "bool" => Some(Self::Bool),
            "str" => Some(Self::String),
            "nil" => Some(Self::Void),
            _ => {
                if let Some(source) = source.strip_prefix("[") {
                    if let Some(source) = source.strip_suffix("]") {
                        let (typ, len) = source.rsplit_once(";")?;
                        Some(Type::Array(Box::new(Type::parse(source)?), len.parse()))
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

    pub fn format(&self) -> String {
        match self {
            Self::Integer => "int".to_string(),
            Self::Number => "num".to_string(),
            Self::Bool => "bool".to_string(),
            Self::String => "str".to_string(),
            Self::Void => "nil".to_string(),
            Self::Array(a) => format!("[{}]", a.format()),
        }
    }
}
