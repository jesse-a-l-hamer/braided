use crate::{
    ArtinGenerator, ArtinValidationError, BandGenerator, BandValidationError, Braid, BraidIndex,
    BraidValidationError, IndexValidationError, Letter, LetterValidationError, Strand,
    StrandValidationError, Word, WordValidationError,
};

/// Newtype wrapper around [`Result<ArtinGenerator, ArtinValidationError>`] returned from fallible
/// [`ArtinGenerator`] construction.
///
/// Use the [`std::ops::Deref`] operator (`*`) for easy access to the inner result type (e.g., in
/// `match` expressions)
///
/// # Examples
///
/// ```
/// use braided::{ArtinGenerator, Sign};
/// use std::assert_matches;
///
/// let valid_result = ArtinGenerator::try_new(1, Sign::Positive);
/// let invalid_result = ArtinGenerator::try_new(0, Sign::Positive);
///
/// assert_matches!(*valid_result, Ok(_));
/// assert_matches!(*invalid_result, Err(_));
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ArtinResult(Result<ArtinGenerator, ArtinValidationError>);
/// Newtype wrapper around [`Result<BandGenerator, BandValidationError>`] returned from fallible
/// [`BandGenerator`] construction.
///
/// Use the [`std::ops::Deref`] operator (`*`) for easy access to the inner result type (e.g., in
/// `match` expressions)
///
/// # Examples
///
/// ```
/// use braided::{BandGenerator, Sign};
/// use std::assert_matches;
///
/// let valid_result = BandGenerator::try_new(1, 3, Sign::Positive);
/// let invalid_result = BandGenerator::try_new(0, 3, Sign::Positive);
///
/// assert_matches!(*valid_result, Ok(_));
/// assert_matches!(*invalid_result, Err(_));
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct BandResult(Result<BandGenerator, BandValidationError>);
/// Newtype wrapper around [`Result<Braid, BraidValidationError>`] returned from fallible [`Braid`]
/// construction.
///
/// Use the [`std::ops::Deref`] operator (`*`) for easy access to the inner result type (e.g., in
/// `match` expressions)
///
/// # Examples
///
/// ```
/// use braided::{Braid, Sign};
/// use std::assert_matches;
///
/// let valid_result = Braid::try_from_data(None::<u16>, vec![(1, None::<u16>, Sign::Positive)]);
/// let invalid_result = Braid::try_from_data(Some(1), vec![(1, None::<u16>, Sign::Positive)]);
///
/// assert_matches!(*valid_result, Ok(_));
/// assert_matches!(*invalid_result, Err(_));
/// ```
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BraidResult(Result<Braid, BraidValidationError>);
/// Newtype wrapper around [`Result<BraidIndex, IndexValidationError>`] returned from fallible
/// [`BraidIndex`] construction.
///
/// Use the [`std::ops::Deref`] operator (`*`) for easy access to the inner result type (e.g., in
/// `match` expressions)
///
/// # Examples
///
/// ```
/// use braided::BraidIndex;
/// use std::assert_matches;
///
/// let valid_result = BraidIndex::try_new(1);
/// let invalid_result = BraidIndex::try_new(0);
///
/// assert_matches!(*valid_result, Ok(_));
/// assert_matches!(*invalid_result, Err(_));
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct IndexResult(Result<BraidIndex, IndexValidationError>);
/// Newtype wrapper around [`Result<Letter, LetterValidationError>`] returned from fallible [`Letter`]
/// construction.
///
/// Use the [`std::ops::Deref`] operator (`*`) for easy access to the inner result type (e.g., in
/// `match` expressions)
///
/// # Examples
///
/// ```
/// use braided::{Letter, Sign};
/// use std::assert_matches;
///
/// let valid_result = Letter::try_new(1, Some(3), Sign::Positive);
/// let invalid_result = Letter::try_new(0, Some(3), Sign::Positive);
///
/// assert_matches!(*valid_result, Ok(_));
/// assert_matches!(*invalid_result, Err(_));
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct LetterResult(Result<Letter, LetterValidationError>);
/// Newtype wrapper around [`Result<Strand, StrandValidationError>`] returned from fallible [`Strand`]
/// construction.
///
/// Use the [`std::ops::Deref`] operator (`*`) for easy access to the inner result type (e.g., in
/// `match` expressions)
///
/// # Examples
///
/// ```
/// use braided::Strand;
/// use std::assert_matches;
///
/// let valid_result = Strand::try_new(1);
/// let invalid_result = Strand::try_new(0);
///
/// assert_matches!(*valid_result, Ok(_));
/// assert_matches!(*invalid_result, Err(_));
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct StrandResult(Result<Strand, StrandValidationError>);
/// Newtype wrapper around [`Result<Word, WordValidationError>`] returned from fallible [`Word`]
/// construction.
///
/// Use the [`std::ops::Deref`] operator (`*`) for easy access to the inner result type (e.g., in
/// `match` expressions)
///
/// # Examples
///
/// ```
/// use braided::{Sign, Word};
/// use std::assert_matches;
///
/// let valid_result = Word::try_new(vec![(1, None::<u16>, Sign::Positive)]);
/// let invalid_result = Word::try_new(
///     vec![(1, None::<u16>, Sign::Positive); u16::MAX as usize + 1]
/// );
///
/// assert_matches!(*valid_result, Ok(_));
/// assert_matches!(*invalid_result, Err(_));
/// ```
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct WordResult(Result<Word, WordValidationError>);

