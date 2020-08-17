mod ast;
mod errors;
mod lexer;
mod stack;
mod token;

use ast::nodes::{BinaryOperationNode, Node, NodeType, OperationType, UnaryOperationNode};
use stack::Stack;
pub use token::types::Type;

use crate::interpreter::ast::nodes::{Assignment, IfNode, UnaryOperation};
use crate::interpreter::token::ident::Ident;
use crate::interpreter::token::types::Integer;
pub use errors::*;
use token::Token;

#[derive(Clone, Debug)]
pub struct Context {
    pos: Position,
    stack: Stack,
    context: Option<Box<Context>>,
}

impl Context {
    pub fn new(pos: Position, stack: Stack, context: Option<Context>) -> Self {
        Self {
            pos,
            context: if let Some(ctx) = context {
                Some(Box::new(ctx))
            } else {
                None
            },
            stack,
        }
    }

    pub fn context(&self) -> Option<Box<Context>> {
        self.context.clone()
    }

    pub fn get_position(&self) -> Position {
        self.pos.clone()
    }

    pub fn combine(&self, other: Self) -> Self {
        let mut stack = self.stack.clone();
        stack.combine(other.stack.clone());
        Self::new(
            Position::new(
                self.pos.index,
                self.pos.filename.clone(),
                self.pos.line,
                self.pos.column,
                (other.pos.index - self.pos.index) as u64 + other.pos.len,
            ),
            stack,
            if self.context.is_some() {
                Some(*self.context().unwrap())
            } else if other.context.is_some() {
                Some(*other.context().unwrap())
            } else {
                None
            },
        )
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

    pub fn len(&self) -> u64 {
        self.len
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
        )
    }
    // TODO use this in modules
    /*pub fn add_context(&self, context: Box<Position>) -> Self {
        Self {
            index: self.index,
            filename: self.filename.clone(),
            line: self.line,
            column: self.column,
            len: self.len,
            context: Some(context),

        }
    }*/
}

pub struct InterpretionResult {
    pub res: Result<Type, Error>,
    pub stack: stack::Stack,
}

impl InterpretionResult {
    fn new(res: Result<Type, Error>, stack: stack::Stack) -> Self {
        Self { res, stack }
    }
}

pub fn new_stack() -> Stack {
    stack::Stack::new(None)
}

pub fn run(text: String, file_name: String, stack: Option<Stack>) -> InterpretionResult {
    let mut stack = stack.unwrap_or_else(new_stack);

    // Built-in variables
    stack.set(
        Ident::new("null".to_string()),
        Type::Integer(Integer::new(0)),
    );
    stack.set(
        Ident::new("true".to_string()),
        Type::Integer(Integer::new(1)),
    );
    stack.set(
        Ident::new("false".to_string()),
        Type::Integer(Integer::new(0)),
    );

    let mut lexer = lexer::Lexer::new(text, file_name.clone());
    let tokens = match lexer.make_tokens() {
        Ok(t) => t,
        Err(e) => return InterpretionResult::new(Err(e), stack),
    };
    let mut parser = ast::Parser::new(tokens);
    let ast = match parser.parse() {
        Ok(n) => n,
        Err(e) => return InterpretionResult::new(Err(e), stack),
    };
    let pos = Position::new(0, file_name, 0, 0, 0);
    let context = Context::new(pos, stack.clone(), None);
    let (ty, context) = match visit(ast, context) {
        Ok(t) => t,
        Err(e) => return InterpretionResult::new(Err(e), stack),
    };
    InterpretionResult::new(Ok(ty), context.stack)
}

pub fn visit(node: Node, context: Context) -> Result<(Type, Context), Error> {
    let position = node.get_pos();
    match node.get_type() {
        NodeType::Node(ty) => Ok((ty, context)),
        NodeType::Operation(op) => match op {
            OperationType::BinaryOperationNode(op) => visit_binary_operation(*op, context),
            OperationType::UnaryOperationNode(op) => visit_unary_operation(*op, position, context),
        },
        NodeType::Var(id) => visit_var(id, context),
        NodeType::Assign(a) => visit_assignment(a, context),
        NodeType::IfNode(node) => visit_if_node(node, context),
    }
}

fn visit_assignment(node: Assignment, context: Context) -> Result<(Type, Context), Error> {
    let value = visit(*node.get_expr(), context)?;
    let mut context = value.1;
    context.stack.set(node.get_name(), value.0.clone());
    Ok((value.0, context))
}

fn visit_var(node: Ident, context: Context) -> Result<(Type, Context), Error> {
    let value = context.stack.get(node.clone());
    match value {
        Some(val) => Ok((val, context)),
        None => Err(Error::new(
            ErrorKind::NameError,
            &*format!("{} is not defined", node.get()),
            Some(context.get_position()),
        )),
    }
}

fn visit_if_node(node: IfNode, context: Context) -> Result<(Type, Context), Error> {
    for (condition, expr) in node.get_cases().into_iter() {
        let condition_value = visit(condition, context.clone())?;
        if condition_value.0.as_conversion().__bool__()? {
            return visit(expr, context);
        }
    }
    if let Some(else_case) = *node.get_else_case() {
        visit(else_case, context)
    } else {
        Ok((Type::none(), context))
    }
}

// TODO improve position marking
fn visit_binary_operation(
    node: BinaryOperationNode,
    context: Context,
) -> Result<(Type, Context), Error> {
    let (left_ty, left_ctx) = visit(node.get_left(), context.clone())?;
    let (right_ty, right_ctx) = visit(node.get_right(), context)?;
    let ctx = left_ctx.combine(right_ctx);
    let full = match node.get_operation() {
        Token::Plus => left_ty.as_operators().add(right_ty),
        Token::Minus => left_ty.as_operators().sub(right_ty),
        Token::Star => left_ty.as_operators().mul(right_ty),
        Token::Slash => left_ty.as_operators().div(right_ty),
        Token::DoubleEqual => left_ty.as_operators().eq(right_ty),
        Token::NonEqual => left_ty.as_operators().neq(right_ty),
        Token::LessThan => left_ty.as_operators().lt(right_ty),
        Token::GreaterThan => left_ty.as_operators().gt(right_ty),
        Token::LessThanEq => left_ty.as_operators().lte(right_ty),
        Token::GreaterThanEq => left_ty.as_operators().gte(right_ty),
        Token::DoubleAnd => left_ty.as_operators().and(right_ty),
        Token::DoubleOr => left_ty.as_operators().or(right_ty),
        t => Err(Error::new(
            ErrorKind::Undefined,
            &*format!("can't operate on token {}", t),
            Some(ctx.get_position()),
        )),
    };
    // fill position of full
    match full {
        Ok(f) => Ok((f, ctx)),
        Err(e) => Err(e.with_context(ctx)),
    }
}

fn visit_unary_operation(
    node: UnaryOperationNode,
    _pos: Position,
    context: Context,
) -> Result<(Type, Context), Error> {
    let (ty, ctx) = visit(node.get_node(), context.clone())?;
    let new_pos = Position::new(
        ctx.pos.index - 1,
        ctx.pos.filename,
        ctx.pos.line,
        ctx.pos.column,
        ctx.pos.len + 1,
    );
    let new_ctx = Context::new(new_pos, context.stack.clone(), Some(context));
    Ok((
        match node.get_operation() {
            UnaryOperation::Plus => ty,
            UnaryOperation::Minus => ty.as_operators().neg()?,
            UnaryOperation::Not => ty.as_operators().not()?,
        },
        new_ctx,
    ))
}
