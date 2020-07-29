use std::fmt::{self, Formatter};

#[derive(Debug)]
pub enum ErrorKind {
    SyntaxError,
}

pub struct Error {
    kind: ErrorKind,
    message: String,
    position: Position,
}

impl Error {
    fn new(kind: ErrorKind, message: String, position: Position) -> Self {
        Self {
            kind,
            message,
            position,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let file = self.position.file_name.clone();
        let line = self.position.line;
        let column = self.position.column;
        write!(
            f,
            "File {}, line {}, position {}:\n {:?}: {}",
            file,
            line + 1,
            column + 1,
            self.kind,
            self.message
        )
    }
}

#[derive(Debug)]
pub enum Type {
    Integer(i64),
    Float(f64),
}

pub enum Token {
    Plus,
    Minus,
    Star,
    Slash,
    LeftParenthesis,
    RightParenthesis,
    Type(Type),
}

impl fmt::Display for Token {
    fn fmt<'a>(&self, f: &mut Formatter<'a>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Token::Minus => "-".to_string(),
                Token::Plus => "+".to_string(),
                Token::Star => "*".to_string(),
                Token::Slash => "/".to_string(),
                Token::LeftParenthesis => "(".to_string(),
                Token::RightParenthesis => ")".to_string(),
                Token::Type(t) => format!("{:?}", t),
            }
        )
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <Self as fmt::Display>::fmt(self, f)
    }
}

#[derive(Clone, Debug)]
struct Position {
    index: i64,
    line: u64,
    column: i64,
    file_name: String,
}

impl Position {
    fn advance(&mut self, current_char: Option<char>) {
        self.index += 1;
        self.column += 1;
        if current_char.unwrap_or(' ') == '\n' {
            self.line += 1;
            self.column = 0;
        }
    }
}

struct Lexer {
    text: String,
    pos: Position,
    current_char: Option<char>,
}

impl Lexer {
    fn new(text: String, file_name: String) -> Self {
        let mut lexer = Self {
            text,
            pos: Position {
                index: -1,
                line: 0,
                column: -1,
                file_name,
            },
            current_char: None,
        };
        lexer.advance();
        lexer
    }

    fn advance(&mut self) {
        self.pos.advance(self.current_char);
        if self.pos.index >= 0 && self.pos.index < self.text.len() as i64 {
            self.current_char =
                Some(self.text.chars().collect::<Vec<_>>()[self.pos.index as usize]);
        } else {
            self.current_char = None;
        }
    }

    fn make_tokens(&mut self) -> Result<Vec<Token>, Error> {
        let mut tokens: Vec<Token> = Vec::new();
        while let Some(c) = self.current_char {
            match c {
                ' ' | '\t' => self.advance(),
                '+' => {
                    tokens.push(Token::Plus);
                    self.advance();
                }
                '-' => {
                    tokens.push(Token::Minus);
                    self.advance();
                }
                '*' => {
                    tokens.push(Token::Star);
                    self.advance();
                }
                '/' => {
                    tokens.push(Token::Slash);
                    self.advance();
                }
                '(' => {
                    tokens.push(Token::LeftParenthesis);
                    self.advance();
                }
                ')' => {
                    tokens.push(Token::RightParenthesis);
                    self.advance();
                }
                c if c.is_digit(10) => {
                    tokens.push(Token::Type(self.make_number()));
                }
                c => {
                    let position = self.pos.clone();
                    return Err(Error::new(
                        ErrorKind::SyntaxError,
                        format!("Illegal character: {}", c),
                        position,
                    ));
                }
            }
        }
        Ok(tokens)
    }

    fn make_number(&mut self) -> Type {
        let mut number = String::new();
        let mut dots: u8 = 0;
        while let Some(c) = self.current_char {
            if !(c.is_digit(10) || c == '.') {
                break;
            }
            if c == '.' {
                if dots == 1 {
                    break;
                }
                dots += 1;
                number += ".";
            } else {
                number += &c.to_string();
            }
            self.advance();
        }
        if dots == 0 {
            Type::Integer(number.parse().unwrap())
        } else {
            Type::Float(number.parse().unwrap())
        }
    }
}

pub fn run(text: String, file_name: String) -> Result<Vec<Token>, Error> {
    let mut lexer = Lexer::new(text, file_name);
    lexer.make_tokens()
}