impl From<Result<ArtinGenerator, ArtinValidationError>> for ArtinResult {
    fn from(value: Result<ArtinGenerator, ArtinValidationError>) -> Self {
        Self(value)
    }
}
impl From<ArtinGenerator> for ArtinResult {
    fn from(value: ArtinGenerator) -> Self {
        Self(Ok(value))
    }
}
impl From<ArtinValidationError> for ArtinResult {
    fn from(value: ArtinValidationError) -> Self {
        Self(Err(value))
    }
}
impl std::ops::Deref for ArtinResult {
    type Target = Result<ArtinGenerator, ArtinValidationError>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for ArtinResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Result<BandGenerator, BandValidationError>> for BandResult {
    fn from(value: Result<BandGenerator, BandValidationError>) -> Self {
        Self(value)
    }
}
impl From<BandGenerator> for BandResult {
    fn from(value: BandGenerator) -> Self {
        Self(Ok(value))
    }
}
impl From<BandValidationError> for BandResult {
    fn from(value: BandValidationError) -> Self {
        Self(Err(value))
    }
}
impl std::ops::Deref for BandResult {
    type Target = Result<BandGenerator, BandValidationError>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for BandResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl BraidResult {
    /// Clones the inner result object and unwraps the [`Ok(Braid)`](Braid).
    ///
    /// Panics if the wrapped result is an [`Err(BraidValidationError)`](BraidValidationError) variant.
    ///
    /// # Examples
    ///
    /// ```
    /// use::braided::{Braid, Sign};
    ///
    /// let braid = Braid::try_from_data(
    ///     None::<u16>,
    ///     vec![(1, None::<u16>, Sign::Negative), (2, Some(5), Sign::Positive)],
    /// ).clone_unwrap();
    ///
    /// assert_eq!(braid.writhe(), 0);
    /// ```
    pub fn clone_unwrap(&self) -> Braid {
        <Result<Braid, BraidValidationError> as Clone>::clone(self).unwrap()
    }
    /// Clones the inner result object and unwraps the
    /// [`Err(BraidValidationError)`](BraidValidationError).
    ///
    /// Panics if the wrapped result is an [`Ok(Braid)`](Braid) variant.
    ///
    /// # Examples
    ///
    /// ```
    /// use::braided::{Braid, BraidIndex, Sign, BraidValidationError};
    /// use::std::assert_matches;
    ///
    /// let invalid_braid = Braid::try_from_data(
    ///     Some(4),
    ///     vec![(1, None::<u16>, Sign::Negative), (2, Some(5), Sign::Positive)],
    /// ).clone_unwrap_err();
    ///
    /// assert_eq!(invalid_braid, BraidValidationError::IndexTooSmall {
    ///     index:BraidIndex::try_new(4).unwrap(),
    ///     minimal_required_index: BraidIndex::try_new(5).unwrap()
    /// });
    /// ```
    pub fn clone_unwrap_err(&self) -> BraidValidationError {
        <Result<Braid, BraidValidationError> as Clone>::clone(self).unwrap_err()
    }
}
impl From<Result<Braid, BraidValidationError>> for BraidResult {
    fn from(value: Result<Braid, BraidValidationError>) -> Self {
        Self(value)
    }
}
impl From<Braid> for BraidResult {
    fn from(value: Braid) -> Self {
        Self(Ok(value))
    }
}
impl From<BraidValidationError> for BraidResult {
    fn from(value: BraidValidationError) -> Self {
        Self(Err(value))
    }
}
impl std::ops::Deref for BraidResult {
    type Target = Result<Braid, BraidValidationError>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for BraidResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Result<BraidIndex, IndexValidationError>> for IndexResult {
    fn from(value: Result<BraidIndex, IndexValidationError>) -> Self {
        Self(value)
    }
}
impl From<BraidIndex> for IndexResult {
    fn from(value: BraidIndex) -> Self {
        Self(Ok(value))
    }
}
impl From<IndexValidationError> for IndexResult {
    fn from(value: IndexValidationError) -> Self {
        Self(Err(value))
    }
}
impl std::ops::Deref for IndexResult {
    type Target = Result<BraidIndex, IndexValidationError>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for IndexResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Result<Letter, LetterValidationError>> for LetterResult {
    fn from(value: Result<Letter, LetterValidationError>) -> Self {
        Self(value)
    }
}
impl From<Letter> for LetterResult {
    fn from(value: Letter) -> Self {
        Self(Ok(value))
    }
}
impl From<LetterValidationError> for LetterResult {
    fn from(value: LetterValidationError) -> Self {
        Self(Err(value))
    }
}
impl std::ops::Deref for LetterResult {
    type Target = Result<Letter, LetterValidationError>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for LetterResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Result<Strand, StrandValidationError>> for StrandResult {
    fn from(value: Result<Strand, StrandValidationError>) -> Self {
        Self(value)
    }
}
impl From<Strand> for StrandResult {
    fn from(value: Strand) -> Self {
        Self(Ok(value))
    }
}
impl From<StrandValidationError> for StrandResult {
    fn from(value: StrandValidationError) -> Self {
        Self(Err(value))
    }
}
impl std::ops::Deref for StrandResult {
    type Target = Result<Strand, StrandValidationError>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for StrandResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl WordResult {
    /// Clones the inner result object and unwraps the [`Ok(Word)`](Word).
    ///
    /// Panics if the wrapped result is an [`Err(WordValidationError)`](WordValidationError) variant.
    ///
    /// # Examples
    ///
    /// ```
    /// use::braided::{Sign, Word};
    ///
    /// let word = Word::try_new(
    ///     vec![(1, None::<u16>, Sign::Negative), (2, Some(5), Sign::Positive)],
    /// ).clone_unwrap();
    ///
    /// assert_eq!(word.artin_length(), 6);
    /// ```
    pub fn clone_unwrap(&self) -> Word {
        <Result<Word, WordValidationError> as Clone>::clone(self).unwrap()
    }
    /// Clones the inner result object and unwraps the
    /// [`Err(WordValidationError)`](WordValidationError).
    ///
    /// Panics if the wrapped result is an [`Ok(Word)`](Word) variant.
    ///
    /// # Examples
    ///
    /// ```
    /// use::braided::{Sign, Word, WordValidationError};
    /// use::std::assert_matches;
    ///
    /// let invalid_word = Word::try_new(
    ///     vec![(1, None::<u16>, Sign::Negative), (5, Some(1), Sign::Positive)],
    /// ).clone_unwrap_err();
    ///
    /// assert_matches!(invalid_word, WordValidationError::LetterValidation(_));
    /// ```
    pub fn clone_unwrap_err(&self) -> WordValidationError {
        <Result<Word, WordValidationError> as Clone>::clone(self).unwrap_err()
    }
}
impl From<Result<Word, WordValidationError>> for WordResult {
    fn from(value: Result<Word, WordValidationError>) -> Self {
        Self(value)
    }
}
impl From<Word> for WordResult {
    fn from(value: Word) -> Self {
        Self(Ok(value))
    }
}
impl From<WordValidationError> for WordResult {
    fn from(value: WordValidationError) -> Self {
        Self(Err(value))
    }
}
impl std::ops::Deref for WordResult {
    type Target = Result<Word, WordValidationError>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for WordResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ArtinGenerator, ArtinResult, ArtinValidationError, BandGenerator, BandResult,
        BandValidationError, Braid, BraidIndex, BraidResult, BraidValidationError, IndexResult,
        IndexValidationError, Letter, LetterResult, LetterValidationError, Sign, Strand,
        StrandResult, StrandValidationError, Word, WordResult, WordValidationError,
    };
    use googletest::matchers::{eq, err, ok};
    use googletest::{expect_that, gtest};

