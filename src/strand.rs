#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Strand(u16);

impl Strand {
    pub fn new(index: u16) -> Self {
        Self(index)
    }

    pub fn index(&self) -> u16 {
        self.0
    }
}
