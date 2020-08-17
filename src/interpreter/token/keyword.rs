#[derive(Clone, PartialEq)]
pub enum Keyword {
    Let,
    If,
    Else,
    Then,
    Elif,
    For,
    While,
    In,
    To,
}

pub fn keywords() -> Vec<String> {
    [
        "let", "if", "else", "then", "elif", "for", "while", "in", "to",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect()
}

pub fn is_keyword(word: String) -> bool {
    keywords().contains(&word)
}

impl std::fmt::Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Let => "let",
                Self::If => "if",
                Self::Else => "else",
                Self::Then => "then",
                Self::Elif => "elif",
                Self::For => "for",
                Self::While => "while",
                Self::In => "in",
                Self::To => "to",
            }
        )
    }
}

pub fn from_str(v: &str) -> Option<Keyword> {
    match v {
        "let" => Some(Keyword::Let),
        "if" => Some(Keyword::If),
        "else" => Some(Keyword::Else),
        "then" => Some(Keyword::Then),
        "elif" => Some(Keyword::Elif),
        "for" => Some(Keyword::For),
        "while" => Some(Keyword::While),
        "in" => Some(Keyword::In),
        "to" => Some(Keyword::To),
        _ => None,
    }
}