    fn get_valids() -> (
        ArtinGenerator,
        BandGenerator,
        Braid,
        BraidIndex,
        Letter,
        Strand,
        Word,
    ) {
        (
            ArtinGenerator::try_new(1, Sign::Positive).unwrap(),
            BandGenerator::try_new(1, 3, Sign::Positive).unwrap(),
            Braid::try_from_data(None::<u16>, vec![(1, None::<u16>, Sign::Positive)])
                .clone_unwrap(),
            BraidIndex::try_new(2).unwrap(),
            Letter::try_new(1, None::<u16>, Sign::Positive).unwrap(),
            Strand::try_new(1).unwrap(),
            Word::try_new(vec![(1, None::<u16>, Sign::Positive)]).clone_unwrap(),
        )
    }

    fn get_invalids() -> (
        ArtinValidationError,
        BandValidationError,
        BraidValidationError,
        IndexValidationError,
        LetterValidationError,
        StrandValidationError,
        WordValidationError,
    ) {
        (
            ArtinGenerator::try_new(0, Sign::Positive).unwrap_err(),
            BandGenerator::try_new(0, 3, Sign::Positive).unwrap_err(),
            Braid::try_from_data(Some(1), vec![(1, None::<u16>, Sign::Positive)])
                .clone_unwrap_err(),
            BraidIndex::try_new(0).unwrap_err(),
            Letter::try_new(0, None::<u16>, Sign::Positive).unwrap_err(),
            Strand::try_new(0).unwrap_err(),
            Word::try_new(vec![(0, None::<u16>, Sign::Positive)]).clone_unwrap_err(),
        )
    }
    #[gtest]
    fn can_construct_results_from_ok_type() {
        let valids = get_valids();
        expect_that!(
            ArtinResult::from(valids.0),
            eq(ArtinResult::from(Ok(valids.0)))
        );
        expect_that!(
            BandResult::from(valids.1),
            eq(BandResult::from(Ok(valids.1)))
        );
        expect_that!(
            BraidResult::from(valids.2.clone()),
            eq(&BraidResult::from(Ok(valids.2)))
        );
        expect_that!(
            IndexResult::from(valids.3),
            eq(IndexResult::from(Ok(valids.3)))
        );
        expect_that!(
            LetterResult::from(valids.4),
            eq(LetterResult::from(Ok(valids.4)))
        );
        expect_that!(
            StrandResult::from(valids.5),
            eq(StrandResult::from(Ok(valids.5)))
        );
        expect_that!(
            WordResult::from(valids.6.clone()),
            eq(&WordResult::from(Ok(valids.6)))
        );
    }

