use std::fmt::{self, Display, Formatter};

pub mod number;

pub use number::{Integer, Float};

trait TypeTrait
where Self: std::marker::Sized {
    fn add(&self, other: Type) -> Self {
        unimplemented!()
    }
    fn mul(&self, other: Type) -> Self {
        unimplemented!()
    }
    fn sub(&self, other: Type) -> Self {
        unimplemented!()
    }
    fn div(&self, other: Type) -> Self {
        unimplemented!()
    }
}

#[derive(Clone, PartialEq)]
pub enum Type {
    Integer(Integer),
    Float(Float)
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Integer(i) => format!("{}", i),
            Self::Float(i) => format!("{}", i)
        })
    }
}
