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

/// Represents a generator in the standard Artin presentation of the braid group.
///
/// Geometrically, an Artin generator corresponds to a crossing of _adjacent_ [strands][Strand]. In
/// particular, suppose we picture the braid strands as being a collection of vertically stacked
/// parallel lines (except near crossings), each of which is oriented left-to-right. Choose any
/// strand besides the topmost, and suppose its index is `k`. Then an `ArtinGenerator` with _foot_
/// strand `k` and [_positive_](Sign::Positive) ([_negative_](Sign::Negative)) _sign_ corresponds to
/// the crossing of strand `k + 1` _over_ (_under_) the strand `k`.
///
/// Note that interacting directly with [`ArtinGenerator`] is _not_ recommended; instead one should
/// work with the [`Letter`] enum, which abstracts over the specific choice of generating set.
///
/// # Construction
///
/// An [`ArtinGenerator`] may be constructed in one of two ways: _directly_, using the associated
/// function [`ArtinGenerator::new`], or by converting a [`BandGenerator`] or [`Letter`] using the
/// associated function [`ArtinGenerator::try_from`].
///
/// 1. [`ArtinGenerator::new`] can construct an [`ArtinGenerator`] given a [`Sign`] and any value
///    which can be coerced into a [`u16`]. Failure occurs if the underlying foot strand cannot be
///    constructed, or if one attempts to use a foot strand with index [`u16::MAX`] (as then the
///    corresponding headstrand would be invalid).
///
/// ```
/// use braided::{ArtinGenerator, Sign, Strand};
/// use std::assert_matches;
///
/// let artin_from_u16 = ArtinGenerator::new(1, Sign::Negative);
/// let artin_from_isize = ArtinGenerator::new(-(1_isize - u16::MAX as isize), Sign::Positive);
/// let artin_from_strand = ArtinGenerator::new(Strand::new(2).unwrap(), Sign::Negative);
///
/// assert_matches!(artin_from_u16, Ok(_));
/// assert_matches!(artin_from_isize, Ok(_));
/// assert_matches!(artin_from_strand, Ok(_));
/// ```
///
/// 2. [`ArtinGenerator::try_from`] attempts to consruct an [`ArtinGenerator`] given an already
///    existing [`BandGenerator`] or [`Letter`]. Failure occurs if the input is a [`BandGenerator`]
///    or [`Letter::Band`] which is not a valid Artin generator (its foot and head strand are not adjacent.)
///
/// ```
/// use braided::{ArtinGenerator, BandGenerator, Letter, Sign};
/// use std::assert_matches;
///
/// let artin_from_band = ArtinGenerator::try_from(
///     BandGenerator::new(3, 4, Sign::Positive).unwrap()
/// );
/// let artin_from_artin_letter = ArtinGenerator::try_from(
///     Letter::new(4, None::<u16>, Sign::Negative).unwrap()
/// );
/// let artin_from_band_letter = ArtinGenerator::try_from(
///     Letter::new(5, Some(6), Sign::Positive).unwrap()
/// );
///
/// assert_matches!(artin_from_band, Ok(_));
/// assert_matches!(artin_from_artin_letter, Ok(_));
/// assert_matches!(artin_from_band_letter, Ok(_));
/// ```
///
/// See the documentation for [`ArtinValidationError`] for more details on possible contsruction failure.
///
/// # Accessors & Basic Properties
///
/// [`ArtinGenerator`] exposes several methods for accessing underlying data or computing simple
/// properties of the Artin generator. The examples below demonstrate what can be computed.
///
/// ```
/// use braided::{ArtinGenerator, BraidIndex, Sign, Strand};
///
/// let artin_generator = ArtinGenerator::new(1, Sign::Positive).unwrap();
///
/// // Basic accessors
/// assert_eq!(artin_generator.foot(), Strand::new(1).unwrap());
/// assert_eq!(artin_generator.sign(), Sign::Positive);
///
/// // Computed properties
/// assert_eq!(artin_generator.head(), Strand::new(2).unwrap());
/// assert_eq!(artin_generator.inverse(), ArtinGenerator::new(1, Sign::Negative).unwrap());
/// assert_eq!(artin_generator.minimal_required_braid_index(), BraidIndex::new(2).unwrap());
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ArtinGenerator {
    foot: Strand,
    sign: Sign,
}

