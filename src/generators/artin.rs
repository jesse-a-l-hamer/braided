use crate::{
    ArtinResult, ArtinValidationError, BandGenerator, BraidIndex, Letter, Sign, Strand,
    StrandValidationError,
};

/// Represents a generator in the standard Artin presentation of the braid group.
///
/// <div class="warning">
///
/// Consider using [`Letter`] instead of [`ArtinGenerator`] unless you need low-level access to the
/// underlying generating set.
///
/// </div>
///
/// Geometrically, an Artin generator corresponds to a crossing of _adjacent_ [strands][Strand]. In
/// particular, suppose we picture the braid strands as being a collection of vertically stacked
/// parallel lines (except near crossings), each of which is oriented left-to-right. Choose any
/// strand besides the topmost, and suppose its index is `k`. Then an `ArtinGenerator` with _foot_
/// strand `k` and [_positive_](Sign::Positive) ([_negative_](Sign::Negative)) _sign_ corresponds to
/// the crossing of strand `k + 1` _over_ (_under_) the strand `k`.
///
/// # Construction
///
/// An [`ArtinGenerator`] may be constructed in one of three ways: _directly_, using the associated
/// function [`ArtinGenerator::try_new`], or by converting a [`BandGenerator`] or [`Letter`] using the
/// associated functions [`ArtinGenerator::try_from_band`] and [`ArtinGenerator::try_from_letter`],
/// respectively.
///
/// <div class="warning">
///
/// The return type in all cases is [`ArtinResult`](ArtinResult), which is a new-type wrapper around
/// [`Result<ArtinGenerator, ArtinValidationError>`]. Use the dereference operator "*" for easy access to
/// the inner value.
///
/// </div>
///
/// 1. [`ArtinGenerator::try_new`] can construct an [`ArtinGenerator`] given a [`Sign`] and any value
///    which can be coerced into a [`u16`]. Failure occurs if the underlying foot strand cannot be
///    constructed, or if one attempts to use a foot strand with index [`u16::MAX`] (as then the
///    corresponding headstrand would be invalid).
///
/// ```
/// use braided::{ArtinGenerator, Sign, Strand};
/// use std::assert_matches;
///
/// let artin_from_u16 = ArtinGenerator::try_new(1, Sign::Negative);
/// let artin_from_isize = ArtinGenerator::try_new(-(1_isize - u16::MAX as isize), Sign::Positive);
/// let artin_from_strand = ArtinGenerator::try_new(Strand::try_new(2).unwrap(), Sign::Negative);
///
/// assert_matches!(*artin_from_u16, Ok(_));
/// assert_matches!(*artin_from_isize, Ok(_));
/// assert_matches!(*artin_from_strand, Ok(_));
/// ```
///
/// 2. [`ArtinGenerator::try_from_band`] attempts to consruct an [`ArtinGenerator`] given an already
///    existing [`BandGenerator`]. Failure occurs if the input is a [`BandGenerator`] which is not
///    a valid Artin generator (its foot and head strand are not adjacent.)
///
/// ```
/// use braided::{ArtinGenerator, BandGenerator, Sign};
/// use std::assert_matches;
///
/// let artin_from_band = ArtinGenerator::try_from_band(
///     BandGenerator::try_new(3, 4, Sign::Positive).unwrap()
/// );
///
/// assert_matches!(*artin_from_band, Ok(_));
/// ```
///
/// 3. [`ArtinGenerator::try_from_letter`] attempts to consruct an [`ArtinGenerator`] given an already
///    existing [`Letter`]. Failure occurs if the input is a [`Letter::Band`] which is not a valid
///    Artin generator (its foot and head strand are not adjacent.)
///
/// ```
/// use braided::{ArtinGenerator, BandGenerator, Letter, Sign};
/// use std::assert_matches;
///
/// let artin_from_artin_letter = ArtinGenerator::try_from_letter(
///     Letter::try_new(4, None::<u16>, Sign::Negative).unwrap()
/// );
/// let artin_from_band_letter = ArtinGenerator::try_from_letter(
///     Letter::try_new(5, Some(6), Sign::Positive).unwrap()
/// );
///
/// assert_matches!(*artin_from_artin_letter, Ok(_));
/// assert_matches!(*artin_from_band_letter, Ok(_));
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
/// let artin_generator = ArtinGenerator::try_new(1, Sign::Positive).unwrap();
///
/// // Basic accessors
/// assert_eq!(artin_generator.foot(), Strand::try_new(1).unwrap());
/// assert_eq!(artin_generator.sign(), Sign::Positive);
///
/// // Computed properties
/// assert_eq!(artin_generator.head(), Strand::try_new(2).unwrap());
/// assert_eq!(artin_generator.inverse(), ArtinGenerator::try_new(1, Sign::Negative).unwrap());
/// assert_eq!(artin_generator.minimal_required_braid_index(), BraidIndex::try_new(2).unwrap());
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ArtinGenerator {
    foot: Strand,
    sign: Sign,
}