    #[gtest]
    fn can_construct_results_from_error_type() {
        let invalids = get_invalids();
        expect_that!(
            ArtinResult::from(invalids.0),
            eq(ArtinResult::from(Err(invalids.0)))
        );
        expect_that!(
            BandResult::from(invalids.1),
            eq(BandResult::from(Err(invalids.1)))
        );
        expect_that!(
            BraidResult::from(invalids.2),
            eq(&BraidResult::from(Err(invalids.2)))
        );
        expect_that!(
            IndexResult::from(invalids.3),
            eq(IndexResult::from(Err(invalids.3)))
        );
        expect_that!(
            LetterResult::from(invalids.4),
            eq(LetterResult::from(Err(invalids.4)))
        );
        expect_that!(
            StrandResult::from(invalids.5),
            eq(StrandResult::from(Err(invalids.5)))
        );
        expect_that!(
            WordResult::from(invalids.6),
            eq(&WordResult::from(Err(invalids.6)))
        );
    }

    #[gtest]
    fn can_deref_results_into_underlying_result_type() {
        let valids = get_valids();
        expect_that!(*ArtinResult::from(valids.0), ok(eq(valids.0)));
        expect_that!(*BandResult::from(valids.1), ok(eq(valids.1)));
        expect_that!(*BraidResult::from(valids.2.clone()), ok(eq(&valids.2)));
        expect_that!(*IndexResult::from(valids.3), ok(eq(valids.3)));
        expect_that!(*LetterResult::from(valids.4), ok(eq(valids.4)));
        expect_that!(*StrandResult::from(valids.5), ok(eq(valids.5)));
        expect_that!(*WordResult::from(valids.6.clone()), ok(eq(&valids.6)));

        let invalids = get_invalids();
        expect_that!(*ArtinResult::from(invalids.0), err(eq(invalids.0)));
        expect_that!(*BandResult::from(invalids.1), err(eq(invalids.1)));
        expect_that!(*BraidResult::from(invalids.2), err(eq(&invalids.2)));
        expect_that!(*IndexResult::from(invalids.3), err(eq(invalids.3)));
        expect_that!(*LetterResult::from(invalids.4), err(eq(invalids.4)));
        expect_that!(*StrandResult::from(invalids.5), err(eq(invalids.5)));
        expect_that!(*WordResult::from(invalids.6), err(eq(&invalids.6)));
    }

