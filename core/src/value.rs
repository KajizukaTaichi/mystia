use crate::*;

#[derive(Debug, Clone)]
pub enum Value {
    Integer(i32),
    Float(f64),
    Array(Vec<i32>),
    String(String),
}

#[derive(Clone, Debug)]
pub enum Type {
    Integer,
    Float,
    Void,
}

impl Node for Type {
    fn parse(source: &str) -> Option<Self> {
        match source.trim() {
            "int" | "ptr" => Some(Self::Integer),
            "float" => Some(Self::Float),
            "void" => Some(Self::Void),
            _ => None,
        }
    }

    fn compile(&self, _: &mut Compiler) -> String {
        match self {
            Self::Integer => "i32",
            Self::Float => "f64",
            Self::Void => todo!(),
        }
        .to_string()
    }

    fn type_infer(&self, _: &mut Compiler) -> Type {
        self.clone()
    }
}
