use crate::{
    ArtinGenerator, BandGenerator, BraidIndex, LetterResult, LetterValidationError, Sign, Strand,
    StrandValidationError,
};

/// Represents a single letter of a braid word, from an opaque generating set.
///
/// [`Letter`] provides a layer of abstraction around the underlying generator types
/// [`ArtinGenerator`] and [`BandGenerator`], allowing either type of generator to be used
/// interchangeably in contexts where the differences between the two don't matter.
///
/// # Construction
///
/// <div class="warning">
///
/// Also see the macro [`letter!`](crate::letter) for a more ergonomic way of constructing
/// [`Letter`].
///
/// </div>
///
/// The associated function [`Letter::try_new`] exposes a flexible interface for constructing either
/// variant of [`Letter`]. In particular, the argument for the head strand is an [`Option<H>`] type,
/// where `H` is any type implementing [`TryFrom<u16>`]; when [`None`] is passed for this argument,
/// a [`Letter::Artin`] variant is constructed, whereas if a [`Some`] is passed then a
/// [`Letter::Band`] variant is constructed.
///
/// <div class="warning">
///
/// The return type is [`LetterResult`](crate::LetterResult), which is a new-type wrapper around
/// [`Result<Letter, LetterValidationError>`]. Use the dereference operator "*" for easy access to
/// the inner value.
///
/// </div>
///
/// ```
/// use braided::{ArtinGenerator, BandGenerator, Letter, Sign, Strand};
/// use std::assert_matches;
///
/// let artin_generator = ArtinGenerator::try_new(1, Sign::Positive).unwrap();
/// let band_generator = BandGenerator::try_new(2, 4, Sign::Negative).unwrap();
///
/// // The head type must be disambiguated when defining an Artin generator via `try_new`:
/// assert_eq!(
///     *Letter::try_new(1, None::<u16>, Sign::Positive),
///     Ok(Letter::Artin(artin_generator)),
/// );
///
/// assert_eq!(
///     *Letter::try_new(2, Some(4), Sign::Negative),
///     Ok(Letter::Band(band_generator)),
/// );
///
/// // You can pass anything that coerces to a `u16` to `new`:
/// assert_matches!(
///     *Letter::try_new(6isize, Some(Strand::try_new(7).unwrap()), Sign::Positive),
///     Ok(Letter::Band(_))
/// );
/// ```
///
/// [`Letter::try_new`] is fallible; for more information on the possible failure causes, see the
/// documentation for the associated error type [`LetterValidationError`].
///
/// It is also possible to convert directly from a generator using [`Letter::from`]:
///
/// ```
/// use braided::{ArtinGenerator, BandGenerator, Letter, Sign, Strand};
///
/// let artin_generator = ArtinGenerator::try_new(1, Sign::Positive).unwrap();
/// let band_generator = BandGenerator::try_new(2, 4, Sign::Negative).unwrap();
///
/// assert_eq!(
///     Letter::from(artin_generator),
///     Letter::Artin(artin_generator),
/// );
///
/// assert_eq!(
///     Letter::from(band_generator),
///     Letter::Band(band_generator),
/// );
/// ```
///
/// # Equality and Multiplication
///
/// [`Letter`] implements [`PartialEq`] in such a way that letters wrapping different generator
/// types are faithfully compared for equality:
///
/// ```
/// use braided::{Letter, Sign};
///
/// // Equality comparisons are reflexive and symmetric:
/// let artin_letter = Letter::try_new(1, None::<u16>, Sign::Positive).unwrap();
/// let band_letter = Letter::try_new(1, Some(2), Sign::Positive).unwrap();
///
/// assert!(artin_letter == artin_letter);
/// assert!(band_letter == band_letter);
/// assert!(artin_letter == band_letter);
/// assert!(band_letter == artin_letter);
/// ```
///
/// Additionally, [`Letter`] also implements the [`std::ops::Mul`] trait, giving the basis for braid
/// arithmetic in `braided`. In general, the output of multiplying two letters is a
/// [`WordResult`](crate::WordResult), and will work regardless of the underlying generator variants of
/// the letters:
/// ```
/// use braided::{Letter, Sign, Word};
///
/// let l1 = Letter::try_new(1, None::<u16>, Sign::Positive).unwrap();
/// let l2 = Letter::try_new(2, Some(4), Sign::Negative).unwrap();
///
/// let product_data = [vec![l1, l1], vec![l1, l2], vec![l2, l1], vec![l2, l2]];
///
/// for pair in product_data {
///     assert_eq!(pair[0] * pair[1], Word::try_from_letters(&pair));
/// }
/// ```
///
/// Beware that multiplication of two letters is fallible, with failure possible in the case of two
/// [band letters](Letter::Band) whose total [Artin length](Letter::artin_length) exceeds the
/// maximum value [`u16::MAX`]:
///
/// ```
/// use braided::{Letter, Sign, WordValidationError};
///
/// // l1 is an Artin generator, so its Artin length is 1
/// let l1 = Letter::try_new(1, None::<u16>, Sign::Positive).unwrap();
/// // l2 has height 2e15, hence its Artin length is u16::MAX = 2e16 - 1:
/// let l2 = Letter::try_new(1, Some(2u16.pow(15) + 1), Sign::Negative).unwrap();
///
/// assert_eq!(l1.artin_length(), 1);
/// assert_eq!(l2.artin_length(), u16::MAX);
///
/// // The Artin length of the product thus exceeds the maximal value `u16::MAX`, yielding failure
/// assert_eq!(*(l1 * l2), Err(WordValidationError::TooLong(u16::MAX as usize + 1)));
/// assert_eq!(*(l2 * l1), Err(WordValidationError::TooLong(u16::MAX as usize + 1)));
/// ```
///
/// # Computable Properties
///
/// [`Letter`] exposes an interface to the various properties common to both [`ArtinGenerator`] and
/// [`BandGenerator`]:
///
/// ```
/// use braided::{BraidIndex, Letter, Sign, Strand};
/// let input_data = [
///     (1, None::<u16>, Sign::Positive),
///     (3, Some(5), Sign::Negative),
///     (6, Some(7), Sign::Positive),
/// ];
///
/// for (foot, head, sign) in input_data {
///     let letter = Letter::try_new(foot, head, sign).unwrap();
///
///     assert_eq!(letter.sign(), sign); // [+, -, +]
///     assert_eq!(letter.foot(), Strand::try_new(foot).unwrap()); // [1, 3, 6]
///     assert_eq!(
///         letter.head(),
///         Strand::try_new(head.unwrap_or(foot + 1)).unwrap()
///     ); // [2, 5, 7]
///     assert_eq!(
///         letter.inverse(),
///         Letter::try_new(foot, head, -sign).unwrap()
///     ); // [[1; -], [3 => 5; +], [6 => 7; -]]
///     assert_eq!(letter.is_artin(), head.unwrap_or(foot + 1) - foot == 1); // [T, F, T]
///     assert_eq!(letter.height(), head.unwrap_or(foot + 1) - foot); // [1, 2, 1]
///     assert_eq!(
///         letter.artin_length(),
///         2 * (head.unwrap_or(foot + 1) - foot) - 1
///     ); // [1, 3, 1]
///     assert_eq!(
///         letter.minimal_required_braid_index(),
///         BraidIndex::try_new(head.unwrap_or(foot + 1)).unwrap(),
///     ); // [2, 5, 7]
/// }
/// ```
#[derive(Debug, Clone, Copy, Eq)]
pub enum Letter {
    /// [`Letter`] variant wrapping an [`ArtinGenerator`].
    Artin(ArtinGenerator),
    /// [`Letter`] variant wrapping a [`BandGenerator`].
    Band(BandGenerator),
}

