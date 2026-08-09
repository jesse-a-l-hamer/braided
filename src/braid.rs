use crate::{
    BraidIndex, BraidResult, BraidValidationError, IndexValidationError, Letter, Sign,
    StrandValidationError, Word,
};

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
/// use braided::{Braid, Sign, Word};
/// use std::assert_matches;
///
/// // Use Braid::new if you already have a valid Word
///
/// let word = Word::new(vec![
///     (1, None::<u16>, Sign::Positive),
///     (2, Some(5), Sign::Negative),
///     (3, None::<u16>, Sign::Negative),
///     (4, Some(5), Sign::Positive),
/// ])
/// .unwrap();
///
/// // The braid index can be inferred:
/// assert_matches!(Braid::new(None::<u16>, word.clone()), Ok(_)); // braid index is 5
///
/// // Or you can explicitly specify the braid index:
/// assert_matches!(Braid::new(Some(10), word), Ok(_))
/// ```
///
/// 2. Using [`Braid::from_data`]
///
/// ```
/// use braided::{Braid, Sign, Word};
/// use std::assert_matches;
///
/// // Use Braid::from_data to construct a braid directly from letter-data
///
/// let letter_data = vec![
///     (1, None::<u16>, Sign::Positive),
///     (2, Some(5), Sign::Negative),
///     (3, None::<u16>, Sign::Negative),
///     (4, Some(5), Sign::Positive),
/// ];
///
/// // The braid index can be inferred:
/// assert_matches!(Braid::from_data(None::<u16>, letter_data.clone()), Ok(_)); // braid index is 5
///
/// // Or you can explicitly specify the braid index:
/// assert_matches!(Braid::from_data(Some(10), letter_data), Ok(_))
/// ```
///
/// 3. Using [`Braid::from`]
///
/// ```
/// use braided::{Braid, Sign, Word};
///
/// let valid_word = Word::new(vec![
///     (1, None::<u16>, Sign::Positive),
///     (2, Some(5), Sign::Negative),
///     (3, None::<u16>, Sign::Negative),
///     (4, Some(5), Sign::Positive),
/// ])
/// .unwrap();
///
/// let braid_from_borrow = Braid::from(&valid_word);
///
/// assert_eq!(braid_from_borrow.word(), valid_word.clone());
///
/// // The braid index is inferred from the word
/// let braid_from_move = Braid::from(valid_word.clone());
///
/// assert_eq!(braid_from_move.braid_index(), valid_word.minimal_required_braid_index());
/// ```
///
/// 4. Using [`Braid::try_from`]
///
/// ```
/// use braided::{Braid, Letter, Sign, Word};
/// use std::assert_matches;
///
/// // Use Braid::from_data to construct a braid directly from letter-data
///
/// let letters = vec![
///     Letter::new(1, None::<u16>, Sign::Positive).unwrap(),
///     Letter::new(2, Some(5), Sign::Negative).unwrap(),
///     Letter::new(3, None::<u16>, Sign::Negative).unwrap(),
///     Letter::new(4, Some(5), Sign::Positive).unwrap(),
/// ];
///
/// // The braid index is automatically inferred
/// assert_matches!(Braid::try_from(letters), Ok(_))
/// ```
///
/// 5. Using [`Braid::trivial`]
///
/// ```
/// use braided::{Braid, Word};
///
/// // Consruct the trivial braid of a given index:
/// assert_eq!(Braid::trivial(10), Braid::new(Some(10), Word::trivial()));
/// ```
///
/// 6. Using [`Braid::default`]
///
/// ```
/// use braided::{Braid, Word};
///
/// // The default braid is a trivial unknot:
///
/// assert_eq!(Braid::default(), Braid::new(Some(1), Word::trivial()).unwrap());
/// ```
///
/// # [Decomposition](Braid::decompose) and [Coalescing](Braid::coalesce)
///
/// [`Braid::decompose`] returns an equivalent braid each of whose [band letters](Letter::Band) has
/// been decomposed into a product of [Artin letters](Letter::Artin):
///
/// ```
/// use braided::{Braid, Sign};
/// let braid = Braid::from_data(
///     None::<u16>,
///     [
///         (1, Some(3), Sign::Positive),
///         (2, None, Sign::Negative),
///         (1, Some(2), Sign::Positive),
///     ],
/// )
/// .unwrap();
/// let expected_decomposition = Braid::from_data(
///     None::<u16>,
///     [
///         (1, None::<u16>, Sign::Negative),
///         (2, None, Sign::Positive),
///         (1, None, Sign::Positive),
///         (2, None, Sign::Negative),
///         (1, None, Sign::Positive),
///     ],
/// ).unwrap();
///
/// assert_eq!(braid.decompose(), expected_decomposition);
/// ```
///
/// [`Braid::coalesce`] returns an equivalent braid by "coalescing" maximal spans of
/// [Artin letters](Letter::Artin) into [band letters](Letter::Band):
///
/// ```
/// use braided::{Braid, Sign};
///
/// let braid = Braid::from_data(
///     None::<u16>,
///     [
///         (2, None::<u16>, Sign::Positive),
///         (1, None, Sign::Positive),
///         (2, None, Sign::Negative),
///         (2, None, Sign::Negative),
///         (1, None, Sign::Positive),
///     ],
/// )
/// .unwrap();
/// let expected_coalescence = Braid::from_data(
///     None::<u16>,
///     [
///         (1, Some(3), Sign::Positive),
///         (2, None, Sign::Negative),
///         (1, Some(2), Sign::Positive),
///     ],
/// )
/// .unwrap();
///
/// assert_eq!(braid.coalesce(), expected_coalescence);
/// ```
///
/// # Convenience Traits - [`IntoIterator`], [`Deref`](std::ops::Deref), and [`AsRef`]
///
/// The implementation of the [`IntoIterator`] trait allows iterating over the underlying _letter
/// data_ of the braid. (Use [Braid::letters] if you're looking for an iterable of the
/// underlying [letters](Letter)).
///
/// ```
/// use braided::{Braid, Sign};
///
/// let letters_data: Vec<(u16, Option<u16>, Sign)> = vec![
///     (1, None, Sign::Positive),
///     (2, Some(5), Sign::Negative),
///     (3, None, Sign::Negative),
///     (4, Some(5), Sign::Positive),
/// ];
///
/// let braid = Braid::from_data(None::<u16>, letters_data.clone()).unwrap();
///
/// for (actual, expected) in braid.into_iter().zip(letters_data) {
///     assert_eq!(actual, expected);
/// }
/// ```
///
/// The implementation of the [`Deref`](std::ops::Deref) trait allows dereferencing a [`Braid`] into
/// a slice of [letters](Letter):
///
/// ```
/// use braided::{Braid, Letter, Sign};
///
/// let letters = [
///     Letter::new(1, Some(3), Sign::Positive).unwrap(),
///     Letter::new(2, None::<u16>, Sign::Negative).unwrap(),
///     Letter::new(1, Some(2), Sign::Positive).unwrap(),
/// ];
/// let braid = Braid::try_from(&letters[..]).unwrap();
///
/// assert_eq!(*braid, letters[..]);
/// ```
///
/// The implementation of [`AsRef<Letter>`] allows for passing Braids to functions that only need a
/// shared reference to a slice of [letters](Letter):
///
/// ```
/// use braided::{Braid, Letter, Sign};
///
/// fn as_ref_tester<B: AsRef<[Letter]>>(b: B, v: &[Letter]) -> bool {
///     b.as_ref() == v
/// }
/// let letters = [
///     Letter::new(1, Some(3), Sign::Positive).unwrap(),
///     Letter::new(2, None::<u16>, Sign::Negative).unwrap(),
///     Letter::new(1, Some(2), Sign::Positive).unwrap(),
/// ];
/// let braid = Braid::try_from(&letters[..]).unwrap();
///
/// assert!(as_ref_tester(&braid, &letters));
/// ```
///
/// # Accessors and Basic Properties
///
/// The underlying data of the [`Braid`] may be accessed as follows:
///
/// ```
/// use braided::{Braid, BraidIndex, Letter, Sign, Word};
///
/// let word = Word::try_from(vec![
///     Letter::new(1, Some(3), Sign::Positive).unwrap(),
///     Letter::new(2, None::<u16>, Sign::Negative).unwrap(),
///     Letter::new(1, Some(2), Sign::Positive).unwrap(),
/// ])
/// .unwrap();
/// let braid = Braid::new(Some(9), word.clone()).unwrap();
///
/// assert_eq!(braid.word(), word.clone());
/// assert_eq!(braid.letters(), word.letters());
/// assert_eq!(braid.braid_index(), BraidIndex::new(9).unwrap());
/// ```
///
/// One may also compute several basic [`Braid`] properties:
///
/// ```
/// use braided::{Braid, BraidIndex, Letter, Sign, Word};
///
/// let word = Word::try_from(vec![
///     Letter::new(1, Some(3), Sign::Positive).unwrap(),
///     Letter::new(2, None::<u16>, Sign::Negative).unwrap(),
///     Letter::new(1, Some(2), Sign::Positive).unwrap(),
/// ])
/// .unwrap();
/// let braid = Braid::new(Some(9), word.clone()).unwrap();
///
/// assert_eq!(
///     braid.inverse(),
///     Braid::new(Some(9), word.inverse()).unwrap(),
/// );
/// assert!(!braid.is_trivial());
/// assert!(Braid::trivial(9).unwrap().is_trivial());
/// assert_eq!(braid.letter_length(), word.length());
/// assert_eq!(braid.artin_length(), word.artin_length());
/// assert_eq!(
///     braid.writhe(),
///     word.iter().fold(0i32, |writhe, l| {
///         if l.sign() == Sign::Positive {
///             writhe + 1
///         } else {
///             writhe - 1
///         }
///     })
/// );
/// assert_eq!(
///     braid.minimal_required_braid_index(),
///     word.minimal_required_braid_index(),
/// );
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
/// use braided::{Braid, Letter, Sign, Word};
///
/// let letters = vec![
///     Letter::new(1, Some(3), Sign::Positive).unwrap(),
///     Letter::new(2, None::<u16>, Sign::Negative).unwrap(),
///     Letter::new(2, Some(8), Sign::Positive).unwrap(),
/// ];
/// let braid = Braid::try_from(&letters[..]).unwrap();
///
/// // One may multiply a braid and a letter:
/// let other_letter = Letter::new(3, Some(7), Sign::Negative).unwrap();
/// assert_eq!(&braid * other_letter, Braid::try_from([letters, vec![other_letter]].concat()));
///
/// // Or a braid and a word:
/// let some_word = Word::try_from(vec![
///     Letter::new(3, None::<u16>, Sign::Negative).unwrap(),
///     Letter::new(2, Some(7), Sign::Negative).unwrap(),
/// ])
/// .unwrap();
/// assert_eq!(
///     &some_word * &braid,
///     Braid::new(None::<u16>, (&some_word * braid.word()).unwrap()),
/// );
///
/// // Or two braids, as long as their braid indexes are equal:
/// let other_braid = Braid::new(Some(8), some_word).unwrap();
///
/// assert_eq!(
///     braid * other_braid,
///     Braid::from_data(
///         None::<u16>,
///         vec![
///             (1, Some(3), Sign::Positive),
///             (2, None::<u16>, Sign::Negative),
///             (2, Some(8), Sign::Positive),
///             (3, None::<u16>, Sign::Negative),
///             (2, Some(7), Sign::Negative),
///         ],
///     ),
/// );
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
    /// use braided::{Braid, Sign, Word};
    /// use std::assert_matches;
    ///
    /// // Use Braid::new if you already have a valid Word
    ///
    /// let word = Word::new(vec![
    ///     (1, None::<u16>, Sign::Positive),
    ///     (2, Some(5), Sign::Negative),
    ///     (3, None::<u16>, Sign::Negative),
    ///     (4, Some(5), Sign::Positive),
    /// ])
    /// .unwrap();
    ///
    /// // The braid index can be inferred:
    /// assert_matches!(Braid::new(None::<u16>, word.clone()), Ok(_)); // braid index is 5
    ///
    /// // Or you can explicitly specify the braid index:
    /// assert_matches!(Braid::new(Some(10), word), Ok(_))
    /// ```
    ///
    /// # Errors
    ///
    /// See the documentation for the associated error type [`BraidValidationError`] for more
    /// information.
    pub fn try_new<N>(index: Option<N>, word: Word) -> BraidResult
    where
        N: TryInto<u16>,
        IndexValidationError: From<<N as TryInto<u16>>::Error>,
    {
        let minimal_required_index = word.minimal_required_braid_index();
        let index = if let Some(index) = index {
            match *BraidIndex::try_new(index) {
                Ok(index) => index,
                Err(e) => return BraidResult::from(BraidValidationError::from(e)),
            }
        } else {
            minimal_required_index
        };

        if index < minimal_required_index {
            BraidResult::from(BraidValidationError::IndexTooSmall {
                index,
                minimal_required_index,
            })
        } else {
            BraidResult::from(Self { index, word })
        }
    }
    pub fn try_from_letters<L>(letters: &[L]) -> BraidResult
    where
        L: Into<Letter> + Clone + Copy,
    {
        let word_result = Word::try_from_letters(letters);
        let word = match &*word_result {
            Ok(word) => word,
            Err(e) => return BraidResult::from(BraidValidationError::from(*e)),
        };
        let index = word.minimal_required_braid_index();

        BraidResult::from(Self {
            index,
            word: word.clone(),
        })
    }
    /// Constructs a [`Braid`] from an optional [`BraidIndex`] and an iterable of [`Word`] data.
    ///
    /// The input data to this function is identical to that of the [`Word::new`] constructor,
    /// except for the [`index`](BraidIndex) argument.
    ///
    /// # Examples
    ///
    /// ```
    /// use braided::{Braid, Sign, Word};
    /// use std::assert_matches;
    ///
    /// // Use Braid::from_data to construct a braid directly from letter-data
    ///
    /// let letter_data = vec![
    ///     (1, None::<u16>, Sign::Positive),
    ///     (2, Some(5), Sign::Negative),
    ///     (3, None::<u16>, Sign::Negative),
    ///     (4, Some(5), Sign::Positive),
    /// ];
    ///
    /// // The braid index can be inferred:
    /// assert_matches!(Braid::from_data(None::<u16>, letter_data.clone()), Ok(_)); // braid index is 5
    ///
    /// // Or you can explicitly specify the braid index:
    /// assert_matches!(Braid::from_data(Some(10), letter_data), Ok(_))
    /// ```
    ///
    /// # Errors
    ///
    /// See the documentation for the associated error type [`BraidValidationError`] for more
    /// information.
    pub fn from_data<N, D, F, H>(index: Option<N>, word_data: D) -> BraidResult
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
        let word_result = Word::try_new(word_data);
        let word = match &*word_result {
            Ok(word) => word,
            Err(e) => return BraidResult::from(BraidValidationError::from(*e)),
        };
        Self::try_new(index, word.clone())
    }
    /// Constructs the trivial [braid](Braid) of the given [index](BraidIndex).
    ///
    /// Serves as the multiplicative identity.
    ///
    /// # Examples
    ///
    /// ```
    /// use braided::{Braid, Word};
    ///
    /// // Consruct the trivial braid of a given index:
    /// assert_eq!(Braid::trivial(10), Braid::new(Some(10), Word::trivial()));
    /// ```
    ///
    /// # Errors
    ///
    /// See the documentation for the associated error type [`BraidValidationError`] for more
    /// information.
    pub fn trivial<N>(index: N) -> BraidResult
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
    /// ```
    /// use braided::{Braid, Sign};
    /// let braid = Braid::from_data(
    ///     None::<u16>,
    ///     [
    ///         (1, Some(3), Sign::Positive),
    ///         (2, None, Sign::Negative),
    ///         (1, Some(2), Sign::Positive),
    ///     ],
    /// )
    /// .unwrap();
    /// let expected_decomposition = Braid::from_data(
    ///     None::<u16>,
    ///     [
    ///         (1, None::<u16>, Sign::Negative),
    ///         (2, None, Sign::Positive),
    ///         (1, None, Sign::Positive),
    ///         (2, None, Sign::Negative),
    ///         (1, None, Sign::Positive),
    ///     ],
    /// ).unwrap();
    ///
    /// assert_eq!(braid.decompose(), expected_decomposition);
    /// ```
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
    /// ```
    /// use braided::{Braid, Sign};
    ///
    /// let braid = Braid::from_data(
    ///     None::<u16>,
    ///     [
    ///         (2, None::<u16>, Sign::Positive),
    ///         (1, None, Sign::Positive),
    ///         (2, None, Sign::Negative),
    ///         (2, None, Sign::Negative),
    ///         (1, None, Sign::Positive),
    ///     ],
    /// )
    /// .unwrap();
    /// let expected_coalescence = Braid::from_data(
    ///     None::<u16>,
    ///     [
    ///         (1, Some(3), Sign::Positive),
    ///         (2, None, Sign::Negative),
    ///         (1, Some(2), Sign::Positive),
    ///     ],
    /// )
    /// .unwrap();
    ///
    /// assert_eq!(braid.coalesce(), expected_coalescence);
    /// ```
    pub fn coalesce(&self) -> Self {
        Self {
            index: self.index,
            word: self.word.coalesce(),
        }
    }

    /// Accessor method to the contained [`BraidIndex`].
    ///
    /// # Examples
    ///
    /// ```
    /// use braided::{Braid, Letter, Sign, Word};
    ///
    /// let word = Word::try_from(vec![
    ///     Letter::new(1, Some(3), Sign::Positive).unwrap(),
    ///     Letter::new(2, None::<u16>, Sign::Negative).unwrap(),
    ///     Letter::new(1, Some(2), Sign::Positive).unwrap(),
    /// ])
    /// .unwrap();
    /// let braid = Braid::new(Some(9), word.clone()).unwrap();
    ///
    /// assert_eq!(braid.letters(), word.letters());
    /// ```
    pub fn braid_index(&self) -> BraidIndex {
        self.index
    }
    /// Accessor method to (a clone of the) the underlying [`Word`] contained in the [`Braid`].
    ///
    /// # Examples
    ///
    /// ```
    /// use braided::{Braid, BraidIndex, Letter, Sign, Word};
    ///
    /// let word = Word::try_from(vec![
    ///     Letter::new(1, Some(3), Sign::Positive).unwrap(),
    ///     Letter::new(2, None::<u16>, Sign::Negative).unwrap(),
    ///     Letter::new(1, Some(2), Sign::Positive).unwrap(),
    /// ])
    /// .unwrap();
    /// let braid = Braid::new(Some(9), word.clone()).unwrap();
    ///
    /// assert_eq!(braid.braid_index(), BraidIndex::new(9).unwrap());
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
    /// use braided::{Braid, Letter, Sign, Word};
    ///
    /// let word = Word::try_from(vec![
    ///     Letter::new(1, Some(3), Sign::Positive).unwrap(),
    ///     Letter::new(2, None::<u16>, Sign::Negative).unwrap(),
    ///     Letter::new(1, Some(2), Sign::Positive).unwrap(),
    /// ])
    /// .unwrap();
    /// let braid = Braid::new(Some(9), word.clone()).unwrap();
    ///
    /// assert_eq!(braid.word(), word.clone());
    /// ```
    pub fn letters(&self) -> Vec<Letter> {
        self.word.letters()
    }

    /// Computes the minimal [`BraidIndex`] required for the [braid's](Braid) [word](Word) to exist.
    ///
    /// Note that this is not necessarily the same as the actual [Braid::braid_index()].
    ///
    /// # Examples
    ///
    /// ```
    /// use braided::{Braid, BraidIndex, Letter, Sign, Word};
    ///
    /// let word = Word::try_from(vec![
    ///     Letter::new(1, Some(3), Sign::Positive).unwrap(),
    ///     Letter::new(2, None::<u16>, Sign::Negative).unwrap(),
    ///     Letter::new(1, Some(2), Sign::Positive).unwrap(),
    /// ])
    /// .unwrap();
    /// let braid = Braid::new(Some(9), word.clone()).unwrap();
    ///
    /// assert_eq!(
    ///     braid.minimal_required_braid_index(),
    ///     word.minimal_required_braid_index(),
    /// );
    /// ```
    pub fn minimal_required_braid_index(&self) -> BraidIndex {
        self.word.minimal_required_braid_index()
    }
    /// Computes the sum of all [signs](Sign) across the [braid's](Braid) [word](Word).
    ///
    /// # Examples
    ///
    /// ```
    /// use braided::{Braid, Letter, Sign, Word};
    ///
    /// let word = Word::try_from(vec![
    ///     Letter::new(1, Some(3), Sign::Positive).unwrap(),
    ///     Letter::new(2, None::<u16>, Sign::Negative).unwrap(),
    ///     Letter::new(1, Some(2), Sign::Positive).unwrap(),
    /// ])
    /// .unwrap();
    /// let braid = Braid::new(Some(9), word.clone()).unwrap();
    ///
    /// assert_eq!(
    ///     braid.writhe(),
    ///     word.iter().fold(0i32, |writhe, l| {
    ///         if l.sign() == Sign::Positive {
    ///             writhe + 1
    ///         } else {
    ///             writhe - 1
    ///         }
    ///     })
    /// );
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
    /// use braided::{Braid, Letter, Sign, Word};
    ///
    /// let word = Word::try_from(vec![
    ///     Letter::new(1, Some(3), Sign::Positive).unwrap(),
    ///     Letter::new(2, None::<u16>, Sign::Negative).unwrap(),
    ///     Letter::new(1, Some(2), Sign::Positive).unwrap(),
    /// ])
    /// .unwrap();
    /// let braid = Braid::new(Some(9), word.clone()).unwrap();
    ///
    /// assert_eq!(braid.letter_length(), word.length());
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
    /// use braided::{Braid, Letter, Sign, Word};
    ///
    /// let word = Word::try_from(vec![
    ///     Letter::new(1, Some(3), Sign::Positive).unwrap(),
    ///     Letter::new(2, None::<u16>, Sign::Negative).unwrap(),
    ///     Letter::new(1, Some(2), Sign::Positive).unwrap(),
    /// ])
    /// .unwrap();
    /// let braid = Braid::new(Some(9), word.clone()).unwrap();
    ///
    /// assert_eq!(braid.artin_length(), word.artin_length());
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
    /// use braided::{Braid, Letter, Sign, Word};
    ///
    /// let word = Word::try_from(vec![
    ///     Letter::new(1, Some(3), Sign::Positive).unwrap(),
    ///     Letter::new(2, None::<u16>, Sign::Negative).unwrap(),
    ///     Letter::new(1, Some(2), Sign::Positive).unwrap(),
    /// ])
    /// .unwrap();
    /// let braid = Braid::new(Some(9), word.clone()).unwrap();
    ///
    /// assert_eq!(
    ///     braid.inverse(),
    ///     Braid::new(Some(9), word.inverse()).unwrap(),
    /// );
    /// ```
    pub fn inverse(&self) -> Self {
        Self {
            index: self.index,
            word: self.word.inverse(),
        }
    }
    /// Returns a bool indicating whether the [`Braid`] is the [trivial braid](Braid::trivial) for
    /// its [index](BraidIndex).
    ///
    /// # Examples
    ///
    /// ```
    /// use braided::{Braid, Letter, Sign, Word};
    ///
    /// let word = Word::try_from(vec![
    ///     Letter::new(1, Some(3), Sign::Positive).unwrap(),
    ///     Letter::new(2, None::<u16>, Sign::Negative).unwrap(),
    ///     Letter::new(1, Some(2), Sign::Positive).unwrap(),
    /// ])
    /// .unwrap();
    /// let braid = Braid::new(Some(9), word.clone()).unwrap();
    ///
    /// assert!(!braid.is_trivial());
    /// assert!(Braid::trivial(9).unwrap().is_trivial());
    /// ```
    pub fn is_trivial(&self) -> bool {
        self.word.is_trivial()
    }
}