impl ArtinGenerator {
    /// Attempts to construct a new [`ArtinGenerator`] given a [foot index](Strand) and
    /// [sign](Sign).
    ///
    /// <div class="warning">
    ///
    /// The return type is [`ArtinResult`](ArtinResult), which is a new-type wrapper around
    /// [`Result<ArtinGenerator, ArtinValidationError>`]. Use the dereference operator "*" for easy access to
    /// the inner value.
    ///
    /// </div>
    ///
    /// # Examples
    ///
    /// ```
    /// use braided::{ArtinGenerator, Sign, Strand};
    /// use std::assert_matches;
    ///
    /// let artin_from_u16 = ArtinGenerator::try_new(1, Sign::Negative);
    /// let artin_from_isize = ArtinGenerator::try_new(-(1_isize - u16::MAX as isize), Sign::Positive);
    /// let artin_from_strand = ArtinGenerator::try_new(Strand::try_new(2).unwrap(), Sign::Negative);
    ///
    /// assert_matches!(*artin_from_u16, Ok(_));
    /// assert_matches!(*artin_from_isize, Ok(_));
    /// assert_matches!(*artin_from_strand, Ok(_));
    /// ```
    ///
    /// # Errors
    ///
    /// Retusn [`ArtinValidationError`] if the underlying foot [`Strand`] fails construction, or if
    /// one attempts to use [`u16::MAX`] as the foot strand index.
    #[tracing::instrument(level = "info")]
    pub fn try_new<F>(foot: F, sign: Sign) -> ArtinResult
    where
        F: TryInto<u16> + std::fmt::Debug,
        StrandValidationError: From<<F as TryInto<u16>>::Error> + From<std::convert::Infallible>,
    {
        let foot = match *Strand::try_new(foot) {
            Ok(foot) => foot,
            Err(e) => return ArtinResult::from(ArtinValidationError::from(e)),
        };
        if *foot == u16::MAX {
            ArtinResult::from(ArtinValidationError::InvalidHead)
        } else {
            ArtinResult::from(Self { foot, sign })
        }
    }

    /// Attempts to construct an [`ArtinGenerator`] from a [band](BandGenerator).
    ///
    /// <div class="warning">
    ///
    /// The return type is [`ArtinResult`](ArtinResult), which is a new-type wrapper around
    /// [`Result<ArtinGenerator, ArtinValidationError>`]. Use the dereference operator "*" for easy access to
    /// the inner value.
    ///
    /// </div>
    ///
    /// # Examples
    ///
    /// ```
    /// use braided::{ArtinGenerator, BandGenerator, Sign};
    /// use std::assert_matches;
    ///
    /// let artin_from_band = ArtinGenerator::try_from_band(
    ///     BandGenerator::try_new(3, 4, Sign::Positive).unwrap()
    /// );
    ///
    /// assert_matches!(*artin_from_band, Ok(_));
    /// ```
    ///
    /// # Errors
    ///
    /// Fails if the given [band](BandGenerator) does not satisfy [BandGenerator::is_artin].
    #[tracing::instrument(level = "info")]
    pub fn try_from_band(band: BandGenerator) -> ArtinResult {
        if band.is_artin() {
            ArtinResult::from(Self {
                foot: band.foot(),
                sign: band.sign(),
            })
        } else {
            ArtinResult::from(ArtinValidationError::FromBand(band))
        }
    }

    /// Attempts to construct an [`ArtinGenerator`] from a [band letter](Letter::Band).
    ///
    /// <div class="warning">
    ///
    /// The return type is [`ArtinResult`](ArtinResult), which is a new-type wrapper around
    /// [`Result<ArtinGenerator, ArtinValidationError>`]. Use the dereference operator "*" for easy access to
    /// the inner value.
    ///
    /// </div>
    ///
    /// # Examples
    ///
    /// ```
    /// use braided::{ArtinGenerator, BandGenerator, Letter, Sign};
    /// use std::assert_matches;
    ///
    /// let artin_from_artin_letter = ArtinGenerator::try_from_letter(
    ///     Letter::try_new(4, None::<u16>, Sign::Negative).unwrap()
    /// );
    /// let artin_from_band_letter = ArtinGenerator::try_from_letter(
    ///     Letter::try_new(5, Some(6), Sign::Positive).unwrap()
    /// );
    ///
    /// assert_matches!(*artin_from_artin_letter, Ok(_));
    /// assert_matches!(*artin_from_band_letter, Ok(_));
    /// ```
    ///
    /// # Errors
    ///
    /// Fails if the given [band letter](Letter::Band) does not satisfy [Letter::is_artin].
    #[tracing::instrument(level = "info")]
    pub fn try_from_letter(letter: Letter) -> ArtinResult {
        match letter {
            Letter::Artin(artin) => ArtinResult::from(artin),
            Letter::Band(band) => Self::try_from_band(band),
        }
    }

