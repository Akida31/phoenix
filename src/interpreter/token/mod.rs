use std::fmt::{self, Display, Formatter};

pub mod ident;
pub mod keyword;
pub mod types;
use ident::Ident;
use keyword::Keyword;
use types::Type;

#[derive(Clone, PartialEq)]
pub enum Sign {
    Plus,
    Minus,
}

impl Display for Sign {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Plus => "+",
                Self::Minus => "-",
            }
        )
    }
}

impl Sign {
    pub fn from_token(t: Token) -> Option<Self> {
        match t {
            Token::Plus => Some(Sign::Plus),
            Token::Minus => Some(Sign::Minus),
            _ => None,
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum Token {
    Plus,
    Minus,
    Star,
    Slash,
    LeftParenthesis,
    RightParenthesis,
    Equal,
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
                Self::Type(t) => format!("{}", t),
                Self::Equal => "=".to_string(),
                Self::Ident(i) => format!("{}", i),
                Self::Keyword(k) => format!("{}", k),
                Self::EOF => "EOF".to_string(),
            }
        )
    }
}
