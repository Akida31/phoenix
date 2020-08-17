use std::fmt::{self, Display, Formatter};

pub mod ident;
pub mod keyword;
pub mod types;
use ident::Ident;
use keyword::Keyword;
use types::Type;

#[derive(Clone, PartialEq)]
pub enum Token {
    Plus,
    Minus,
    Star,
    Slash,
    LeftParenthesis,
    RightParenthesis,
    LeftCurlyBrackets,
    RightCurlyBrackets,
    Equal,
    DoubleEqual,
    NonEqual,
    LessThan,
    GreaterThan,
    LessThanEq,
    GreaterThanEq,
    DoubleAnd,
    DoubleOr,
    Bang,
    Ident(Ident),
    Type(Type),
    Keyword(Keyword),
    EOF,
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Minus => "-".to_string(),
                Self::Plus => "+".to_string(),
                Self::Star => "*".to_string(),
                Self::Slash => "/".to_string(),
                Self::LeftParenthesis => "(".to_string(),
                Self::RightParenthesis => ")".to_string(),
                Self::LeftCurlyBrackets => "{".to_string(),
                Self::RightCurlyBrackets => "}".to_string(),
                Self::Type(t) => format!("{}", t),
                Self::Equal => "=".to_string(),
                Self::DoubleEqual => "==".to_string(),
                Self::NonEqual => "!=".to_string(),
                Self::LessThan => "<".to_string(),
                Self::GreaterThan => ">".to_string(),
                Self::LessThanEq => "<=".to_string(),
                Self::GreaterThanEq => ">=".to_string(),
                Self::DoubleAnd => "&&".to_string(),
                Self::DoubleOr => "||".to_string(),
                Self::Bang => "!".to_string(),
                Self::Ident(i) => format!("{}", i),
                Self::Keyword(k) => format!("{}", k),
                Self::EOF => "EOF".to_string(),
            }
        )
    }
}
