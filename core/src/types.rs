use crate::ast::expr::ExprKind;
use crate::error::error::Error::InvalidType;
use crate::lexer::token::Token;
use crate::Result;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum Type {
    Int,
    Bool,
    Void,
    Unresolved,
    Error,
    String,
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let type_name = match self {
            Type::Int => "int",
            Type::Bool => "bool",
            Type::Unresolved => "unresolved",
            Type::Void => "void",
            Type::Error => "?",
            Type::String => "string",
        };

        write!(f, "{}", type_name)
    }
}

impl Type {
    pub fn is_assignable_to(&self, other: &Type) -> bool {
        matches!(
            (self, other),
            (Type::Int, Type::Int)
                | (Type::Bool, Type::Bool)
                | (Type::Error, _)
                | (_, Type::Error)
                | (Type::String, Type::String)
        )
    }

    pub fn from_str(s: &str) -> Option<Type> {
        match s {
            "int" => Some(Type::Int),
            "bool" => Some(Type::Bool),
            "void" => Some(Type::Void),
            "string" => Some(Type::String),
            _ => None,
        }
    }

    pub fn to_str(&self) -> String {
        match self {
            Type::Int => "int".to_string(),
            Type::Bool => "bool".to_string(),
            Type::Void => "void".to_string(),
            Type::String => "string".to_string(),
            Type::Unresolved => "unresolved".to_string(),
            Type::Error => "?".to_string(),
        }
    }
}

pub fn parse_type(s: &Token, content: &String) -> Result<Type> {
    let name = Type::from_str(&s.span.literal);

    match name {
        Some(t) => Ok(t),
        None => Err(InvalidType(
            s.span.literal.clone(),
            s.span.clone(),
            content.clone(),
        )),
    }
}
