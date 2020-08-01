use crate::interpreter::{Error, Position, Token};

pub mod nodes;

use crate::interpreter::ast::nodes::{NodeType, OperationType, UnaryOperationNode};
use crate::interpreter::token::Sign;
use crate::interpreter::ErrorKind::{EndOfFile, SyntaxError};

use nodes::{BinaryOperationNode, Node};

pub struct Parser {
    tokens: Vec<(Token, Position)>,
    index: isize,
    current_token: Option<(Token, Position)>,
}

impl Parser {
    pub fn new(tokens: Vec<(Token, Position)>) -> Self {
        let mut parser = Self {
            tokens,
            index: -1,
            current_token: None,
        };
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
        self.current_token.clone()
    }

    fn advance(&mut self) -> Option<(Token, Position)> {
        // TODO improve this, maybe remove self.current_token
        self.index += 1;
        if self.index < self.tokens.len() as isize {
            self.current_token = Some(self.tokens[self.index as usize].clone());
        }
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
        self.binary_operation(&mut term, vec![Token::Plus, Token::Minus])
    }
}

// TODO move this into method
fn number(parser: &mut Parser) -> Result<Node, Error> {
    match parser.current_token() {
        Some((Token::Type(ty), pos)) => {
            parser.advance();
            Ok(Node::new(NodeType::Node(ty), pos))
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

// TODO move this into method
fn term(parser: &mut Parser) -> Result<Node, Error> {
    parser.binary_operation(&mut number, vec![Token::Star, Token::Slash])
}
