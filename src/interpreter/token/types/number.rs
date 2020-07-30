use std::fmt::{self, Display, Formatter};

trait Number
where
    Self: std::marker::Sized + Display + Clone,
{
    fn add(&self, other: Self) -> Self;
    fn subtract(&self, other: Self) -> Self;
    fn multiply(&self, other: Self) -> Self;
    fn divide(&self, other: Self) -> Self;
    fn save_divide(&self, other: Self) -> Option<Self>;
}

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

impl Number for Integer {
    fn add(&self, other: Self) -> Self {
        Self::new(self.value + other.value)
    }
    fn subtract(&self, other: Self) -> Self {
        Self::new(self.value - other.value)
    }
    fn multiply(&self, other: Self) -> Self {
        Self::new(self.value * other.value)
    }
    fn divide(&self, other: Self) -> Self {
        Self::new(self.value / other.value)
    }
    fn save_divide(&self, other: Self) -> Option<Self> {
        if other.value == 0 {
            None
        } else {
            Some(self.divide(other))
        }
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

impl Number for Float {
    fn add(&self, other: Self) -> Self {
        Self::new(self.value + other.value)
    }
    fn subtract(&self, other: Self) -> Self {
        Self::new(self.value - other.value)
    }
    fn multiply(&self, other: Self) -> Self {
        Self::new(self.value * other.value)
    }
    fn divide(&self, other: Self) -> Self {
        Self::new(self.value / other.value)
    }
    fn save_divide(&self, other: Self) -> Option<Self> {
        if other.value == 0.0 {
            None
        } else {
            Some(self.divide(other))
        }
    }
}
