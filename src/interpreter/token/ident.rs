#[derive(Clone, Debug, PartialEq, Hash, PartialOrd, Eq)]
pub struct Ident {
    name: String,
}

impl Ident {
    pub fn new(name: String) -> Self {
        Self { name }
    }
    pub fn get(&self) -> String {
        self.name.clone()
    }
}

impl std::fmt::Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
