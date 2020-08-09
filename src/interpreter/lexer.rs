use crate::interpreter::token::types::{Float, Integer, Type};
use crate::interpreter::token::{ident, keyword};
use crate::interpreter::{Error, ErrorKind, Position, Token};

pub struct Lexer {
    text: String,
    pos: Position,
    current_char: Option<char>,
}

impl Lexer {
    pub fn new(text: String, file_name: String) -> Self {
        let mut lexer = Self {
            text,
            pos: Position::new(-1, file_name, 0, -1, 1),
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

    pub fn make_tokens(&mut self) -> Result<Vec<(Token, Position)>, Error> {
        let mut tokens: Vec<(Token, Position)> = Vec::new();
        while let Some(c) = self.current_char {
            match c {
                ' ' | '\t' => self.advance(),
                '+' => {
                    tokens.push((Token::Plus, self.pos.clone()));
                    self.advance();
                }
                '-' => {
                    tokens.push((Token::Minus, self.pos.clone()));
                    self.advance();
                }
                '*' => {
                    tokens.push((Token::Star, self.pos.clone()));
                    self.advance();
                }
                '/' => {
                    tokens.push((Token::Slash, self.pos.clone()));
                    self.advance();
                }
                '(' => {
                    tokens.push((Token::LeftParenthesis, self.pos.clone()));
                    self.advance();
                }
                ')' => {
                    tokens.push((Token::RightParenthesis, self.pos.clone()));
                    self.advance();
                }
                '=' => {
                    tokens.push((Token::Equal, self.pos.clone()));
                    self.advance();
                }
                c if c.is_digit(10) => match self.make_number() {
                    Ok(number) => tokens.push(number),
                    Err(e) => return Err(e),
                },
                c if c.is_ascii_alphabetic() => match self.make_ident() {
                    Ok(i) => tokens.push(i),
                    Err(e) => return Err(e),
                },
                c => {
                    let position = self.pos.clone();
                    return Err(Error::new(
                        ErrorKind::SyntaxError,
                        format!("Illegal character: {}", c),
                        Some(position),
                    ));
                }
            }
        }
        tokens.push((Token::EOF, self.pos.clone()));
        Ok(tokens)
    }

    fn make_ident(&mut self) -> Result<(Token, Position), Error> {
        let mut ident = String::new();
        let pos_start = self.pos.clone();
        while let Some(c) = self.current_char {
            if !(c.is_ascii_alphanumeric() || c == '_') {
                break;
            }
            ident += &c.to_string();
            self.advance();
        }
        Ok((
            if keyword::is_keyword(ident.clone()) {
                Token::Keyword(keyword::Keyword::Let)
            } else {
                Token::Ident(ident::Ident::new(ident))
            },
            pos_start.combine(self.pos.clone()),
        ))
    }

    fn make_number(&mut self) -> Result<(Token, Position), Error> {
        let mut number = String::new();
        let mut dots: u8 = 0;
        let mut pos = self.pos.clone();
        pos.set_len(0);
        while let Some(c) = self.current_char {
            if c == '.' {
                if dots == 1 {
                    break;
                }
                dots += 1;
                number += ".";
            } else if c.is_digit(10) {
                pos.set_len(pos.len() + 1);
                number += &c.to_string();
            } else {
                break;
            }
            self.advance();
        }
        Ok((
            Token::Type(if dots == 0 {
                Type::Integer(Integer::new(number.parse().unwrap()))
            } else {
                Type::Float(Float::new(number.parse().unwrap()))
            }),
            pos,
        ))
    }
}
