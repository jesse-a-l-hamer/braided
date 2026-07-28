use crate::generators::{artin_to_band, band_to_artin};
use crate::{ArtinGenerator, BandGenerator, BraidIndex, Sign};
use std::ops::{Mul, Neg};

/// Enum representing possible errors that may occur during construction of a new braid.
#[derive(Debug, thiserror::Error)]
pub enum BraidValidationError {
    #[error(
        "Braid index {index:?} too small for Artin generator requiring minimal index {min_idx:?}.",
        min_idx = .generator.minimal_required_braid_index(),
    )]
    BadArtin {
        index: BraidIndex,
        generator: ArtinGenerator,
    },
    #[error(
        "Braid index {index:?} too small for band requiring minimal index {min_idx:?}.",
        min_idx = .band.minimal_required_braid_index(),
        )]
    BadBand {
        index: BraidIndex,
        band: BandGenerator,
    },
    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}

/// The heart of the library: a braid.
///
/// We choose to represent braids internally using _band generators_, though a constructor exists
/// to create them using the standard Artin generators.
#[derive(Debug, PartialEq, Eq)]
pub struct Braid {
    index: BraidIndex,
    word: Vec<BandGenerator>,
}

impl Braid {
    /// Constructor to create a braid from an index and a list of bands.
    pub fn new(index: BraidIndex, bands: &[BandGenerator]) -> Result<Self, BraidValidationError> {
        for band in bands {
            if index < band.minimal_required_braid_index() {
                return Err(BraidValidationError::BadBand { index, band: *band });
            }
        }
        Ok(Self {
            index,
            word: bands.to_vec(),
        })
    }
    /// Constructor to create a braid from an index and a list of Artin generators. Internally, we
    /// first convert the list of Artin generators into a list of band generators.
    pub fn from_artin(
        index: BraidIndex,
        generators: &[ArtinGenerator],
    ) -> Result<Self, BraidValidationError> {
        for generator in generators {
            if index < generator.minimal_required_braid_index() {
                return Err(BraidValidationError::BadArtin {
                    index,
                    generator: *generator,
                });
            }
        }
        Ok(Self {
            index,
            word: artin_to_band(generators).to_vec(),
        })
    }
    /// Construct a trivial braid of a given index.
    pub fn trivial(index: BraidIndex) -> Self {
        Self::new(index, &[]).unwrap()
    }

    /// Accessor method for the braid's index.
    pub fn index(&self) -> BraidIndex {
        self.index
    }
    /// Accessor method for braid's word in band generators.
    pub fn word(&self) -> &[BandGenerator] {
        &self.word
    }
    pub fn artin_word(&self) -> Vec<ArtinGenerator> {
        band_to_artin(self.word())
    }

    /// Computes the writhe of the braid, meaning the sum of signs across all bands in the braid.
    pub fn writhe(&self) -> i16 {
        self.word.iter().fold(0, |a, b| {
            if b.sign() == Sign::Positive {
                a + 1
            } else {
                a - 1
            }
        })
    }
    /// Computes the length of the braid, meaning the number of bands used to define it. For the
    /// number of Artin generators, see the `artin_length` method.
    pub fn length(&self) -> usize {
        self.word.len()
    }
    pub fn artin_length(&self) -> u16 {
        self.word.iter().fold(0, |a, b| a + b.artin_length())
    }
}

impl Default for Braid {
    /// Returns the index-1 trivial braid.
    fn default() -> Self {
        Self::trivial(BraidIndex::new(1).unwrap())
    }
}

impl Neg for Braid {
    type Output = Self;

    /// Computes the algebraic negation of the braid.
    fn neg(self) -> Self::Output {
        let index = self.index;
        let mut word = Vec::new();

        for band in self.word.iter().rev() {
            word.push(-*band);
        }

        Self { index, word }
    }
}

impl Mul for Braid {
    type Output = Self;

    /// Multiplies two braids by concatenating their words.
    fn mul(self, rhs: Self) -> Self::Output {
        let index = self.index.max(rhs.index);
        let mut word = self.word;
        word.extend(rhs.word);
        Self { index, word }
    }
}
