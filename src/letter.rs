use crate::{
    ArtinGenerator, ArtinValidationError, BandGenerator, BandValidationError, BraidIndex, Sign,
    Strand, StrandValidationError,
};

/// Represents potential failures from attempted construction of [`Letter`] using [`Letter::new`].
///
/// As a [`Letter`] is a wrapper around either an [`ArtinGenerator`] or a [`BandGenerator`], so too
/// does [`LetterValidationError`] transparently wrap a [`ArtinValidationError`] or a
/// [`BandValidationError`].
///
/// # Examples
///
/// ```
/// use braided::{ArtinValidationError, BandValidationError, Letter, LetterValidationError, Sign};
/// use std::assert_matches;
///
/// let failed_artin_letter = Letter::new(0, None::<u16>, Sign::Positive);
/// assert_matches!(
///     failed_artin_letter,
///     Err(LetterValidationError::ArtinValidation(ArtinValidationError::StrandValidation(_))),
/// );
///
/// let failed_band_letter = Letter::new(4, Some(1), Sign::Positive);
/// assert_matches!(
///     failed_band_letter,
///     Err(LetterValidationError::BandValidation(BandValidationError::FootOverHead { .. })),
/// );
/// ```
#[derive(Debug, thiserror::Error, PartialEq, Eq, Clone, Copy)]
pub enum LetterValidationError {
    /// Indicates failed attempt to construct an [`ArtinGenerator`].
    ///
    /// Transparent wrapper around [`ArtinValidationError`].
    #[error(transparent)]
    ArtinValidation(#[from] ArtinValidationError),
    /// Indicates failed attempt to construct a [`BandGenerator`].
    ///
    /// Transparent wrapper around [`BandValidationError`].
    #[error(transparent)]
    BandValidation(#[from] BandValidationError),
}

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
/// The associated function [`Letter::new`] exposes a flexible interface for constructing either
/// variant of [`Letter`]. In particular, the argument for the head strand is an [`Option<H>`] type,
/// where `H` is any type implementing [`TryFrom<u16>`]; when [`None`] is passed for this argument,
/// a [`Letter::Artin`] variant is constructed, whereas if a [`Some`] is passed then a
/// [`Letter::Band`] variant is constructed.
///
/// ```
/// use braided::{ArtinGenerator, BandGenerator, Letter, Sign, Strand};
/// use std::assert_matches;
///
/// let artin_generator = ArtinGenerator::new(1, Sign::Positive).unwrap();
/// let band_generator = BandGenerator::new(2, 4, Sign::Negative).unwrap();
///
/// // The head type must be disambiguated when defining an Artin generator via `new`:
/// assert_eq!(
///     Letter::new(1, None::<u16>, Sign::Positive),
///     Ok(Letter::Artin(artin_generator)),
/// );
///
/// assert_eq!(
///     Letter::new(2, Some(4), Sign::Negative),
///     Ok(Letter::Band(band_generator)),
/// );
///
/// // You can pass anything that coerces to a `u16` to `new`:
/// assert_matches!(
///     Letter::new(6isize, Some(Strand::new(7).unwrap()), Sign::Positive),
///     Ok(Letter::Band(_))
/// );
/// ```
///
/// [`Letter::new`] is fallible; for more information on the possible failure causes, see the
/// documentation for the associated error type [`LetterValidationError`].
///
/// As an even more ergonomic means of constructing a [`Letter`], see the [`letter!`](crate::letter)
/// macro.
///
/// It is also possible to convert directly from a generator using [`Letter::from`]:
///
/// ```
/// use braided::{ArtinGenerator, BandGenerator, Letter, Sign, Strand};
///
/// let artin_generator = ArtinGenerator::new(1, Sign::Positive).unwrap();
/// let band_generator = BandGenerator::new(2, 4, Sign::Negative).unwrap();
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
/// let artin_letter = Letter::new(1, None::<u16>, Sign::Positive).unwrap();
/// let band_letter = Letter::new(1, Some(2), Sign::Positive).unwrap();
///
/// assert!(artin_letter == artin_letter);
/// assert!(band_letter == band_letter);
/// assert!(artin_letter == band_letter);
/// assert!(band_letter == artin_letter);
/// ```
///
/// Additionally, [`Letter`] also implements the [`std::ops::Mul`] trait, giving the basis for braid
/// arithmetic in `braided`. In general, the output of multiplying two letters is a
/// [`Result<Word, ...>`](Word), and will work regardless of the underlying generator variants of
/// the letters:
/// ```
/// use braided::{Letter, Sign, Word};
///
/// let l1 = Letter::new(1, None::<u16>, Sign::Positive).unwrap();
/// let l2 = Letter::new(2, Some(4), Sign::Negative).unwrap();
///
/// let product_data = [vec![l1, l1], vec![l1, l2], vec![l2, l1], vec![l2, l2]];
///
/// for pair in product_data {
///     assert_eq!(pair[0] * pair[1], Word::try_from(pair));
/// }
/// ```
///
/// Inverses behave as the name suggests:
///
/// ```
/// use braided::{Letter, Sign, Word};
///
/// let l1 = Letter::new(1, None::<u16>, Sign::Positive).unwrap();
/// let l2 = Letter::new(2, Some(4), Sign::Negative).unwrap();
/// let l3 = Letter::new(1, Some(2), Sign::Negative).unwrap();
///
/// let product_data = [
///     [l1, l1.inverse()],
///     [l1.inverse(), l1],
///     [l2, l2.inverse()],
///     [l2.inverse(), l2],
///     [l1, l3],
///     [l3, l1],
/// ];
///
/// for pair in product_data {
///     assert_eq!(pair[0] * pair[1], Ok(Word::trivial()));
/// }
/// ```
///
/// Beware that multiplication of two letters is fallible, with failure possible in the case of two
/// [band letters](Letter::Band) whose total [Artin length](Letter::artin_length) exceeds the
/// maximum value [`u16::MAX`]:
///
/// ```
/// use braided::{Letter, Sign};
/// use std::assert_matches;
///
/// // l1 is an Artin generator, so its Artin length is 1
/// let l1 = Letter::new(1, None::<u16>, Sign::Positive).unwrap();
/// // l2 has height 2e15, hence its Artin length is u16::MAX = 2e16 - 1:
/// let l2 = Letter::new(1, Some(2u16.pow(15) + 1), Sign::Negative).unwrap();
///
/// assert_eq!(l1.artin_length(), 1);
/// assert_eq!(l2.artin_length(), u16::MAX);
///
/// // The Artin length of the product thus exceeds the maximal value `u16::MAX`, yielding failure
/// assert_matches!(l1 * l2, Err(_));
/// assert_matches!(l2 * l1, Err(_));
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
///     let letter = Letter::new(foot, head, sign).unwrap();
///
///     assert_eq!(letter.sign(), sign); // [+, -, +]
///     assert_eq!(letter.foot(), Strand::new(foot).unwrap()); // [1, 3, 6]
///     assert_eq!(
///         letter.head(),
///         Strand::new(head.unwrap_or(foot + 1)).unwrap()
///     ); // [2, 5, 7]
///     assert_eq!(
///         letter.inverse(),
///         Letter::new(foot, head, -sign).unwrap()
///     ); // [[1; -], [3 => 5; +], [6 => 7; -]]
///     assert_eq!(letter.is_artin(), head.unwrap_or(foot + 1) - foot == 1); // [T, F, T]
///     assert_eq!(letter.height(), head.unwrap_or(foot + 1) - foot); // [1, 2, 1]
///     assert_eq!(
///         letter.artin_length(),
///         2 * (head.unwrap_or(foot + 1) - foot) - 1
///     ); // [1, 3, 1]
///     assert_eq!(
///         letter.minimal_required_braid_index(),
///         BraidIndex::new(head.unwrap_or(foot + 1)).unwrap(),
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
    /// Constructs a new [`Letter`], or fails with a [`LetterValidationError`] if the construction
    /// cannot be validated.
    ///
    /// If [`None`] is passed for the second argument, then a [`Letter::Artin`] variant is
    /// constructed, while if a [`Some`] is passed, a [`Letter::Band`] variant is constructed.
    ///
    /// # Examples
    ///
    /// ```
    /// use braided::{ArtinGenerator, BandGenerator, Letter, Sign, Strand};
    /// use std::assert_matches;
    ///
    /// let artin_generator = ArtinGenerator::new(1, Sign::Positive).unwrap();
    /// let band_generator = BandGenerator::new(2, 4, Sign::Negative).unwrap();
    ///
    /// // The head type must be disambiguated when defining an Artin generator via `new`:
    /// assert_eq!(
    ///     Letter::new(1, None::<u16>, Sign::Positive),
    ///     Ok(Letter::Artin(artin_generator)),
    /// );
    ///
    /// assert_eq!(
    ///     Letter::new(2, Some(4), Sign::Negative),
    ///     Ok(Letter::Band(band_generator)),
    /// );
    ///
    /// // You can pass anything that coerces to a `u16` to `new`:
    /// assert_matches!(
    ///     Letter::new(6isize, Some(Strand::new(7).unwrap()), Sign::Positive),
    ///     Ok(Letter::Band(_))
    /// );
    /// ```
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
    ///     let letter = Letter::new(foot, head, sign).unwrap();
    ///
    ///     assert_eq!(letter.sign(), sign); // [+, -, +]
    /// }
    /// ```
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
    ///     let letter = Letter::new(foot, head, sign).unwrap();
    ///
    ///     assert_eq!(letter.foot(), Strand::new(foot).unwrap()); // [1, 3, 6]
    /// }
    /// ```
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
    ///     let letter = Letter::new(foot, head, sign).unwrap();
    ///
    ///     assert_eq!(
    ///         letter.head(),
    ///         Strand::new(head.unwrap_or(foot + 1)).unwrap()
    ///     ); // [2, 5, 7]
    /// }
    /// ```
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
    ///     let letter = Letter::new(foot, head, sign).unwrap();
    ///
    ///     assert_eq!(
    ///         letter.inverse(),
    ///         Letter::new(foot, head, -sign).unwrap()
    ///     ); // [[1; -], [3 => 5; +], [6 => 7; -]]
    /// }
    /// ```
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
    ///     let letter = Letter::new(foot, head, sign).unwrap();
    ///
    ///     assert_eq!(letter.is_artin(), head.unwrap_or(foot + 1) - foot == 1); // [T, F, T]
    /// }
    /// ```
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
    ///     let letter = Letter::new(foot, head, sign).unwrap();
    ///
    ///     assert_eq!(letter.height(), head.unwrap_or(foot + 1) - foot); // [1, 2, 1]
    /// }
    /// ```
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
    ///     let letter = Letter::new(foot, head, sign).unwrap();
    ///
    ///     assert_eq!(
    ///         letter.artin_length(),
    ///         2 * (head.unwrap_or(foot + 1) - foot) - 1
    ///     ); // [1, 3, 1]
    /// }
    /// ```
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
    ///     let letter = Letter::new(foot, head, sign).unwrap();
    ///
    ///     assert_eq!(
    ///         letter.minimal_required_braid_index(),
    ///         BraidIndex::new(head.unwrap_or(foot + 1)).unwrap(),
    ///     ); // [2, 5, 7]
    /// }
    /// ```
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
                lhs.is_artin() && ArtinGenerator::new(lhs.foot(), lhs.sign()).unwrap() == *rhs
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
            expect_that!(letter.height(), eq(head.unwrap_or(foot + 1) - foot));
            expect_that!(
                letter.artin_length(),
                eq(2 * (head.unwrap_or(foot + 1) - foot) - 1)
            );
            expect_that!(
                letter.minimal_required_braid_index(),
                eq(BraidIndex::new(head.unwrap_or(foot + 1)).unwrap()),
            );
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
}
