#[derive(Clone, PartialEq)]
pub enum Keyword {
    Let,
}

pub fn keywords() -> Vec<String> {
    ["let"].iter().map(|s| s.to_string()).collect()
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
            }
        )
    }
}
