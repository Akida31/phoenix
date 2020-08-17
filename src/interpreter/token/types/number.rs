use crate::interpreter::token::types::{Cmp, CmpResult, Conversion, Operators, Type};
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

impl Cmp for Integer {
    fn cmp(&self, other: Type) -> Result<CmpResult, Error> {
        match other {
            Type::Integer(v) => Ok(self.value.cmp(&v.value).into()),
            _ => Err(Error::new(ErrorKind::TypeError, "No valid type", None)),
        }
    }
}

impl Operators for Integer {
    fn add(&self, other: Type) -> Result<Type, Error> {
        match other {
            Type::Integer(v) => Ok(Type::Integer(Self::new(self.value + v.value))),
            _ => Err(Error::new(ErrorKind::TypeError, "No valid type", None)),
        }
    }

    fn sub(&self, other: Type) -> Result<Type, Error> {
        match other {
            Type::Integer(v) => Ok(Type::Integer(Self::new(self.value - v.value))),
            _ => Err(Error::new(ErrorKind::TypeError, "No valid type", None)),
        }
    }

    fn mul(&self, other: Type) -> Result<Type, Error> {
        match other {
            Type::Integer(v) => Ok(Type::Integer(Self::new(self.value * v.value))),
            _ => Err(Error::new(ErrorKind::TypeError, "No valid type", None)),
        }
    }
    fn div(&self, other: Type) -> Result<Type, Error> {
        if other == Type::Integer(Integer::new(0)) {
            Err(Error::new(
                ErrorKind::ZeroDivision,
                "can't divide by 0",
                None,
            ))
        } else {
            match other {
                Type::Integer(v) => Ok(Type::Integer(Self::new(self.value / v.value))),
                _ => Err(Error::new(ErrorKind::TypeError, "No valid type", None)),
            }
        }
    }

    fn neg(&self) -> Result<Type, Error> {
        Ok(Type::Integer(Self::new(-self.value)))
    }

    fn and(&self, other: Type) -> Result<Type, Error> {
        match other {
            Type::Integer(v) => Ok(Type::Integer(Self::new(
                if self.value != 0 && v.value != 0 {
                    1
                } else {
                    0
                },
            ))),
            _ => Err(Error::new(ErrorKind::TypeError, "No valid type", None)),
        }
    }

    fn or(&self, other: Type) -> Result<Type, Error> {
        match other {
            Type::Integer(v) => Ok(Type::Integer(Self::new(
                if self.value != 0 || v.value != 0 {
                    1
                } else {
                    0
                },
            ))),
            _ => Err(Error::new(ErrorKind::TypeError, "No valid type", None)),
        }
    }

    fn not(&self) -> Result<Type, Error> {
        Ok(Type::Integer(Integer::new(if self.value == 0 {
            1
        } else {
            0
        })))
    }
}

impl Conversion for Integer {
    fn __bool__(&self) -> Result<bool, Error> {
        Ok(self.value != 0)
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

impl Cmp for Float {
    fn cmp(&self, other: Type) -> Result<CmpResult, Error> {
        match other {
            Type::Float(v) => match self.value.partial_cmp(&v.value) {
                Some(v) => Ok(v.into()),
                None => Err(Error::new(
                    ErrorKind::Undefined,
                    &format!(
                        "Invalid float comparison between {} and {}",
                        self.value, v.value
                    ),
                    None,
                )),
            },
            _ => Err(Error::new(ErrorKind::TypeError, "No valid type", None)),
        }
    }
}

impl Operators for Float {
    fn add(&self, other: Type) -> Result<Type, Error> {
        match other {
            Type::Float(v) => Ok(Type::Float(Self::new(self.value + v.value))),
            _ => Err(Error::new(ErrorKind::TypeError, "No valid type", None)),
        }
    }

    fn sub(&self, other: Type) -> Result<Type, Error> {
        match other {
            Type::Float(v) => Ok(Type::Float(Self::new(self.value - v.value))),
            _ => Err(Error::new(ErrorKind::TypeError, "No valid type", None)),
        }
    }

    fn mul(&self, other: Type) -> Result<Type, Error> {
        match other {
            Type::Float(v) => Ok(Type::Float(Self::new(self.value * v.value))),
            _ => Err(Error::new(ErrorKind::TypeError, "No valid type", None)),
        }
    }

    fn div(&self, other: Type) -> Result<Type, Error> {
        if other == Type::Float(Float::new(0.0)) {
            Err(Error::new(
                ErrorKind::ZeroDivision,
                "can't divide by 0",
                None,
            ))
        } else {
            match other {
                Type::Float(v) => Ok(Type::Float(Self::new(self.value / v.value))),
                _ => Err(Error::new(ErrorKind::TypeError, "No valid type", None)),
            }
        }
    }

    fn neg(&self) -> Result<Type, Error> {
        Ok(Type::Float(Self::new(-self.value)))
    }
}

impl Conversion for Float {
    fn __bool__(&self) -> Result<bool, Error> {
        Ok(self.value != 0.0)
    }
}
