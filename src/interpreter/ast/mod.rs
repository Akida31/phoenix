use crate::interpreter::{Error, Position, Token};

mod exprs;
pub mod nodes;

use exprs::*;

use crate::interpreter::ast::nodes::{NodeType, OperationType, UnaryOperationNode};
use crate::interpreter::token::keyword::Keyword;
use crate::interpreter::ErrorKind::{EndOfFile, SyntaxError};
use nodes::{BinaryOperationNode, Node};

#[derive(Clone)]
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
        match expr(self) {
            Ok(res) => {
                if let Some(current_token) = self.current_token() {
                    if current_token.0 != Token::EOF {
                        return Err(Error::new(
                            SyntaxError,
                            "Expected one of the following: '+' '-' '*' or '/'",
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

    fn advance(&mut self) {
        self.index += 1;
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
}

fn atom(parser: &mut Parser) -> Result<Node, Error> {
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
            match atom(parser) {
                Ok(ty) => Ok(Node::new(
                    NodeType::Operation(OperationType::UnaryOperationNode(
                        // unwrap is safe because of the check above (Minus or Plus)
                        Box::new(UnaryOperationNode::from_token(token, ty).unwrap()),
                    )),
                    pos,
                )),
                Err(e) => Err(e),
            }
        }
        Some((Token::EOF, position)) => Err(Error::new(
            EndOfFile,
            "expected something but reached the end of file",
            Some(position),
        )),
        Some((Token::LeftParenthesis, position)) => {
            parser.advance();
            match expr(parser) {
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
                        Err(Error::new(SyntaxError, "expected )", Some(position)))
                    }
                }
                Err(e) => Err(e),
            }
        }
        Some((Token::LeftCurlyBrackets, position)) => {
            parser.advance();
            match expr(parser) {
                Ok(expr) => {
                    if parser
                        .current_token()
                        .unwrap_or((Token::EOF, position.clone()))
                        .0
                        == Token::RightCurlyBrackets
                    {
                        parser.advance();
                        Ok(expr)
                    } else {
                        Err(Error::new(SyntaxError, "expected }", Some(position)))
                    }
                }
                Err(e) => Err(e),
            }
        }
        Some((Token::Keyword(Keyword::If), position)) => if_expr(parser, position),
        Some((Token::Keyword(Keyword::For), position)) => for_expr(parser, position),
        Some((Token::Keyword(Keyword::While), position)) => while_expr(parser, position),
        Some((token, position)) => Err(Error::new(
            SyntaxError,
            &*format!("'{}' is not valid in this context", token),
            Some(position),
        )),
        None => Err(Error::new(SyntaxError, "can't parse empty token", None)),
    }
}

fn term(parser: &mut Parser) -> Result<Node, Error> {
    parser.binary_operation(&mut atom, vec![Token::Star, Token::Slash])
}
