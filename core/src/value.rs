use crate::*;

#[derive(Debug, Clone)]
pub enum Value {
    Integer(i32),
    Float(f64),
}

#[derive(Clone, Debug)]
pub enum Type {
    Integer,
    Float,
}

impl Node for Type {
    fn parse(source: &str) -> Option<Self> {
        match source.trim() {
            "int" => Some(Self::Integer),
            "float" => Some(Self::Float),
            _ => None,
        }
    }

    fn compile(&self, _: &mut Compiler) -> String {
        match self {
            Self::Integer => "i32",
            Self::Float => "f64",
        }
        .to_string()
    }

    fn type_infer(&self, _: &mut Compiler) -> Type {
        self.clone()
    }
}
