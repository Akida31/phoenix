use crate::interpreter::{Error, ErrorKind, Position, Token};
use crate::interpreter::token::types::{Integer, Float, Type};

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
                c if c.is_digit(10) => match self.make_number() {
                    Ok(number) => tokens.push(number),
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