impl ArtinGenerator {
    /// Constructs a new [`ArtinGenerator`] given a [`u16`]-coercible foot index and [`Sign`].
    ///
    /// # Examples
    ///
    /// ```
    /// use braided::{ArtinGenerator, Sign, Strand};
    /// use std::assert_matches;
    ///
    /// let artin_from_u16 = ArtinGenerator::new(1, Sign::Negative);
    /// let artin_from_isize = ArtinGenerator::new(-(1_isize - u16::MAX as isize), Sign::Positive);
    /// let artin_from_strand = ArtinGenerator::new(Strand::new(2).unwrap(), Sign::Negative);
    ///
    /// assert_matches!(artin_from_u16, Ok(_));
    /// assert_matches!(artin_from_isize, Ok(_));
    /// assert_matches!(artin_from_strand, Ok(_));
    /// ```
    ///
    /// # Errors
    ///
    /// Retusn [`ArtinValidationError`] if the underlying foot [`Strand`] fails construction, or if
    /// one attempts to use [`u16::MAX`] as the foot strand index.
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

    /// Returns the foot [`Strand`] stored in the [`ArtinGenerator`].
    ///
    /// # Example
    ///
    /// ```
    /// use braided::{ArtinGenerator, Sign, Strand};
    ///
    /// let artin_generator = ArtinGenerator::new(1, Sign::Positive).unwrap();
    ///
    /// assert_eq!(artin_generator.foot(), Strand::new(1).unwrap());
    /// ```
    pub fn foot(&self) -> Strand {
        self.foot
    }
    /// Returns the [sign](Sign) stored in the [`ArtinGenerator`].
    ///
    /// # Example
    ///
    /// ```
    /// use braided::{ArtinGenerator, Sign, Strand};
    ///
    /// let artin_generator = ArtinGenerator::new(1, Sign::Positive).unwrap();
    ///
    /// assert_eq!(artin_generator.sign(), Sign::Positive);
    /// ```
    pub fn sign(&self) -> Sign {
        self.sign
    }

    /// Computes the head [`Strand`] of the [`ArtinGenerator`].
    ///
    /// # Example
    ///
    /// ```
    /// use braided::{ArtinGenerator, Sign, Strand};
    ///
    /// let artin_generator = ArtinGenerator::new(1, Sign::Positive).unwrap();
    ///
    /// assert_eq!(artin_generator.head(), Strand::new(2).unwrap());
    /// ```
    pub fn head(&self) -> Strand {
        (self.foot + 1).unwrap()
    }
    /// Computes the inverse of the [`ArtinGenerator`], which amounts to negating its [sign](Sign).
    ///
    /// # Example
    ///
    /// ```
    /// use braided::{ArtinGenerator, Sign, Strand};
    ///
    /// let artin_generator = ArtinGenerator::new(1, Sign::Positive).unwrap();
    ///
    /// assert_eq!(artin_generator.inverse(), ArtinGenerator::new(1, Sign::Negative).unwrap());
    /// ```
    pub fn inverse(&self) -> Self {
        Self {
            foot: self.foot,
            sign: -self.sign,
        }
    }
    /// Computes the minimal [`BraidIndex`] required for a braid to use the [`ArtinGenerator`].
    ///
    /// For Artin generators, this is equal to the index of the [head](ArtinGenerator::head)
    /// [Strand].
    ///
    /// # Example
    ///
    /// ```
    /// use braided::{ArtinGenerator, BraidIndex, Sign, Strand};
    ///
    /// let artin_generator = ArtinGenerator::new(1, Sign::Positive).unwrap();
    ///
    /// assert_eq!(artin_generator.minimal_required_braid_index(), BraidIndex::new(2).unwrap());
    /// ```
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
            ArtinGenerator::new(Strand::new(8).unwrap(), Sign::Negative),
            ArtinGenerator::new(u16::MAX - 1, Sign::Positive),
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
