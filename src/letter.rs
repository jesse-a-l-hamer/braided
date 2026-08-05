use crate::{
    ArtinGenerator, ArtinValidationError, BandGenerator, BandValidationError, BraidIndex, Sign,
    Strand, StrandValidationError, Word, WordValidationError,
};

#[derive(Debug, thiserror::Error, PartialEq, Eq, Clone, Copy)]
pub enum LetterValidationError {
    #[error(transparent)]
    ArtinValidation(#[from] ArtinValidationError),
    #[error(transparent)]
    BandValidation(#[from] BandValidationError),
}

#[derive(Debug, Clone, Copy, Eq)]
pub enum Letter {
    Artin(ArtinGenerator),
    Band(BandGenerator),
}

impl Letter {
    pub fn new<F, H>(foot: F, head: Option<H>, sign: Sign) -> Result<Self, LetterValidationError>
    where
        F: TryInto<u16>,
        H: TryInto<u16>,
        StrandValidationError: From<<F as TryInto<u16>>::Error>
            + From<<H as TryInto<u16>>::Error>
            + From<std::convert::Infallible>,
    {
        match head {
            Some(head) => Ok(Self::Band(BandGenerator::new(foot, head, sign)?)),
            None => Ok(Self::Artin(ArtinGenerator::new(foot, sign)?)),
        }
    }

    pub fn sign(&self) -> Sign {
        match self {
            Self::Artin(artin) => artin.sign(),
            Self::Band(band) => band.sign(),
        }
    }
    pub fn foot(&self) -> Strand {
        match self {
            Self::Artin(artin) => artin.foot(),
            Self::Band(band) => band.foot(),
        }
    }
    pub fn head(&self) -> Strand {
        match self {
            Self::Artin(artin) => artin.head(),
            Self::Band(band) => band.head(),
        }
    }

    pub fn inverse(&self) -> Self {
        match self {
            Self::Artin(artin) => Self::Artin(artin.inverse()),
            Self::Band(band) => Self::Band(band.inverse()),
        }
    }

    pub fn is_artin(&self) -> bool {
        match self {
            Self::Artin(_) => true,
            Self::Band(band) => band.is_artin(),
        }
    }

    pub fn artin_length(&self) -> u16 {
        match self {
            Self::Artin(_) => 1,
            Self::Band(band) => band.artin_length(),
        }
    }
    pub fn minimal_required_braid_index(&self) -> BraidIndex {
        match self {
            Self::Artin(artin) => artin.minimal_required_braid_index(),
            Self::Band(band) => band.minimal_required_braid_index(),
        }
    }
}

impl From<ArtinGenerator> for Letter {
    fn from(value: ArtinGenerator) -> Self {
        Self::Artin(value)
    }
}
impl From<BandGenerator> for Letter {
    fn from(value: BandGenerator) -> Self {
        Self::Band(value)
    }
}

impl std::cmp::PartialEq for Letter {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Artin(lhs), Self::Artin(rhs)) => lhs == rhs,
            (Self::Artin(lhs), Self::Band(rhs)) => {
                rhs.is_artin() && *lhs == ArtinGenerator::new(rhs.foot(), rhs.sign()).unwrap()
            }
            (Self::Band(lhs), Self::Artin(rhs)) => {
                lhs.is_artin() && ArtinGenerator::new(lhs.foot(), rhs.sign()).unwrap() == *rhs
            }
            (Self::Band(lhs), Self::Band(rhs)) => lhs == rhs,
        }
    }
}

