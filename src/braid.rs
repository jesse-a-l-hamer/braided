use crate::{
    BraidIndex, IndexValidationError, Letter, Sign, StrandValidationError, Word,
    WordValidationError,
};

/// Represents failure during attempt to construct a [`Braid`].
///
/// The only _infallible_ context in which a [braid](Braid) can be constructed is via the
/// [`Braid::from`] method, by passing an already-validated [`Word`] and _inferring_ the
/// [`BraidIndex`] from it. Every other constructor---including the [braid!](crate::braid) macro as
/// well as multiplication---may return a [`BraidValidationError`]. We go through the possible
/// failure cases now.
///
/// <div class="warning">
///
/// Please see the documentation for the [braid!](crate::braid) macro for more information on its
/// failure scenarios.
///
/// </div>
///
/// # Invalid Construction Using [`Braid::new`]
///
/// 1. Failure to construct an explicitly provided [`BraidIndex`]
///    ([`BraidValidationError::IndexValidation`]):
///
/// ```
/// ```
///
/// 2. An explicitly provided [`BraidIndex`] is smaller than required by the given [`Word`]
///    ([`BraidValidationError::IndexTooSmall`]):
///
/// ```
/// ```
///
/// # Invalid Construction Using [`Braid::from_data`]
///
/// 1. Failure to construct an explicitly provided [`BraidIndex`]
///    ([`BraidValidationError::IndexValidation`]):
///
/// ```
/// ```
///
/// 2. An explicitly provided [`BraidIndex`] is smaller than required by the given [`Letter`] data
///    ([`BraidValidationError::IndexTooSmall`]):
///
/// ```
/// ```
///
/// 3. Failure to construct a valid [`Word`] from the given [`Letter`] data
///    ([`BraidValidationError::WordValidation`]):
///
/// ```
/// ```
///
/// # Invalid Construction Using [`Braid::try_from`]
///
/// 1. Failure to construct a valid [`Word`] from the given [letters](Letter), (e.g., because the
///    number of [letters](Letter) provided exceeds [`u16::MAX`]; uses a
///    [`BraidValidationError::WordValidation`]).
///
/// ```
/// ```
///
/// # Invalid Construction When Using  [`Braid::trivial`]
///
/// 1. Failure to construct a valid [`BraidIndex`] ([`BraidValidationError::IndexValidation`]):
///
/// ```
/// ```
///
/// # Invalid Construction When Multiplying a [`Braid`] and a ([`Letter`], [`Word`], or [`Braid`])
///
///
/// 1. The [index](BraidIndex) of one of the [`Braid`] operands is smaller than required by some
///    [letter](Letter) of the other operand ([`BraidValidationError::IndexTooSmall`]):
///
/// ```
/// ```
///
/// 2. The [Artin length](Braid::artin_length) of the product exceeds the maximum length of
///    [`u16::MAX`] ([`BraidValidationError::WordValidation`]):
///
/// ```
/// ```
///
/// 3. Attempting to multiply two [braids](Braids) whose [braid indices](BraidIndex) are not equal
///    ([`BraidValidationError::UnequalIndices`]):
///
/// ```
/// ```
#[derive(Debug, thiserror::Error, PartialEq, Eq, Clone, Copy)]
pub enum BraidValidationError {
    /// Indicates that the [index](BraidIndex) of the [`Braid`] is not large enough to accommodate a
    /// certain [`Letter`].
    ///
    /// This variant may be returned when explicitly providing a [`BraidIndex`] to a [`Braid`]
    /// constructor, or when multiplying an existing [braid](Braid) by an offending [`Letter`],
    /// [`Word`], or [`Braid`].
    #[error("Given index {index:?} less than minimal required index {minimal_required_index:?}.")]
    IndexTooSmall {
        /// The [index](`BraidIndex`) of the inadequate braid.
        index: BraidIndex,
        /// The [index](`BraidIndex`) which is required to accommodate the offending [`Letter`].
        minimal_required_index: BraidIndex,
    },
    /// Indicates an attempt to multiply two [braids](Braid) of unequal (`index`)[BraidIndex].
    #[error("Attempt to multiply braids of unequal indices: {left:?} != {right:?}")]
    UnequalIndices {
        /// The [index](BraidIndex) of the left operand of the product.
        left: BraidIndex,
        /// The [index](BraidIndex) of the right operand of the product.
        right: BraidIndex,
    },
    /// Indicates failure to construct the [index](BraidIndex) of the [braid](Braid).
    ///
    /// Transparent wrapper around [`IndexValidationError`].
    #[error(transparent)]
    IndexValidation(#[from] IndexValidationError),
    /// Indicates failure to construct the [word](Word) of the [braid](Braid).
    ///
    /// Transparent wrapper around [`WordValidationError`].
    #[error(transparent)]
    WordValidation(#[from] WordValidationError),
}

/// The core struct of `braided`; may be thought of as describing a [weaving pattern](Word) among a
/// [fixed number](BraidIndex) of disjoint [strands](crate::Strand).
///
/// # Constructing a [`Braid`]
///
/// <div class="warning">
///
/// The most ergonomic way to construct a [`Braid`] is via the [`braid!`](crate::braid) macro,
/// though we do not discuss this macro here. Please consult the macro's docs for usage details and
/// examples.
///
/// </div>
///
/// The following account for all means of directly constructing a [`Braid`] using associated
/// functions and trait implementations on [`Braid`] itself.
///
/// 1. Using [`Braid::new`]
///
/// ```
/// ```
///
/// 2. Using [`Braid::from_data`]
///
/// ```
/// ```
///
/// 3. Using [`Braid::from`]
///
/// ```
/// ```
///
/// 4. Using [`Braid::try_from`]
///
/// ```
/// ```
///
/// 5. Using [`Braid::trivial`]
///
/// ```
/// ```
///
/// 6. Using [`Braid::default`]
///
/// ```
/// ```
///
/// # [Decomposition](Braid::decompose) and [Coalescing](Braid::coalesce)
///
/// ```
/// ```
///
/// # Convenience Traits - [`IntoIterator`], [`Deref`](std::ops::Deref), and [`AsRef`]
///
/// ```
/// ```
///
/// # Accessors and Basic Properties
///
/// The underlying data of the [`Braid`] may be accessed as follows:
///
/// ```
/// ```
///
/// One may also compute several basic [`Braid`] properties:
///
/// ```
/// ```
///
/// # Multiplication of [Braids](Braid)
///
/// The collection of all [braids](Braid) of a given [braid index](`BraidIndex`) form a mathematical
/// structure known as a [_group_](https://en.wikipedia.org/wiki/Group_(mathematics)), which means
/// that there is an associative multiplication operation between [braids](Braid), such that an
/// identity element exists (the [trivial braid](Braid::trivial) of the given [index](BraidIndex))
/// and an [inverse](Braid::inverse) with respect to the multiplication exists for every
/// [braid](Braid).
///
/// There are many different _relations_ among [braids](Braid) (i.e., equations involving the
/// [braid](Braid) multiplication) which take different forms depending on the generating set (e.g.,
/// _far commutativity_ and the _braid relations_, to name the two sets of relations that hold in
/// the standard Artin presentation of the group). Of primary importance on the roadmap of
/// `braided` is the implementation of mechanisms to detect and apply as many of these relations as
/// possible. However, as of the initial release (v0.1.0), only the bare multiplication operation
/// has been implemented.
///
/// The multiplication of two [braids](Braid) amounts to a simple concatenation of their
/// [words](Word). By default, the product is simplified as much as possible, meaning that as many
/// cancelling pairs of [letters](Letter) are removed as possible. However, note that the
/// operand [braids](Braid) are not necessarily simplified in this sense _prior_ to the
/// multiplication, so there is no guarantee that the product has no cancelling pairs.
///
/// ```
/// ```
///
/// # Errors
///
/// All of the constructors mentioned above, as well as the multiplication operation, are fallible.
/// See the documentation of the associated error type [`BraidValidationError`] for more details and
/// examples as to possible failures.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Braid {
    index: BraidIndex,
    word: Word,
}

impl Braid {
    /// Constructs a [`Braid`] from an optional [`BraidIndex`] and a valid [`Word`].
    ///
    /// If [None] is given for the `index` argument, then the [`BraidIndex`] is inferred from the
    /// [`Word`].
    ///
    /// # Examples
    ///
    /// ```
    /// ```
    ///
    /// # Errors
    ///
    /// See the documentation for the associated error type [`BraidValidationError`] for more
    /// information.
    pub fn new<N>(index: Option<N>, word: Word) -> Result<Self, BraidValidationError>
    where
        N: TryInto<u16>,
        IndexValidationError: From<<N as TryInto<u16>>::Error>,
    {
        let minimal_required_index = word.minimal_required_braid_index();
        let index = if let Some(index) = index {
            BraidIndex::new(index)?
        } else {
            minimal_required_index
        };

        if index < minimal_required_index {
            Err(BraidValidationError::IndexTooSmall {
                index,
                minimal_required_index,
            })
        } else {
            Ok(Self { index, word })
        }
    }

