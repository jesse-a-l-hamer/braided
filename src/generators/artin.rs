use crate::{BandGenerator, BraidIndex, Letter, Sign, Strand, StrandValidationError};

/// Represents failure during construction of an [`ArtinGenerator`].
///
/// An [`ArtinGenerator`] can either be constructed directly (i.e., using [`ArtinGenerator::new`]),
/// or by converting from a [`BandGenerator`] or [`Letter`] (using [`ArtinGenerator::try_from`]).
///
/// # Errors from [`ArtinGenerator::new`]
///
/// 1. Construction will fail with [`ArtinValidationError::StrandValidation`] if construction of the
///    underlying [`Strand`] fails (see [`StrandValidationError`] for more details on the wrapped
///    error type):
///
/// ```
/// use braided::{ArtinGenerator, ArtinValidationError, Sign};
/// use std::assert_matches;
///
/// assert_matches!(
///     ArtinGenerator::new(-1, Sign::Positive),
///     Err(ArtinValidationError::StrandValidation(_)),
/// );
///
/// assert_matches!(
///     ArtinGenerator::new(0, Sign::Negative),
///     Err(ArtinValidationError::StrandValidation(_)),
/// );
///
/// assert_matches!(
///     ArtinGenerator::new(u16::MAX as u32 + 1, Sign::Positive),
///     Err(ArtinValidationError::StrandValidation(_)),
/// );
/// ```
///
/// 2. Construction will fail if attempting to use [`u16::MAX`] as the foot strand, since then the
///    corresponding head strand would not be a valid [`u16`]:
///
/// ```
/// use braided::{ArtinGenerator, ArtinValidationError, Sign};
///
/// assert_eq!(
///     ArtinGenerator::new(u16::MAX, Sign::Negative),
///     Err(ArtinValidationError::InvalidHead),
/// );
/// ```
///
/// # Errors from [`ArtinGenerator::try_from`]
///
/// Construction using [`ArtinGenerator::try_from`] will fail whenever an attempt is made at
/// converting from a [`BandGenerator`] for which [`BandGenerator::is_artin`] is false, which is
/// equivalent to the band's head strand being more than one strand above its foot.
///
/// ```
/// use braided::{ArtinGenerator, ArtinValidationError, BandGenerator, Letter, Sign};
///
/// let non_artin_band = BandGenerator::new(1, 3, Sign::Positive).unwrap();
///
/// assert_eq!(non_artin_band.is_artin(), false);
/// assert_eq!(
///     ArtinGenerator::try_from(non_artin_band),
///     Err(ArtinValidationError::FromBand(non_artin_band)),
/// );
///
/// let non_artin_letter = Letter::new(2, Some(7), Sign::Negative).unwrap();
///
/// assert_eq!(non_artin_letter.is_artin(), false);
/// assert_eq!(
///     ArtinGenerator::try_from(non_artin_letter),
///     Err(ArtinValidationError::FromBand(BandGenerator::new(2, 7, Sign::Negative).unwrap())),
/// )
/// ```
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum ArtinValidationError {
    /// Indicates attempt to construct [`ArtinGenerator`] with foot index [`u16::MAX`].
    #[error("The head strand index for such an Artin generator exceeds {max:?}", max = u16::MAX)]
    InvalidHead,
    /// Indicates failed conversion from [`BandGenerator`] or [`Letter`] when using
    /// [`ArtinGenerator::try_from`].
    ///
    /// Wraps the offending [`BandGenerator`].
    #[error("Given band {0:?} cannot be coerced to Artin generator.")]
    FromBand(BandGenerator),
    /// Indicates failed attepmt to build foot [`Strand`] when using [`ArtinGenerator::new`].
    ///
    /// Wrapper around [`StrandValidationError`].
    #[error(transparent)]
    StrandValidation(#[from] StrandValidationError),
    /// Included purely to make the type system happy; cannot occur in practice.
    ///
    /// Wraps [`std::convert::Infallible`].
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
            Err(ArtinValidationError::InvalidHead)
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
                ArtinValidationError::InvalidHead,
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
