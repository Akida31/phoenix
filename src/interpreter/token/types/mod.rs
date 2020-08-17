extern crate derive_more;

pub mod number;

use crate::interpreter::{Error, ErrorKind};
pub use number::{Float, Integer};
use std::cmp::Ordering;

pub trait Cmp {
    fn cmp(&self, other: Type) -> Result<CmpResult, Error>;
}

#[derive(PartialEq, Eq)]
pub enum CmpResult {
    Equal,
    Less,
    Greater,
}

impl From<Ordering> for CmpResult {
    fn from(o: Ordering) -> Self {
        match o {
            Ordering::Less => Self::Less,
            Ordering::Greater => Self::Greater,
            Ordering::Equal => Self::Equal,
        }
    }
}

pub trait Conversion
where
    Self: std::fmt::Display,
{
    fn __int__(&self) -> Result<Integer, Error> {
        Err(Error::new(
            ErrorKind::Unimplemented,
            &format!("can't convert {} to {}", self, "int"),
            None,
        ))
    }

    fn __float__(&self) -> Result<Float, Error> {
        Err(Error::new(
            ErrorKind::Unimplemented,
            &format!("can't convert {} to {}", self, "int"),
            None,
        ))
    }

    fn __bool__(&self) -> Result<bool, Error> {
        Err(Error::new(
            ErrorKind::Unimplemented,
            &format!("can't convert {} to {}", self, "int"),
            None,
        ))
    }
}

macro_rules! new_op {
    ($id: ident, $name: literal + other) => {
        fn $id(&self, _other: Type) -> Result<Type, Error> {
            Err(Error::new(
                ErrorKind::Unimplemented,
                &format!("method '{}' is not implemented for {}", $name, self),
                None,
            ))
        }
    };
    ($id: ident, $name: literal) => {
        fn $id(&self) -> Result<Type, Error> {
            Err(Error::new(
                ErrorKind::Unimplemented,
                &format!("method '{}' is not implemented for {}", $name, self),
                None,
            ))
        }
    };
}

pub trait Operators
where
    Self: std::fmt::Display + Cmp,
{
    new_op! {add, "add" + other}
    new_op! {sub, "sub" + other}
    new_op! {mul, "mul" + other}
    // Important when implementing this method you should care for a ZeroDevisionError in your implementation
    new_op! {div, "div" + other}
    new_op! {neg, "neg"}
    new_op! {not, "not"}
    new_op! {and, "and" + other}
    new_op! {or, "or" + other}

    fn eq(&self, other: Type) -> Result<Type, Error> {
        Ok((self.cmp(other)? == CmpResult::Equal).into())
    }

    fn neq(&self, other: Type) -> Result<Type, Error> {
        Ok((self.cmp(other)? != CmpResult::Equal).into())
    }

    fn gt(&self, other: Type) -> Result<Type, Error> {
        Ok((self.cmp(other)? == CmpResult::Greater).into())
    }

    fn lt(&self, other: Type) -> Result<Type, Error> {
        Ok((self.cmp(other)? == CmpResult::Less).into())
    }

    fn gte(&self, other: Type) -> Result<Type, Error> {
        let c = self.cmp(other)?;
        Ok((c == CmpResult::Equal || c == CmpResult::Greater).into())
    }

    fn lte(&self, other: Type) -> Result<Type, Error> {
        let c = self.cmp(other)?;
        Ok((c == CmpResult::Equal || c == CmpResult::Less).into())
    }
}

#[derive(Clone, PartialEq, derive_more::Display)]
pub enum Type {
    Integer(Integer),
    Float(Float),
    None(NoneType),
}

impl std::fmt::Debug for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Type {
    pub fn as_operators(&self) -> &(dyn Operators + 'static) {
        match self {
            Type::Integer(v) => v,
            Type::Float(v) => v,
            Type::None(v) => v,
        }
    }
    pub fn as_conversion(&self) -> &(dyn Conversion + 'static) {
        match self {
            Type::Integer(v) => v,
            Type::Float(v) => v,
            Type::None(v) => v,
        }
    }

    pub fn none() -> Self {
        Self::None(NoneType::new())
    }
}

impl std::convert::From<bool> for Type {
    fn from(b: bool) -> Self {
        if b {
            Type::Integer(Integer::new(1))
        } else {
            Type::Integer(Integer::new(0))
        }
    }
}

#[derive(Copy, Clone, PartialOrd, PartialEq)]
pub struct NoneType {}

impl NoneType {
    fn new() -> Self {
        Self {}
    }
}

impl std::fmt::Display for NoneType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "None")
    }
}

impl Cmp for NoneType {
    fn cmp(&self, _other: Type) -> Result<CmpResult, Error> {
        Err(Error::new(
            ErrorKind::Unimplemented,
            "can't compare None",
            None,
        ))
    }
}

impl Operators for NoneType {}

impl Conversion for NoneType {}
