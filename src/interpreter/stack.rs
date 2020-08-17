use crate::interpreter::token::ident::Ident;
use crate::interpreter::Type;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Stack {
    symbols: HashMap<Ident, Type>,
    parent_stack: Option<Box<Stack>>,
}

impl Stack {
    pub fn new(parent_stack: Option<Stack>) -> Self {
        Self {
            symbols: HashMap::new(),
            parent_stack: match parent_stack {
                Some(s) => Some(Box::new(s)),
                None => None,
            },
        }
    }
    pub fn get(&self, name: &Ident) -> Option<Type> {
        match self.symbols.get(name) {
            Some(a) => Some(a.clone()),
            None if self.parent_stack.is_some() => self.parent_stack.as_ref().unwrap().get(name),
            None => None,
        }
    }
    pub fn set(&mut self, name: Ident, value: Type) {
        *self.symbols.entry(name).or_insert(value) = value.clone()
    }

    pub fn combine(&mut self, other: Self) {
        self.symbols.extend(other.symbols)
    }

    // TODO use this
    /*
    pub fn remove(&mut self, name: Ident) {
        self.symbols.remove(&name);
    }*/
}
