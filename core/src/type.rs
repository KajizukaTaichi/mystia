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
                    for line in tokenize(source, &[","], false, true, false)? {
                        let (name, value) = line.split_once(":")?;
                        let typ = Type::parse(value)?;
                        result.insert(name.trim().to_string(), (0, typ.clone()));
                    }
                    Some(Type::Dict(result))
                } else if source.starts_with("(") && source.ends_with(")") {
                    let source = source.get(1..source.len() - 1)?.trim();
                    let result = tokenize(source, &["|"], false, true, false)?;
                    let result: Vec<String> = result.iter().map(|x| x.trim().to_string()).collect();
                    if !result.iter().all(|x| is_identifier(x)) {
                        return None;
                    };
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
                Type::Number => "f32",
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
        self.solve_alias(ctx, vec![])
    }
}

impl Type {
    pub fn solve_alias(&self, ctx: &mut Compiler, xpct: Vec<Type>) -> Option<Type> {
        for x in &xpct {
            if x.compress_alias(ctx).format() == self.compress_alias(ctx).format() {
                return Some(self.compress_alias(ctx));
            }
        }
        let result = match self {
            Type::Alias(name) => {
                let Some(typ) = ctx.type_alias.get(name).cloned() else {
                    let msg = format!("undefined type alias `{name}`");
                    ctx.occurred_error = Some(msg);
                    return None;
                };
                typ.solve_alias(ctx, xpct.clone())
            }
            Type::Array(typ) => Some(Type::Array(Box::new(
                typ.solve_alias(ctx, [xpct.clone(), vec![self.clone()]].concat())?,
            ))),
            Type::Dict(dict) => {
                let mut a = IndexMap::new();
                let mut offset = 0;
                for (name, (_, typ)) in dict {
                    let typ = typ.solve_alias(ctx, [xpct.clone(), vec![self.clone()]].concat())?;
                    a.insert(name.clone(), (offset.clone(), typ.clone()));
                    offset += 4
                }
                Some(Type::Dict(a))
            }
            _ => Some(self.clone()),
        };
        result
    }

    pub fn compress_alias(&self, ctx: &Compiler) -> Type {
        let mut aliases = ctx.type_alias.iter();
        let typ = match self {
            Type::Array(typ) => Type::Array(Box::new(typ.compress_alias(ctx))),
            Type::Dict(dict) => Type::Dict(
                dict.iter()
                    .map(|(k, (o, t))| (k.clone(), (o.clone(), t.compress_alias(ctx))))
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
