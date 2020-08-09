use crate::interpreter::{Context, Position};

#[derive(Debug)]
pub enum ErrorKind {
    SyntaxError,
    EndOfFile,
    Undefined,
    ZeroDivision,
    NameError,
    Unimplemented,
    TypeError,
}

pub struct Error {
    kind: ErrorKind,
    message: String,
    position: Option<Position>,
    context: Option<Context>,
}

impl Error {
    pub fn new(kind: ErrorKind, message: String, position: Option<Position>) -> Self {
        Self {
            kind,
            message,
            position,
            context: None,
        }
    }

    // TODO use this
    /*pub fn with_position(self, position: Position) -> Self {
        Self {
            kind: self.kind,
            message: self.message,
            position: Some(position),
            context: None
        }
    }*/

    pub fn with_context(self, context: Context) -> Self {
        Self {
            kind: self.kind,
            message: self.message,
            position: None,
            context: Some(context),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut position_hint = String::new();
        let mut traceback = String::new();
        if let Some(pos) = &self.position {
            traceback = format!(
                "  File {}, line {}, column {}:\n",
                pos.filename,
                pos.line + 1,
                pos.column,
            );
        }
        if let Some(context) = &self.context {
            let mut context = Some(Box::new(context.clone()));
            while let Some(ctx) = context {
                traceback = format!(
                    "  File {}, line {}, column {}:\n{}",
                    ctx.pos.filename,
                    ctx.pos.line + 1,
                    ctx.pos.column,
                    traceback
                );
                context = ctx.context;
            }
        }
        if !traceback.is_empty() {
            position_hint = format!("ERROR - Traceback:\n{}", traceback);
        }
        write!(f, "{}{:?}: {}", position_hint, self.kind, self.message)
    }
}
