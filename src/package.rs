use crate::types::Str;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Pkg {
    name: Str,
    size: u64,
}

impl Pkg {
    pub fn new(name: Str, size: u64) -> Self {
        Self { name, size }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn into_hash(self) -> (Str, u64) {
        (self.name, self.size)
    }

    pub fn from_hash((name, size): (Str, u64)) -> Self {
        Self::new(name, size)
    }
}