impl Letter {
    /// Attempts to construct a new [`Letter`] given data for the foot, head (optional), and sign.
    ///
    /// If [`None`] is passed for the second argument, then a [`Letter::Artin`] variant is
    /// constructed, while if a [`Some`] is passed, a [`Letter::Band`] variant is constructed.
    ///
    /// <div class="warning">
    ///
    /// The return type is [`LetterResult`](crate::LetterResult), which is a new-type wrapper around
    /// [`Result<Letter, LetterValidationError>`]. Use the dereference operator "*" for easy access to
    /// the inner value.
    ///
    /// </div>
    ///
    /// # Examples
    ///
    /// ```
    /// use braided::{ArtinGenerator, BandGenerator, Letter, Sign, Strand};
    /// use std::assert_matches;
    ///
    /// let artin_generator = ArtinGenerator::try_new(1, Sign::Positive).unwrap();
    /// let band_generator = BandGenerator::try_new(2, 4, Sign::Negative).unwrap();
    ///
    /// // The head type must be disambiguated when defining an Artin generator via `try_new`:
    /// assert_eq!(
    ///     *Letter::try_new(1, None::<u16>, Sign::Positive),
    ///     Ok(Letter::Artin(artin_generator)),
    /// );
    ///
    /// assert_eq!(
    ///     *Letter::try_new(2, Some(4), Sign::Negative),
    ///     Ok(Letter::Band(band_generator)),
    /// );
    ///
    /// // You can pass anything that coerces to a `u16` to `new`:
    /// assert_matches!(
    ///     *Letter::try_new(6isize, Some(Strand::try_new(7).unwrap()), Sign::Positive),
    ///     Ok(Letter::Band(_))
    /// );
    /// ```
    ///
    /// # Errors
    ///
    /// See the documentation for [`LetterValidationError`] for more details and examples concerning
    /// the possible error cases.
    #[tracing::instrument(level = "info")]
    pub fn try_new<F, H>(foot: F, head: Option<H>, sign: Sign) -> LetterResult
    where
        F: TryInto<u16> + std::fmt::Debug,
        H: TryInto<u16> + std::fmt::Debug,
        StrandValidationError: From<<F as TryInto<u16>>::Error>
            + From<<H as TryInto<u16>>::Error>
            + From<std::convert::Infallible>,
    {
        match head {
            Some(head) => match *BandGenerator::try_new(foot, head, sign) {
                Ok(band) => LetterResult::from(Self::Band(band)),
                Err(e) => LetterResult::from(LetterValidationError::from(e)),
            },
            None => match *ArtinGenerator::try_new(foot, sign) {
                Ok(artin) => LetterResult::from(Self::Artin(artin)),
                Err(e) => LetterResult::from(LetterValidationError::from(e)),
            },
        }
    }

