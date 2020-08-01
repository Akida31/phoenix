use crate::interpreter::token::types::{Operators, Type};
use crate::interpreter::{Error, ErrorKind};
use std::fmt::{self, Display, Formatter};

#[derive(Clone, PartialEq)]
pub struct Integer {
    value: i64,
}

impl Integer {
    pub fn new(value: i64) -> Self {
        Integer { value }
    }
}

impl Display for Integer {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Operators for Integer {
    fn add(&self, other: Type) -> Result<Type, Error> {
        match other {
            Type::Integer(v) => Ok(Type::Integer(Self::new(self.value + v.value))),
            _ => Err(Error::new(
                ErrorKind::Undefined,
                "No valid type".to_string(),
                None,
            )),
        }
    }

    fn sub(&self, other: Type) -> Result<Type, Error> {
        match other {
            Type::Integer(v) => Ok(Type::Integer(Self::new(self.value - v.value))),
            _ => Err(Error::new(
                ErrorKind::Undefined,
                "No valid type".to_string(),
                None,
            )),
        }
    }

    fn mul(&self, other: Type) -> Result<Type, Error> {
        match other {
            Type::Integer(v) => Ok(Type::Integer(Self::new(self.value * v.value))),
            _ => Err(Error::new(
                ErrorKind::Undefined,
                "No valid type".to_string(),
                None,
            )),
        }
    }
    fn div(&self, other: Type) -> Result<Type, Error> {
        if other == Type::Integer(Integer::new(0)) {
            Err(Error::new(
                ErrorKind::ZeroDivision,
                "can't divide by 0".to_string(),
                None,
            ))
        } else {
            self.div(other)
        }
    }
    fn neg(&self) -> Result<Type, Error> {
        Ok(Type::Integer(Self::new(-self.value)))
    }
}

#[derive(Clone, PartialEq)]
pub struct Float {
    value: f64,
}

impl Float {
    pub fn new(value: f64) -> Self {
        Float { value }
    }
}

impl Display for Float {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Operators for Float {
    fn add(&self, other: Type) -> Result<Type, Error> {
        match other {
            Type::Float(v) => Ok(Type::Float(Self::new(self.value + v.value))),
            _ => Err(Error::new(
                ErrorKind::Undefined,
                "No valid type".to_string(),
                None,
            )),
        }
    }

    fn sub(&self, other: Type) -> Result<Type, Error> {
        match other {
            Type::Float(v) => Ok(Type::Float(Self::new(self.value - v.value))),
            _ => Err(Error::new(
                ErrorKind::Undefined,
                "No valid type".to_string(),
                None,
            )),
        }
    }

    fn mul(&self, other: Type) -> Result<Type, Error> {
        match other {
            Type::Float(v) => Ok(Type::Float(Self::new(self.value * v.value))),
            _ => Err(Error::new(
                ErrorKind::Undefined,
                "No valid type".to_string(),
                None,
            )),
        }
    }
    fn div(&self, other: Type) -> Result<Type, Error> {
        if other == Type::Float(Float::new(0.0)) {
            Err(Error::new(
                ErrorKind::ZeroDivision,
                "can't divide by 0".to_string(),
                None,
            ))
        } else {
            self.div(other)
        }
    }
    fn neg(&self) -> Result<Type, Error> {
        Ok(Type::Float(Self::new(-self.value)))
    }
}
