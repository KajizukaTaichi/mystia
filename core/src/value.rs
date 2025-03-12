use crate::*;

#[derive(Debug, Clone)]
pub enum Value {
    Integer(i32),
    Float(f64),
    String(String),
}

#[derive(Clone, Debug)]
pub enum Type {
    Integer,
    Float,
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

    fn compile(&self, _: &mut Compiler) -> String {
        match self {
            Self::Integer | Self::Pointer => "i32",
            Self::Float => "f64",
            Self::Void => todo!(),
        }
        .to_string()
    }

    fn type_infer(&self, _: &mut Compiler) -> Type {
        self.clone()
    }
}
