use crate::{BandGenerator, BraidIndex, Sign, Strand};

#[derive(Debug, thiserror::Error)]
pub enum BraidValidationError {
    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}

#[derive(Debug)]
pub struct Braid {
    index: BraidIndex,
    word: Vec<BandGenerator>,
}

impl Braid {
    pub fn new(index: BraidIndex, bands: Vec<BandGenerator>) -> Self {
        Self { index, word: bands }
    }
    pub fn trivial(index: BraidIndex) -> Self {
        Self::new(index, Vec::new())
    }

    pub fn index(&self) -> BraidIndex {
        self.index
    }
    pub fn writhe(&self) -> i16 {
        self.word.iter().fold(0, |a, b| {
            if b.sign() == Sign::Positive {
                a + 1
            } else {
                a - 1
            }
        })
    }
    pub fn length(&self) -> usize {
        self.word.len()
    }
}

impl Default for Braid {
    fn default() -> Self {
        Self::trivial(BraidIndex::new(1))
    }
}
