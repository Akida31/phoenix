use crate::interpreter::Position;

#[derive(Debug)]
pub enum ErrorKind {
    SyntaxError,
    EndOfFile,
    Undefined,
    ZeroDivision,
}

pub struct Error {
    kind: ErrorKind,
    message: String,
    position: Option<Position>,
}

impl Error {
    pub fn new(kind: ErrorKind, message: String, position: Option<Position>) -> Self {
        Self {
            kind,
            message,
            position,
        }
    }

    pub fn with_position(self, position: Position) -> Self {
        Self {
            kind: self.kind,
            message: self.message,
            position: Some(position),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut position_hint = String::new();
        if let Some(position) = &self.position {
            let mut traceback = String::new();
            let mut context = Some(Box::new(position.clone()));
            while let Some(ctx) = context {
                traceback = format!(
                    "  File {}, line {}, column {}:\n{}",
                    ctx.filename,
                    ctx.line + 1,
                    ctx.column,
                    traceback
                );
                context = ctx.context;
            }
            position_hint = format!("ERROR - Traceback:\n{}", traceback);
        }
        write!(f, "{}{:?}: {}", position_hint, self.kind, self.message)
    }
}