    /// Constructs a [`Braid`] from an optional [`BraidIndex`] and an iterable of [`Word`] data.
    ///
    /// The input data to this function is identical to that of the [`Word::new`] constructor,
    /// except for the [`index`](BraidIndex) argument.
    ///
    /// # Examples
    ///
    /// ```
    /// ```
    ///
    /// # Errors
    ///
    /// See the documentation for the associated error type [`BraidValidationError`] for more
    /// information.
    pub fn from_data<N, D, F, H>(
        index: Option<N>,
        word_data: D,
    ) -> Result<Self, BraidValidationError>
    where
        N: TryInto<u16>,
        IndexValidationError: From<<N as TryInto<u16>>::Error>,
        D: IntoIterator<Item = (F, Option<H>, Sign)>,
        F: TryInto<u16>,
        H: TryInto<u16>,
        StrandValidationError: From<<F as TryInto<u16>>::Error>
            + From<<H as TryInto<u16>>::Error>
            + From<std::convert::Infallible>,
    {
        let word: Word = Word::new(word_data)?;
        Self::new(index, word)
    }
    /// Constructs the trivial [braid](Braid) of the given [index](BraidIndex).
    ///
    /// Serves as the multiplicative identity.
    ///
    /// # Examples
    ///
    /// ```
    /// ```
    ///
    /// # Errors
    ///
    /// See the documentation for the associated error type [`BraidValidationError`] for more
    /// information.
    pub fn trivial<N>(index: N) -> Result<Self, BraidValidationError>
    where
        N: TryInto<u16>,
        IndexValidationError: From<<N as TryInto<u16>>::Error>,
    {
        Self::from_data(Some(index), Vec::<(u16, Option<u16>, Sign)>::new())
    }