impl std::ops::Mul for Letter {
    type Output = Result<Word, WordValidationError>;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Artin(lhs), Self::Artin(rhs)) => {
                if lhs == rhs.inverse() {
                    Ok(Word::trivial())
                } else {
                    Word::try_from(vec![lhs, rhs])
                }
            }
            (Self::Artin(lhs), Self::Band(rhs)) => {
                if rhs.inverse() == lhs.into() {
                    Ok(Word::trivial())
                } else {
                    Word::try_from(vec![Self::Artin(lhs), Self::Band(rhs)])
                }
            }
            (Self::Band(lhs), Self::Artin(rhs)) => {
                if lhs.inverse() == rhs.into() {
                    Ok(Word::trivial())
                } else {
                    Word::try_from(vec![Self::Band(lhs), Self::Artin(rhs)])
                }
            }
            (Self::Band(lhs), Self::Band(rhs)) => {
                if lhs == rhs.inverse() {
                    Ok(Word::trivial())
                } else {
                    Word::try_from(vec![lhs, rhs])
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ArtinGenerator, BandGenerator, BraidIndex, Letter, LetterValidationError, Sign, Strand,
        Word, WordValidationError,
    };
    use googletest::matchers::{anything, each, eq, err, is_false, is_true, ok};
    use googletest::{assert_that, expect_that, gtest};

    #[test]
    fn valid_inputs_yield_successful_construction() {
        let valid_letters = [
            Letter::new(1, None::<u16>, Sign::Positive),
            Letter::new(2, Some(4), Sign::Negative),
            Letter::new(Strand::new(5).unwrap(), None::<u16>, Sign::Negative),
            Letter::new(6isize, Some(Strand::new(7).unwrap()), Sign::Positive),
            Letter::new(
                Strand::new(8).unwrap(),
                Some(Strand::new(30).unwrap()),
                Sign::Negative,
            ),
            Letter::new(u16::MAX as u32 - 1, None::<u16>, Sign::Positive),
            Letter::new(1, Some(2u16.pow(15) + 1), Sign::Negative),
        ];

        assert_that!(valid_letters, each(ok(anything())));
    }

    #[gtest]
    fn conversion_from_generators_works_as_expected() {
        let letter_from_artin = Letter::from(ArtinGenerator::new(1, Sign::Positive).unwrap());
        let letter_from_band = Letter::from(BandGenerator::new(3, 5, Sign::Negative).unwrap());

        expect_that!(
            letter_from_artin,
            eq(Letter::new(1, None::<u16>, Sign::Positive).unwrap())
        );
        expect_that!(
            letter_from_band,
            eq(Letter::new(3, Some(5), Sign::Negative).unwrap())
        );
    }

    #[gtest]
    fn properties_compute_as_expected() {
        let input_data = [
            (1, None::<u16>, Sign::Positive),
            (3, Some(5), Sign::Negative),
            (6, Some(7), Sign::Positive),
        ];

        for (foot, head, sign) in input_data {
            let letter = Letter::new(foot, head, sign).unwrap();

            expect_that!(letter.sign(), eq(sign));
            expect_that!(letter.foot(), eq(Strand::new(foot).unwrap()));
            expect_that!(
                letter.head(),
                eq(Strand::new(head.unwrap_or(foot + 1)).unwrap())
            );
            expect_that!(
                letter.inverse(),
                eq(Letter::new(foot, head, -sign).unwrap())
            );
            expect_that!(letter.is_artin(), eq(head.unwrap_or(foot + 1) - foot == 1));
            expect_that!(
                letter.artin_length(),
                eq(2 * (head.unwrap_or(foot + 1) - foot) - 1)
            );
            expect_that!(
                letter.minimal_required_braid_index(),
                eq(BraidIndex::new(head.unwrap_or(foot + 1)).unwrap()),
            )
        }
    }

    #[gtest]
    fn equality_comparison_behaves_as_expected() {
        let artin_letter = Letter::new(1, None::<u16>, Sign::Positive).unwrap();
        let other_artin_letter = Letter::new(2, None::<u16>, Sign::Positive).unwrap();
        let band_letter = Letter::new(1, Some(2), Sign::Positive).unwrap();
        let other_band_letter = Letter::new(1, Some(2), Sign::Negative).unwrap();

        expect_that!(artin_letter == artin_letter, is_true());
        expect_that!(band_letter == band_letter, is_true());
        expect_that!(artin_letter == band_letter, is_true());
        expect_that!(band_letter == artin_letter, is_true());

        expect_that!(artin_letter == other_artin_letter, is_false());
        expect_that!(band_letter == other_artin_letter, is_false());
        expect_that!(artin_letter == other_band_letter, is_false());
        expect_that!(band_letter == other_band_letter, is_false());
    }

    #[gtest]
    fn valid_multiplication_succeeds_and_computes_as_expected() {
        let l1 = Letter::new(1, None::<u16>, Sign::Positive).unwrap();
        let l2 = Letter::new(2, Some(4), Sign::Negative).unwrap();

        let product_data = [vec![l1, l1], vec![l1, l2], vec![l2, l1], vec![l2, l2]];

        for pair in product_data {
            expect_that!(pair[0] * pair[1], eq(&Word::try_from(pair)));
        }
    }

    #[gtest]
    fn inverse_is_multiplicative_inverse() {
        let l1 = Letter::new(1, None::<u16>, Sign::Positive).unwrap();
        let l2 = Letter::new(2, Some(4), Sign::Negative).unwrap();
        let l3 = Letter::new(1, Some(2), Sign::Negative).unwrap();

        let product_data = [
            [l1, l1.inverse()],
            [l1.inverse(), l1],
            [l2, l2.inverse()],
            [l2.inverse(), l2],
            [l1, l3],
            [l3, l1],
        ];

        for pair in product_data {
            expect_that!(pair[0] * pair[1], ok(eq(&Word::trivial())));
        }
    }

    #[gtest]
    fn invalid_inputs_yield_failed_construction() {
        let invalid_letters = [
            (
                Letter::new(0, None::<u16>, Sign::Positive),
                LetterValidationError::from(ArtinGenerator::new(0, Sign::Positive).err().unwrap()),
            ),
            (
                Letter::new(4, Some(1), Sign::Negative),
                LetterValidationError::from(
                    BandGenerator::new(4, 1, Sign::Negative).err().unwrap(),
                ),
            ),
        ];

        for (invalid_letter, error) in invalid_letters {
            expect_that!(invalid_letter, err(eq(error)));
        }
    }

    #[gtest]
    fn invalid_multiplication_fails() {
        let l1 = Letter::new(1, Some(3), Sign::Positive).unwrap();
        let l2 = Letter::new(1, Some(2u16.pow(15) + 1), Sign::Negative).unwrap();
        let error = WordValidationError::TooLong(u16::MAX as usize + 3);

        expect_that!(l1 * l2, err(eq(&error)));
        expect_that!(l2 * l1, err(eq(&error)));
    }
}
