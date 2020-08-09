extern crate derive_more;

pub mod number;

use crate::interpreter::{Error, ErrorKind};
pub use number::{Float, Integer};

pub trait Operators
where
    Self: std::fmt::Display,
{
    fn add(&self, _other: Type) -> Result<Type, Error> {
        Err(Error::new(
            ErrorKind::Unimplemented,
            format!("method 'add' is not implemented for {}", self),
            None,
        ))
    }

    fn sub(&self, _other: Type) -> Result<Type, Error> {
        Err(Error::new(
            ErrorKind::Unimplemented,
            format!("method 'sub' is not implemented for {}", self),
            None,
        ))
    }

    fn mul(&self, _other: Type) -> Result<Type, Error> {
        Err(Error::new(
            ErrorKind::Unimplemented,
            format!("method 'mul' is not implemented for {}", self),
            None,
        ))
    }

    /// Important when implementing this method you should care for a ZeroDevisionError in your implementation
    fn div(&self, _other: Type) -> Result<Type, Error> {
        Err(Error::new(
            ErrorKind::Unimplemented,
            format!("method 'div' is not implemented for {}", self),
            None,
        ))
    }

    fn neg(&self) -> Result<Type, Error> {
        Err(Error::new(
            ErrorKind::Unimplemented,
            format!("method 'neg' is not implemented for {}", self),
            None,
        ))
    }
}

#[derive(Clone, PartialEq, derive_more::Display)]
pub enum Type {
    Integer(Integer),
    Float(Float),
}

impl std::fmt::Debug for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::convert::AsRef<dyn Operators> for Type {
    fn as_ref(&self) -> &(dyn Operators + 'static) {
        match self {
            Type::Integer(v) => v,
            Type::Float(v) => v,
        }
    }
}