    /// Decomposes all [band letters](Letter::Band) of the underlying [`Word`] into equivalent
    /// sub-words of [Artin letters](Letter::Artin).
    ///
    /// See the documentation for [`BandGenerator`](crate::BandGenerator) and [`Word::decompose`]
    /// for more details as to how this method works.
    ///
    /// # Examples
    ///
    /// ````
    /// ````
    pub fn decompose(&self) -> Self {
        Self {
            index: self.index,
            word: self.word.decompose(),
        }
    }
    /// Coalesces all maximal spans of [Artin letters](Letter::Artin) in the underlying [`Word`]
    /// into [band letters](Letter::Band).
    ///
    /// See the documentation for [`BandGenerator`](crate::BandGenerator) and [`Word::coalesce`]
    /// for more details as to how this method works.
    ///
    /// # Examples
    ///
    /// ````
    /// ````
    pub fn coalesce(&self) -> Self {
        Self {
            index: self.index,
            word: self.word.coalesce(),
        }
    }

    /// Computes the minimal [`BraidIndex`] required for the [braid's](Braid) [word](Word) to exist.
    ///
    /// Note that this is not necessarily the same as the actual [Braid::braid_index()].
    ///
    /// # Examples
    ///
    /// ```
    /// ```
    pub fn minimal_required_braid_index(&self) -> BraidIndex {
        self.word.minimal_required_braid_index()
    }
    /// Computes the sum of all [signs](Sign) across the [braid's](Braid) [word](Word).
    ///
    /// # Examples
    ///
    /// ```
    /// ```
    pub fn writhe(&self) -> i32 {
        self.word.iter().fold(0, |a, b| {
            if b.sign() == Sign::Positive {
                a + 1
            } else {
                a - 1
            }
        })
    }
    /// Computes the total number of [letters](Letter) (in any generating set) of the
    /// [braid's](Braid) [word](Word).
    ///
    /// # Examples
    ///
    /// ```
    /// ```
    pub fn letter_length(&self) -> u16 {
        // Length checks performed on underlying word: safe to unwrap
        self.word.length()
    }
    /// Computes the _equivalent_ number of [Artin letters](Letter::Artin) of the [braid's](Braid)
    /// [word](Word).
    ///
    /// See the documentation for [`Word::artin_length`] for more details.
    ///
    /// # Examples
    ///
    /// ```
    /// ```
    pub fn artin_length(&self) -> u16 {
        // Length checks performed on underlying word: safe to unwrap
        self.word.iter().fold(0, |a, b| a + b.artin_length())
    }
    /// Computes the multiplicative inverse of the [`braid`](Braid).
    ///
    /// # Examples
    ///
    /// ```
    /// ```
    pub fn inverse(&self) -> Self {
        Self {
            index: self.index,
            word: self.word.inverse(),
        }
    }
    /// Accessor method to the contained [`BraidIndex`].
    ///
    /// # Examples
    ///
    /// ```
    /// ```
    pub fn braid_index(&self) -> BraidIndex {
        self.index
    }
    /// Accessor method to (a clone of the) the underlying [`Word`] contained in the [`Braid`].
    ///
    /// # Examples
    ///
    /// ```
    /// ```
    pub fn word(&self) -> Word {
        self.word.clone()
    }
    /// Accessor method to the underlying [letters](Letter) of the [`Word`] contained in the
    /// [`Braid`].
    ///
    /// # Examples
    ///
    /// ```
    /// ```
    pub fn letters(&self) -> Vec<Letter> {
        self.word.letters()
    }
    /// Returns a bool indicating whether the [`Braid`] is the [trivial braid](Braid::trivial) for
    /// its [index](BraidIndex).
    ///
    /// # Examples
    ///
    /// ```
    /// ```
    pub fn is_trivial(&self) -> bool {
        self.word.is_trivial()
    }
}

impl Default for Braid {
    fn default() -> Self {
        Self::trivial(1).unwrap()
    }
}

impl From<Word> for Braid {
    fn from(value: Word) -> Self {
        Self {
            index: value.minimal_required_braid_index(),
            word: value,
        }
    }
}
impl From<&Word> for Braid {
    fn from(value: &Word) -> Self {
        Self::from(value.clone())
    }
}

impl<L> TryFrom<Vec<L>> for Braid
where
    L: Into<Letter>,
{
    type Error = BraidValidationError;
    fn try_from(value: Vec<L>) -> Result<Self, Self::Error> {
        let word = Word::try_from(value)?;
        let index = word.minimal_required_braid_index();

        Ok(Self { index, word })
    }
}
impl<L> TryFrom<&[L]> for Braid
where
    L: Into<Letter> + std::clone::Clone,
{
    type Error = BraidValidationError;
    fn try_from(value: &[L]) -> Result<Self, Self::Error> {
        Self::try_from(value.to_vec())
    }
}

impl IntoIterator for Braid {
    type Item = <Word as IntoIterator>::Item;
    type IntoIter = <Word as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.word.into_iter()
    }
}
impl std::ops::Deref for Braid {
    type Target = [Letter];