impl Default for Braid {
    fn default() -> Self {
        Self::trivial(1).clone_unwrap()
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

#[cfg(test)]
mod tests {
    use crate::{Braid, BraidIndex, BraidResult, BraidValidationError, Letter, Sign, Word};
    use googletest::matchers::{anything, eq, err, is_false, is_true, ok, result_of_ref};
    use googletest::{assert_that, expect_that, gtest};

    #[gtest]
    fn construction_from_a_valid_word_works_as_expected() {
        let valid_word = Word::try_new(vec![
            (1, None::<u16>, Sign::Positive),
            (2, Some(5), Sign::Negative),
            (3, None::<u16>, Sign::Negative),
            (4, Some(5), Sign::Positive),
        ])
        .clone_unwrap();

        let braid_index = valid_word.minimal_required_braid_index();

        let braid_from_borrow = Braid::from(&valid_word);
        expect_that!(braid_from_borrow.word(), eq(&valid_word));

        let braid_from_move = Braid::from(&valid_word);
        expect_that!(braid_from_move.braid_index(), eq(braid_index));
    }

    #[gtest]
    fn valid_construction_with_new_is_successful() {
        let word = Word::try_new(vec![
            (1, None::<u16>, Sign::Positive),
            (2, Some(5), Sign::Negative),
            (3, None::<u16>, Sign::Negative),
            (4, Some(5), Sign::Positive),
        ])
        .clone_unwrap();

        let valid_braids = [
            Braid::try_new(None::<u16>, word.clone()),
            Braid::try_new(Some(10), word),
        ];

        for valid_braid in valid_braids {
            expect_that!(*valid_braid, ok(anything()));
        }
    }

    #[gtest]
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

        for valid_braid in valid_braids {
            expect_that!(*valid_braid, ok(anything()));
        }
    }

    #[test]
    fn valid_construction_with_try_from_letters_is_successful() {
        let letters = vec![
            Letter::try_new(1, None::<u16>, Sign::Positive).unwrap(),
            Letter::try_new(2, Some(5), Sign::Negative).unwrap(),
            Letter::try_new(3, None::<u16>, Sign::Negative).unwrap(),
            Letter::try_new(4, Some(5), Sign::Positive).unwrap(),
        ];

        let valid_braid = Braid::try_from_letters(&letters);

        assert_that!(*valid_braid, ok(anything()));
    }

    #[test]
    fn vaild_construction_of_trivial_braid_is_successful_and_works_as_expected() {
        let braid_index = 9;

        let trivial_braid = Braid::trivial(braid_index);

        assert_that!(*trivial_braid, ok(anything()));

        assert_that!(
            trivial_braid,
            eq(&Braid::try_new(Some(braid_index), Word::trivial()))
        );
    }

    #[test]
    fn default_braid_is_trivial_unknot() {
        let default_braid = Braid::default();

        assert_that!(default_braid, eq(&Braid::trivial(1).clone_unwrap()));
    }

    #[gtest]
    fn into_iterator_returns_a_vector_of_underlying_letter_data() {
        let letters_data: Vec<(u16, Option<u16>, Sign)> = vec![
            (1, None, Sign::Positive),
            (2, Some(5), Sign::Negative),
            (3, None, Sign::Negative),
            (4, Some(5), Sign::Positive),
        ];

        let braid = Braid::from_data(None::<u16>, letters_data.clone()).clone_unwrap();

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
        .clone_unwrap();
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
        .clone_unwrap();

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
        .clone_unwrap();
        let expected_coalescence = Braid::from_data(
            None::<u16>,
            [
                (1, Some(3), Sign::Positive),
                (2, None, Sign::Negative),
                (1, Some(2), Sign::Positive),
            ],
        )
        .clone_unwrap();

        assert_that!(braid.coalesce(), eq(&expected_coalescence));
    }

    #[test]
    fn deref_to_slice_of_letters_works_as_expected() {
        let letters = [
            Letter::try_new(1, Some(3), Sign::Positive).unwrap(),
            Letter::try_new(2, None::<u16>, Sign::Negative).unwrap(),
            Letter::try_new(1, Some(2), Sign::Positive).unwrap(),
        ];
        let braid = Braid::try_from_letters(&letters).clone_unwrap();

        assert_that!(*braid, eq(&letters));
    }

    #[test]
    fn can_pass_braid_as_ref_where_ref_to_letter_slice_is_expected() {
        fn as_ref_tester<B: AsRef<[Letter]>>(b: B, v: &[Letter]) -> bool {
            b.as_ref() == v
        }
        let letters = [
            Letter::try_new(1, Some(3), Sign::Positive).unwrap(),
            Letter::try_new(2, None::<u16>, Sign::Negative).unwrap(),
            Letter::try_new(1, Some(2), Sign::Positive).unwrap(),
        ];
        let braid = Braid::try_from_letters(&letters).clone_unwrap();

        assert_that!(
            braid,
            result_of_ref!(|b: &Braid| as_ref_tester(b, &letters), is_true()),
        );
    }

    #[gtest]
    fn properties_compute_as_expected() {
        let word = Word::try_from_letters(&[
            Letter::try_new(1, Some(3), Sign::Positive).unwrap(),
            Letter::try_new(2, None::<u16>, Sign::Negative).unwrap(),
            Letter::try_new(1, Some(2), Sign::Positive).unwrap(),
        ])
        .clone_unwrap();
        let braid = Braid::try_new(Some(9), word.clone()).clone_unwrap();

        expect_that!(braid.word(), eq(&word));
        expect_that!(braid.letters(), eq(&word.letters()));
        expect_that!(
            braid.inverse(),
            eq(&Braid::try_new(Some(9), word.inverse()).clone_unwrap())
        );
        expect_that!(braid.is_trivial(), is_false());
        expect_that!(Braid::trivial(9).clone_unwrap().is_trivial(), is_true());
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
        expect_that!(braid.braid_index(), eq(BraidIndex::try_new(9).unwrap()),);
    }

    #[gtest]
    fn invalid_construction_with_new_fails_as_expected() {
        let invalid_braids: Vec<(BraidResult, BraidValidationError, &'static str)> = vec![
            (
                Braid::try_new(
                    Some(3),
                    Word::try_new(vec![(1, Some(5), Sign::Positive)]).clone_unwrap(),
                ),
                BraidValidationError::IndexTooSmall {
                    index: BraidIndex::try_new(3).unwrap(),
                    minimal_required_index: BraidIndex::try_new(5).unwrap(),
                },
                "index too small",
            ),
            (
                Braid::try_new(
                    Some(0),
                    Word::try_new(vec![(1, Some(5), Sign::Positive)]).clone_unwrap(),
                ),
                BraidValidationError::from(BraidIndex::try_new(0).err().unwrap()),
                "index validation failure",
            ),
        ];

        for (invalid_braid, error, label) in invalid_braids {
            expect_that!(*invalid_braid, err(eq(&error)), "{label}")
        }
    }

    #[gtest]
    fn invalid_construction_with_from_data_fails_as_expected() {
        let invalid_braids: Vec<(BraidResult, BraidValidationError, &'static str)> = vec![
            (
                Braid::from_data(Some(3), vec![(1, Some(5), Sign::Positive)]),
                BraidValidationError::IndexTooSmall {
                    index: BraidIndex::try_new(3).unwrap(),
                    minimal_required_index: BraidIndex::try_new(5).unwrap(),
                },
                "index too small",
            ),
            (
                Braid::from_data(Some(0), vec![(1, Some(5), Sign::Positive)]),
                BraidValidationError::from(BraidIndex::try_new(0).err().unwrap()),
                "index validation failure",
            ),
            (
                Braid::from_data(
                    None::<u16>,
                    vec![(1, None::<u16>, Sign::Positive); u16::MAX as usize + 1],
                ),
                BraidValidationError::from(
                    Word::try_new(vec![
                        (1, None::<u16>, Sign::Positive);
                        u16::MAX as usize + 1
                    ])
                    .clone_unwrap_err(),
                ),
                "word validation failure",
            ),
        ];

        for (invalid_braid, error, label) in invalid_braids {
            expect_that!(*invalid_braid, err(eq(&error)), "{label}")
        }
    }

    #[gtest]
    fn invalid_construction_with_try_from_letters_fails_as_expected() {
        let invalid_braids: Vec<(BraidResult, BraidValidationError, &'static str)> = vec![
            (
                Braid::try_from_letters(
                    &[Letter::try_new(1, None::<u16>, Sign::Positive).unwrap();
                        u16::MAX as usize + 1],
                ),
                BraidValidationError::from(
                    Word::try_from_letters(
                        &[Letter::try_new(1, None::<u16>, Sign::Positive).unwrap();
                            u16::MAX as usize + 1],
                    )
                    .clone_unwrap_err(),
                ),
                "Word validation failure - from Vec",
            ),
            (
                Braid::try_from_letters(
                    &[Letter::try_new(2, Some(3), Sign::Positive).unwrap(); u16::MAX as usize + 1],
                ),
                BraidValidationError::from(
                    Word::try_from_letters(
                        &[Letter::try_new(2, Some(3), Sign::Positive).unwrap();
                            u16::MAX as usize + 1],
                    )
                    .clone_unwrap_err(),
                ),
                "Word validation failure - from slice",
            ),
        ];

        for (invalid_braid, error, label) in invalid_braids {
            expect_that!(*invalid_braid, err(eq(&error)), "{label}")
        }
    }

    #[gtest]
    fn invalid_construction_of_trivial_braid_fails_as_expected() {
        let invalid_braids: Vec<(BraidResult, BraidValidationError, &'static str)> = vec![
            (
                Braid::trivial(0),
                BraidValidationError::from(BraidIndex::try_new(0).err().unwrap()),
                "zero index",
            ),
            (
                Braid::trivial(-1),
                BraidValidationError::from(BraidIndex::try_new(-1).err().unwrap()),
                "negative index",
            ),
            (
                Braid::trivial(u16::MAX as u32 + 1),
                BraidValidationError::from(BraidIndex::try_new(u16::MAX as u32 + 1).err().unwrap()),
                "big index",
            ),
        ];

        for (invalid_braid, error, label) in invalid_braids {
            expect_that!(*invalid_braid, err(eq(&error)), "{label}")
        }
    }
}
