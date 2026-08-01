use crate::generators::{artin_to_band, band_to_artin};
use crate::{ArtinGenerator, BandGenerator, BraidIndex, Sign};
use anyhow::Context;
use std::ops::Mul;

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

#[derive(Debug, PartialEq, Eq)]
pub struct Braid {
    index: BraidIndex,
    word: Vec<BandGenerator>,
}

impl Braid {
    pub fn from_bands(index: u16, bands: &[BandGenerator]) -> Result<Self, BraidValidationError> {
        let index = BraidIndex::new(index).context(format!(
            "Falied to define braid index from given integer {}",
            index
        ))?;
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
    pub fn from_artin(
        index: u16,
        generators: &[ArtinGenerator],
    ) -> Result<Self, BraidValidationError> {
        let index = BraidIndex::new(index).context(format!(
            "Falied to define braid index from given integer {}",
            index
        ))?;
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
    pub fn trivial(index: u16) -> Result<Self, BraidValidationError> {
        Self::from_bands(index, &[])
    }

    pub fn inverse(&self) -> Self {
        let index = self.index;
        let mut word = Vec::new();

        for band in self.word.iter().rev() {
            word.push(band.inverse());
        }

        Self { index, word }
    }

    pub fn index(&self) -> BraidIndex {
        self.index
    }
    pub fn band_word(&self) -> &[BandGenerator] {
        &self.word
    }
    pub fn artin_word(&self) -> Vec<ArtinGenerator> {
        band_to_artin(self.band_word())
    }

    pub fn minimal_required_braid_index(&self) -> BraidIndex {
        match self
            .word
            .iter()
            .map(|b| b.minimal_required_braid_index())
            .max()
        {
            Some(index) => index,
            None => BraidIndex::new(1).unwrap(),
        }
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
    pub fn band_length(&self) -> usize {
        self.word.len()
    }
    pub fn artin_length(&self) -> u16 {
        self.word.iter().fold(0, |a, b| a + b.artin_length())
    }
}

impl Default for Braid {
    fn default() -> Self {
        Self::trivial(1).unwrap()
    }
}

impl Mul for Braid {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let index = self.index.max(rhs.index);
        let mut word = self.word;
        word.extend(rhs.word);
        Self { index, word }
    }
}

#[cfg(test)]
mod tests {
    use super::{Braid, BraidValidationError};
    use crate::{BandGenerator, BraidIndex, artin, band};
    use googletest::assert_that;
    use googletest::matchers::{eq, ok};
    use std::assert_matches;

    #[test]
    fn construction_from_valid_bands_is_successful() {
        let bands = [
            band![1 => 3; 3].unwrap(),
            band![1 => 2; 1].unwrap(),
            band![2 => 3; -4].unwrap(),
            band![1 => 3; -2].unwrap(),
        ]
        .concat();
        let braid = Braid::from_bands(3, &bands);
        assert_that!(
            braid,
            ok(eq(&Braid {
                index: BraidIndex::new(3).unwrap(),
                word: bands.to_vec(),
            }))
        )
    }

    #[test]
    fn construction_from_bad_bands_fails() {
        let bands = [
            band![1 => 3; 3].unwrap(),
            band![1 => 2; 1].unwrap(),
            band![2 => 3; -4].unwrap(),
            band![1 => 4; -2].unwrap(),
        ]
        .concat();
        let braid = Braid::from_bands(3, &bands);
        assert_matches!(braid, Err(BraidValidationError::BadBand { .. }))
    }

    #[test]
    fn construction_from_zero_index_fails() {
        let bands = [
            band![1 => 3; 3].unwrap(),
            band![1 => 2; 1].unwrap(),
            band![2 => 3; -4].unwrap(),
            band![1 => 4; -2].unwrap(),
        ]
        .concat();
        let braid = Braid::from_bands(0, &bands);
        assert_matches!(braid, Err(BraidValidationError::Unexpected(_)))
    }

    #[test]
    fn construction_from_valid_artin_generators_is_successful() {
        let generators = [
            // band 1
            artin![3; 1].unwrap(),
            artin![2; 1].unwrap(),
            artin![1; 1].unwrap(),
            artin![2; -1].unwrap(),
            artin![3; -1].unwrap(),
            // band 2  1
            artin![3; 1].unwrap(),
            artin![2; -1].unwrap(),
            artin![3; -1].unwrap(),
            // band 3  1
            artin![4; 1].unwrap(),
            artin![2; -1].unwrap(),
            artin![3; -1].unwrap(),
            artin![4; -1].unwrap(),
            artin![2; 1].unwrap(),
            // band 4  1
            artin![1; -1].unwrap(),
            // band 5  1
            artin![2; 1].unwrap(),
            artin![1; 1].unwrap(),
            artin![2; -1].unwrap(),
            // band 6  1
            artin![4; 1].unwrap(),
            artin![3; 1].unwrap(),
            artin![4; -1].unwrap(),
        ]
        .concat();
        let braid = Braid::from_artin(5, &generators);
        assert_that!(
            braid,
            ok(eq(&Braid {
                index: BraidIndex::new(5).unwrap(),
                word: [
                    band![1 => 4; 1].unwrap(),
                    band![2 => 4; -1].unwrap(),
                    band![2 => 5; -1].unwrap(),
                    band![1 => 2; -1].unwrap(),
                    band![1 => 3; 1].unwrap(),
                    band![3 => 5; 1].unwrap(),
                ]
                .concat(),
            }))
        )
    }

    #[test]
    fn construction_from_bad_artin_generators_fails() {
        let generators = [
            // band 1
            artin![3; 1].unwrap(),
            artin![2; 1].unwrap(),
            artin![1; 1].unwrap(),
            artin![2; -1].unwrap(),
            artin![3; -1].unwrap(),
            // band 2  1
            artin![3; 1].unwrap(),
            artin![2; -1].unwrap(),
            artin![3; -1].unwrap(),
            // band 3  1
            artin![4; 1].unwrap(),
            artin![2; -1].unwrap(),
            artin![3; -1].unwrap(),
            artin![4; -1].unwrap(),
            artin![2; 1].unwrap(),
            // band 4  1
            artin![1; -1].unwrap(),
            // band 5  1
            artin![2; 1].unwrap(),
            artin![1; 1].unwrap(),
            artin![2; -1].unwrap(),
            // band 6  1
            artin![4; 1].unwrap(),
            artin![3; 1].unwrap(),
            artin![4; -1].unwrap(),
        ]
        .concat();
        let braid = Braid::from_artin(3, &generators);
        assert_matches!(braid, Err(BraidValidationError::BadArtin { .. }))
    }

    #[test]
    fn trivial_constructor_works_as_expected() {
        let trivial = Braid::trivial(3);
        assert_that!(trivial, ok(eq(&Braid::from_bands(3, &[]).unwrap())));
    }

    #[test]
    fn default_braid_is_trivial_unknot() {
        let unknot = Braid::from_bands(1, &[]).unwrap();
        let default = Braid::default();
        assert_that!(default, eq(&unknot));
    }

    #[test]
    fn round_trip_from_artin_word_with_band_crossings_at_top_succeeds() {
        let generators = [
            // band 1
            artin![1; -1].unwrap(),
            artin![2; -1].unwrap(),
            artin![3; 1].unwrap(),
            artin![2; 1].unwrap(),
            artin![1; 1].unwrap(),
            // band 2  1
            artin![2; -1].unwrap(),
            artin![3; -1].unwrap(),
            artin![2; 1].unwrap(),
            // band 3  1
            artin![2; -1].unwrap(),
            artin![3; -1].unwrap(),
            artin![4; -1].unwrap(),
            artin![3; 1].unwrap(),
            artin![2; 1].unwrap(),
            // band 4  1
            artin![1; -1].unwrap(),
            // band 5  1
            artin![1; -1].unwrap(),
            artin![2; 1].unwrap(),
            artin![1; 1].unwrap(),
            // band 6  1
            artin![3; -1].unwrap(),
            artin![4; 1].unwrap(),
            artin![3; 1].unwrap(),
        ]
        .concat();
        let braid = Braid::from_artin(5, &generators).unwrap();
        assert_that!(braid.artin_word(), eq(&generators.to_vec()),)
    }

    #[test]
    fn index_computes_as_expected() {
        let bands = [
            band![1 => 3; 3].unwrap(),
            band![1 => 2; 1].unwrap(),
            band![2 => 3; -4].unwrap(),
            band![1 => 3; -2].unwrap(),
        ]
        .concat();
        let braid = Braid::from_bands(3, &bands).unwrap();
        assert_that!(
            braid.index(),
            eq(BraidIndex::new(bands.iter().map(|b| b.head()).max().unwrap().index()).unwrap())
        )
    }

    #[test]
    fn length_computes_as_expected() {
        let bands = [
            band![1 => 3; 3].unwrap(),
            band![1 => 2; 1].unwrap(),
            band![2 => 3; -4].unwrap(),
            band![1 => 3; -2].unwrap(),
        ]
        .concat();
        let braid = Braid::from_bands(3, &bands).unwrap();
        assert_that!(braid.band_length(), eq(bands.len()))
    }

    #[test]
    fn writhe_computes_as_expected() {
        let bands = [
            band![1 => 3; 3].unwrap(),
            band![1 => 2; 1].unwrap(),
            band![2 => 3; -4].unwrap(),
            band![1 => 3; -2].unwrap(),
        ]
        .concat();
        let braid = Braid::from_bands(3, &bands).unwrap();
        assert_that!(braid.writhe(), eq(3 + 1 - 4 - 2))
    }

    #[test]
    fn inverse_of_braid_computes_as_expected() {
        let bands = [
            band![1 => 3; 3].unwrap(),
            band![1 => 2; 1].unwrap(),
            band![2 => 3; -4].unwrap(),
            band![1 => 3; -2].unwrap(),
        ]
        .concat();
        let inverted_band_word: Vec<BandGenerator> =
            bands.iter().rev().map(|b| b.inverse()).collect();
        let inverse_braid = Braid::from_bands(3, &bands).unwrap().inverse();
        assert_that!(
            inverse_braid,
            eq(&Braid::from_bands(3, &inverted_band_word).unwrap())
        )
    }

    #[test]
    fn double_inverse_returns_braid_unchanged() {
        let bands = [
            band![1 => 3; 3].unwrap(),
            band![1 => 2; 1].unwrap(),
            band![2 => 3; -4].unwrap(),
            band![1 => 3; -2].unwrap(),
        ]
        .concat();
        let braid = Braid::from_bands(3, &bands).unwrap();
        assert_that!(braid.inverse().inverse(), eq(&braid))
    }

    #[test]
    fn multiplication_computes_as_expected() {
        let bands = [
            band![1 => 3; 3].unwrap(),
            band![1 => 2; 1].unwrap(),
            band![2 => 3; -4].unwrap(),
            band![1 => 3; -2].unwrap(),
        ]
        .concat();
        let braid = Braid::from_bands(3, &bands).unwrap();
        let other_bands = [
            band![1 => 3; 2].unwrap(),
            band![2 => 3; 4].unwrap(),
            band![1 => 2; -1].unwrap(),
            band![1 => 3; -3].unwrap(),
        ]
        .concat();
        let other_braid = Braid::from_bands(3, &other_bands).unwrap();
        let product_braid = braid * other_braid;
        assert_that!(
            product_braid,
            eq(&Braid::from_bands(3, &[bands, other_bands].concat()).unwrap())
        )
    }

    #[test]
    fn writhe_of_braid_times_inverse_is_zero() {
        let bands = [
            band![1 => 3; 3].unwrap(),
            band![1 => 2; 1].unwrap(),
            band![2 => 3; -4].unwrap(),
            band![1 => 3; -2].unwrap(),
        ]
        .concat();
        let braid = Braid::from_bands(3, &bands).unwrap();
        let other_bands = [
            band![1 => 3; 2].unwrap(),
            band![2 => 3; 4].unwrap(),
            band![1 => 2; -1].unwrap(),
            band![1 => 3; -3].unwrap(),
        ]
        .concat();
        let other_braid = Braid::from_bands(3, &other_bands).unwrap();
        let product_braid = braid * other_braid;
        assert_that!(product_braid.writhe(), eq(0))
    }
}
