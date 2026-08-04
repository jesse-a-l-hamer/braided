use crate::{BandGenerator, BraidIndex, Letter, Sign, Strand, StrandValidationError};

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum ArtinValidationError {
    #[error("The head strand index for such an Artin generator exceeds {max:?}", max = u16::MAX)]
    HeadTooLarge,
    #[error("Given band {0:?} cannot be coerced to Artin generator.")]
    FromBand(BandGenerator),
    #[error(transparent)]
    StrandValidation(#[from] StrandValidationError),
    #[error(transparent)]
    Infallible(#[from] std::convert::Infallible),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ArtinGenerator {
    foot: Strand,
    sign: Sign,
}

impl ArtinGenerator {
    pub fn new<F>(foot: F, sign: Sign) -> Result<Self, ArtinValidationError>
    where
        F: TryInto<u16>,
        StrandValidationError: From<<F as TryInto<u16>>::Error> + From<std::convert::Infallible>,
    {
        let foot = Strand::new(foot)?;
        if *foot == u16::MAX {
            Err(ArtinValidationError::HeadTooLarge)
        } else {
            Ok(Self { foot, sign })
        }
    }

    pub fn foot(&self) -> Strand {
        self.foot
    }
    pub fn sign(&self) -> Sign {
        self.sign
    }

    pub fn head(&self) -> Strand {
        (self.foot + 1).unwrap()
    }
    pub fn inverse(&self) -> Self {
        Self {
            foot: self.foot,
            sign: -self.sign,
        }
    }
    pub fn minimal_required_braid_index(&self) -> BraidIndex {
        BraidIndex::new(self.head()).unwrap()
    }
}

impl TryFrom<BandGenerator> for ArtinGenerator {
    type Error = ArtinValidationError;

    fn try_from(value: BandGenerator) -> Result<Self, Self::Error> {
        if value.is_artin() {
            Ok(Self {
                foot: value.foot(),
                sign: value.sign(),
            })
        } else {
            Err(ArtinValidationError::FromBand(value))
        }
    }
}
impl TryFrom<Letter> for ArtinGenerator {
    type Error = ArtinValidationError;

    fn try_from(value: Letter) -> Result<Self, Self::Error> {
        match value {
            Letter::Artin(artin) => Ok(artin),
            Letter::Band(band) => Self::try_from(band),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ArtinGenerator, ArtinValidationError, BandGenerator, BraidIndex, Letter, Sign, Strand,
    };
    use googletest::matchers::{anything, each, eq, err, ok};
    use googletest::{assert_that, expect_that, gtest};

    #[test]
    fn valid_inputs_yield_successful_construction() {
        let valid_artin_generators = [
            ArtinGenerator::new(1, Sign::Negative),
            ArtinGenerator::new(2_usize, Sign::Positive),
            ArtinGenerator::new(3_isize, Sign::Negative),
            ArtinGenerator::try_from(BandGenerator::new(4, 5, Sign::Positive).unwrap()),
            ArtinGenerator::try_from(Letter::new(6, None::<u16>, Sign::Negative).unwrap()),
            ArtinGenerator::try_from(Letter::new(7, Some(8), Sign::Positive).unwrap()),
            ArtinGenerator::new(u16::MAX - 1, Sign::Negative),
        ];
        assert_that!(valid_artin_generators, each(ok(anything())));
    }

    #[gtest]
    fn properties_compute_as_expected() {
        let input_data = [
            (1, Sign::Negative),
            (10, Sign::Positive),
            (u16::MAX - 1, Sign::Negative),
        ];

        for (foot, sign) in input_data {
            let artin_generator = ArtinGenerator::new(foot, sign).unwrap();
            expect_that!(artin_generator.foot(), eq(Strand::new(foot).unwrap()));
            expect_that!(artin_generator.sign(), eq(sign));
            expect_that!(artin_generator.head(), eq(Strand::new(foot + 1).unwrap()));
            expect_that!(
                artin_generator.inverse(),
                eq(ArtinGenerator::new(foot, -sign).unwrap())
            );
            expect_that!(
                artin_generator.minimal_required_braid_index(),
                eq(BraidIndex::new(foot + 1).unwrap())
            );
        }
    }

    #[gtest]
    fn invalid_inputs_yield_failed_construction() {
        let invalid_artin_generators = [
            (
                ArtinGenerator::new(-1, Sign::Positive),
                ArtinValidationError::from(Strand::new(-1).err().unwrap()),
            ),
            (
                ArtinGenerator::new(0, Sign::Negative),
                ArtinValidationError::from(Strand::new(0).err().unwrap()),
            ),
            (
                ArtinGenerator::new(u16::MAX as u32 + 1, Sign::Positive),
                ArtinValidationError::from(Strand::new(u16::MAX as u32 + 1).err().unwrap()),
            ),
            (
                ArtinGenerator::new(u16::MAX, Sign::Negative),
                ArtinValidationError::HeadTooLarge,
            ),
            (
                ArtinGenerator::try_from(BandGenerator::new(1, 3, Sign::Positive).unwrap()),
                ArtinValidationError::FromBand(BandGenerator::new(1, 3, Sign::Positive).unwrap()),
            ),
            (
                ArtinGenerator::try_from(Letter::new(2, Some(4), Sign::Negative).unwrap()),
                ArtinValidationError::FromBand(BandGenerator::new(2, 4, Sign::Negative).unwrap()),
            ),
        ];

        for (invalid_artin_generator, error) in invalid_artin_generators {
            expect_that!(invalid_artin_generator, err(eq(&error)));
        }
    }
}