    /// Decompose the [`Letter`] into a vector of [`Letter::Artin`].
    ///
    /// This method does nothing to a letter which is already a [`Letter::Artin`] variant. For
    /// details on how a [`Letter::Band`] variant is decomposed, see the documentation for
    /// [`BandGenerator`].
    ///
    /// # Examples
    ///
    /// ```
    /// use braided::{Letter, Sign};
    ///
    /// let artin_letter = Letter::try_new(1, None::<u16>, Sign::Positive).unwrap();
    /// let band_letter = Letter::try_new(2, Some(5), Sign::Negative).unwrap();
    ///
    /// assert_eq!(artin_letter.decompose(), vec![artin_letter]);
    /// assert_eq!(
    ///     band_letter.decompose(),
    ///     vec![
    ///         Letter::try_new(2, None::<u16>, Sign::Negative).unwrap(),
    ///         Letter::try_new(3, None::<u16>, Sign::Negative).unwrap(),
    ///         Letter::try_new(4, None::<u16>, Sign::Negative).unwrap(),
    ///         Letter::try_new(3, None::<u16>, Sign::Positive).unwrap(),
    ///         Letter::try_new(2, None::<u16>, Sign::Positive).unwrap(),
    ///     ],
    ///     );
    /// ```
    #[tracing::instrument(level = "info")]
    pub fn decompose(&self) -> Vec<Self> {
        match self {
            Letter::Artin(_) => vec![*self],
            Letter::Band(band) => band.decompose().iter().map(|&a| Letter::Artin(a)).collect(),
        }
    }

