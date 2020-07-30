use std::fmt::{self, Formatter};

mod ast;
pub mod lexer;
pub mod token;

use ast::nodes::{BinaryOperationNode, Node, OperationType, UnaryOperationNode, NodeType};
use crate::interpreter::token::types::Type;
pub use token::Token;

#[derive(Debug)]
pub enum ErrorKind {
    SyntaxError,
    EndOfFile,
    Undefined,
}

pub struct Error {
    kind: ErrorKind,
    message: String,
    position: Option<Position>,
}

impl Error {
    fn new(kind: ErrorKind, message: String, position: Option<Position>) -> Self {
        Self {
            kind,
            message,
            position,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut position_hint = String::new();
        if let Some(position) = &self.position {
            let file = position.filename().clone();
            let line = position.line();
            let column = position.column();
            position_hint = format!("File {}, line {}, position {}:", file, line + 1, column + 1,)
        }
        write!(f, "{}\n {:?}: {}", position_hint, self.kind, self.message)
    }
}

#[derive(Clone, Debug)]
pub struct Position {
    index: i64,
    filename: String,
    line: u64,
    column: i64,
    len: u64,
}

impl Position {
    pub fn new(index: i64, filename: String, line: u64, column: i64, len: u64) -> Self {
        Self {
            index,
            filename,
            line,
            column,
            len,
        }
    }

    pub fn advance(&mut self, current_char: Option<char>) {
        self.index += 1;
        self.column += 1;
        if current_char.unwrap_or(' ') == '\n' {
            self.line += 1;
            self.column = 0;
        }
    }

    pub fn index(&self) -> i64 {
        self.index
    }

    pub fn filename(&self) -> String {
        self.filename.clone()
    }

    pub fn line(&self) -> u64 {
        self.line
    }

    pub fn column(&self) -> i64 {
        self.column
    }

    pub fn len(&self) -> u64 {
        self.len
    }

    pub fn set_len(&mut self, len: u64) {
        self.len = len
    }
}

pub fn run(text: String, file_name: String) -> Result<(), Error> {
    let mut lexer = lexer::Lexer::new(text, file_name);
    let tokens = lexer.make_tokens()?;
    let mut parser = ast::Parser::new(tokens);
    let ast = parser.parse()?;
    Ok(interpreter::visit(ast))
}

mod interpreter {
    use super::*;

    pub fn visit(node: Node) -> Type{
        match node.get_type() {
            NodeType::Node(ty) => visit_type(ty),
            NodeType::Operation(op) => match op {
                OperationType::BinaryOperationNode(op) => visit_binary_operation(*op),
                OperationType::UnaryOperationNode(op) => visit_unary_operation(*op),
            },
        }
    }

    fn no_visit(node: Node) -> Error {
        Error::new(
            ErrorKind::Undefined,
            format!("No visit method for node {} defined", node),
            None,
        )
    }

    fn visit_type(ty: Type) -> Type {
        println!("visited type, node: {}", node);
        ty
    }


    fn visit_binary_operation(node: BinaryOperationNode) -> Type {
        println!("visited binary operator, node: {}", node);
        let left = visit(node.get_left());
        let right = visit(node.get_right());
        left
    }
    fn visit_unary_operation(node: UnaryOperationNode) -> Type {
        println!("visited unary operator, node: {}", node);
        visit(node.get_node())
    }
}
