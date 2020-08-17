mod ast;
mod errors;
mod lexer;
mod stack;
mod token;
mod visit;

use stack::Stack;
pub use token::types::Type;

use crate::interpreter::token::ident::Ident;
use crate::interpreter::token::types::Integer;
pub use errors::*;
use token::Token;
use visit::*;

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
    // create a stack if none is supplied
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

    // lexing
    let mut lexer = lexer::Lexer::new(text, file_name.clone());
    let tokens = match lexer.make_tokens() {
        Ok(t) => t,
        Err(e) => return InterpretionResult::new(Err(e), stack),
    };

    // parsing to ast
    let mut parser = ast::Parser::new(tokens);
    let ast = match parser.parse() {
        Ok(n) => n,
        Err(e) => return InterpretionResult::new(Err(e), stack),
    };

    // execute the ast
    let pos = Position::new(0, file_name, 0, 0, 0);
    let context = Context::new(pos, stack.clone(), None);
    let (ty, context) = match visit(ast, context) {
        Ok(t) => t,
        Err(e) => return InterpretionResult::new(Err(e), stack),
    };
    InterpretionResult::new(Ok(ty), context.stack)
}
