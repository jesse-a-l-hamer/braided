#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BraidIndex(u16);

impl BraidIndex {
    pub fn new(index: u16) -> Self {
        Self(index)
    }
}