    #[gtest]
    fn can_mutably_deref_into_mutable_underlying_result_type() {
        let valids = get_valids();
        let invalids = get_invalids();

        let mut result = ArtinResult::from(valids.0);
        expect_that!(*result, ok(eq(valids.0)));
        *result = Err(invalids.0);
        expect_that!(*result, err(eq(invalids.0)));

        let mut result = BandResult::from(valids.1);
        expect_that!(*result, ok(eq(valids.1)));
        *result = Err(invalids.1);
        expect_that!(*result, err(eq(invalids.1)));

        let mut result = BraidResult::from(valids.2.clone());
        expect_that!(*result, ok(eq(&valids.2)));
        *result = Err(invalids.2);
        expect_that!(*result, err(eq(&invalids.2)));

        let mut result = IndexResult::from(valids.3);
        expect_that!(*result, ok(eq(valids.3)));
        *result = Err(invalids.3);
        expect_that!(*result, err(eq(invalids.3)));

        let mut result = LetterResult::from(valids.4);
        expect_that!(*result, ok(eq(valids.4)));
        *result = Err(invalids.4);
        expect_that!(*result, err(eq(invalids.4)));

        let mut result = StrandResult::from(valids.5);
        expect_that!(*result, ok(eq(valids.5)));
        *result = Err(invalids.5);
        expect_that!(*result, err(eq(invalids.5)));

        let mut result = WordResult::from(valids.6.clone());
        expect_that!(*result, ok(eq(&valids.6)));
        *result = Err(invalids.6);
        expect_that!(*result, err(eq(&invalids.6)));
    }
}
