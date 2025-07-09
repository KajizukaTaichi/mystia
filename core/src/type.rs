use crate::*;

pub type Dict = IndexMap<String, (i32, Type)>;
pub type Enum = Vec<String>;
#[derive(Clone, Debug)]
pub enum Type {
    Integer,
    Number,
    Bool,
    String,
    Array(Box<Type>),
    Dict(Dict),
    Enum(Enum),
    Alias(String),
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
                if !source.is_ascii() {
                    return None;
                }
                if source.starts_with("[") && source.ends_with("]") {
                    let source = source.get(1..source.len() - 1)?.trim();
                    Some(Type::Array(Box::new(Type::parse(source)?)))
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
                } else if is_identifier(&source) {
                    Some(Type::Alias(source))
                } else {
                    None
                }
            }
        }
    }

    fn compile(&self, ctx: &mut Compiler) -> Option<String> {
        Some(
            match self.type_infer(ctx)? {
                Type::Number => "f64",
                Type::Integer
                | Type::Bool
                | Type::String
                | Type::Array(_)
                | Type::Dict(_)
                | Type::Enum(_) => "i32",
                _ => return None,
            }
            .to_string(),
        )
    }

    fn type_infer(&self, ctx: &mut Compiler) -> Option<Type> {
        self.compress_alias(ctx, &None)
    }
}

impl Type {
    pub fn pointer_length(&self) -> i32 {
        match self {
            Type::Array(_)
            | Type::String
            | Type::Bool
            | Type::Dict(_)
            | Type::Integer
            | Type::Enum(_) => 4,
            Type::Number => 8,
            _ => 0,
        }
    }

    pub fn compress_alias(&self, ctx: &mut Compiler, xpct: &Option<String>) -> Option<Type> {
        macro_rules! ctx {
            () => {{
                let mut ctx = ctx.clone();
                if let Some(name) = xpct {
                    let typ = ctx.type_alias.get(name)?;
                    ctx.type_alias = IndexMap::from([(name.to_owned(), typ.clone())]);
                } else {
                    ctx.type_alias.clear();
                }
                ctx
            }};
        }
        match self {
            Type::Alias(name) => {
                let Some(typ) = ctx.type_alias.get(name).cloned() else {
                    let msg = format!("undefined type alias `{name}`");
                    ctx.occurred_error = Some(msg);
                    return None;
                };
                if xpct.clone().map(|xpct| &xpct == name).unwrap_or(false) {
                    Some(self.clone())
                } else {
                    typ.compress_alias(ctx, &Some(name.to_owned()))
                }
            }
            Type::Array(typ) => Some(Type::Array(Box::new(
                typ.compress_alias(ctx, xpct)?.decompress_alias(&ctx!()),
            ))),
            Type::Dict(dict) => {
                let mut a = IndexMap::new();
                for (name, (offset, typ)) in dict {
                    a.insert(
                        name.clone(),
                        (
                            offset.clone(),
                            typ.compress_alias(ctx, xpct)?.decompress_alias(&ctx!()),
                        ),
                    );
                }
                Some(Type::Dict(a))
            }
            _ => Some(self.clone()),
        }
    }

    pub fn decompress_alias(&self, ctx: &Compiler) -> Type {
        let mut aliases = ctx.type_alias.iter();
        let typ = match self {
            Type::Array(typ) => Type::Array(Box::new(typ.decompress_alias(ctx))),
            Type::Dict(dict) => Type::Dict(
                dict.iter()
                    .map(|(k, (o, t))| (k.clone(), (o.clone(), t.decompress_alias(ctx))))
                    .collect(),
            ),
            _ => self.clone(),
        };
        if let Some(i) = aliases.find(|(_, v)| v.format() == typ.format()) {
            Type::Alias(i.0.clone())
        } else {
            typ
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
            Type::Array(typ) => format!("[{}]", typ.format()),
            Type::Alias(name) => name.to_string(),
            Type::Any => "any".to_string(),
        }
    }
}
