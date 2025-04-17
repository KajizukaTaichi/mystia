use crate::*;

pub type Dict = IndexMap<String, (i32, Type)>;
#[derive(Clone, Debug)]
pub enum Type {
    Integer,
    Number,
    Bool,
    String,
    Array(Box<Type>, usize),
    Dict(Dict),
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
                Type::Integer | Self::Bool | Self::String | Self::Array(_, _) | Self::Dict(_) => {
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
    pub fn pointer_length(&self) -> i32 {
        match self {
            Type::Array(_, _) | Type::String | Type::Bool | Type::Dict(_) | Type::Integer => 4,
            Type::Number => 8,
            Type::Void => 0,
        }
    }

    pub fn bytes_length(&self) -> Option<usize> {
        match self {
            Self::Integer => Some(4),
            Self::Number => Some(8),
            Self::Bool => Some(4),
            Self::Void => Some(0),
            Self::Dict(dict) => Some(dict.len() * 4),
            Self::Array(_, len) => Some(len * 4),
            Self::String => None,
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
                "{{ {} }}",
                dict.iter()
                    .map(|(k, t)| format!("{k}: {}", t.1.format()))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Self::Array(typ, len) => format!("[{}; {len}]", typ.format()),
        }
    }
}
