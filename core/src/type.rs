use crate::*;

#[derive(Clone, Debug)]
pub enum Type {
    Integer,
    Number,
    Bool,
    Array(Box<Type>, usize),
    Dict(IndexMap<String, Type>),
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
                let source = source.strip_prefix("[")?.strip_suffix("]")?;
                let (typ, len) = source.rsplit_once(";")?;
                Some(Type::Array(
                    Box::new(Type::parse(typ)?),
                    ok!(len.trim().parse())?,
                ))
            }
        }
    }

    fn compile(&self, _: &mut Compiler) -> Option<String> {
        Some(
            match self {
                Self::Number => "f64",
                Self::Array(_, _) | Self::Dict(_) | Self::String | Self::Bool | Type::Integer => {
                    "i32"
                }
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
            Type::Array(_, _) | Type::String | Type::Bool | Type::Dict(_) | Type::Integer => 4,
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
            Self::Dict(dict) => format!(
                "dict{{ {} }}",
                join!(
                    dict.iter()
                        .map(|(k, t)| format!("{k} {}", t.format()))
                        .collect::<Vec<_>>()
                )
            ),
            Self::Array(typ, len) => format!("[{}; {len}]", typ.format()),
        }
    }
}