    fn deref(&self) -> &Self::Target {
        self.word.deref()
    }
}
impl AsRef<[Letter]> for Braid {
    fn as_ref(&self) -> &[Letter] {
        self.word.as_ref()
    }
}

impl std::ops::Mul<Letter> for Braid {
    type Output = Result<Self, BraidValidationError>;
    fn mul(self, rhs: Letter) -> Self::Output {
        if let required_index = rhs.minimal_required_braid_index()
            && self.index < required_index
        {
            Err(BraidValidationError::IndexTooSmall {
                index: self.index,
                minimal_required_index: required_index,
            })
        } else {
            Ok(Self {
                index: self.index,
                word: (self.word * rhs)?,
            })
        }
    }
}
impl std::ops::Mul<Braid> for Letter {
    type Output = Result<Braid, BraidValidationError>;

    fn mul(self, rhs: Braid) -> Self::Output {
        if let required_index = self.minimal_required_braid_index()
            && rhs.braid_index() < required_index
        {
            Err(BraidValidationError::IndexTooSmall {
                index: rhs.braid_index(),
                minimal_required_index: required_index,
            })
        } else {
            Ok(Braid {
                index: rhs.index,
                word: (self * rhs.word)?,
            })
        }
    }
}
impl std::ops::Mul<Word> for Braid {
    type Output = Result<Self, BraidValidationError>;
    fn mul(self, rhs: Word) -> Self::Output {
        if let required_index = rhs.minimal_required_braid_index()
            && self.index < required_index
        {
            Err(BraidValidationError::IndexTooSmall {
                index: self.index,
                minimal_required_index: required_index,
            })
        } else {
            Ok(Self {
                index: self.index,
                word: (self.word * rhs)?,
            })
        }
    }
}
impl std::ops::Mul<Braid> for Word {
    type Output = Result<Braid, BraidValidationError>;

