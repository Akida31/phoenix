mod ast;
mod errors;
pub mod lexer;
pub mod token;

use ast::nodes::{BinaryOperationNode, Node, NodeType, OperationType, UnaryOperationNode};
use token::types::Type;
use token::Sign;

pub use errors::*;
pub use token::Token;

#[derive(Clone, Debug)]
pub struct Position {
    index: i64,
    filename: String,
    line: u64,
    column: i64,
    len: u64,
    context: Option<Box<Position>>,
}

impl Position {
    pub fn new(
        index: i64,
        filename: String,
        line: u64,
        column: i64,
        len: u64,
        context: Option<Position>,
    ) -> Self {
        Self {
            index,
            filename,
            line,
            column,
            len,
            context: if let Some(ctx) = context {
                Some(Box::new(ctx))
            } else {
                None
            },
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

    pub fn len(&self) -> u64 {
        self.len
    }

    pub fn context(&self) -> Option<Box<Position>> {
        self.context.clone()
    }

    pub fn set_len(&mut self, len: u64) {
        self.len = len
    }

    pub fn combine(&self, other: Self) -> Self {
        Self::new(
            self.index,
            self.filename.clone(),
            self.line,
            self.column,
            (other.index - self.index) as u64 + other.len,
            if self.context.is_some() {
                Some(*self.context().unwrap())
            } else if other.context.is_some() {
                Some(*other.context().unwrap())
            } else {
                None
            },
        )
    }

    // TODO use this in modules
    pub fn add_context(&self, context: Box<Position>) -> Self {
        Self {
            index: self.index,
            filename: self.filename.clone(),
            line: self.line,
            column: self.column,
            len: self.len,
            context: Some(context),
        }
    }
}

pub fn run(text: String, file_name: String) -> Result<Type, Error> {
    let mut lexer = lexer::Lexer::new(text, file_name.clone());
    let tokens = lexer.make_tokens()?;
    let mut parser = ast::Parser::new(tokens);
    let ast = parser.parse()?;
    let context = Position::new(0, file_name, 0, 0, 0, None);
    let (ty, _pos) = visit(ast, context)?;
    Ok(ty)
}

pub fn visit(node: Node, context: Position) -> Result<(Type, Position), Error> {
    let position = node.get_pos();
    match node.get_type() {
        NodeType::Node(ty) => Ok((ty, position)),
        NodeType::Operation(op) => match op {
            OperationType::BinaryOperationNode(op) => {
                visit_binary_operation(*op, position, context)
            }
            OperationType::UnaryOperationNode(op) => visit_unary_operation(*op, position, context),
        },
    }
}

// TODO improve position marking
fn visit_binary_operation(
    node: BinaryOperationNode,
    _pos: Position,
    context: Position,
) -> Result<(Type, Position), Error> {
    let (left_ty, left_pos) = visit(node.get_left(), context.clone())?;
    let (right_ty, right_pos) = visit(node.get_right(), context)?;
    let pos = left_pos.combine(right_pos);
    let full = match node.get_operation() {
        Token::Plus => left_ty.as_ref().add(right_ty),
        Token::Minus => left_ty.as_ref().sub(right_ty),
        Token::Star => left_ty.as_ref().mul(right_ty),
        Token::Slash => left_ty.as_ref().div(right_ty),
        t => Err(Error::new(
            ErrorKind::Undefined,
            format!("can't operate on token {}", t),
            Some(pos.clone()),
        )),
    };
    // fill position of full
    match full {
        Ok(f) => Ok((f, pos)),
        Err(e) => Err(e.with_position(pos)),
    }
}

fn visit_unary_operation(
    node: UnaryOperationNode,
    _pos: Position,
    context: Position,
) -> Result<(Type, Position), Error> {
    let (ty, pos) = visit(node.get_node(), context.clone())?;
    let new_pos = Position::new(
        pos.index - 1,
        pos.filename,
        pos.line,
        pos.column,
        pos.len + 1,
        Some(context),
    );
    Ok((
        match node.get_operation() {
            Sign::Plus => ty,
            Sign::Minus => ty.as_ref().neg()?,
        },
        new_pos,
    ))
}