    /// Returns the foot [`Strand`] stored in the [`ArtinGenerator`].
    ///
    /// # Example
    ///
    /// ```
    /// use braided::{ArtinGenerator, Sign, Strand};
    ///
    /// let artin_generator = ArtinGenerator::try_new(1, Sign::Positive).unwrap();
    ///
    /// assert_eq!(artin_generator.foot(), Strand::try_new(1).unwrap());
    /// ```
    #[tracing::instrument(level = "debug")]
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
    /// let artin_generator = ArtinGenerator::try_new(1, Sign::Positive).unwrap();
    ///
    /// assert_eq!(artin_generator.sign(), Sign::Positive);
    /// ```
    #[tracing::instrument(level = "debug")]
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
    /// let artin_generator = ArtinGenerator::try_new(1, Sign::Positive).unwrap();
    ///
    /// assert_eq!(artin_generator.head(), Strand::try_new(2).unwrap());
    /// ```
    #[tracing::instrument(level = "debug")]
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
    /// let artin_generator = ArtinGenerator::try_new(1, Sign::Positive).unwrap();
    ///
    /// assert_eq!(artin_generator.inverse(), ArtinGenerator::try_new(1, Sign::Negative).unwrap());
    /// ```
    #[tracing::instrument(level = "info")]
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
    /// let artin_generator = ArtinGenerator::try_new(1, Sign::Positive).unwrap();
    ///
    /// assert_eq!(artin_generator.minimal_required_braid_index(), BraidIndex::try_new(2).unwrap());
    /// ```
    #[tracing::instrument(level = "info")]
    pub fn minimal_required_braid_index(&self) -> BraidIndex {
        BraidIndex::try_new(self.head()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ArtinGenerator, ArtinValidationError, BandGenerator, BraidIndex, Letter, Sign, Strand,
    };
    use googletest::matchers::{anything, derefs_to, each, eq, err, ok};
    use googletest::{assert_that, expect_that, gtest};

    #[test]
    fn valid_inputs_yield_successful_construction() {
        let valid_artin_generators = [
            ArtinGenerator::try_new(1, Sign::Negative),
            ArtinGenerator::try_new(2_usize, Sign::Positive),
            ArtinGenerator::try_new(3_isize, Sign::Negative),
            ArtinGenerator::try_from_band(BandGenerator::try_new(4, 5, Sign::Positive).unwrap()),
            ArtinGenerator::try_from_letter(
                Letter::try_new(6, None::<u16>, Sign::Negative).unwrap(),
            ),
            ArtinGenerator::try_from_letter(Letter::try_new(7, Some(8), Sign::Positive).unwrap()),
            ArtinGenerator::try_new(Strand::try_new(8).unwrap(), Sign::Negative),
            ArtinGenerator::try_new(u16::MAX - 1, Sign::Positive),
        ];
        assert_that!(&valid_artin_generators, each(derefs_to(ok(anything()))));
    }

    #[gtest]
    fn properties_compute_as_expected() {
        let input_data = [
            (1, Sign::Negative),
            (10, Sign::Positive),
            (u16::MAX - 1, Sign::Negative),
        ];

        for (foot, sign) in input_data {
            let artin_generator = ArtinGenerator::try_new(foot, sign).unwrap();
            expect_that!(artin_generator.foot(), eq(Strand::try_new(foot).unwrap()));
            expect_that!(artin_generator.sign(), eq(sign));
            expect_that!(
                artin_generator.head(),
                eq(Strand::try_new(foot + 1).unwrap())
            );
            expect_that!(
                artin_generator.inverse(),
                eq(ArtinGenerator::try_new(foot, -sign).unwrap())
            );
            expect_that!(
                artin_generator.minimal_required_braid_index(),
                eq(BraidIndex::try_new(foot + 1).unwrap())
            );
        }
    }

    #[gtest]
    fn invalid_inputs_yield_failed_construction() {
        let invalid_artin_generators = [
            (
                ArtinGenerator::try_new(-1, Sign::Positive),
                ArtinValidationError::from(Strand::try_new(-1).err().unwrap()),
            ),
            (
                ArtinGenerator::try_new(0, Sign::Negative),
                ArtinValidationError::from(Strand::try_new(0).err().unwrap()),
            ),
            (
                ArtinGenerator::try_new(u16::MAX as u32 + 1, Sign::Positive),
                ArtinValidationError::from(Strand::try_new(u16::MAX as u32 + 1).err().unwrap()),
            ),
            (
                ArtinGenerator::try_new(u16::MAX, Sign::Negative),
                ArtinValidationError::InvalidHead,
            ),
            (
                ArtinGenerator::try_from_band(
                    BandGenerator::try_new(1, 3, Sign::Positive).unwrap(),
                ),
                ArtinValidationError::FromBand(
                    BandGenerator::try_new(1, 3, Sign::Positive).unwrap(),
                ),
            ),
            (
                ArtinGenerator::try_from_letter(
                    Letter::try_new(2, Some(4), Sign::Negative).unwrap(),
                ),
                ArtinValidationError::FromBand(
                    BandGenerator::try_new(2, 4, Sign::Negative).unwrap(),
                ),
            ),
        ];

        for (invalid_artin_generator, error) in invalid_artin_generators {
            expect_that!(*invalid_artin_generator, err(eq(error)));
        }
    }
}
