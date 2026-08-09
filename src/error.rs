use crate::generators::{FromArtinError, MAX_BAND_HEIGHT};
use crate::{BandGenerator, BraidIndex, Strand};

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
#[derive(Debug, thiserror::Error, PartialEq, Eq, Clone, Copy)]
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
}

/// Represents failed attempt to construct a [`BandGenerator`].
///
/// [Bands](BandGenerator) can be fallibly constructed in two ways: by providing band data directly
/// to [`BandGenerator::new`], or by passing a list of [`ArtinGenerator`] to
/// [`BandGenerator::coalesce`].
///
/// # Failure When Using [`BandGenerator::new`]
///
/// 1. The given foot and head strand have equal index ([`BandValidationError::FootOnHead`]):
///
/// ```
/// use braided::{BandGenerator, BandValidationError, Sign};
/// use std::assert_matches;
///
/// assert_matches!(
///     BandGenerator::new(1, 1, Sign::Positive),
///     Err(BandValidationError::FootOnHead(_)),
/// );
/// ```
///
/// 2. The given foot index is larger than the given head index (
///    [`BandValidationError::FootOverHead`]):
///
///
/// ```
/// use braided::{BandGenerator, BandValidationError, Sign};
/// use std::assert_matches;
///
/// assert_matches!(
///     BandGenerator::new(4, 1, Sign::Negative),
///     Err(BandValidationError::FootOverHead { .. }),
/// );
/// ```
///
/// 3. The distance between the foot and head strands exceeds the maximum band height, `2e15`.
///    This is an error since then the Artin length of the resulting band would exceed maximum
///    allowed word length, [`u16::MAX`].
///
///
/// ```
/// use braided::{BandGenerator, BandValidationError, Sign};
/// use std::assert_matches;
///
/// assert_matches!(
///     BandGenerator::new(1, 2u16.pow(15) + 2, Sign::Positive),
///     Err(BandValidationError::TooTall(_)),
/// );
/// ```
///
/// 4. A valid strand cannot be constructed from one of the given foot or head indices
///    ([`BandValidationError::StrandValidation`]):
///
///
/// ```
/// use braided::{BandGenerator, BandValidationError, Sign};
/// use std::assert_matches;
///
/// let zero_foot = BandGenerator::new(0, 4, Sign::Negative);
/// let negative_foot = BandGenerator::new(-1, 4, Sign::Positive);
/// let big_head = BandGenerator::new(1, u16::MAX as u32 + 1, Sign::Negative);
///
/// assert_matches!(zero_foot, Err(BandValidationError::StrandValidation(_)));
/// assert_matches!(negative_foot, Err(BandValidationError::StrandValidation(_)));
/// assert_matches!(big_head, Err(BandValidationError::StrandValidation(_)));
/// ```
///
/// # Failure When Using [`BandGenerator::coalesce`]
///
/// In each of these failure contexts, a [`BandValidationError::FromArtin`] is returned.
///
/// 1. The input slice is empty:
///
///
/// ```
/// use braided::{ArtinGenerator, BandGenerator, BandValidationError, Sign};
/// use std::assert_matches;
///
/// let empty_input = BandGenerator::coalesce(&[]);
///
/// assert_matches!(empty_input, Err(BandValidationError::FromArtin(_)))
/// ```
///
/// 2. The input slice contains an even number of [Artin generators](ArtinGenerator):
///
///
/// ```
/// use braided::{ArtinGenerator, BandGenerator, BandValidationError, Sign};
/// use std::assert_matches;
///
/// let even_length_input = BandGenerator::coalesce(&[
///     ArtinGenerator::new(1, Sign::Positive).unwrap(),
///     ArtinGenerator::new(2, Sign::Negative).unwrap(),
/// ]);
///
/// assert_matches!(even_length_input, Err(BandValidationError::FromArtin(_)))
/// ```
///
/// 3. The input slice is longer than [`u16::MAX`].
///
///
/// ```
/// use braided::{ArtinGenerator, BandGenerator, BandValidationError, Sign};
/// use std::assert_matches;
///
/// let too_long_input = BandGenerator::coalesce(&vec![
///     ArtinGenerator::new(1, Sign::Positive).unwrap(); u16::MAX as usize + 1
/// ]);
///
/// assert_matches!(too_long_input, Err(BandValidationError::FromArtin(_)))
/// ```
///
/// 4. The coalescing algorithm failed because some [Artin generator](ArtinGenerator) is not
///    contiguous with the partially constructed band:
///
///
/// ```
/// use braided::{ArtinGenerator, BandGenerator, BandValidationError, Sign};
/// use std::assert_matches;
///
/// let non_contiguous_generator = BandGenerator::coalesce(&[
///     // should be 1-2-3 on the left, not 2-1-3
///     ArtinGenerator::new(2, Sign::Negative).unwrap(),
///     ArtinGenerator::new(1, Sign::Negative).unwrap(),
///     ArtinGenerator::new(3, Sign::Positive).unwrap(),
///     ArtinGenerator::new(2, Sign::Positive).unwrap(),
///     ArtinGenerator::new(1, Sign::Positive).unwrap(),
/// ]);
///
/// assert_matches!(non_contiguous_generator, Err(BandValidationError::FromArtin(_)))
/// ```
///
/// 5. The coalescing algorithm failed because set of [Artin generators](ArtinGenerator) left of the
///    crossing generator fails to mirror those to its right:
///
///
/// ```
/// use braided::{ArtinGenerator, BandGenerator, BandValidationError, Sign};
/// use std::assert_matches;
///
///
/// let imbalanced = BandGenerator::coalesce(&[
///     ArtinGenerator::new(4, Sign::Positive).unwrap(), // above crossing
///     ArtinGenerator::new(2, Sign::Negative).unwrap(), // below crossing
///     ArtinGenerator::new(3, Sign::Positive).unwrap(), // crossing
///     ArtinGenerator::new(2, Sign::Positive).unwrap(), // below crossing
///     ArtinGenerator::new(1, Sign::Positive).unwrap(), // below crossing
/// ]);
///
/// assert_matches!(imbalanced, Err(BandValidationError::FromArtin(_)))
/// ```
#[derive(thiserror::Error, Debug, PartialEq, Eq, Clone, Copy)]
pub enum BandValidationError {
    /// Indicates that the given foot and head strands are the same.
    ///
    /// Wraps the offending [strand](Strand).
    #[error("Foot strand and head strand are the same ({0:?}).")]
    FootOnHead(Strand),
    /// Indicates that the given foot strand lies above the given head strand.
    ///
    /// Wraps both offending [strands](Strand).
    #[error("Foot strand ({foot:?}) is over head strand ({head:?}).")]
    FootOverHead {
        /// The offending foot [strand](Strand).
        foot: Strand,
        /// The offending head [strand](Strand).
        head: Strand,
    },
    /// Indicates that the distance between the foot and head strand exceeds the maximal allowed
    /// band height of `2e15`.
    ///
    /// Wraps the offending height
    #[error(
        "Attempting to construct band of height {0} exceeding {max}.",
        max = MAX_BAND_HEIGHT,
    )]
    TooTall(u16),
    /// Indicates failure to construct at least one of the foot or head [strand](Strand).
    ///
    /// Transparent wrapper around [`StrandValidationError`].
    #[error(transparent)]
    StrandValidation(#[from] StrandValidationError),
    /// Indicates failure to construct band by coalescing a collection of
    /// [Artin generators](ArtinGenerator).
    ///
    /// Transparent wrapper around an internal error type relating to the coalescing algorithm.
    #[error(transparent)]
    FromArtin(#[from] FromArtinError),
}

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
/// use braided::{Braid, BraidValidationError, Sign, Word};
/// use std::assert_matches;
///
/// let word = Word::new(vec![(1, None::<u16>, Sign::Positive)]).unwrap();
///
/// let zero_index = Braid::new(Some(0), word.clone());
/// let negative_index = Braid::new(Some(-1), word.clone());
/// let big_index = Braid::new(
///     Some(u16::MAX as u32 + 1),
///     word,
/// );
///
/// assert_matches!(zero_index, Err(BraidValidationError::IndexValidation(_)));
/// assert_matches!(negative_index, Err(BraidValidationError::IndexValidation(_)));
/// assert_matches!(big_index, Err(BraidValidationError::IndexValidation(_)));
/// ```
///
/// 2. An explicitly provided [`BraidIndex`] is smaller than required by the given [`Word`]
///    ([`BraidValidationError::IndexTooSmall`]):
///
/// ```
/// use braided::{Braid, BraidIndex, BraidValidationError, Sign, Word};
///
/// let word = Word::new(vec![(2, None::<u16>, Sign::Positive)]).unwrap();
/// let index_too_small = Braid::from_data(Some(2), word);
///
/// assert_eq!(
///     index_too_small,
///     Err(BraidValidationError::IndexTooSmall {
///         index: BraidIndex::new(2).unwrap(),
///         minimal_required_index: BraidIndex::new(3).unwrap(),
///     }),
/// );
/// ```
///
/// # Invalid Construction Using [`Braid::from_data`]
///
/// 1. Failure to construct an explicitly provided [`BraidIndex`]
///    ([`BraidValidationError::IndexValidation`]):
///
/// ```
/// use braided::{Braid, BraidValidationError, Sign};
/// use std::assert_matches;
///
/// let zero_index = Braid::from_data(Some(0), vec![(1, None::<u16>, Sign::Positive)]);
/// let negative_index = Braid::from_data(Some(-1), vec![(1, None::<u16>, Sign::Positive)]);
/// let big_index = Braid::from_data(
///     Some(u16::MAX as u32 + 1),
///     vec![(1, None::<u16>, Sign::Positive)],
/// );
///
/// assert_matches!(zero_index, Err(BraidValidationError::IndexValidation(_)));
/// assert_matches!(negative_index, Err(BraidValidationError::IndexValidation(_)));
/// assert_matches!(big_index, Err(BraidValidationError::IndexValidation(_)));
/// ```
///
/// 2. An explicitly provided [`BraidIndex`] is smaller than required by the given [`Letter`] data
///    ([`BraidValidationError::IndexTooSmall`]):
///
/// ```
/// use braided::{Braid, BraidIndex, BraidValidationError, Sign};
///
/// let index_too_small = Braid::from_data(Some(2), vec![(2, None::<u16>, Sign::Positive)]);
///
/// assert_eq!(
///     index_too_small,
///     Err(BraidValidationError::IndexTooSmall {
///         index: BraidIndex::new(2).unwrap(),
///         minimal_required_index: BraidIndex::new(3).unwrap(),
///     }),
/// );
/// ```
///
/// 3. Failure to construct a valid [`Word`] from the given [`Letter`] data
///    ([`BraidValidationError::WordValidation`]):
///
/// ```
/// use braided::{Braid, BraidValidationError, Sign};
/// use std::assert_matches;
///
/// let bad_letter = Braid::from_data(None::<u16>, vec![(0, None::<u16>, Sign::Positive)]);
/// let long_word = Braid::from_data(
///     Some(2),
///     vec![(1, None::<u16>, Sign::Positive); u16::MAX as usize + 1],
/// );
///
/// assert_matches!(bad_letter, Err(BraidValidationError::WordValidation(_)));
/// assert_matches!(long_word, Err(BraidValidationError::WordValidation(_)));
/// ```
///
/// # Invalid Construction Using [`Braid::try_from`]
///
/// 1. Failure to construct a valid [`Word`] from the given [letters](Letter), (e.g., because the
///    number of [letters](Letter) provided exceeds [`u16::MAX`]; uses a
///    [`BraidValidationError::WordValidation`]).
///
/// ```
/// use braided::{Braid, BraidValidationError, Letter, Sign};
/// use std::assert_matches;
///
/// let many_letters = vec![
///         Letter::new(1, None::<u16>, Sign::Positive).unwrap();
///         u16::MAX as usize + 1
///     ];
/// let one_short_one_tall = vec![
///     Letter::new(3, None::<u16>, Sign::Negative).unwrap(),
///     Letter::new(1, Some(2u16.pow(15) + 1), Sign::Negative).unwrap(),
/// ];
///
/// assert_matches!(Braid::try_from(many_letters), Err(BraidValidationError::WordValidation(_)));
/// assert_matches!(Braid::try_from(one_short_one_tall), Err(BraidValidationError::WordValidation(_)));
/// ```
///
/// # Invalid Construction When Using  [`Braid::trivial`]
///
/// 1. Failure to construct a valid [`BraidIndex`] ([`BraidValidationError::IndexValidation`]):
///
/// ```
/// use braided::{Braid, BraidValidationError, Sign};
/// use std::assert_matches;
///
/// let zero_index = Braid::trivial(0);
/// let negative_index = Braid::trivial(-1);
/// let big_index = Braid::trivial(u16::MAX as u32 + 1);
///
/// assert_matches!(zero_index, Err(BraidValidationError::IndexValidation(_)));
/// assert_matches!(negative_index, Err(BraidValidationError::IndexValidation(_)));
/// assert_matches!(big_index, Err(BraidValidationError::IndexValidation(_)));
/// ```
///
/// # Invalid Construction When Multiplying a [`Braid`] and a ([`Letter`], [`Word`], or [`Braid`])
///
///
/// 1. The [index](BraidIndex) of one of the [`Braid`] operands is smaller than required by some
///    [letter](Letter) of the other operand ([`BraidValidationError::IndexTooSmall`]):
///
/// ```
/// use braided::{Braid, BraidIndex, BraidValidationError, Letter, Sign};
///
/// let braid = Braid::from_data(
///     None::<u16>,
///     vec![
///         (1, None::<u16>, Sign::Positive),
///         (2, Some(5), Sign::Negative),
///         (3, None::<u16>, Sign::Negative),
///         (4, Some(5), Sign::Positive),
///     ],
/// )
/// .unwrap();
/// let letter = Letter::new(7, None::<u16>, Sign::Positive).unwrap();
///
/// assert_eq!(braid * letter, Err(BraidValidationError::IndexTooSmall {
///         index: BraidIndex::new(5).unwrap(),
///         minimal_required_index: BraidIndex::new(8).unwrap(),
///     })
/// );
/// ```
///
/// 2. The [Artin length](Braid::artin_length) of the product exceeds the maximum length of
///    [`u16::MAX`] ([`BraidValidationError::WordValidation`]):
///
/// ```
/// use braided::{Braid, BraidIndex, BraidValidationError, Sign, Word};
/// use std::assert_matches;
///
/// let braid = Braid::from_data(
///     Some(10),
///     vec![(1, None::<u16>, Sign::Positive); u16::MAX as usize],
/// )
/// .unwrap();
/// let word = Word::new(vec![
///     (2, Some(8), Sign::Negative),
///     (1, None::<u16>, Sign::Positive),
/// ]).unwrap();
///
/// assert_matches!(word * braid, Err(BraidValidationError::WordValidation(_)));
/// ```
///
/// 3. Attempting to multiply two [braids](Braid) whose [braid indices](BraidIndex) are not equal
///    ([`BraidValidationError::UnequalIndices`]):
///
/// ```
/// use braided::{Braid, BraidIndex, BraidValidationError, Sign};
///
/// let left_braid = Braid::from_data(Some(2), vec![(1, None::<u16>, Sign::Positive)]).unwrap();
/// let right_braid = Braid::from_data(Some(3), vec![(1, None::<u16>, Sign::Positive)]).unwrap();
///
/// assert_eq!(
///     left_braid * right_braid,
///     Err(BraidValidationError::UnequalIndices {
///         left: BraidIndex::new(2).unwrap(),
///         right: BraidIndex::new(3).unwrap(),
///     }),
/// );
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

/// Represents a failed validation when attepting to construct a [`BraidIndex`].
///
/// A [`BraidIndex`] wraps a [`u16`], so the variants of [`IndexValidationError`] correspond
/// to different failure modes of the attempted conversion.
///
/// Note that there is as yet no variant corresponding to the attempt to pass a negative number
/// to an index constructor. While this would definitely be an error, there is currently no code
/// path through which such an error is possible.
///
/// # Examples
///
/// ```
/// use braided::{BraidIndex, IndexValidationError};
/// use std::assert_matches;
///
/// assert_matches!(
///     BraidIndex::new(0),
///     Err(IndexValidationError::Zero),
/// );
///
/// assert_matches!(
///     BraidIndex::new(u16::MAX as u32 + 1),
///     Err(IndexValidationError::FromInt(_))
/// );
/// ```
#[derive(Debug, thiserror::Error, PartialEq, Eq, Clone, Copy)]
pub enum IndexValidationError {
    /// Indicates failure to construct a [`BraidIndex`] from zero.
    #[error("Braid index cannot be zero")]
    Zero,
    /// Wrapper around [`std::num::TryFromIntError`], indicating failure to convert an integer
    /// value into a [`u16`] during [`BraidIndex`] construction.
    #[error(transparent)]
    FromInt(#[from] std::num::TryFromIntError),
    /// Wrapper around a [`std::convert::Infallible`].
    ///
    /// This variant exists purely to make the type system happy; in practice this variant cannot
    /// occur.
    #[error(transparent)]
    Infallible(#[from] std::convert::Infallible),
}

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

/// Represents failure during construction of a [`Strand`].
///
/// A [`Strand`] wraps a [`u16`], so all variants of [`StrandValidationError`] concern the failure
/// to convert an input into a [`u16`].
///
/// # Examples
///
/// ```
/// use braided::{Strand, StrandValidationError};
/// use std::assert_matches;
///
/// assert_matches!(
///     Strand::new(0),
///     Err(StrandValidationError::Zero),
/// );
///
/// assert_matches!(
///     Strand::new(u16::MAX as u32 + 1),
///     Err(StrandValidationError::FromInt(_)),
/// );
///
/// assert_matches!(
///     Strand::new(1).unwrap() - 2,
///     Err(StrandValidationError::Subtraction { .. }),
///     );
///
/// assert_matches!(
///     Strand::new(u16::MAX - 1).unwrap() + 2,
///     Err(StrandValidationError::Addition { .. }),
/// );
/// ```
#[derive(Debug, thiserror::Error, PartialEq, Eq, Clone, Copy)]
pub enum StrandValidationError {
    /// Indicates failure to construct a [`Strand`] from zero.
    #[error("Strand index cannot be zero.")]
    Zero,
    /// Indicates a failed [`Strand`] subtraction result: the subtraction would yield a non-positive
    /// strand index.
    #[error("Attempt to subtract {right:?} from {left:?} results in non-positive-indexed strand.")]
    Subtraction {
        /// The index of the left operand in the failed subtraction.
        left: u16,
        /// The index of the right operand in the failed subtraction.
        right: u16,
    },
    /// Indicates a failed [`Strand`] addition result: the addition would yield a result that
    /// exceeds [`u16::MAX`].
    #[error(
        "Attempt to add {left:?} to {right:?} results in strand index larger than {max}",
        max = u16::MAX,
    )]
    Addition {
        /// The index of the left operand in the failed addition.
        left: u16,
        /// The index of the right operand in the failed addition.
        right: u16,
    },
    /// Wrapper around a [`std::num::TryFromIntError`], indicating failure to convert an integer
    /// value into a [`u16`] during [`Strand`] construction.
    #[error(transparent)]
    FromInt(#[from] std::num::TryFromIntError),
    /// Wrapper around a [`std::convert::Infallible`].
    ///
    /// This variant exists purely to make the type system happy; in practice this variant cannot
    /// occur.
    #[error(transparent)]
    Infallible(#[from] std::convert::Infallible),
}

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
///     &long_word * &long_word,
///     Err(WordValidationError::TooLong(2 * (u16::MAX as u32))),
/// );
///
/// // Note: you can still multiply two long words than cancel into a short one
/// assert_eq!(
///     &long_word * long_word.inverse(),
///     Ok(Word::trivial())
/// );
///
/// // Failure can also occur when multiplying a word by a letter with large Artin length
/// let tall_letter = Letter::new(1, Some(2u16.pow(15) + 1), Sign::Negative).unwrap();
/// let short_word = Word::new(
///     vec![(2, None::<u16>, Sign::Positive), (1, Some(5), Sign::Negative)]
/// ).unwrap();
/// assert_eq!(
///     &short_word * tall_letter,
///     Err(WordValidationError::TooLong(u16::MAX as u32 + 8)),
/// );
/// assert_eq!(
///     tall_letter * &short_word,
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
    TooLong(usize),
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