    fn mul(self, rhs: Braid) -> Self::Output {
        if let required_index = self.minimal_required_braid_index()
            && rhs.braid_index() < required_index
        {
            Err(BraidValidationError::IndexTooSmall {
                index: rhs.braid_index(),
                minimal_required_index: required_index,
            })
        } else {
            Ok(Braid {
                index: rhs.index,
                word: (self * rhs.word)?,
            })
        }
    }
}
impl std::ops::Mul for Braid {
    type Output = Result<Self, BraidValidationError>;
    fn mul(self, rhs: Self) -> Self::Output {
        if self.index != rhs.index {
            Err(BraidValidationError::UnequalIndices {
                left: self.index,
                right: rhs.index,
            })
        } else {
            Ok(Self {
                index: self.index,
                word: (self.word * rhs.word)?,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Braid, BraidIndex, BraidValidationError, Letter, Sign, Word};
    use googletest::matchers::{anything, each, eq, err, is_false, is_true, ok, result_of_ref};
    use googletest::{assert_that, expect_that, gtest};

    #[gtest]
    fn construction_from_a_valid_word_works_as_expected() {
        let valid_word = Word::new(vec![
            (1, None::<u16>, Sign::Positive),
            (2, Some(5), Sign::Negative),
            (3, None::<u16>, Sign::Negative),
            (4, Some(5), Sign::Positive),
        ])
        .unwrap();

        let braid_index = valid_word.minimal_required_braid_index();

        let braid_from_borrow = Braid::from(&valid_word);
        expect_that!(braid_from_borrow.word(), eq(&valid_word));

        let braid_from_move = Braid::from(valid_word);
        expect_that!(braid_from_move.braid_index(), eq(braid_index));
    }

    #[test]
    fn valid_construction_with_new_is_successful() {
        let word = Word::new(vec![
            (1, None::<u16>, Sign::Positive),
            (2, Some(5), Sign::Negative),
            (3, None::<u16>, Sign::Negative),
            (4, Some(5), Sign::Positive),
        ])
        .unwrap();

        let valid_braids = [
            Braid::new(None::<u16>, word.clone()),
            Braid::new(Some(10), word),
        ];

        assert_that!(valid_braids, each(ok(anything())));
    }

    #[test]
    fn valid_construction_with_from_data_is_successful() {
        let letters_data = vec![
            (1, None::<u16>, Sign::Positive),
            (2, Some(5), Sign::Negative),
            (3, None::<u16>, Sign::Negative),
            (4, Some(5), Sign::Positive),
        ];

        let valid_braids = [
            Braid::from_data(None::<u16>, letters_data.clone()),
            Braid::from_data(Some(10), letters_data),
        ];

        assert_that!(valid_braids, each(ok(anything())));
    }

    #[test]
    fn valid_construction_with_try_from_is_successful() {
        let letters = vec![
            Letter::new(1, None::<u16>, Sign::Positive).unwrap(),
            Letter::new(2, Some(5), Sign::Negative).unwrap(),
            Letter::new(3, None::<u16>, Sign::Negative).unwrap(),
            Letter::new(4, Some(5), Sign::Positive).unwrap(),
        ];

        let valid_braids = [Braid::try_from(letters.clone()), Braid::try_from(letters)];

        assert_that!(valid_braids, each(ok(anything())));
        assert_that!(
            valid_braids
                .iter()
                .map(|b| b.clone().unwrap().braid_index())
                .collect::<Vec<_>>(),
            each(eq(&BraidIndex::new(5).unwrap()))
        );
    }

    #[test]
    fn vaild_construction_of_trivial_braid_is_successful_and_works_as_expected() {
        let braid_index = 9;

        let trivial_braid = Braid::trivial(braid_index);

        assert_that!(trivial_braid, ok(anything()));

        assert_that!(
            trivial_braid,
            eq(&Braid::new(Some(braid_index), Word::trivial()))
        );
    }

    #[test]
    fn default_braid_is_trivial_unknot() {
        let default_braid = Braid::default();

        assert_that!(default_braid, eq(&Braid::trivial(1).unwrap()));
    }

    #[gtest]
    fn into_iterator_returns_a_vector_of_underlying_letter_data() {
        let letters_data: Vec<(u16, Option<u16>, Sign)> = vec![
            (1, None, Sign::Positive),
            (2, Some(5), Sign::Negative),
            (3, None, Sign::Negative),
            (4, Some(5), Sign::Positive),
        ];

        let braid = Braid::from_data(None::<u16>, letters_data.clone()).unwrap();

        for (actual, expected) in braid.into_iter().zip(letters_data) {
            expect_that!(actual, eq(expected));
        }
    }

    #[test]
    fn decompose_computes_as_expected() {
        let braid = Braid::from_data(
            None::<u16>,
            [
                (1, Some(3), Sign::Positive),
                (2, None, Sign::Negative),
                (1, Some(2), Sign::Positive),
            ],
        )
        .unwrap();
        let expected_decomposition = Braid::from_data(
            None::<u16>,
            [
                (1, None::<u16>, Sign::Negative),
                (2, None, Sign::Positive),
                (1, None, Sign::Positive),
                (2, None, Sign::Negative),
                (1, None, Sign::Positive),
            ],
        )
        .unwrap();

        assert_that!(braid.decompose(), eq(&expected_decomposition));
    }

    #[test]
    fn coalesce_computes_as_expected() {
        let braid = Braid::from_data(
            None::<u16>,
            [
                (2, None::<u16>, Sign::Positive),
                (1, None, Sign::Positive),
                (2, None, Sign::Negative),
                (2, None, Sign::Negative),
                (1, None, Sign::Positive),
            ],
        )
        .unwrap();
        let expected_coalescence = Braid::from_data(
            None::<u16>,
            [
                (1, Some(3), Sign::Positive),
                (2, None, Sign::Negative),
                (1, Some(2), Sign::Positive),
            ],
        )
        .unwrap();

        assert_that!(braid.coalesce(), eq(&expected_coalescence));
    }

    #[test]
    fn deref_to_slice_of_letters_works_as_expected() {
        let letters = [
            Letter::new(1, Some(3), Sign::Positive).unwrap(),
            Letter::new(2, None::<u16>, Sign::Negative).unwrap(),
            Letter::new(1, Some(2), Sign::Positive).unwrap(),
        ];
        let braid = Braid::try_from(&letters[..]).unwrap();

        assert_that!(*braid, eq(&letters));
    }

    #[test]
    fn can_pass_braid_as_ref_where_ref_to_letter_slice_is_expected() {
        fn as_ref_tester<B: AsRef<[Letter]>>(b: B, v: &[Letter]) -> bool {
            b.as_ref() == v
        }
        let letters = [
            Letter::new(1, Some(3), Sign::Positive).unwrap(),
            Letter::new(2, None::<u16>, Sign::Negative).unwrap(),
            Letter::new(1, Some(2), Sign::Positive).unwrap(),
        ];
        let braid = Braid::try_from(&letters[..]).unwrap();

        assert_that!(
            braid,
            result_of_ref!(|b: &Braid| as_ref_tester(b, &letters), is_true()),
        );
    }

    #[gtest]
    fn properties_compute_as_expected() {
        let word = Word::try_from(vec![
            Letter::new(1, Some(3), Sign::Positive).unwrap(),
            Letter::new(2, None::<u16>, Sign::Negative).unwrap(),
            Letter::new(1, Some(2), Sign::Positive).unwrap(),
        ])
        .unwrap();
        let braid = Braid::new(Some(9), word.clone()).unwrap();

        expect_that!(braid.letters(), eq(&word.letters()));
        expect_that!(
            braid.inverse(),
            eq(&Braid::new(Some(9), word.inverse()).unwrap())
        );
        expect_that!(braid.is_trivial(), is_false());
        expect_that!(Braid::trivial(9).unwrap().is_trivial(), is_true());
        expect_that!(braid.letter_length(), eq(word.length()));
        expect_that!(braid.artin_length(), eq(word.artin_length()));
        expect_that!(
            braid.writhe(),
            eq(word.iter().fold(0i32, |writhe, l| {
                if l.sign() == Sign::Positive {
                    writhe + 1
                } else {
                    writhe - 1
                }
            }))
        );
        expect_that!(
            braid.minimal_required_braid_index(),
            eq(word.minimal_required_braid_index()),
        );
        expect_that!(braid.braid_index(), eq(BraidIndex::new(9).unwrap()),);
    }

    #[gtest]
    fn valid_multiplication_with_letter_succeeds_and_computes_as_expected() {
        let letters = vec![
            Letter::new(1, Some(3), Sign::Positive).unwrap(),
            Letter::new(2, None::<u16>, Sign::Negative).unwrap(),
            Letter::new(2, Some(8), Sign::Positive).unwrap(),
        ];
        let braid = Braid::try_from(&letters[..]).unwrap();
        let other_letter = Letter::new(3, Some(7), Sign::Negative).unwrap();

        expect_that!(
            braid.clone() * other_letter,
            eq(&Braid::try_from(
                [letters.clone(), vec![other_letter]].concat()
            ))
        );
        expect_that!(
            other_letter * braid,
            eq(&Braid::try_from([vec![other_letter], letters].concat()))
        );
    }

    #[gtest]
    fn valid_multiplication_with_word_succeeds_and_computes_as_expected() {
        let braid = Braid::try_from(vec![
            Letter::new(1, Some(3), Sign::Positive).unwrap(),
            Letter::new(2, None::<u16>, Sign::Negative).unwrap(),
            Letter::new(2, Some(8), Sign::Positive).unwrap(),
        ])
        .unwrap();
        let word = Word::try_from(vec![
            Letter::new(3, None::<u16>, Sign::Negative).unwrap(),
            Letter::new(2, Some(7), Sign::Negative).unwrap(),
        ])
        .unwrap();

        expect_that!(
            braid.clone() * word.clone(),
            eq(&Braid::new(
                None::<u16>,
                (braid.word() * word.clone()).unwrap()
            )),
        );
        expect_that!(
            word.clone() * braid.clone(),
            eq(&Braid::new(None::<u16>, (word * braid.word()).unwrap()))
        );
    }

    #[gtest]
    fn valid_multiplication_with_braid_succeeds_and_computes_as_expected() {
        let braid1 = Braid::try_from(vec![
            Letter::new(1, Some(3), Sign::Positive).unwrap(),
            Letter::new(2, None::<u16>, Sign::Negative).unwrap(),
            Letter::new(2, Some(8), Sign::Positive).unwrap(),
        ])
        .unwrap();
        let braid2 = Braid::new(
            Some(8),
            Word::try_from(vec![
                Letter::new(3, None::<u16>, Sign::Negative).unwrap(),
                Letter::new(2, Some(7), Sign::Negative).unwrap(),
            ])
            .unwrap(),
        )
        .unwrap();

        expect_that!(
            braid1.clone() * braid2.clone(),
            eq(&Braid::new(
                None::<u16>,
                (braid1.word() * braid2.word()).unwrap()
            )),
        );
        expect_that!(
            braid2.clone() * braid1.clone(),
            eq(&Braid::new(
                None::<u16>,
                (braid2.word() * braid1.word()).unwrap()
            ))
        );
    }

    #[gtest]
    fn invalid_construction_with_new_fails_as_expected() {
        let invalid_braids: Vec<(
            Result<Braid, BraidValidationError>,
            BraidValidationError,
            &'static str,
        )> = vec![
            (
                Braid::new(
                    Some(3),
                    Word::new(vec![(1, Some(5), Sign::Positive)]).unwrap(),
                ),
                BraidValidationError::IndexTooSmall {
                    index: BraidIndex::new(3).unwrap(),
                    minimal_required_index: BraidIndex::new(5).unwrap(),
                },
                "index too small",
            ),
            (
                Braid::new(
                    Some(0),
                    Word::new(vec![(1, Some(5), Sign::Positive)]).unwrap(),
                ),
                BraidValidationError::from(BraidIndex::new(0).err().unwrap()),
                "index validation failure",
            ),
        ];

        for (invalid_braid, error, label) in invalid_braids {
            expect_that!(invalid_braid, err(eq(&error)), "{label}")
        }
    }

    #[gtest]
    fn invalid_construction_with_from_data_fails_as_expected() {
        let invalid_braids: Vec<(
            Result<Braid, BraidValidationError>,
            BraidValidationError,
            &'static str,
        )> = vec![
            (
                Braid::from_data(Some(3), vec![(1, Some(5), Sign::Positive)]),
                BraidValidationError::IndexTooSmall {
                    index: BraidIndex::new(3).unwrap(),
                    minimal_required_index: BraidIndex::new(5).unwrap(),
                },
                "index too small",
            ),
            (
                Braid::from_data(Some(0), vec![(1, Some(5), Sign::Positive)]),
                BraidValidationError::from(BraidIndex::new(0).err().unwrap()),
                "index validation failure",
            ),
            (
                Braid::from_data(
                    None::<u16>,
                    vec![(1, None::<u16>, Sign::Positive); u16::MAX as usize + 1],
                ),
                BraidValidationError::from(
                    Word::new(vec![
                        (1, None::<u16>, Sign::Positive);
                        u16::MAX as usize + 1
                    ])
                    .err()
                    .unwrap(),
                ),
                "word validation failure",
            ),
        ];

        for (invalid_braid, error, label) in invalid_braids {
            expect_that!(invalid_braid, err(eq(&error)), "{label}")
        }
    }

    #[gtest]
    fn invalid_construction_with_try_from_fails_as_expected() {
        let invalid_braids: Vec<(
            Result<Braid, BraidValidationError>,
            BraidValidationError,
            &'static str,
        )> = vec![
            (
                Braid::try_from(vec![
                    Letter::new(1, None::<u16>, Sign::Positive).unwrap();
                    u16::MAX as usize + 1
                ]),
                BraidValidationError::from(
                    Word::try_from(vec![
                        Letter::new(1, None::<u16>, Sign::Positive).unwrap();
                        u16::MAX as usize + 1
                    ])
                    .err()
                    .unwrap(),
                ),
                "Word validation failure - from Vec",
            ),
            (
                Braid::try_from(
                    &vec![Letter::new(2, Some(3), Sign::Positive).unwrap(); u16::MAX as usize + 1]
                        [..],
                ),
                BraidValidationError::from(
                    Word::try_from(
                        &vec![
                            Letter::new(2, Some(3), Sign::Positive).unwrap();
                            u16::MAX as usize + 1
                        ][..],
                    )
                    .err()
                    .unwrap(),
                ),
                "Word validation failure - from slice",
            ),
        ];

        for (invalid_braid, error, label) in invalid_braids {
            expect_that!(invalid_braid, err(eq(&error)), "{label}")
        }
    }

    #[gtest]
    fn invalid_construction_of_trivial_braid_fails_as_expected() {
        let invalid_braids: Vec<(
            Result<Braid, BraidValidationError>,
            BraidValidationError,
            &'static str,
        )> = vec![
            (
                Braid::trivial(0),
                BraidValidationError::from(BraidIndex::new(0).err().unwrap()),
                "zero index",
            ),
            (
                Braid::trivial(-1),
                BraidValidationError::from(BraidIndex::new(-1).err().unwrap()),
                "negative index",
            ),
            (
                Braid::trivial(u16::MAX as u32 + 1),
                BraidValidationError::from(BraidIndex::new(u16::MAX as u32 + 1).err().unwrap()),
                "big index",
            ),
        ];

        for (invalid_braid, error, label) in invalid_braids {
            expect_that!(invalid_braid, err(eq(&error)), "{label}")
        }
    }

    #[gtest]
    fn invalid_multiplication_fails_as_expected() {
        let letter = Letter::new(7, None::<u16>, Sign::Positive).unwrap();
        let word = Word::new(vec![
            (2, Some(8), Sign::Negative),
            (1, None::<u16>, Sign::Positive),
        ])
        .unwrap();
        let invalid_braids: Vec<(
            Result<Braid, BraidValidationError>,
            BraidValidationError,
            &'static str,
        )> = vec![
            (
                Braid::from_data(
                    None::<u16>,
                    vec![
                        (1, None::<u16>, Sign::Positive),
                        (2, Some(5), Sign::Negative),
                        (3, None::<u16>, Sign::Negative),
                        (4, Some(5), Sign::Positive),
                    ],
                )
                .unwrap()
                    * letter,
                BraidValidationError::IndexTooSmall {
                    index: BraidIndex::new(5).unwrap(),
                    minimal_required_index: BraidIndex::new(8).unwrap(),
                },
                "index too small, braid * letter",
            ),
            (
                letter
                    * Braid::from_data(
                        None::<u16>,
                        vec![
                            (1, None::<u16>, Sign::Positive),
                            (2, Some(5), Sign::Negative),
                            (3, None::<u16>, Sign::Negative),
                            (4, Some(5), Sign::Positive),
                        ],
                    )
                    .unwrap(),
                BraidValidationError::IndexTooSmall {
                    index: BraidIndex::new(5).unwrap(),
                    minimal_required_index: BraidIndex::new(8).unwrap(),
                },
                "index too small, letter * braid",
            ),
            (
                Braid::from_data(
                    None::<u16>,
                    vec![
                        (1, None::<u16>, Sign::Positive),
                        (2, Some(5), Sign::Negative),
                        (3, None::<u16>, Sign::Negative),
                        (4, Some(5), Sign::Positive),
                    ],
                )
                .unwrap()
                    * word.clone(),
                BraidValidationError::IndexTooSmall {
                    index: BraidIndex::new(5).unwrap(),
                    minimal_required_index: BraidIndex::new(8).unwrap(),
                },
                "index too small, braid * word",
            ),
            (
                word.clone()
                    * Braid::from_data(
                        None::<u16>,
                        vec![
                            (1, None::<u16>, Sign::Positive),
                            (2, Some(5), Sign::Negative),
                            (3, None::<u16>, Sign::Negative),
                            (4, Some(5), Sign::Positive),
                        ],
                    )
                    .unwrap(),
                BraidValidationError::IndexTooSmall {
                    index: BraidIndex::new(5).unwrap(),
                    minimal_required_index: BraidIndex::new(8).unwrap(),
                },
                "index too small, word * braid",
            ),
            (
                Braid::from_data(Some(10), word.clone()).unwrap()
                    * Braid::from_data(Some(11), word.clone()).unwrap(),
                BraidValidationError::UnequalIndices {
                    left: BraidIndex::new(10).unwrap(),
                    right: BraidIndex::new(11).unwrap(),
                },
                "unequal indices",
            ),
            (
                Braid::from_data(
                    Some(10),
                    vec![(1, None::<u16>, Sign::Positive); u16::MAX as usize],
                )
                .unwrap()
                    * letter,
                BraidValidationError::from(
                    Word::new(vec![
                        (1, None::<u16>, Sign::Positive);
                        u16::MAX as usize + 1
                    ])
                    .err()
                    .unwrap(),
                ),
                "word failed validation, braid * letter",
            ),
            (
                letter
                    * Braid::from_data(
                        Some(10),
                        vec![(1, None::<u16>, Sign::Positive); u16::MAX as usize],
                    )
                    .unwrap(),
                BraidValidationError::from(
                    Word::new(vec![
                        (1, None::<u16>, Sign::Positive);
                        u16::MAX as usize + 1
                    ])
                    .err()
                    .unwrap(),
                ),
                "word failed validation, letter * braid",
            ),
            (
                Braid::from_data(
                    Some(10),
                    vec![(1, None::<u16>, Sign::Positive); u16::MAX as usize],
                )
                .unwrap()
                    * word.clone(),
                BraidValidationError::from(
                    Word::new(vec![
                        (1, None::<u16>, Sign::Positive);
                        u16::MAX as usize + 12
                    ])
                    .err()
                    .unwrap(),
                ),
                "word failed validation, braid * word",
            ),
            (
                word * Braid::from_data(
                    Some(10),
                    vec![(1, None::<u16>, Sign::Positive); u16::MAX as usize],
                )
                .unwrap(),
                BraidValidationError::from(
                    Word::new(vec![
                        (1, None::<u16>, Sign::Positive);
                        u16::MAX as usize + 12
                    ])
                    .err()
                    .unwrap(),
                ),
                "word failed validation, word * braid",
            ),
            (
                Braid::from_data(
                    None::<u16>,
                    vec![(1, None::<u16>, Sign::Positive); u16::MAX as usize],
                )
                .unwrap()
                    * Braid::from_data(None::<u16>, vec![(1, None::<u16>, Sign::Positive)])
                        .unwrap(),
                BraidValidationError::from(
                    Word::new(vec![
                        (1, None::<u16>, Sign::Positive);
                        u16::MAX as usize + 1
                    ])
                    .err()
                    .unwrap(),
                ),
                "word failed validation, long_braid * short_braid",
            ),
            (
                Braid::from_data(None::<u16>, vec![(1, None::<u16>, Sign::Positive)]).unwrap()
                    * Braid::from_data(
                        None::<u16>,
                        vec![(1, None::<u16>, Sign::Positive); u16::MAX as usize],
                    )
                    .unwrap(),
                BraidValidationError::from(
                    Word::new(vec![
                        (1, None::<u16>, Sign::Positive);
                        u16::MAX as usize + 1
                    ])
                    .err()
                    .unwrap(),
                ),
                "word failed validation, short_braid * long_braid",
            ),
        ];

        for (invalid_braid, error, label) in invalid_braids {
            expect_that!(invalid_braid, err(eq(&error)), "{label}")
        }
    }
}