    /// Returns the [sign](Sign) of the [`Letter`].
    ///
    /// # Example
    ///
    /// ```
    /// use braided::{Letter, Sign};
    /// let input_data = [
    ///     (1, None::<u16>, Sign::Positive),
    ///     (3, Some(5), Sign::Negative),
    ///     (6, Some(7), Sign::Positive),
    /// ];
    ///
    /// for (foot, head, sign) in input_data {
    ///     let letter = Letter::try_new(foot, head, sign).unwrap();
    ///
    ///     assert_eq!(letter.sign(), sign); // [+, -, +]
    /// }
    /// ```
    #[tracing::instrument(level = "debug")]
    pub fn sign(&self) -> Sign {
        match self {
            Self::Artin(artin) => artin.sign(),
            Self::Band(band) => band.sign(),
        }
    }
    /// Returns the [foot strand](Strand) of the [`Letter`].
    ///
    /// # Example
    ///
    /// ```
    /// use braided::{Letter, Sign, Strand};
    /// let input_data = [
    ///     (1, None::<u16>, Sign::Positive),
    ///     (3, Some(5), Sign::Negative),
    ///     (6, Some(7), Sign::Positive),
    /// ];
    ///
    /// for (foot, head, sign) in input_data {
    ///     let letter = Letter::try_new(foot, head, sign).unwrap();
    ///
    ///     assert_eq!(letter.foot(), Strand::try_new(foot).unwrap()); // [1, 3, 6]
    /// }
    /// ```
    #[tracing::instrument(level = "debug")]
    pub fn foot(&self) -> Strand {
        match self {
            Self::Artin(artin) => artin.foot(),
            Self::Band(band) => band.foot(),
        }
    }
    /// Returns the [head strand](Strand) of the [`Letter`].
    ///
    /// # Examples
    ///
    /// ```
    /// use braided::{Letter, Sign, Strand};
    /// let input_data = [
    ///     (1, None::<u16>, Sign::Positive),
    ///     (3, Some(5), Sign::Negative),
    ///     (6, Some(7), Sign::Positive),
    /// ];
    ///
    /// for (foot, head, sign) in input_data {
    ///     let letter = Letter::try_new(foot, head, sign).unwrap();
    ///
    ///     assert_eq!(
    ///         letter.head(),
    ///         Strand::try_new(head.unwrap_or(foot + 1)).unwrap()
    ///     ); // [2, 5, 7]
    /// }
    /// ```
    #[tracing::instrument(level = "debug")]
    pub fn head(&self) -> Strand {
        match self {
            Self::Artin(artin) => artin.head(),
            Self::Band(band) => band.head(),
        }
    }

    /// Returns the inverse of the [`Letter`], which amounts to reversing its [sign](Sign).
    ///
    /// # Examples
    ///
    /// ```
    /// use braided::{Letter, Sign};
    /// let input_data = [
    ///     (1, None::<u16>, Sign::Positive),
    ///     (3, Some(5), Sign::Negative),
    ///     (6, Some(7), Sign::Positive),
    /// ];
    ///
    /// for (foot, head, sign) in input_data {
    ///     let letter = Letter::try_new(foot, head, sign).unwrap();
    ///
    ///     assert_eq!(
    ///         letter.inverse(),
    ///         Letter::try_new(foot, head, -sign).unwrap()
    ///     ); // [[1; -], [3 => 5; +], [6 => 7; -]]
    /// }
    /// ```
    #[tracing::instrument(level = "info")]
    pub fn inverse(&self) -> Self {
        match self {
            Self::Artin(artin) => Self::Artin(artin.inverse()),
            Self::Band(band) => Self::Band(band.inverse()),
        }
    }

    /// Returns a bool indicating whether or not the [`Letter`] can be used in contexts where an
    /// Artin generator is expected.
    ///
    /// Equivalent to [Letter::height] equaling 1.
    ///
    /// # Examples
    ///
    /// ```
    /// use braided::{Letter, Sign};
    /// let input_data = [
    ///     (1, None::<u16>, Sign::Positive),
    ///     (3, Some(5), Sign::Negative),
    ///     (6, Some(7), Sign::Positive),
    /// ];
    ///
    /// for (foot, head, sign) in input_data {
    ///     let letter = Letter::try_new(foot, head, sign).unwrap();
    ///
    ///     assert_eq!(letter.is_artin(), head.unwrap_or(foot + 1) - foot == 1); // [T, F, T]
    /// }
    /// ```
    #[tracing::instrument(level = "info")]
    pub fn is_artin(&self) -> bool {
        match self {
            Self::Artin(_) => true,
            Self::Band(band) => band.is_artin(),
        }
    }

