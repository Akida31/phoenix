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
            ErrorKind::Undefined,
            "method 'add' is not implemented".to_string(),
            None,
        ))
    }

    fn sub(&self, _other: Type) -> Result<Type, Error> {
        Err(Error::new(
            ErrorKind::Undefined,
            "method 'sub' is not implemented".to_string(),
            None,
        ))
    }

    fn mul(&self, _other: Type) -> Result<Type, Error> {
        Err(Error::new(
            ErrorKind::Undefined,
            "method 'mul' is not implemented".to_string(),
            None,
        ))
    }

    /// Important when implementing this method you should care for a ZeroDevisionError in your implementation
    fn div(&self, _other: Type) -> Result<Type, Error> {
        Err(Error::new(
            ErrorKind::Undefined,
            "method 'div' is not implemented".to_string(),
            None,
        ))
    }

    fn neg(&self) -> Result<Type, Error> {
        Err(Error::new(
            ErrorKind::Undefined,
            "method 'neg' is not implemented".to_string(),
            None,
        ))
    }
}

#[derive(Clone, PartialEq, derive_more::Display)]
pub enum Type {
    Integer(Integer),
    Float(Float),
}

impl std::convert::AsRef<dyn Operators> for Type {
    fn as_ref(&self) -> &(dyn Operators + 'static) {
        match self {
            Type::Integer(v) => v,
            Type::Float(v) => v,
        }
    }
}
