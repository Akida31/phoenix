use crate::interpreter::{Error, ErrorKind, Position, Token};

pub mod nodes;

use crate::interpreter::ast::nodes::{Assignment, NodeType, OperationType, UnaryOperationNode};
use crate::interpreter::token::keyword::Keyword;
use crate::interpreter::token::Sign;
use crate::interpreter::ErrorKind::{EndOfFile, SyntaxError};
use nodes::{BinaryOperationNode, Node};

pub struct Parser {
    tokens: Vec<(Token, Position)>,
    index: isize,
}

impl Parser {
    pub fn new(tokens: Vec<(Token, Position)>) -> Self {
        let mut parser = Self { tokens, index: -1 };
        parser.advance();
        parser
    }

    pub fn parse(&mut self) -> Result<Node, Error> {
        match self.expr() {
            Ok(res) => {
                if let Some(current_token) = self.current_token() {
                    if current_token.0 != Token::EOF {
                        return Err(Error::new(
                            SyntaxError,
                            "Expected one of the following: '+' '-' '*' or '/'".to_string(),
                            Some(current_token.1),
                        ));
                    }
                }
                Ok(res)
            }
            Err(e) => Err(e),
        }
    }

    fn current_token(&self) -> Option<(Token, Position)> {
        if self.index >= 0 && self.index < self.tokens.len() as isize {
            Some(self.tokens[self.index as usize].clone())
        } else {
            None
        }
    }

    fn advance(&mut self) -> Option<(Token, Position)> {
        self.index += 1;
        self.current_token()
    }

    fn binary_operation(
        &mut self,
        func: &mut dyn FnMut(&mut Self) -> Result<Node, Error>,
        operations: Vec<Token>,
    ) -> Result<Node, Error> {
        let mut left = match func(self) {
            Ok(res) => res,
            Err(e) => return Err(e),
        };
        while let Some(current_token) = self.current_token() {
            if !operations.contains(&current_token.0) {
                break;
            }
            let operation = current_token;
            self.advance();
            let right = match func(self) {
                Ok(res) => res,
                Err(e) => return Err(e),
            };
            left = Node::new(
                NodeType::Operation(OperationType::BinaryOperationNode(Box::new(
                    BinaryOperationNode::new(left.clone(), operation.clone().0, right.clone()),
                ))),
                left.get_pos().combine(right.get_pos()),
            );
        }
        Ok(left)
    }

    fn expr(&mut self) -> Result<Node, Error> {
        let current_token = self.current_token();
        if current_token.is_some() && current_token.unwrap().0 == Token::Keyword(Keyword::Let) {
            self.advance();
            if let Some(c) = self.current_token() {
                let pos = c.1;
                if let Token::Ident(i) = c.0 {
                    let name = i;
                    self.advance();
                    if let Some(c) = self.current_token() {
                        if Token::Equal == c.0 {
                            self.advance();
                            let expr = self.expr()?;
                            return Ok(Node::new(
                                NodeType::Assign(Assignment::new(name, expr)),
                                pos,
                            ));
                        }
                    }
                    Err(Error::new(
                        ErrorKind::SyntaxError,
                        "expected =".to_string(),
                        Some(pos),
                    ))
                } else {
                    Err(Error::new(
                        ErrorKind::SyntaxError,
                        "expected identifier".to_string(),
                        Some(pos),
                    ))
                }
            } else {
                Err(Error::new(
                    ErrorKind::SyntaxError,
                    "expected expression".to_string(),
                    None,
                ))
            }
        } else {
            self.binary_operation(&mut term, vec![Token::Plus, Token::Minus])
        }
    }
}

fn number(parser: &mut Parser) -> Result<Node, Error> {
    match parser.current_token() {
        Some((Token::Type(ty), pos)) => {
            parser.advance();
            Ok(Node::new(NodeType::Node(ty), pos))
        }
        Some((Token::Ident(ident), pos)) => {
            parser.advance();
            Ok(Node::new(NodeType::Var(ident), pos))
        }
        Some((token, pos)) if token == Token::Minus || token == Token::Plus => {
            parser.advance();
            match number(parser) {
                Ok(ty) => Ok(Node::new(
                    NodeType::Operation(OperationType::UnaryOperationNode(
                        // unwrap is safe because of the check above
                        Box::new(UnaryOperationNode::new(
                            Sign::from_token(token).unwrap(),
                            ty,
                        )),
                    )),
                    pos,
                )),
                Err(e) => Err(e),
            }
        }
        Some((Token::EOF, position)) => Err(Error::new(
            EndOfFile,
            "expected something but reached the end of file".to_string(),
            Some(position),
        )),
        Some((Token::LeftParenthesis, position)) => {
            parser.advance();
            match parser.expr() {
                Ok(expr) => {
                    if parser
                        .current_token()
                        .unwrap_or((Token::EOF, position.clone()))
                        .0
                        == Token::RightParenthesis
                    {
                        parser.advance();
                        Ok(expr)
                    } else {
                        Err(Error::new(
                            SyntaxError,
                            "expected )".to_string(),
                            Some(position),
                        ))
                    }
                }
                Err(e) => Err(e),
            }
        }
        Some((token, position)) => Err(Error::new(
            SyntaxError,
            format!("{} is not valid in this context", token),
            Some(position),
        )),
        None => Err(Error::new(
            SyntaxError,
            "can't parse empty token".to_string(),
            None,
        )),
    }
}

fn term(parser: &mut Parser) -> Result<Node, Error> {
    parser.binary_operation(&mut number, vec![Token::Star, Token::Slash])
}