    /// Returns the distance from the [letter's](Letter) [foot strand](`Letter::foot`) to its
    /// [head strand](`Letter::head`).
    ///
    /// # Examples
    ///
    /// ```
    /// use braided::{Letter, Sign};
    /// let input_data = [
    ///     (1, None::<u16>, Sign::Positive),
    ///     (3, Some(5), Sign::Negative),
    ///     (6, Some(7), Sign::Positive),
    /// ];
    ///
    /// for (foot, head, sign) in input_data {
    ///     let letter = Letter::try_new(foot, head, sign).unwrap();
    ///
    ///     assert_eq!(letter.height(), head.unwrap_or(foot + 1) - foot); // [1, 2, 1]
    /// }
    /// ```
    #[tracing::instrument(level = "info")]
    pub fn height(&self) -> u16 {
        match self {
            Self::Artin(_) => 1,
            Self::Band(band) => band.height(),
        }
    }
    /// Returns the length of the [letter](Letter) in [Artin generators](ArtinGenerator).
    ///
    /// # Examples
    ///
    /// ```
    /// use braided::{Letter, Sign};
    /// let input_data = [
    ///     (1, None::<u16>, Sign::Positive),
    ///     (3, Some(5), Sign::Negative),
    ///     (6, Some(7), Sign::Positive),
    /// ];
    ///
    /// for (foot, head, sign) in input_data {
    ///     let letter = Letter::try_new(foot, head, sign).unwrap();
    ///
    ///     assert_eq!(
    ///         letter.artin_length(),
    ///         2 * (head.unwrap_or(foot + 1) - foot) - 1
    ///     ); // [1, 3, 1]
    /// }
    /// ```
    #[tracing::instrument(level = "info")]
    pub fn artin_length(&self) -> u16 {
        match self {
            Self::Artin(_) => 1,
            Self::Band(band) => band.artin_length(),
        }
    }
    /// Returns the minimal [braid index](BraidIndex) required to use use the [letter](Letter).
    ///
    /// # Examples
    ///
    /// ```
    /// use braided::{BraidIndex, Letter, Sign};
    /// let input_data = [
    ///     (1, None::<u16>, Sign::Positive),
    ///     (3, Some(5), Sign::Negative),
    ///     (6, Some(7), Sign::Positive),
    /// ];
    ///
    /// for (foot, head, sign) in input_data {
    ///     let letter = Letter::try_new(foot, head, sign).unwrap();
    ///
    ///     assert_eq!(
    ///         letter.minimal_required_braid_index(),
    ///         BraidIndex::try_new(head.unwrap_or(foot + 1)).unwrap(),
    ///     ); // [2, 5, 7]
    /// }
    /// ```
    #[tracing::instrument(level = "info")]
    pub fn minimal_required_braid_index(&self) -> BraidIndex {
        match self {
            Self::Artin(artin) => artin.minimal_required_braid_index(),
            Self::Band(band) => band.minimal_required_braid_index(),
        }
    }
}

impl From<ArtinGenerator> for Letter {
    #[tracing::instrument(level = "debug")]
    fn from(value: ArtinGenerator) -> Self {
        Self::Artin(value)
    }
}
impl From<BandGenerator> for Letter {
    #[tracing::instrument(level = "debug")]
    fn from(value: BandGenerator) -> Self {
        Self::Band(value)
    }
}

