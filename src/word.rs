use crate::{
    ArtinGenerator, BandGenerator, BraidIndex, Letter, LetterValidationError, Sign,
    StrandValidationError,
};

/// Represents possible failures when attempting to construct a new [`Word`].
///
/// Note that the [`WordValidationError::FromInt`] variant is only possible when supplying a bad
/// exponent to the [`word!`](crate::word) macro. Please see the documentation for that macro for
/// more details and examples.
///
/// # Examples
///
/// 1. Attempting to multiply two words whose combined [Artin length](Word::artin_length) exceeds
///    [`u16::MAX`] ([`WordValidationError::TooLong`]):
///
/// ```
/// use braided::{Letter, Sign, Word, WordValidationError};
///
/// let letter = Letter::new(1, None::<u16>, Sign::Positive).unwrap();
/// let long_word = Word::try_from(vec![letter; u16::MAX as usize]).unwrap();
///
/// assert_eq!(
///     long_word.clone() * long_word.clone(),
///     Err(WordValidationError::TooLong(2 * (u16::MAX as u32))),
/// );
///
/// // Note: you can still multiply two long words than cancel into a short one
/// assert_eq!(
///     long_word.clone() * long_word.inverse(),
///     Ok(Word::trivial())
/// );
///
/// // Failure can also occur when multiplying a word by a letter with large Artin length
/// let tall_letter = Letter::new(1, Some(2u16.pow(15) + 1), Sign::Negative).unwrap();
/// let short_word = Word::new(
///     vec![(2, None::<u16>, Sign::Positive), (1, Some(5), Sign::Negative)]
/// ).unwrap();
/// assert_eq!(
///     short_word.clone() * tall_letter,
///     Err(WordValidationError::TooLong(u16::MAX as u32 + 8)),
/// );
/// assert_eq!(
///     tall_letter * short_word.clone(),
///     Err(WordValidationError::TooLong(u16::MAX as u32 + 8)),
/// );
/// ```
///
/// 2. Attempting to construct a word with a malformed [letter](Letter)
///    ([`WordValidationError::LetterValidation`]):
///
/// ```
/// use braided::{Word, Sign, WordValidationError};
/// use std::assert_matches;
///
/// assert_matches!(
///     Word::new(vec![(0, Some(3), Sign::Positive)]),
///     Err(WordValidationError::LetterValidation(_)),
/// );
/// assert_matches!(
///     Word::new(vec![(-1, None::<u16>, Sign::Positive)]),
///     Err(WordValidationError::LetterValidation(_)),
/// );
/// assert_matches!(
///     Word::new(vec![(u16::MAX, None::<u16>, Sign::Positive)]),
///     Err(WordValidationError::LetterValidation(_)),
/// );
/// assert_matches!(
///     Word::new(vec![(3, Some(3), Sign::Positive)]),
///     Err(WordValidationError::LetterValidation(_)),
/// );
/// assert_matches!(
///     Word::new(vec![(4, Some(3), Sign::Positive)]),
///     Err(WordValidationError::LetterValidation(_)),
/// );
/// ```
#[derive(Debug, thiserror::Error, PartialEq, Eq, Clone, Copy)]
pub enum WordValidationError {
    /// Occurs when attempting to multiply two [words](Word) whose combined
    /// [Artin length](Word::artin_length) exceeds [`u16::MAX`].
    ///
    /// Wraps the total Artin length.
    #[error("Attempting to create word of length {0} > {max}", max = u16::MAX)]
    TooLong(u32),
    /// Indicates failure to validate one of the [letters](Letter) of the word.
    ///
    /// Transparent wrapper around [`LetterValidationError`].
    #[error(transparent)]
    LetterValidation(#[from] LetterValidationError),
    /// Indicates failure to coerce an integer into [`u16`].
    ///
    /// This variant is only possible when providing a bad exponent to the [word!](crate::word) macro.
    ///
    /// Transparent wrapper around [`std::num::TryFromIntError`].
    #[error(transparent)]
    FromInt(#[from] std::num::TryFromIntError),
    /// This variant exists purely to make the type system happy; it cannot occur in practice.
    ///
    /// Transparent wrapper around [`std::convert::Infallible`].
    #[error(transparent)]
    Infallible(#[from] std::convert::Infallible),
}

/// A formal _word_ in the [letters](Letter) of a braid group.
///
/// # Construction
///
/// <div class="warning">
///
/// Also see the documentation for the [`word!`](crate::word) macro for a more ergonomic way to
/// construct a [`Word`].
///
/// </div>
///
/// 1. From raw letter data, using [Word::new]:
///
/// ```
/// use braided::{Sign, Word};
/// use std::assert_matches;
///
/// let new_word = Word::new(vec![
///     (1, None::<u16>, Sign::Negative),
///     (2, Some(5), Sign::Positive),
///     (3, Some(4), Sign::Negative),
///     (4, None::<u16>, Sign::Positive),
/// ]);
///
/// assert_matches!(new_word, Ok(_));
/// ```
///
/// 2. From a [`Vec<L>`] or `&[L]`, where `L` is a generic bounded by [`Into<Letter>`]. This
///    approach uses the associated function [`Word::try_from`]:
///
/// ```
/// use braided::{Letter, Sign, Word};
///
/// let letter1 = Letter::new(1, None::<u16>, Sign::Positive).unwrap();
/// let letter2 = Letter::new(2, Some(5), Sign::Negative).unwrap();
/// let word_from_letters = Word::try_from(vec![letter1, letter2]);
///
/// assert_eq!(
///     word_from_letters,
///     Word::new(vec![(1, None::<u16>, Sign::Positive), (2, Some(5), Sign::Negative)]),
/// );
/// ```
///
/// 3. Constructing a trivial (empty) word using [`Word::trivial`] (note that [`Default`] is also
///    implemented for [`Word`]: it returns the trivial word):
///
/// ```
/// use braided::Word;
///
/// let trivial = Word::trivial();
///
/// assert_eq!(trivial.len(), 0);
///
/// assert_eq!(trivial, Word::default());
/// ```
///
/// # Decomposition and Coalescence
///
/// Because [`Letter`] abstracts over the underlying generating set, a priori an instance of
/// [`Word`] may consist of a mixture of different [`Letter`] variants. For situations where one
/// needs to ensure that every letter belongs to one generating set or the other, we expose two methods.
///
/// First, [`Word::decompose`] transforms a given [`Word`] by replacing each of its [`Letter::Band`]
/// variants with an equivalent product of [`Letter::Artin`] variants. For more details on how this
/// is accomplished, see the documentation for [`BandGenerator`].
///
/// ```
/// use braided::{Sign, Word};
///
/// let word = Word::new([
///     (1, Some(3), Sign::Positive),
///     (2, None, Sign::Negative),
///     (1, Some(2), Sign::Positive),
/// ])
/// .unwrap();
/// let expected_decomposition = Word::new([
///     (1, None::<u16>, Sign::Negative),
///     (2, None, Sign::Positive),
///     (1, None, Sign::Positive),
///     (2, None, Sign::Negative),
///     (1, None, Sign::Positive),
/// ])
/// .unwrap();
///
/// assert_eq!(word.decompose(), expected_decomposition);
/// ```
///
/// Conversely, [`Word::coalesce`] transforms a given [`Word`] by replacing maximal spans of
/// [`Letter::Artin`] variants with equivalent [`Letter::Band`] generators. If the window about a
/// particular [`Letter::Artin`] variant has radius zero, then the coalesced [`Letter::Band`]
/// variant is simply the Artin variant cast as a band variant (e.g., an underlying
/// `ArtinGenerator { foot: Strand(1), sign: Sign::Positive }` is replaced by
/// `BandGenerator { foot: Strand(1), head: Strand(2), sign: Sign::Positive }`).
///
/// ```
/// use braided::{Sign, Word};
///
/// let word = Word::new([
///     (2, None::<u16>, Sign::Positive),
///     (1, None, Sign::Positive),
///     (2, None, Sign::Negative),
///     (2, None, Sign::Negative),
///     (1, None, Sign::Positive),
/// ])
/// .unwrap();
/// let expected_coalescence = Word::new([
///     (1, Some(3), Sign::Positive),
///     (2, None, Sign::Negative),
///     (1, Some(2), Sign::Positive),
/// ])
/// .unwrap();
///
/// assert_eq!(word.coalesce(), expected_coalescence);
/// ```
///
/// # Convenience Traits - [`IntoIterator`], [`Deref`](std::ops::Deref), and [AsRef]:
///
/// 1. The impl of [`IntoIterator`] for [`Word`] returns an iterator over the underlying letter
///    _data_, not over the [letters](Letter) themselves (see [`Word::letters`] instead):
///
/// ```
/// use braided::{Sign, Word};
///
/// let letter_data = [
///     (1, Some(3), Sign::Positive),
///     (2, None, Sign::Negative),
///     (1, Some(2), Sign::Positive),
/// ];
/// let word = Word::new(letter_data).unwrap();
///
/// for (actual, expected) in word.into_iter().zip(letter_data) {
///     assert_eq!(actual, expected);
/// }
/// ```
///
/// 2. The impl of [`std::ops::Deref`] for [`Word`] returns the contained [letters](`Letter`) as a
///    slice:
///
/// ```
/// use braided::{Sign, Letter, Word};
///
/// let letters = vec![
///     Letter::new(1, Some(3), Sign::Positive).unwrap(),
///     Letter::new(2, None::<u16>, Sign::Negative).unwrap(),
///     Letter::new(1, Some(2), Sign::Positive).unwrap(),
/// ];
/// let word = Word::try_from(letters.clone()).unwrap();
///
/// assert_eq!(*word, *letters.as_slice());
/// ```
///
/// 3. The impl of [`AsRef<[Letter]>`](AsRef) allows for more generic function definitions involving
///    [words](Word):
///
/// ```
/// use braided::{Sign, Letter, Word};
///
/// fn as_ref_tester<W: AsRef<[Letter]>>(w: W, v: &[Letter]) -> bool {
///     w.as_ref() == v
/// }
/// let letters = vec![
///     Letter::new(1, Some(3), Sign::Positive).unwrap(),
///     Letter::new(2, None::<u16>, Sign::Negative).unwrap(),
///     Letter::new(1, Some(2), Sign::Positive).unwrap(),
/// ];
/// let word = Word::try_from(&letters[..]).unwrap();
///
/// assert!(as_ref_tester(word, &letters[..]));
/// ```
///
/// # Multiplication
///
/// 1. Multiplying a [letter](Letter) and a [word](Word):
///
/// ```
/// use braided::{Letter, Sign, Word};
///
/// let data = vec![
///     (1, None::<u16>, Sign::Negative),
///     (2, Some(5), Sign::Positive),
///     (3, Some(4), Sign::Negative),
///     (4, None::<u16>, Sign::Positive),
/// ];
///
/// // letter * word will concatenate the letter to the front of the word:
/// if let Some((letter_data, word_data)) = data.split_first() {
///     let letter = Letter::new(letter_data.0, letter_data.1, letter_data.2).unwrap();
///     let word = Word::new(word_data.to_vec()).unwrap();
///
///     assert_eq!(letter * word, Word::new(data.clone()));
/// }
///
/// // word * letter will concatenate the letter to the end of the word:
/// if let Some((letter_data, word_data)) = data.split_last() {
///     let letter = Letter::new(letter_data.0, letter_data.1, letter_data.2).unwrap();
///     let word = Word::new(word_data.to_vec()).unwrap();
///
///     assert_eq!(word * letter, Word::new(data));
/// }
///
/// // Cancellation is also performed automatically:
///
/// let word = Word::new(vec![
///     (1, None::<u16>, Sign::Positive),
///     (2, Some(5), Sign::Negative),
/// ]).unwrap();
/// let left_letter_inverse = Letter::new(1, None::<u16>, Sign::Negative).unwrap();
/// let right_letter_inverse = Letter::new(2, Some(5), Sign::Positive).unwrap();
///
/// assert_eq!(left_letter_inverse * word.clone(), Word::new(vec![(2, Some(5), Sign::Negative)]));
/// assert_eq!(word * right_letter_inverse, Word::new(vec![(1, None::<u16>, Sign::Positive)]));
/// ```
///
/// 2. Multiplying two [words](Word):
///
/// ```
/// use braided::{Sign, Word};
///
/// let data = vec![
///     (1, None::<u16>, Sign::Negative),
///     (2, Some(5), Sign::Positive),
///     (3, Some(4), Sign::Negative),
///     (4, None::<u16>, Sign::Positive),
/// ];
///
/// let (left_word_data, right_word_data) = data.split_at(2);
/// let left_word = Word::new(left_word_data.to_vec()).unwrap();
/// let right_word = Word::new(right_word_data.to_vec()).unwrap();
///
/// assert_eq!(left_word.clone() * right_word.clone(), Word::new(data));
///
/// // Cancellation is automatically performed:
///
/// assert_eq!(
///     (right_word.inverse() * left_word.inverse()).unwrap() * (left_word * right_word).unwrap(),
///     Ok(Word::trivial()),
/// )
/// ```
///
/// # Accessors and Basic Properties
///
/// The data of a [`Word`] is accessed using the following methods:
///
/// ```
/// use braided::{Letter, Sign, Word};
///
/// let letters = vec![
///     Letter::new(1, None::<u16>, Sign::Negative).unwrap(),
///     Letter::new(2, Some(5), Sign::Positive).unwrap(),
///     Letter::new(3, Some(4), Sign::Negative).unwrap(),
///     Letter::new(4, None::<u16>, Sign::Positive).unwrap(),
/// ];
/// let word = Word::try_from(letters.clone()).unwrap();
///
/// assert_eq!(word.letters(), letters)
/// ```
///
/// Moreover, one may compute various simple properties of a [`Word`]:
///
/// ```
/// use braided::{BraidIndex, Letter, Sign, Word};
///
/// let letters = vec![
///     Letter::new(1, None::<u16>, Sign::Negative).unwrap(),
///     Letter::new(2, Some(5), Sign::Positive).unwrap(),
///     Letter::new(3, Some(4), Sign::Negative).unwrap(),
///     Letter::new(4, None::<u16>, Sign::Positive).unwrap(),
/// ];
/// let word = Word::try_from(letters.clone()).unwrap();
///
/// assert!(!word.is_trivial());
///
/// assert_eq!(word.length(), 4);
///
/// assert_eq!(word.artin_length(), 8);
///
/// assert_eq!(word.minimal_required_braid_index(), BraidIndex::new(5).unwrap());
///
/// assert_eq!(
///     word.inverse(),
///     Word::try_from(
///         letters.iter().rev().map(|l| l.inverse()).collect::<Vec<Letter>>()
///     ).unwrap(),
/// );
/// ```
///
/// # Errors
///
/// All [`Word`] constructors described above are fallible, _including multiplication_. Please see
/// the associated error type [`WordValidationError`] for more details on possible causes of failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Word(Vec<Letter>);

impl Word {
    /// Constructs a new word from an iterable of low-level [`Letter`] data.
    ///
    /// # Examples
    ///
    /// ```
    /// use braided::{Sign, Word};
    /// use std::assert_matches;
    ///
    /// let new_word = Word::new(vec![
    ///     (1, None::<u16>, Sign::Negative),
    ///     (2, Some(5), Sign::Positive),
    ///     (3, Some(4), Sign::Negative),
    ///     (4, None::<u16>, Sign::Positive),
    /// ]);
    ///
    /// assert_matches!(new_word, Ok(_));
    /// ```
    ///
    /// # Errors
    ///
    /// This function returns an error in any of the following circumstances (see the documentation
    /// for the associated error type [`WordValidationError`] for more details and examples):
    ///
    /// 1. The data given for a [letter](Letter) does not pass validation
    ///    ([`WordValidationError::LetterValidation`]).
    /// 2. The total [Artin length](Letter::artin_length) across all constructed [letters](Letter)
    ///    exceeds [`u16::MAX`].
    pub fn new<D, F, H>(letter_data: D) -> Result<Self, WordValidationError>
    where
        D: IntoIterator<Item = (F, Option<H>, Sign)>,
        F: TryInto<u16>,
        H: TryInto<u16>,
        StrandValidationError: From<<F as TryInto<u16>>::Error>
            + From<<H as TryInto<u16>>::Error>
            + From<std::convert::Infallible>,
    {
        let mut letters = Vec::new();
        for (foot, head, sign) in letter_data {
            letters.push(Letter::new(foot, head, sign)?)
        }
        Word::try_from(letters)
    }
    /// Returns a trivial (i.e., empty) [`Word`].
    ///
    /// # Examples
    ///
    /// ```
    /// use braided::Word;
    ///
    /// let trivial = Word::trivial();
    ///
    /// assert_eq!(trivial.len(), 0);
    ///
    /// assert_eq!(trivial, Word::default());
    /// ```
    pub fn trivial() -> Self {
        Self(Vec::new())
    }

    /// Decomposes each [`Letter::Band`] variant in a [`Word`] into an equivalent sequence of
    /// [`Letter::Artin`] variants.
    ///
    /// See the documentation for [`BandGenerator`] for an explanation of this conversion. This
    /// method is _almost_ an inverse of the [`Word::coalesce`] method; the obstruction comes from
    /// the fact that a decomposition into [Artin generators](ArtinGenerator) needn't be unique. To
    /// address this issue, we adopt the convention of decomposing every [band](BandGenerator) into
    /// the product of [Artin generators](ArtinGenerator) which situates the "crossing" generator at
    /// its maximal index (i.e., the [Artin generator](ArtinGenerator) with
    /// [foot strand](ArtinGenerator::foot) at [`band.head() - 1`](BandGenerator::head)).
    ///
    /// # Examples
    ///
    /// ```
    /// use braided::{Sign, Word};
    ///
    /// let word = Word::new([
    ///     (1, Some(3), Sign::Positive),
    ///     (2, None, Sign::Negative),
    ///     (1, Some(2), Sign::Positive),
    /// ])
    /// .unwrap();
    /// let expected_decomposition = Word::new([
    ///     (1, None::<u16>, Sign::Negative),
    ///     (2, None, Sign::Positive),
    ///     (1, None, Sign::Positive),
    ///     (2, None, Sign::Negative),
    ///     (1, None, Sign::Positive),
    /// ])
    /// .unwrap();
    ///
    /// assert_eq!(word.decompose(), expected_decomposition);
    /// ```
    pub fn decompose(&self) -> Self {
        let mut artin_generators: Vec<ArtinGenerator> = Vec::new();
        for letter in self.iter() {
            match letter {
                Letter::Artin(artin_generator) => artin_generators.push(*artin_generator),
                Letter::Band(band_generator) => artin_generators.extend(band_generator.decompose()),
            }
        }
        Self::try_from(artin_generators).unwrap()
    }
    /// Coalesces every maximal span of [`Letter::Artin`] variants within a [`Word`] into an
    /// equivalent [`Letter::Band`].
    ///
    /// See the documentation for [`BandGenerator`] for an explanation of this conversion. Note that
    /// while every band can [decompose](Word::decompose) into several equivalent words of
    /// [Artin generators](ArtinGenerator) (although, as discussed in the documentation to
    /// [Word::decompose], we adopt a convention to deterministically choose a decomposition), any
    /// two equivalent words of [Artin generators](ArtinGenerator) will coalesce into the same
    /// [band](`BandGenerator`).
    ///
    /// # Examples
    ///
    /// ```
    /// use braided::{Sign, Word};
    ///
    /// let word = Word::new([
    ///     (2, None::<u16>, Sign::Positive),
    ///     (1, None, Sign::Positive),
    ///     (2, None, Sign::Negative),
    ///     (2, None, Sign::Negative),
    ///     (1, None, Sign::Positive),
    /// ])
    /// .unwrap();
    /// let expected_coalescence = Word::new([
    ///     (1, Some(3), Sign::Positive),
    ///     (2, None, Sign::Negative),
    ///     (1, Some(2), Sign::Positive),
    /// ])
    /// .unwrap();
    ///
    /// assert_eq!(word.coalesce(), expected_coalescence);
    /// ```
    pub fn coalesce(&self) -> Self {
        // The coalescing algorithm requires that we start from a word which has been completely
        // decomposed as Artin generators.
        self.decompose().coalesce_decomposed()
    }
    fn coalesce_decomposed(&self) -> Self {
        if self.is_trivial() {
            return self.clone();
        }
        let num_letters = self.len(); // guaranteed to be > 0 since word is not trivial

        let mut radius = (num_letters - 1).div_euclid(2);
        let mut pivot = radius; // pivot is the index of the candidate band crossing

        // The following loop returns eventually, since if radius == 0, then window consists
        // of a single Artin generator, which trivially transforms into a band generator.
        loop {
            while pivot + radius < num_letters {
                let remaining_left = Word(self[0..pivot - radius].to_vec());
                let window: Vec<ArtinGenerator> = self[pivot - radius..pivot + radius + 1]
                    .iter()
                    .map(|l| (*l).try_into().unwrap())
                    .collect();
                let remaining_right = Word(self[pivot + radius + 1..num_letters].to_vec());
                if let Ok(band) = BandGenerator::coalesce(&window) {
                    // We can safely unwrap the following products, since we're operating on parts
                    // of a word which has already been length-checked (at its construction)
                    return ((remaining_left.coalesce_decomposed() * Letter::Band(band)).unwrap()
                        * remaining_right.coalesce_decomposed())
                    .unwrap();
                } else {
                    pivot += 1;
                }
            }
            radius -= 1;
        }
    }

    /// Accessor method that returns a copy of the [word's](Word) [letters](Letter).
    ///
    /// # Examples
    ///
    /// ```
    /// use braided::{Letter, Sign, Word};
    ///
    /// let letters = vec![
    ///     Letter::new(1, None::<u16>, Sign::Negative).unwrap(),
    ///     Letter::new(2, Some(5), Sign::Positive).unwrap(),
    ///     Letter::new(3, Some(4), Sign::Negative).unwrap(),
    ///     Letter::new(4, None::<u16>, Sign::Positive).unwrap(),
    /// ];
    /// let word = Word::try_from(letters.clone()).unwrap();
    ///
    /// assert_eq!(word.letters(), letters)
    /// ```
    pub fn letters(&self) -> Vec<Letter> {
        self.0.clone()
    }
    /// Returns a bool indicating whether the word is [trivial](Word::trivial) (i.e., empty).
    ///
    /// # Examples
    ///
    /// ```
    /// use braided::{Letter, Sign, Word};
    ///
    /// let letters = vec![
    ///     Letter::new(1, None::<u16>, Sign::Negative).unwrap(),
    ///     Letter::new(2, Some(5), Sign::Positive).unwrap(),
    ///     Letter::new(3, Some(4), Sign::Negative).unwrap(),
    ///     Letter::new(4, None::<u16>, Sign::Positive).unwrap(),
    /// ];
    /// let word = Word::try_from(letters).unwrap();
    ///
    /// assert!(!word.is_trivial());
    /// ```
    pub fn is_trivial(&self) -> bool {
        self.0.is_empty()
    }
    /// Returns the number of [_letters_](Letter) in the [`Word`].
    ///
    /// Note that we assert an upper bound of [`u16::MAX`] on the
    /// [_Artin length_](Word::artin_length). The letter length returned by this method is typically
    /// less than [Artin length](Word::artin_length), since [band letters](Letter::Band)
    /// represent several contiguous [Artin letters](Letter::Artin) in general.
    ///
    /// # Examples
    ///
    /// ```
    /// use braided::{Letter, Sign, Word};
    ///
    /// let letters = vec![
    ///     Letter::new(1, None::<u16>, Sign::Negative).unwrap(),
    ///     Letter::new(2, Some(5), Sign::Positive).unwrap(),
    ///     Letter::new(3, Some(4), Sign::Negative).unwrap(),
    ///     Letter::new(4, None::<u16>, Sign::Positive).unwrap(),
    /// ];
    /// let word = Word::try_from(letters).unwrap();
    ///
    /// assert_eq!(word.length(), 4);
    /// ```
    pub fn length(&self) -> u16 {
        // length checks taken care of at construction, so unwrapping here is safe
        self.len().try_into().unwrap()
    }
    /// Returns the _equivalent_ number of [Artin letters](Letter::Artin) within the [`Word`].
    ///
    /// One should be careful not to confuse this method with [Word::length], which returns the
    /// number of [letters](Letter) (whether [Artin](Letter::Artin) or [band](Letter::Band)) in the
    /// [`Word`]. Nor should one confuse the return value of this method with the raw count of
    /// [Artin letters](Letter::Artin) in the word (a value for which we expose no method): instead,
    /// this method returns the raw count of [Artin letters](Letter::Artin), _plus_ the equivalent
    /// number of [Artin letters](Letter::Artin) contained within each [Band letter](Letter::Band).
    /// See the examples below for illustration.
    ///
    /// # Examples
    ///
    /// ```
    /// use braided::{Letter, Sign, Word};
    ///
    /// let letters = vec![
    ///     Letter::new(1, None::<u16>, Sign::Negative).unwrap(),
    ///     Letter::new(2, Some(5), Sign::Positive).unwrap(),
    ///     Letter::new(3, Some(4), Sign::Negative).unwrap(),
    ///     Letter::new(4, None::<u16>, Sign::Positive).unwrap(),
    /// ];
    /// let word = Word::try_from(letters).unwrap();
    ///
    /// assert_eq!(word.artin_length(), 8);
    /// ```
    pub fn artin_length(&self) -> u16 {
        self.iter().map(|l| l.artin_length()).sum()
    }
    /// Returns the minimal [`BraidIndex`] required for a [braid](crate::Braid) to make use of this
    /// [`Word`].
    ///
    /// This amounts to the index of the largest [head strand](Letter::head) across all letters of
    /// the [`Word`], or 1 if the word is [trivial](Word::trivial).
    ///
    /// # Examples
    ///
    /// ```
    /// use braided::{BraidIndex, Letter, Sign, Word};
    ///
    /// let letters = vec![
    ///     Letter::new(1, None::<u16>, Sign::Negative).unwrap(),
    ///     Letter::new(2, Some(5), Sign::Positive).unwrap(),
    ///     Letter::new(3, Some(4), Sign::Negative).unwrap(),
    ///     Letter::new(4, None::<u16>, Sign::Positive).unwrap(),
    /// ];
    /// let word = Word::try_from(letters).unwrap();
    ///
    /// assert_eq!(word.minimal_required_braid_index(), BraidIndex::new(5).unwrap());
    /// ```
    pub fn minimal_required_braid_index(&self) -> BraidIndex {
        self.iter()
            .map(|l| l.minimal_required_braid_index())
            .max()
            .unwrap_or(BraidIndex::new(1).unwrap())
    }
    /// Returns the multiplicative inverse of the [`Word`].
    ///
    /// # Examples
    ///
    /// ```
    /// use braided::{Letter, Sign, Word};
    ///
    /// let letters = vec![
    ///     Letter::new(1, None::<u16>, Sign::Negative).unwrap(),
    ///     Letter::new(2, Some(5), Sign::Positive).unwrap(),
    ///     Letter::new(3, Some(4), Sign::Negative).unwrap(),
    ///     Letter::new(4, None::<u16>, Sign::Positive).unwrap(),
    /// ];
    /// let word = Word::try_from(letters.clone()).unwrap();
    ///
    /// assert_eq!(
    ///     word.inverse(),
    ///     Word::try_from(
    ///         letters.iter().rev().map(|l| l.inverse()).collect::<Vec<Letter>>()
    ///     ).unwrap(),
    /// );
    /// ```
    pub fn inverse(&self) -> Self {
        Self(self.iter().rev().map(|l| l.inverse()).collect())
    }
}

impl Default for Word {
    fn default() -> Self {
        Word::trivial()
    }
}

impl<L> TryFrom<Vec<L>> for Word
where
    L: Into<Letter>,
{
    type Error = WordValidationError;
    fn try_from(value: Vec<L>) -> Result<Self, Self::Error> {
        let (total_len, letters) = value
            .into_iter()
            .map(|l| l.into())
            .map(|l| (l.artin_length() as u32, l))
            .fold((0u32, Vec::<Letter>::new()), |mut acc, (al, l)| {
                acc.0 += al;
                acc.1.push(l);
                acc
            });
        if total_len > u16::MAX as u32 {
            Err(WordValidationError::TooLong(total_len))
        } else {
            Ok(Self(letters))
        }
    }
}
impl<L> TryFrom<&[L]> for Word
where
    L: Into<Letter> + std::clone::Clone,
{
    type Error = WordValidationError;
    fn try_from(value: &[L]) -> Result<Self, Self::Error> {
        Self::try_from(value.to_vec())
    }
}

impl IntoIterator for Word {
    type Item = (u16, Option<u16>, Sign);
    type IntoIter = <Vec<(u16, Option<u16>, Sign)> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0
            .iter()
            .map(|l| match l {
                Letter::Artin(artin) => (artin.foot().into(), None, artin.sign()),
                Letter::Band(band) => (band.foot().into(), Some(band.head().into()), band.sign()),
            })
            .collect::<Vec<_>>()
            .into_iter()
    }
}
impl std::ops::Deref for Word {
    type Target = [Letter];

    fn deref(&self) -> &Self::Target {
        &self.0[..]
    }
}
impl AsRef<[Letter]> for Word {
    fn as_ref(&self) -> &[Letter] {
        self
    }
}

impl std::ops::Mul<Letter> for Word {
    type Output = Result<Word, WordValidationError>;
    fn mul(self, rhs: Letter) -> Self::Output {
        if let Some((lhs_last, lhs_initial)) = self.split_last() {
            match *lhs_last * rhs {
                Ok(tail) => Self::try_from([lhs_initial, &tail].concat()),
                Err(WordValidationError::TooLong(tail_length)) => {
                    Err(WordValidationError::TooLong(
                        lhs_initial
                            .iter()
                            .map(|l| l.artin_length() as u32)
                            .sum::<u32>()
                            + tail_length,
                    ))
                }
                Err(_) => panic!("Unexpected error while computing {self:?} * {rhs:?}"),
            }
        } else {
            Self::try_from(vec![rhs])
        }
    }
}
impl std::ops::Mul<Word> for Letter {
    type Output = Result<Word, WordValidationError>;
    fn mul(self, rhs: Word) -> Self::Output {
        if let Some((rhs_first, rhs_tail)) = rhs.split_first() {
            match self * *rhs_first {
                Ok(initial) => Word::try_from([&initial, rhs_tail].concat()),
                Err(WordValidationError::TooLong(initial_length)) => {
                    Err(WordValidationError::TooLong(
                        initial_length
                            + rhs_tail
                                .iter()
                                .map(|l| l.artin_length() as u32)
                                .sum::<u32>(),
                    ))
                }
                Err(_) => panic!("Unexpected error while computing {self:?} * {rhs:?}"),
            }
        } else {
            Word::try_from(vec![self])
        }
    }
}
#[allow(clippy::suspicious_arithmetic_impl)]
impl std::ops::Mul for Word {
    type Output = Result<Word, WordValidationError>;
    fn mul(self, rhs: Self) -> Self::Output {
        let radius =
            match self
                .iter()
                .rev()
                .zip(rhs.iter())
                .try_fold(0usize, |radius, (left, &right)| {
                    if left.inverse() == right {
                        Ok(radius + 1)
                    } else {
                        Err(radius)
                    }
                }) {
                Ok(radius) => radius,
                Err(radius) => radius,
            };
        Self::try_from([&self[..self.len() - radius], &rhs[radius..]].concat())
    }
}

// TODO: impl Mul for result types

#[cfg(test)]
mod tests {
    use crate::{Letter, Sign, Word, WordValidationError};
    use googletest::matchers::{anything, each, eq, err, is_false, is_true, ok, result_of_ref};
    use googletest::{assert_that, expect_that, gtest};

    #[gtest]
    fn construction_with_try_from_succeeds() {
        let letters = vec![
            Letter::new(1, None::<u16>, Sign::Positive).unwrap(),
            Letter::new(2, Some(5), Sign::Negative).unwrap(),
        ];
        let valid_words = [
            Word::try_from(letters.clone()),
            Word::try_from(&letters[..]),
        ];

        expect_that!(valid_words, each(ok(anything())));
        expect_that!(valid_words[0], eq(&valid_words[1]));
    }

    #[test]
    fn valid_construction_with_new_succeeds_and_is_as_expected() {
        let letter_data = [
            (1, None::<u16>, Sign::Positive),
            (2, Some(5), Sign::Negative),
        ];
        let valid_word = Word::new(letter_data);
        assert_that!(valid_word, ok(anything()));

        let letters: Vec<Letter> = letter_data
            .iter()
            .map(|(foot, head, sign)| Letter::new(*foot, *head, *sign).unwrap())
            .collect();
        assert_that!(valid_word, eq(&Word::try_from(letters)));
    }

    #[test]
    fn trivial_works_as_expected() {
        let trivial = Word::trivial();

        assert_that!(
            trivial,
            eq(&Word::new(Vec::<(u16, Option<u16>, _)>::new()).unwrap())
        );
    }

    #[test]
    fn default_word_is_trivial() {
        assert_that!(Word::default(), eq(&Word::trivial()));
    }

    #[test]
    fn decompose_computes_as_expected() {
        let word = Word::new([
            (1, Some(3), Sign::Positive),
            (2, None, Sign::Negative),
            (1, Some(2), Sign::Positive),
        ])
        .unwrap();
        let expected_decomposition = Word::new([
            (1, None::<u16>, Sign::Negative),
            (2, None, Sign::Positive),
            (1, None, Sign::Positive),
            (2, None, Sign::Negative),
            (1, None, Sign::Positive),
        ])
        .unwrap();

        assert_that!(word.decompose(), eq(&expected_decomposition));
    }

    #[test]
    fn coalesce_computes_as_expected() {
        let word = Word::new([
            (2, None::<u16>, Sign::Positive),
            (1, None, Sign::Positive),
            (2, None, Sign::Negative),
            (2, None, Sign::Negative),
            (1, None, Sign::Positive),
        ])
        .unwrap();
        let expected_coalescence = Word::new([
            (1, Some(3), Sign::Positive),
            (2, None, Sign::Negative),
            (1, Some(2), Sign::Positive),
        ])
        .unwrap();

        assert_that!(word.coalesce(), eq(&expected_coalescence));
    }

    #[gtest]
    fn properties_compute_as_expected() {
        let letters = vec![
            Letter::new(1, Some(3), Sign::Positive).unwrap(),
            Letter::new(2, None::<u16>, Sign::Negative).unwrap(),
            Letter::new(1, Some(2), Sign::Positive).unwrap(),
        ];
        let word = Word::try_from(&letters[..]).unwrap();

        expect_that!(word.letters(), eq(&letters));
        expect_that!(
            word.inverse(),
            eq(&Word::try_from(
                letters
                    .iter()
                    .rev()
                    .map(|&l| l.inverse())
                    .collect::<Vec<Letter>>()
            )
            .unwrap())
        );
        expect_that!(word.is_trivial(), is_false());
        expect_that!(Word::trivial().is_trivial(), is_true());
        expect_that!(word.length(), eq(letters.len().try_into().unwrap()));
        expect_that!(
            word.artin_length(),
            eq(letters.iter().map(|&l| l.artin_length()).sum())
        );
        expect_that!(
            word.minimal_required_braid_index(),
            eq(letters
                .iter()
                .map(|&l| l.minimal_required_braid_index())
                .max()
                .unwrap()),
        );
    }

    #[gtest]
    fn into_iterator_works_as_expected() {
        let letter_data = [
            (1, Some(3), Sign::Positive),
            (2, None, Sign::Negative),
            (1, Some(2), Sign::Positive),
        ];
        let word = Word::new(letter_data).unwrap();

        for (actual, expected) in word.into_iter().zip(letter_data) {
            expect_that!(actual, eq(expected));
        }
    }

    #[test]
    fn deref_yields_slice_of_letters() {
        let letters = [
            Letter::new(1, Some(3), Sign::Positive).unwrap(),
            Letter::new(2, None::<u16>, Sign::Negative).unwrap(),
            Letter::new(1, Some(2), Sign::Positive).unwrap(),
        ];
        let word = Word::try_from(&letters[..]).unwrap();

        assert_that!(*word, eq(&letters));
    }

    #[test]
    fn word_can_be_passed_as_ref_to_slice_of_letters() {
        fn as_ref_tester<W: AsRef<[Letter]>>(w: W, v: &[Letter]) -> bool {
            w.as_ref() == v
        }
        let letters = [
            Letter::new(1, Some(3), Sign::Positive).unwrap(),
            Letter::new(2, None::<u16>, Sign::Negative).unwrap(),
            Letter::new(1, Some(2), Sign::Positive).unwrap(),
        ];
        let word = Word::try_from(&letters[..]).unwrap();

        assert_that!(
            word,
            result_of_ref!(|w: &Word| as_ref_tester(w, &letters), is_true()),
        );
    }

    #[gtest]
    fn valid_multiplication_with_letter_succeeds_and_computes_as_expected() {
        let letters = vec![
            Letter::new(1, Some(3), Sign::Positive).unwrap(),
            Letter::new(2, None::<u16>, Sign::Negative).unwrap(),
            Letter::new(1, Some(2), Sign::Positive).unwrap(),
        ];
        let word = Word::try_from(&letters[..]).unwrap();
        let other_letter = Letter::new(3, Some(7), Sign::Negative).unwrap();

        expect_that!(
            word.clone() * other_letter,
            eq(&Word::try_from(
                [letters.clone(), vec![other_letter]].concat()
            ))
        );
        expect_that!(
            other_letter * word,
            eq(&Word::try_from([vec![other_letter], letters].concat()))
        );
    }

    #[gtest]
    fn valid_multiplication_with_word_succeeds_and_computes_as_expected() {
        let letters1 = vec![
            Letter::new(1, Some(3), Sign::Positive).unwrap(),
            Letter::new(2, None::<u16>, Sign::Negative).unwrap(),
            Letter::new(1, Some(2), Sign::Positive).unwrap(),
        ];
        let letters2 = vec![
            Letter::new(3, None::<u16>, Sign::Negative).unwrap(),
            Letter::new(2, Some(7), Sign::Negative).unwrap(),
        ];

        let word1 = Word::try_from(&letters1[..]).unwrap();
        let word2 = Word::try_from(&letters2[..]).unwrap();

        expect_that!(
            word1.clone() * word2.clone(),
            eq(&Word::try_from(
                [letters1.clone(), letters2.clone()].concat()
            ))
        );
        expect_that!(
            word2 * word1,
            eq(&Word::try_from([letters2, letters1].concat()))
        );
    }

    #[gtest]
    fn trivial_word_is_multiplicative_identity() {
        let letter = Letter::new(1, None::<u16>, Sign::Positive).unwrap();

        expect_that!(
            letter * Word::trivial(),
            ok(eq(&Word::try_from(vec![letter]).unwrap()))
        );
        expect_that!(
            Word::trivial() * letter,
            ok(eq(&Word::try_from(vec![letter]).unwrap()))
        );

        let word = Word::try_from(vec![
            Letter::new(1, Some(3), Sign::Positive).unwrap(),
            Letter::new(2, None::<u16>, Sign::Negative).unwrap(),
            Letter::new(1, Some(2), Sign::Positive).unwrap(),
        ])
        .unwrap();

        expect_that!(word.clone() * Word::trivial(), ok(eq(&word)));
        expect_that!(Word::trivial() * word.clone(), ok(eq(&word)))
    }

    #[gtest]
    fn multiplication_with_inverse_yields_trivial() {
        let word = Word::try_from(vec![
            Letter::new(1, Some(3), Sign::Positive).unwrap(),
            Letter::new(2, None::<u16>, Sign::Negative).unwrap(),
            Letter::new(1, Some(2), Sign::Positive).unwrap(),
        ])
        .unwrap();

        expect_that!(word.clone() * word.inverse(), ok(eq(&Word::trivial())));
        expect_that!(word.inverse() * word.clone(), ok(eq(&Word::trivial())));
    }

    #[gtest]
    fn invalid_construction_with_new_fails() {
        let invalid_words = [
            (
                Word::new(vec![
                    (1, None::<u16>, Sign::Negative);
                    u16::MAX as usize + 1
                ]),
                WordValidationError::TooLong(u16::MAX as u32 + 1),
            ),
            (
                Word::new([
                    (1, Some(3), Sign::Positive),
                    (2, None, Sign::Negative),
                    (4, Some(1), Sign::Positive),
                ]),
                WordValidationError::from(Letter::new(4, Some(1), Sign::Positive).err().unwrap()),
            ),
            (
                Word::new([
                    (-1, Some(3), Sign::Positive),
                    (2, None, Sign::Negative),
                    (1, Some(2), Sign::Positive),
                ]),
                WordValidationError::from(Letter::new(-1, Some(3), Sign::Positive).err().unwrap()),
            ),
        ];

        for (invalid_word, error) in invalid_words {
            expect_that!(invalid_word, err(eq(&error)));
        }
    }

    #[gtest]
    fn invalid_construction_with_try_from_fails() {
        let invalid_words = [
            Word::try_from(vec![
                Letter::new(1, None::<u16>, Sign::Negative).unwrap();
                u16::MAX as usize + 1
            ]),
            Word::try_from(
                &[
                    Letter::new(1, None::<u16>, Sign::Negative).unwrap(),
                    Letter::new(1, Some(2u16.pow(15) + 1), Sign::Positive).unwrap(),
                ][..],
            ),
        ];

        for invalid_word in invalid_words {
            expect_that!(
                invalid_word,
                err(eq(&WordValidationError::TooLong(u16::MAX as u32 + 1)))
            );
        }
    }

    #[gtest]
    fn invalid_mult_with_letter_fails() {
        let short_word = Word::new([
            (1, Some(3), Sign::Positive),
            (2, None, Sign::Negative),
            (1, Some(2), Sign::Positive),
        ])
        .unwrap();
        let long_word = Word::try_from(vec![
            Letter::new(1, None::<u16>, Sign::Positive).unwrap();
            u16::MAX as usize
        ])
        .unwrap();
        let short_letter = Letter::new(2, None::<u16>, Sign::Negative).unwrap();
        let tall_letter = Letter::new(1, Some(2usize.pow(15) + 1), Sign::Positive).unwrap();

        let invalid_products = [
            (
                short_word.clone() * tall_letter,
                u16::MAX as u32 + 5,
                "short_word * tall_letter",
            ),
            (
                tall_letter * short_word,
                u16::MAX as u32 + 5,
                "tall_letter * short_word",
            ),
            (
                long_word.clone() * short_letter,
                u16::MAX as u32 + 1,
                "long_word * short_letter",
            ),
            (
                short_letter * long_word.clone(),
                u16::MAX as u32 + 1,
                "short_letter * long_word",
            ),
            (
                long_word.clone() * tall_letter,
                2 * (u16::MAX as u32),
                "long_word * tall_letter",
            ),
            (
                tall_letter * long_word,
                2 * (u16::MAX as u32),
                "tall_letter * long_word",
            ),
        ];

        for (invalid_product, length, label) in invalid_products {
            expect_that!(
                invalid_product,
                err(eq(&WordValidationError::TooLong(length))),
                "{label}",
            );
        }
    }

    #[gtest]
    fn invalid_mult_with_word_fails() {
        let short_word = Word::new([
            (1, Some(3), Sign::Positive),
            (2, None, Sign::Negative),
            (1, Some(2), Sign::Positive),
        ])
        .unwrap();
        let long_word = Word::try_from(vec![
            Letter::new(1, None::<u16>, Sign::Positive).unwrap();
            u16::MAX as usize
        ])
        .unwrap();

        let invalid_products = [
            (
                short_word.clone() * long_word.clone(),
                u16::MAX as u32 + 5,
                "short_word * long_word",
            ),
            (
                long_word.clone() * short_word.clone(),
                u16::MAX as u32 + 5,
                "long_word * short_word",
            ),
            (
                long_word.clone() * long_word.clone(),
                2 * (u16::MAX as u32),
                "long_word * long_word",
            ),
        ];
        for (invalid_product, length, label) in invalid_products {
            expect_that!(
                invalid_product,
                err(eq(&WordValidationError::TooLong(length))),
                "{label}",
            );
        }
    }
}
