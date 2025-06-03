use crate::*;

pub type Dict = IndexMap<String, (i32, Type)>;
pub type Enum = Vec<String>;
#[derive(Clone, Debug)]
pub enum Type {
    Integer,
    Number,
    Bool,
    String,
    Array(Box<Type>, usize),
    Dict(Dict),
    Enum(Enum),
    Alias(String, Vec<Type>),
    Void,
    Any,
}

impl Node for Type {
    fn parse(source: &str) -> Option<Type> {
        match source.trim() {
            "int" => Some(Type::Integer),
            "num" => Some(Type::Number),
            "bool" => Some(Type::Bool),
            "str" => Some(Type::String),
            "void" => Some(Type::Void),
            _ => {
                let source = source.trim().to_string();
                if source.starts_with("[") && source.ends_with("]") {
                    let source = source.get(1..source.len() - 1)?.trim();
                    let (typ, len) = source.rsplit_once(";")?;
                    Some(Type::Array(
                        Box::new(Type::parse(typ)?),
                        ok!(len.trim().parse())?,
                    ))
                } else if source.starts_with("@{") && source.ends_with("}") {
                    let source = source.get(2..source.len() - 1)?.trim();
                    let mut result = IndexMap::new();
                    let mut index = 0;
                    for line in tokenize(source, &[","], false, true, false)? {
                        let (name, value) = line.split_once(":")?;
                        let typ = Type::parse(value)?;
                        result.insert(name.trim().to_string(), (index, typ.clone()));
                        index += typ.pointer_length();
                    }
                    Some(Type::Dict(result))
                } else if source.starts_with("(") && source.ends_with(")") {
                    let source = source.get(1..source.len() - 1)?.trim();
                    let result = tokenize(source, &["|"], false, true, false)?;
                    let result = result.iter().map(|x| x.trim().to_string()).collect();
                    Some(Type::Enum(result))
                } else if source.contains("(") && source.ends_with(")") {
                    let (name, args) = source.get(..source.len() - 1)?.split_once("(")?;
                    let args = tokenize(args, &[","], false, true, false)?;
                    let args = args.iter().map(|i| Type::parse(&i));
                    let args = args.collect::<Option<Vec<_>>>()?;
                    Some(Type::Alias(name.to_string(), args))
                } else {
                    Some(Type::Alias(source, vec![]))
                }
            }
        }
    }

    fn compile(&self, _: &mut Compiler) -> Option<String> {
        Some(
            match self {
                Type::Number => "f64",
                Type::Integer
                | Type::Bool
                | Type::String
                | Type::Array(_, _)
                | Type::Dict(_)
                | Type::Enum(_) => "i32",
                _ => return None,
            }
            .to_string(),
        )
    }

    fn type_infer(&self, ctx: &mut Compiler) -> Option<Type> {
        match self {
            Type::Alias(name, args) => {
                let Some(typ) = ctx.type_alias.get(name).cloned() else {
                    let msg = format!("undefined type alias `{name}`");
                    ctx.occurred_error = Some(msg);
                    return None;
                };
                let mut new_ctx = ctx.clone();
                for (id, arg) in args.iter().enumerate() {
                    new_ctx.type_alias.insert(format!("T{id}"), arg.clone());
                }
                typ.type_infer(&mut new_ctx)
            }
            Type::Array(typ, len) => Some(Type::Array(Box::new(typ.type_infer(ctx)?), *len)),
            Type::Dict(dict) => {
                let mut a = IndexMap::new();
                for (name, (offset, typ)) in dict {
                    a.insert(name.clone(), (offset.clone(), typ.type_infer(ctx)?));
                }
                Some(Type::Dict(a))
            }
            _ => Some(self.clone()),
        }
    }
}

impl Type {
    pub fn pointer_length(&self) -> i32 {
        match self {
            Type::Array(_, _)
            | Type::String
            | Type::Bool
            | Type::Dict(_)
            | Type::Integer
            | Type::Enum(_) => 4,
            Type::Number => 8,
            _ => 0,
        }
    }

    pub fn bytes_length(&self) -> Option<usize> {
        match self {
            Type::Integer | Type::Bool | Type::String | Type::Enum(_) => Some(4),
            Type::Number => Some(8),
            Type::Void => Some(0),
            Type::Dict(dict) => Some(dict.len() * 4),
            Type::Array(_, len) => Some(len * 4),
            _ => None,
        }
    }

    pub fn decompress_alias(&self, ctx: &Compiler) -> Type {
        let mut aliases = ctx.type_alias.iter();
        if let Some(i) = aliases.find(|(_, v)| v.format() == self.format()) {
            Type::Alias(i.0.clone(), vec![])
        } else {
            match self {
                Type::Array(typ, len) => Type::Array(Box::new(typ.decompress_alias(ctx)), *len),
                Type::Dict(dict) => Type::Dict(
                    dict.iter()
                        .map(|(k, (o, t))| (k.clone(), (o.clone(), t.decompress_alias(ctx))))
                        .collect(),
                ),
                _ => self.clone(),
            }
        }
    }

    pub fn format(&self) -> String {
        match self {
            Type::Integer => "int".to_string(),
            Type::Number => "num".to_string(),
            Type::Bool => "bool".to_string(),
            Type::String => "str".to_string(),
            Type::Void => "void".to_string(),
            Type::Dict(dict) => format!(
                "@{{ {} }}",
                dict.iter()
                    .map(|(key, (_, typ))| format!("{key}: {}", typ.format()))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Type::Enum(e) => format!("( {} )", e.join(" | ")),
            Type::Array(typ, len) => format!("[{}; {len}]", typ.format()),
            Type::Alias(name, _) => name.to_string(),
            Type::Any => "any".to_string(),
        }
    }
}