impl std::cmp::PartialEq for Letter {
    #[tracing::instrument(level = "debug")]
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Artin(lhs), Self::Artin(rhs)) => lhs == rhs,
            (Self::Artin(lhs), Self::Band(rhs)) => {
                rhs.is_artin() && *lhs == ArtinGenerator::try_new(rhs.foot(), rhs.sign()).unwrap()
            }
            (Self::Band(lhs), Self::Artin(rhs)) => {
                lhs.is_artin() && ArtinGenerator::try_new(lhs.foot(), lhs.sign()).unwrap() == *rhs
            }
            (Self::Band(lhs), Self::Band(rhs)) => lhs == rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ArtinGenerator, BandGenerator, BraidIndex, Letter, LetterValidationError, Sign, Strand,
    };
    use googletest::matchers::{anything, derefs_to, each, eq, err, is_false, is_true, ok};
    use googletest::{assert_that, expect_that, gtest};

    #[test]
    fn valid_inputs_yield_successful_construction() {
        let valid_letters = [
            Letter::try_new(1, None::<u16>, Sign::Positive),
            Letter::try_new(2, Some(4), Sign::Negative),
            Letter::try_new(Strand::try_new(5).unwrap(), None::<u16>, Sign::Negative),
            Letter::try_new(6isize, Some(Strand::try_new(7).unwrap()), Sign::Positive),
            Letter::try_new(
                Strand::try_new(8).unwrap(),
                Some(Strand::try_new(30).unwrap()),
                Sign::Negative,
            ),
            Letter::try_new(u16::MAX as u32 - 1, None::<u16>, Sign::Positive),
            Letter::try_new(1, Some(2u16.pow(15) + 1), Sign::Negative),
        ];

        assert_that!(&valid_letters, each(derefs_to(ok(anything()))));
    }

    #[gtest]
    fn conversion_from_generators_works_as_expected() {
        let letter_from_artin = Letter::from(ArtinGenerator::try_new(1, Sign::Positive).unwrap());
        let letter_from_band = Letter::from(BandGenerator::try_new(3, 5, Sign::Negative).unwrap());

        expect_that!(
            letter_from_artin,
            eq(Letter::try_new(1, None::<u16>, Sign::Positive).unwrap())
        );
        expect_that!(
            letter_from_band,
            eq(Letter::try_new(3, Some(5), Sign::Negative).unwrap())
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
            let letter = Letter::try_new(foot, head, sign).unwrap();

            expect_that!(letter.sign(), eq(sign));
            expect_that!(letter.foot(), eq(Strand::try_new(foot).unwrap()));
            expect_that!(
                letter.head(),
                eq(Strand::try_new(head.unwrap_or(foot + 1)).unwrap())
            );
            expect_that!(
                letter.inverse(),
                eq(Letter::try_new(foot, head, -sign).unwrap())
            );
            expect_that!(letter.is_artin(), eq(head.unwrap_or(foot + 1) - foot == 1));
            expect_that!(letter.height(), eq(head.unwrap_or(foot + 1) - foot));
            expect_that!(
                letter.artin_length(),
                eq(2 * (head.unwrap_or(foot + 1) - foot) - 1)
            );
            expect_that!(
                letter.minimal_required_braid_index(),
                eq(BraidIndex::try_new(head.unwrap_or(foot + 1)).unwrap()),
            );
        }
    }

    #[gtest]
    fn equality_comparison_behaves_as_expected() {
        let artin_letter = Letter::try_new(1, None::<u16>, Sign::Positive).unwrap();
        let other_artin_letter = Letter::try_new(2, None::<u16>, Sign::Positive).unwrap();
        let band_letter = Letter::try_new(1, Some(2), Sign::Positive).unwrap();
        let other_band_letter = Letter::try_new(1, Some(2), Sign::Negative).unwrap();

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
    fn invalid_inputs_yield_failed_construction() {
        let invalid_letters = [
            (
                Letter::try_new(0, None::<u16>, Sign::Positive),
                LetterValidationError::from(
                    ArtinGenerator::try_new(0, Sign::Positive).err().unwrap(),
                ),
            ),
            (
                Letter::try_new(4, Some(1), Sign::Negative),
                LetterValidationError::from(
                    BandGenerator::try_new(4, 1, Sign::Negative).err().unwrap(),
                ),
            ),
        ];

        for (invalid_letter, error) in invalid_letters {
            expect_that!(*invalid_letter, err(eq(error)));
        }
    }
}
