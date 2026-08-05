use crate::{ArtinGenerator, BraidIndex, Letter, Sign, Strand, StrandValidationError};

const MAX_BAND_HEIGHT: u16 = 2u16.pow(15);

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

#[derive(thiserror::Error, Debug, PartialEq, Eq, Clone, Copy)]
pub enum FromArtinError {
    #[error("No Artin generators provided.")]
    NoGenerators,
    #[error("Even number of Artin generators provided.")]
    EvenGenerators,
    #[error("Attempt to construct a band from {0} > {max} Artin generators", max = u16::MAX)]
    TooManyGenerators(usize),
    #[error("Could not append {next_step:?} to {previous_step:?} in {quadrant:?} staircase.")]
    IncontiguousSteps {
        quadrant: StaircaseQuadrant,
        next_step: ArtinGenerator,
        previous_step: ArtinGenerator,
    },
    #[error("Staircases are not balanced: difference of {0} steps found.")]
    ImbalancedStaircases(usize),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum StaircaseQuadrant {
    UpperLeft,
    LowerLeft,
    LowerRight,
    UpperRight,
}

/// Represents a _band generator_ of some braid group.
///
/// <div class="warning">
///
/// Consider using [`Letter`] instead of [`BandGenerator`] unless you need low-level access to the
/// underlying generating set.
///
/// </div>
///
/// # A Bit About Band Generators
///
/// The band generators are a generalization of the standard [artin generators](ArtinGenerator) in
/// which each generating element represents a crossing of two [strands](Strand) of _arbitrary_
/// distance apart, with the two interchanging strands passing _over_ any intermediate strands.
/// Consequently, one specifies a band generator by giving the indices of both its foot _and head_
/// strands, together with the [sign](Sign) of the crossing. The resulting picture looks something
/// like one has attached a half-twisted "band" between the foot and head strands, hence the name
/// (when one considers [Seifert surfaces](https://en.wikipedia.org/wiki/Seifert_surface) of
/// [closed braids](https://en.wikipedia.org/wiki/Braid_group#Closed_braids), the "band" is quite
/// literal).
///
/// It is hopefully clear that the band generators form a superset of the standard Artin generators
/// (one which is strictly larger except for 2-stranded braids, where the two sets coincide). While
/// the band generators are thus a less efficient generating set, many arguments are made easier (at
/// least notationally) through their use, and certain concepts which are straightforward to define
/// using band generators are cumbersome when expressed via Artin generators (e.g., certain forms of
/// braid _positivity_, such as
/// [_strong quasipositivity_](https://knotinfo.org/descriptions/strongly_quasipositive.html)).
///
/// That said, the Artin generators still _generate_ the braid groups, thus we ought to be able to
/// [decompose](BandGenerator::decompose) any band generator into a collection of Artin generators
/// (and likewise [coalesce](BandGenerator::coalesce) an appropriate collection of Artin generators
/// into a band generator). Let `b[f, h, s]` denote a band generator with foot `f`, head `h`, and
/// sign `s`. There are in fact `h - f` distinct decompositions of `b[f, h, s]` into Artin
/// generators, depending on where we situate the "crossing". Let `a[i, t]` denote the Artin
/// generator with foot `i` and sign `t`. Then for any `f <= c < h`, we may write
///
/// `b[f, h, s] = a[f,-]...a[c-1,-]a[h-1,+]...a[c+1,+]a[c, s]a[c+1,-]...a[h-1,-]a[c-1,-]...a[f,-]`
///
/// In `braided`, we assume the convention that `c = h-1` when
/// [decomposing](BandGenerator::decompose) a band generator; however, our implementation of the
/// [coalescing algorithm](BandGenerator::coalesce) makes no assumptions as to the value of `c`, or
/// the order of the surrounding Artin generators (as long as the order is obtainable from that
/// given above through applications of the _far commutativity_ relations).
///
/// # Construction
///
/// A [`BandGenerator`] may be constructed in multiple ways. The direct approach passes low-level
/// band data directly to [`BandGenerator::new`]. A band can also be _infallibly_ converted from
/// either an [`ArtinGenerator`] or a [`Letter`] using the [`BandGenerator::from`] function.
/// Finally, one may use [`BandGenerator::coalesce`] in order to convert a collection of Artin
/// generators into a band (see the previous section for more details on the relationship between
/// Artin generators and band generators).
///
/// 1. Direct construction using [`BandGenerator::new`].
///
/// ```
/// use braided::{BandGenerator, Sign, Strand};
/// use std::assert_matches;
///
/// // Anything which coerces to a `u16` can be used for the foot or head data:
///
/// assert_matches!(
///     BandGenerator::new(3, 4, Sign::Positive),
///     Ok(_),
/// );
///
/// // 2e15 is the maximal band height
/// assert_matches!(
///     BandGenerator::new(1, 2u16.pow(15) + 1, Sign::Negative),
///     Ok(_),
/// );
///
/// assert_matches!(
///     BandGenerator::new(2_usize, 5_isize, Sign::Positive),
///     Ok(_),
/// );
///
/// assert_matches!(
///     BandGenerator::new(Strand::new(9).unwrap(), 40_u32, Sign::Negative),
///     Ok(_),
/// );
///
/// assert_matches!(
///     BandGenerator::new(-(-3), Strand::new(10).unwrap(), Sign::Positive),
///     Ok(_),
/// );
/// ```
///
/// 2. Converting from an [`ArtinGenerator`] or [`Letter`] using [`BandGenerator::from`].
///
/// ```
/// use braided::{ArtinGenerator, BandGenerator, Letter, Sign};
///
/// // We unwrap the target since conversions from ArtinGenerator and Letter are infallible
/// let expected_band = BandGenerator::new(1, 2, Sign::Positive).unwrap();
///
/// assert_eq!(
///     BandGenerator::from(ArtinGenerator::new(1, Sign::Positive).unwrap()),
///     expected_band,
/// );
///
/// assert_eq!(
///     BandGenerator::from(Letter::new(1, None::<u16>, Sign::Positive).unwrap()),
///     expected_band,
/// );
///
/// assert_eq!(
///     BandGenerator::from(Letter::new(1, Some(2), Sign::Positive).unwrap()),
///     expected_band,
/// );
/// ```
///
/// 3. Converting from a collection of [`ArtinGenerator`] using [`BandGenerator::coalesce`].
///
/// ```
/// use braided::{ArtinGenerator, BandGenerator, Sign};
///
/// let test_band = BandGenerator::new(1, 4, Sign::Positive);
///
/// // All of the following are valid means of constructing `test_band` via "coalescing":
/// let coalesced_bands = [
///     BandGenerator::coalesce(&[
///         ArtinGenerator::new(1, Sign::Negative).unwrap(),
///         ArtinGenerator::new(2, Sign::Negative).unwrap(),
///         ArtinGenerator::new(3, Sign::Positive).unwrap(),
///         ArtinGenerator::new(2, Sign::Positive).unwrap(),
///         ArtinGenerator::new(1, Sign::Positive).unwrap(),
///     ]),
///     BandGenerator::coalesce(&[
///         ArtinGenerator::new(3, Sign::Positive).unwrap(),
///         ArtinGenerator::new(2, Sign::Positive).unwrap(),
///         ArtinGenerator::new(1, Sign::Positive).unwrap(),
///         ArtinGenerator::new(2, Sign::Negative).unwrap(),
///         ArtinGenerator::new(3, Sign::Negative).unwrap(),
///     ]),
///     BandGenerator::coalesce(&[
///         ArtinGenerator::new(3, Sign::Positive).unwrap(),
///         ArtinGenerator::new(1, Sign::Negative).unwrap(),
///         ArtinGenerator::new(2, Sign::Positive).unwrap(),
///         ArtinGenerator::new(1, Sign::Positive).unwrap(),
///         ArtinGenerator::new(3, Sign::Negative).unwrap(),
///     ]),
///     BandGenerator::coalesce(&[
///         ArtinGenerator::new(1, Sign::Negative).unwrap(),
///         ArtinGenerator::new(3, Sign::Positive).unwrap(),
///         ArtinGenerator::new(2, Sign::Positive).unwrap(),
///         ArtinGenerator::new(1, Sign::Positive).unwrap(),
///         ArtinGenerator::new(3, Sign::Negative).unwrap(),
///     ]),
/// ];
///
/// for coalesced_band in coalesced_bands {
///     assert_eq!(coalesced_band, test_band);
/// }
/// ```
///
/// ## Errors
///
/// Methods 1. and 3. above are _fallible_, and the associated error type for both
/// [`BandGenerator::new`] and [`BandGenerator::coalesce`] is [`BandValidationError`]. Further
/// details and examples can be found in the associated documentation.
///
/// # Decomposition
///
/// As discussed in the first section, every band generator can be decomposed into a product of
/// Artin generators. The [`BandGenerator::decompose`] method implements this _infallible_ process,
/// returning a [`Vec<ArtinGenerator>`].
///
/// ```
/// use braided::{ArtinGenerator, BandGenerator, Sign};
///
/// // In each of the following pairs, the second element is the result of decomposing the first:
///
/// let tests = [
///     (
///         BandGenerator::new(1, 2, Sign::Positive),
///         vec![ArtinGenerator::new(1, Sign::Positive).unwrap()],
///     ),
///     (
///         BandGenerator::new(1, 4, Sign::Positive),
///         vec![
///             ArtinGenerator::new(1, Sign::Negative).unwrap(),
///             ArtinGenerator::new(2, Sign::Negative).unwrap(),
///             ArtinGenerator::new(3, Sign::Positive).unwrap(),
///             ArtinGenerator::new(2, Sign::Positive).unwrap(),
///             ArtinGenerator::new(1, Sign::Positive).unwrap(),
///         ],
///     ),
///     (
///         BandGenerator::coalesce(&[
///             ArtinGenerator::new(1, Sign::Negative).unwrap(),
///             ArtinGenerator::new(2, Sign::Negative).unwrap(),
///             ArtinGenerator::new(3, Sign::Positive).unwrap(),
///             ArtinGenerator::new(2, Sign::Positive).unwrap(),
///             ArtinGenerator::new(1, Sign::Positive).unwrap(),
///         ]),
///         vec![
///             ArtinGenerator::new(1, Sign::Negative).unwrap(),
///             ArtinGenerator::new(2, Sign::Negative).unwrap(),
///             ArtinGenerator::new(3, Sign::Positive).unwrap(),
///             ArtinGenerator::new(2, Sign::Positive).unwrap(),
///             ArtinGenerator::new(1, Sign::Positive).unwrap(),
///         ],
///     ),
///     (
///         BandGenerator::coalesce(&[
///             ArtinGenerator::new(1, Sign::Negative).unwrap(),
///             ArtinGenerator::new(3, Sign::Positive).unwrap(),
///             ArtinGenerator::new(2, Sign::Positive).unwrap(),
///             ArtinGenerator::new(1, Sign::Positive).unwrap(),
///             ArtinGenerator::new(3, Sign::Negative).unwrap(),
///         ]),
///         vec![
///             ArtinGenerator::new(1, Sign::Negative).unwrap(),
///             ArtinGenerator::new(2, Sign::Negative).unwrap(),
///             ArtinGenerator::new(3, Sign::Positive).unwrap(),
///             ArtinGenerator::new(2, Sign::Positive).unwrap(),
///             ArtinGenerator::new(1, Sign::Positive).unwrap(),
///         ],
///     ),
/// ];
///
/// for (test_band, decomposed) in tests {
///     assert_eq!(test_band.unwrap().decompose(), decomposed);
/// }
/// ```
///
/// # Accessors and Basic Properties
///
/// [`BandGenerator`] contains accessor methods for obtaining the inner data:
///
/// ```
/// use braided::{BandGenerator, Sign, Strand};
///
/// let band = BandGenerator::new(2, 5, Sign::Negative).unwrap();
///
/// assert_eq!(band.foot(), Strand::new(2).unwrap());
/// assert_eq!(band.head(), Strand::new(5).unwrap());
/// assert_eq!(band.sign(), Sign::Negative);
/// ```
///
/// There are also various methods for computing basic band properties:
///
/// ```
/// use braided::{BandGenerator, BraidIndex, Sign, Strand};
///
/// let band = BandGenerator::new(2, 5, Sign::Negative).unwrap();
///
/// assert_eq!(
///     band.inverse(),
///     BandGenerator::new(2, 5, Sign::Positive).unwrap(),
/// );
///
/// assert_eq!(
///     band.height(),
///     5 - 2,
/// );
///
/// assert_eq!(
///     band.is_artin(),
///     false,
/// );
///
/// assert_eq!(
///     band.minimal_required_braid_index(),
///     BraidIndex::new(5).unwrap(),
/// );
///
/// assert_eq!(
///     band.artin_length(),
///     2 * (5 - 2) - 1,
/// );
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct BandGenerator {
    foot: Strand,
    head: Strand,
    sign: Sign,
}

impl BandGenerator {
    /// Attempts to construct a new [`BandGenerator`] given [`u16`]-coercible data for the foot and
    /// head [strands](Strand), together with a [sign](Sign).
    ///
    /// # Examples
    ///
    /// ```
    /// use braided::{BandGenerator, Sign, Strand};
    /// use std::assert_matches;
    ///
    /// // Anything which coerces to a `u16` can be used for the foot or head data:
    ///
    /// assert_matches!(
    ///     BandGenerator::new(3, 4, Sign::Positive),
    ///     Ok(_),
    /// );
    ///
    /// // 2e15 is the maximal band height
    /// assert_matches!(
    ///     BandGenerator::new(1, 2u16.pow(15) + 1, Sign::Negative),
    ///     Ok(_),
    /// );
    ///
    /// assert_matches!(
    ///     BandGenerator::new(2_usize, 5_isize, Sign::Positive),
    ///     Ok(_),
    /// );
    ///
    /// assert_matches!(
    ///     BandGenerator::new(Strand::new(9).unwrap(),40_u32, Sign::Negative),
    ///     Ok(_),
    /// );
    ///
    /// assert_matches!(
    ///     BandGenerator::new(-(-3), Strand::new(10).unwrap(), Sign::Positive),
    ///     Ok(_),
    /// );
    /// ```
    ///
    /// # Errors
    ///
    /// Please see the documentation for [`BandValidationError`] for more details on possible
    /// failure causes.
    pub fn new<F, H>(foot: F, head: H, sign: Sign) -> Result<Self, BandValidationError>
    where
        F: TryInto<u16>,
        H: TryInto<u16>,
        StrandValidationError: From<<F as TryInto<u16>>::Error>
            + From<<H as TryInto<u16>>::Error>
            + From<std::convert::Infallible>,
    {
        let foot = Strand::new(foot)?;
        let head = Strand::new(head)?;
        match foot.cmp(&head) {
            std::cmp::Ordering::Less => {
                let height: u16 = (head - foot).unwrap().into();
                if height > MAX_BAND_HEIGHT {
                    Err(BandValidationError::TooTall(height))
                } else {
                    Ok(Self { foot, head, sign })
                }
            }
            std::cmp::Ordering::Equal => Err(BandValidationError::FootOnHead(foot)),
            std::cmp::Ordering::Greater => Err(BandValidationError::FootOverHead { foot, head }),
        }
    }
    /// Attempts to construct a new [`BandGenerator`] by coalescing a collection of [Artin
    /// generators](ArtinGenerator).
    ///
    /// # Examples
    ///
    /// ```
    /// use braided::{ArtinGenerator, BandGenerator, Sign};
    ///
    /// // In each of the following pairs, the second element is the result of decomposing the first:
    ///
    /// let tests = [
    ///     (
    ///         BandGenerator::new(1, 2, Sign::Positive),
    ///         vec![ArtinGenerator::new(1, Sign::Positive).unwrap()],
    ///     ),
    ///     (
    ///         BandGenerator::new(1, 4, Sign::Positive),
    ///         vec![
    ///             ArtinGenerator::new(1, Sign::Negative).unwrap(),
    ///             ArtinGenerator::new(2, Sign::Negative).unwrap(),
    ///             ArtinGenerator::new(3, Sign::Positive).unwrap(),
    ///             ArtinGenerator::new(2, Sign::Positive).unwrap(),
    ///             ArtinGenerator::new(1, Sign::Positive).unwrap(),
    ///         ],
    ///     ),
    ///     (
    ///         BandGenerator::coalesce(&[
    ///             ArtinGenerator::new(1, Sign::Negative).unwrap(),
    ///             ArtinGenerator::new(2, Sign::Negative).unwrap(),
    ///             ArtinGenerator::new(3, Sign::Positive).unwrap(),
    ///             ArtinGenerator::new(2, Sign::Positive).unwrap(),
    ///             ArtinGenerator::new(1, Sign::Positive).unwrap(),
    ///         ]),
    ///         vec![
    ///             ArtinGenerator::new(1, Sign::Negative).unwrap(),
    ///             ArtinGenerator::new(2, Sign::Negative).unwrap(),
    ///             ArtinGenerator::new(3, Sign::Positive).unwrap(),
    ///             ArtinGenerator::new(2, Sign::Positive).unwrap(),
    ///             ArtinGenerator::new(1, Sign::Positive).unwrap(),
    ///         ],
    ///     ),
    ///     (
    ///         BandGenerator::coalesce(&[
    ///             ArtinGenerator::new(1, Sign::Negative).unwrap(),
    ///             ArtinGenerator::new(3, Sign::Positive).unwrap(),
    ///             ArtinGenerator::new(2, Sign::Positive).unwrap(),
    ///             ArtinGenerator::new(1, Sign::Positive).unwrap(),
    ///             ArtinGenerator::new(3, Sign::Negative).unwrap(),
    ///         ]),
    ///         vec![
    ///             ArtinGenerator::new(1, Sign::Negative).unwrap(),
    ///             ArtinGenerator::new(2, Sign::Negative).unwrap(),
    ///             ArtinGenerator::new(3, Sign::Positive).unwrap(),
    ///             ArtinGenerator::new(2, Sign::Positive).unwrap(),
    ///             ArtinGenerator::new(1, Sign::Positive).unwrap(),
    ///         ],
    ///     ),
    /// ];
    ///
    /// for (test_band, decomposed) in tests {
    ///     assert_eq!(test_band.unwrap().decompose(), decomposed);
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// Please see the documentation for [`BandValidationError`] for more details on possible
    /// failure causes.
    pub fn coalesce(band_parts: &[ArtinGenerator]) -> Result<Self, BandValidationError> {
        let num_parts = band_parts.len();

        if num_parts == 0 {
            return Err(BandValidationError::from(FromArtinError::NoGenerators));
        } else if num_parts == 1 {
            let generator = band_parts.last().unwrap();
            return Ok(BandGenerator {
                foot: generator.foot(),
                head: (generator.foot() + 1).unwrap(),
                sign: generator.sign(),
            });
        } else if num_parts.is_multiple_of(2) {
            return Err(BandValidationError::from(FromArtinError::EvenGenerators));
        } else if num_parts > u16::MAX as usize {
            return Err(BandValidationError::from(
                FromArtinError::TooManyGenerators(num_parts),
            ));
        }

        let mut upper_left_staircase = Vec::new();
        let mut lower_left_staircase = Vec::new();
        let mut upper_right_staircase = Vec::new();
        let mut lower_right_staircase = Vec::new();

        let (left_parts, right_parts) = band_parts.split_at(num_parts.div_euclid(2));
        let crossing = right_parts.first().unwrap();
        let right_parts = &right_parts[1..];

        for (left_part, right_part) in left_parts.iter().rev().zip(right_parts.iter()) {
            // Add new parts to staircases, and check for "contiguity" and "mirroring"
            match left_part.sign() {
                Sign::Positive => {
                    let previous_step = upper_left_staircase.last().unwrap_or(crossing);
                    if left_part.foot() == (previous_step.foot() + 1).unwrap() {
                        upper_left_staircase.push(*left_part);
                    } else {
                        return Err(BandValidationError::from(
                            FromArtinError::IncontiguousSteps {
                                quadrant: StaircaseQuadrant::UpperLeft,
                                next_step: *left_part,
                                previous_step: *previous_step,
                            },
                        ));
                    }
                }
                Sign::Negative => {
                    let previous_step = lower_left_staircase.last().unwrap_or(crossing);
                    if (left_part.foot() + 1).unwrap() == previous_step.foot() {
                        lower_left_staircase.push(*left_part);
                    } else {
                        return Err(BandValidationError::from(
                            FromArtinError::IncontiguousSteps {
                                quadrant: StaircaseQuadrant::LowerLeft,
                                next_step: *left_part,
                                previous_step: *previous_step,
                            },
                        ));
                    }
                }
            };
            match right_part.sign() {
                Sign::Positive => {
                    let previous_step = lower_right_staircase.last().unwrap_or(crossing);
                    if (right_part.foot() + 1).unwrap() == previous_step.foot() {
                        lower_right_staircase.push(*right_part);
                    } else {
                        return Err(BandValidationError::from(
                            FromArtinError::IncontiguousSteps {
                                quadrant: StaircaseQuadrant::LowerRight,
                                next_step: *right_part,
                                previous_step: *previous_step,
                            },
                        ));
                    }
                }
                Sign::Negative => {
                    let previous_step = upper_right_staircase.last().unwrap_or(crossing);
                    if right_part.foot() == (previous_step.foot() + 1).unwrap() {
                        upper_right_staircase.push(*right_part);
                    } else {
                        return Err(BandValidationError::from(
                            FromArtinError::IncontiguousSteps {
                                quadrant: StaircaseQuadrant::UpperRight,
                                next_step: *right_part,
                                previous_step: *previous_step,
                            },
                        ));
                    }
                }
            };
        }

        // If one set of staircases is imbalanced, then both are.
        if let difference = lower_left_staircase
            .len()
            .abs_diff(lower_right_staircase.len())
            && difference > 0
        {
            return Err(BandValidationError::from(
                FromArtinError::ImbalancedStaircases(difference),
            ));
        }

        let foot = lower_left_staircase.last().unwrap_or(crossing).foot();
        let head = (upper_left_staircase.last().unwrap_or(crossing).foot() + 1).unwrap();
        let sign = crossing.sign();

        Ok(Self { foot, head, sign })
    }

    /// Accessor method returning the band's foot [strand](Strand).
    ///
    /// # Example
    ///
    /// ```
    /// use braided::{BandGenerator, Sign, Strand};
    ///
    /// let band = BandGenerator::new(2, 5, Sign::Negative).unwrap();
    ///
    /// assert_eq!(band.foot(), Strand::new(2).unwrap());
    /// ```
    pub fn foot(&self) -> Strand {
        self.foot
    }
    /// Accessor method returning the band's head [strand](Strand).
    ///
    /// # Example
    ///
    /// ```
    /// use braided::{BandGenerator, Sign, Strand};
    ///
    /// let band = BandGenerator::new(2, 5, Sign::Negative).unwrap();
    ///
    /// assert_eq!(band.head(), Strand::new(5).unwrap());
    /// ```
    pub fn head(&self) -> Strand {
        self.head
    }
    /// Accessor method returning the band's [sign](Sign).
    ///
    /// # Example
    ///
    /// ```
    /// use braided::{BandGenerator, Sign};
    ///
    /// let band = BandGenerator::new(2, 5, Sign::Negative).unwrap();
    ///
    /// assert_eq!(band.sign(), Sign::Negative);
    /// ```
    pub fn sign(&self) -> Sign {
        self.sign
    }

    /// Computes the band's inverse, which amounts to just reversing its [sign](Sign).
    ///
    /// # Example
    ///
    /// ```
    /// use braided::{BandGenerator, Sign};
    ///
    /// let band = BandGenerator::new(2, 5, Sign::Negative).unwrap();
    ///
    /// assert_eq!(
    ///     band.inverse(),
    ///     BandGenerator::new(2, 5, Sign::Positive).unwrap(),
    /// );
    /// ```
    pub fn inverse(&self) -> Self {
        Self {
            foot: self.foot,
            head: self.head,
            sign: -self.sign,
        }
    }
    /// Computes the band's height, which is equal to `head - foot`.
    ///
    /// # Example
    ///
    /// ```
    /// use braided::{BandGenerator, Sign};
    ///
    /// let band = BandGenerator::new(2, 5, Sign::Negative).unwrap();
    ///
    /// assert_eq!(
    ///     band.height(),
    ///     5 - 2,
    /// );
    /// ```
    pub fn height(&self) -> u16 {
        (self.head - self.foot).unwrap().into()
    }
    /// Computes whether or not the band is an [Artin generator](ArtinGenerator).
    ///
    /// # Example
    ///
    /// ```
    /// use braided::{BandGenerator, Sign};
    ///
    /// let artin_band = BandGenerator::new(1, 2, Sign::Positive).unwrap();
    /// let non_artin_band = BandGenerator::new(2, 5, Sign::Negative).unwrap();
    ///
    /// assert!(artin_band.is_artin());
    /// assert!(!non_artin_band.is_artin());
    /// ```
    pub fn is_artin(&self) -> bool {
        self.height() == 1
    }
    /// Computes the minimal [index](BraidIndex) required to for a braid to use the band.
    ///
    /// # Example
    ///
    /// ```
    /// use braided::{BandGenerator, BraidIndex, Sign};
    ///
    /// let band = BandGenerator::new(2, 5, Sign::Negative).unwrap();
    ///
    /// assert_eq!(
    ///     band.minimal_required_braid_index(),
    ///     BraidIndex::new(5).unwrap(),
    /// );
    /// ```
    pub fn minimal_required_braid_index(&self) -> BraidIndex {
        BraidIndex::new(self.head).unwrap()
    }
    /// Computes the number of [Artin generators](ArtinGenerator) required to represent the band.
    ///
    /// # Example
    ///
    /// ```
    /// use braided::{BandGenerator, Sign};
    ///
    /// let band = BandGenerator::new(2, 5, Sign::Negative).unwrap();
    ///
    /// assert_eq!(
    ///     band.artin_length(),
    ///     2 * (5 - 2) - 1,
    /// );
    /// ```
    pub fn artin_length(&self) -> u16 {
        if self.height() == MAX_BAND_HEIGHT {
            u16::MAX
        } else {
            2 * self.height() - 1
        }
    }

    /// Decomposes the band generator into a sequence of [Artin generators](`ArtinGenerator`).
    ///
    /// See the main documentation for [`BandGenerator`] for more information about the relationship
    /// between Artin generators and band generators.
    ///
    /// # Example
    ///
    /// ```
    /// use braided::{ArtinGenerator, BandGenerator, Sign};
    ///
    /// // In each of the following pairs, the second element is the result of decomposing the first:
    ///
    /// let tests = [
    ///     (
    ///         BandGenerator::new(1, 2, Sign::Positive),
    ///         vec![ArtinGenerator::new(1, Sign::Positive).unwrap()],
    ///     ),
    ///     (
    ///         BandGenerator::new(1, 4, Sign::Positive),
    ///         vec![
    ///             ArtinGenerator::new(1, Sign::Negative).unwrap(),
    ///             ArtinGenerator::new(2, Sign::Negative).unwrap(),
    ///             ArtinGenerator::new(3, Sign::Positive).unwrap(),
    ///             ArtinGenerator::new(2, Sign::Positive).unwrap(),
    ///             ArtinGenerator::new(1, Sign::Positive).unwrap(),
    ///         ],
    ///     ),
    ///     (
    ///         BandGenerator::coalesce(&[
    ///             ArtinGenerator::new(1, Sign::Negative).unwrap(),
    ///             ArtinGenerator::new(2, Sign::Negative).unwrap(),
    ///             ArtinGenerator::new(3, Sign::Positive).unwrap(),
    ///             ArtinGenerator::new(2, Sign::Positive).unwrap(),
    ///             ArtinGenerator::new(1, Sign::Positive).unwrap(),
    ///         ]),
    ///         vec![
    ///             ArtinGenerator::new(1, Sign::Negative).unwrap(),
    ///             ArtinGenerator::new(2, Sign::Negative).unwrap(),
    ///             ArtinGenerator::new(3, Sign::Positive).unwrap(),
    ///             ArtinGenerator::new(2, Sign::Positive).unwrap(),
    ///             ArtinGenerator::new(1, Sign::Positive).unwrap(),
    ///         ],
    ///     ),
    ///     (
    ///         BandGenerator::coalesce(&[
    ///             ArtinGenerator::new(1, Sign::Negative).unwrap(),
    ///             ArtinGenerator::new(3, Sign::Positive).unwrap(),
    ///             ArtinGenerator::new(2, Sign::Positive).unwrap(),
    ///             ArtinGenerator::new(1, Sign::Positive).unwrap(),
    ///             ArtinGenerator::new(3, Sign::Negative).unwrap(),
    ///         ]),
    ///         vec![
    ///             ArtinGenerator::new(1, Sign::Negative).unwrap(),
    ///             ArtinGenerator::new(2, Sign::Negative).unwrap(),
    ///             ArtinGenerator::new(3, Sign::Positive).unwrap(),
    ///             ArtinGenerator::new(2, Sign::Positive).unwrap(),
    ///             ArtinGenerator::new(1, Sign::Positive).unwrap(),
    ///         ],
    ///     ),
    /// ];
    ///
    /// for (test_band, decomposed) in tests {
    ///     assert_eq!(test_band.unwrap().decompose(), decomposed);
    /// }
    /// ```
    pub fn decompose(&self) -> Vec<ArtinGenerator> {
        // Band decomposition is infallible, so it's safe to unwrap any intermediate results
        let crossing = ArtinGenerator::new((self.head() - 1).unwrap(), self.sign()).unwrap();
        let mut left = Vec::new();
        let min_foot: u16 = self.foot.into();
        let max_head: u16 = (self.head - 1).unwrap().into();
        for foot_idx in min_foot..max_head {
            left.push(ArtinGenerator::new(foot_idx, Sign::Negative).unwrap());
        }
        let right = left.iter().rev().map(|a| a.inverse()).collect();
        [left, vec![crossing], right].concat()
    }
}

impl From<ArtinGenerator> for BandGenerator {
    fn from(value: ArtinGenerator) -> Self {
        Self {
            foot: value.foot(),
            head: (value.foot() + 1).unwrap(),
            sign: value.sign(),
        }
    }
}
impl From<Letter> for BandGenerator {
    fn from(value: Letter) -> Self {
        match value {
            Letter::Artin(artin) => Self::from(artin),
            Letter::Band(band) => band,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{FromArtinError, MAX_BAND_HEIGHT, StaircaseQuadrant};
    use crate::{
        ArtinGenerator, BandGenerator, BandValidationError, BraidIndex, Letter, Sign, Strand,
    };
    use googletest::matchers::{anything, each, eq, err, ok};
    use googletest::{assert_that, expect_that, gtest};

    #[test]
    fn valid_inputs_to_new_yield_successful_construction() {
        let valid_bands = [
            BandGenerator::new(3, 4, Sign::Positive),
            BandGenerator::new(1, MAX_BAND_HEIGHT + 1, Sign::Negative),
            BandGenerator::new(2_usize, 5_isize, Sign::Positive),
            BandGenerator::new(Strand::new(9).unwrap(), 40_u32, Sign::Negative),
            BandGenerator::new(-(-3), Strand::new(10).unwrap(), Sign::Positive),
        ];

        assert_that!(valid_bands, each(ok(anything())));
    }

    #[test]
    fn valid_inputs_to_from_yield_expected_construction() {
        let expected_band = BandGenerator::new(1, 2, Sign::Positive).unwrap();
        let test_bands = [
            BandGenerator::from(ArtinGenerator::new(1, Sign::Positive).unwrap()),
            BandGenerator::from(Letter::new(1, None::<u16>, Sign::Positive).unwrap()),
            BandGenerator::from(Letter::new(1, Some(2), Sign::Positive).unwrap()),
        ];

        assert_that!(test_bands, each(eq(expected_band)));
    }

    #[gtest]
    fn valid_inputs_to_coalesce_yield_successful_construction() {
        expect_that!(
            BandGenerator::coalesce(&[ArtinGenerator::new(1, Sign::Negative).unwrap()]),
            eq(BandGenerator::new(1, 2, Sign::Negative))
        );

        let test_band = BandGenerator::new(1, 4, Sign::Positive);
        let valid_bands = [
            BandGenerator::coalesce(&[
                ArtinGenerator::new(1, Sign::Negative).unwrap(),
                ArtinGenerator::new(2, Sign::Negative).unwrap(),
                ArtinGenerator::new(3, Sign::Positive).unwrap(),
                ArtinGenerator::new(2, Sign::Positive).unwrap(),
                ArtinGenerator::new(1, Sign::Positive).unwrap(),
            ]),
            BandGenerator::coalesce(&[
                ArtinGenerator::new(3, Sign::Positive).unwrap(),
                ArtinGenerator::new(2, Sign::Positive).unwrap(),
                ArtinGenerator::new(1, Sign::Positive).unwrap(),
                ArtinGenerator::new(2, Sign::Negative).unwrap(),
                ArtinGenerator::new(3, Sign::Negative).unwrap(),
            ]),
            BandGenerator::coalesce(&[
                ArtinGenerator::new(3, Sign::Positive).unwrap(),
                ArtinGenerator::new(1, Sign::Negative).unwrap(),
                ArtinGenerator::new(2, Sign::Positive).unwrap(),
                ArtinGenerator::new(1, Sign::Positive).unwrap(),
                ArtinGenerator::new(3, Sign::Negative).unwrap(),
            ]),
            BandGenerator::coalesce(&[
                ArtinGenerator::new(1, Sign::Negative).unwrap(),
                ArtinGenerator::new(3, Sign::Positive).unwrap(),
                ArtinGenerator::new(2, Sign::Positive).unwrap(),
                ArtinGenerator::new(1, Sign::Positive).unwrap(),
                ArtinGenerator::new(3, Sign::Negative).unwrap(),
            ]),
        ];

        expect_that!(valid_bands, each(eq(test_band)));
    }

    #[gtest]
    fn properties_compute_as_expected() {
        let input_data = [(1, 2, Sign::Positive), (2, 5, Sign::Negative)];

        for (foot, head, sign) in input_data {
            let band = BandGenerator::new(foot, head, sign).unwrap();

            expect_that!(band.foot(), eq(Strand::new(foot).unwrap()));
            expect_that!(band.head(), eq(Strand::new(head).unwrap()));
            expect_that!(band.sign(), eq(sign));
            expect_that!(
                band.inverse(),
                eq(BandGenerator::new(foot, head, -sign).unwrap())
            );
            expect_that!(band.height(), eq(head - foot));
            expect_that!(band.is_artin(), eq(head - foot == 1));
            expect_that!(
                band.minimal_required_braid_index(),
                eq(BraidIndex::new(head).unwrap())
            );
            expect_that!(band.artin_length(), eq(2 * (head - foot) - 1));
        }
    }

    #[gtest]
    fn decomposition_works_as_expected() {
        let expected_results = [
            (
                BandGenerator::new(1, 2, Sign::Positive),
                vec![ArtinGenerator::new(1, Sign::Positive).unwrap()],
            ),
            (
                BandGenerator::new(1, 4, Sign::Positive),
                vec![
                    ArtinGenerator::new(1, Sign::Negative).unwrap(),
                    ArtinGenerator::new(2, Sign::Negative).unwrap(),
                    ArtinGenerator::new(3, Sign::Positive).unwrap(),
                    ArtinGenerator::new(2, Sign::Positive).unwrap(),
                    ArtinGenerator::new(1, Sign::Positive).unwrap(),
                ],
            ),
            (
                BandGenerator::coalesce(&[
                    ArtinGenerator::new(1, Sign::Negative).unwrap(),
                    ArtinGenerator::new(2, Sign::Negative).unwrap(),
                    ArtinGenerator::new(3, Sign::Positive).unwrap(),
                    ArtinGenerator::new(2, Sign::Positive).unwrap(),
                    ArtinGenerator::new(1, Sign::Positive).unwrap(),
                ]),
                vec![
                    ArtinGenerator::new(1, Sign::Negative).unwrap(),
                    ArtinGenerator::new(2, Sign::Negative).unwrap(),
                    ArtinGenerator::new(3, Sign::Positive).unwrap(),
                    ArtinGenerator::new(2, Sign::Positive).unwrap(),
                    ArtinGenerator::new(1, Sign::Positive).unwrap(),
                ],
            ),
            (
                BandGenerator::coalesce(&[
                    ArtinGenerator::new(1, Sign::Negative).unwrap(),
                    ArtinGenerator::new(3, Sign::Positive).unwrap(),
                    ArtinGenerator::new(2, Sign::Positive).unwrap(),
                    ArtinGenerator::new(1, Sign::Positive).unwrap(),
                    ArtinGenerator::new(3, Sign::Negative).unwrap(),
                ]),
                vec![
                    ArtinGenerator::new(1, Sign::Negative).unwrap(),
                    ArtinGenerator::new(2, Sign::Negative).unwrap(),
                    ArtinGenerator::new(3, Sign::Positive).unwrap(),
                    ArtinGenerator::new(2, Sign::Positive).unwrap(),
                    ArtinGenerator::new(1, Sign::Positive).unwrap(),
                ],
            ),
        ];

        for (band, expected_decomposition) in expected_results {
            expect_that!(band.unwrap().decompose(), eq(&expected_decomposition));
        }
    }

    #[gtest]
    fn invalid_inputs_to_new_yield_failure() {
        let invalid_bands = [
            (
                BandGenerator::new(1, 1, Sign::Positive),
                BandValidationError::FootOnHead(Strand::new(1).unwrap()),
            ),
            (
                BandGenerator::new(4, 1, Sign::Negative),
                BandValidationError::FootOverHead {
                    foot: Strand::new(4).unwrap(),
                    head: Strand::new(1).unwrap(),
                },
            ),
            (
                BandGenerator::new(1, MAX_BAND_HEIGHT + 2, Sign::Positive),
                BandValidationError::TooTall(MAX_BAND_HEIGHT + 1),
            ),
            (
                BandGenerator::new(0, 4, Sign::Negative),
                BandValidationError::from(Strand::new(0).err().unwrap()),
            ),
            (
                BandGenerator::new(-1, 4, Sign::Positive),
                BandValidationError::from(Strand::new(-1).err().unwrap()),
            ),
            (
                BandGenerator::new(1, u16::MAX as u32 + 1, Sign::Negative),
                BandValidationError::from(Strand::new(u16::MAX as u32 + 1).err().unwrap()),
            ),
        ];

        for (invalid_band, error) in invalid_bands {
            expect_that!(invalid_band, err(eq(error)));
        }
    }

    #[gtest]
    fn invalid_inputs_to_coalesce_yield_failure() {
        let invalid_artin_lists: [(Vec<ArtinGenerator>, BandValidationError); 8] = [
            (vec![], FromArtinError::NoGenerators.into()),
            (
                vec![
                    ArtinGenerator::new(1, Sign::Positive).unwrap(),
                    ArtinGenerator::new(2, Sign::Negative).unwrap(),
                ],
                FromArtinError::EvenGenerators.into(),
            ),
            (
                vec![ArtinGenerator::new(1, Sign::Positive).unwrap(); u16::MAX as usize + 2],
                FromArtinError::TooManyGenerators(u16::MAX as usize + 2).into(),
            ),
            (
                vec![
                    ArtinGenerator::new(2, Sign::Negative).unwrap(),
                    ArtinGenerator::new(1, Sign::Negative).unwrap(),
                    ArtinGenerator::new(3, Sign::Positive).unwrap(),
                    ArtinGenerator::new(2, Sign::Positive).unwrap(),
                    ArtinGenerator::new(1, Sign::Positive).unwrap(),
                ],
                FromArtinError::IncontiguousSteps {
                    quadrant: StaircaseQuadrant::LowerLeft,
                    next_step: ArtinGenerator::new(1, Sign::Negative).unwrap(),
                    previous_step: ArtinGenerator::new(3, Sign::Positive).unwrap(),
                }
                .into(),
            ),
            (
                vec![
                    ArtinGenerator::new(2, Sign::Positive).unwrap(),
                    ArtinGenerator::new(3, Sign::Positive).unwrap(),
                    ArtinGenerator::new(1, Sign::Positive).unwrap(),
                    ArtinGenerator::new(2, Sign::Negative).unwrap(),
                    ArtinGenerator::new(3, Sign::Negative).unwrap(),
                ],
                FromArtinError::IncontiguousSteps {
                    quadrant: StaircaseQuadrant::UpperLeft,
                    next_step: ArtinGenerator::new(3, Sign::Positive).unwrap(),
                    previous_step: ArtinGenerator::new(1, Sign::Positive).unwrap(),
                }
                .into(),
            ),
            (
                vec![
                    ArtinGenerator::new(1, Sign::Negative).unwrap(),
                    ArtinGenerator::new(2, Sign::Negative).unwrap(),
                    ArtinGenerator::new(3, Sign::Positive).unwrap(),
                    ArtinGenerator::new(1, Sign::Positive).unwrap(),
                    ArtinGenerator::new(2, Sign::Positive).unwrap(),
                ],
                FromArtinError::IncontiguousSteps {
                    quadrant: StaircaseQuadrant::LowerRight,
                    next_step: ArtinGenerator::new(1, Sign::Positive).unwrap(),
                    previous_step: ArtinGenerator::new(3, Sign::Positive).unwrap(),
                }
                .into(),
            ),
            (
                vec![
                    ArtinGenerator::new(3, Sign::Positive).unwrap(),
                    ArtinGenerator::new(2, Sign::Positive).unwrap(),
                    ArtinGenerator::new(1, Sign::Positive).unwrap(),
                    ArtinGenerator::new(3, Sign::Negative).unwrap(),
                    ArtinGenerator::new(2, Sign::Negative).unwrap(),
                ],
                FromArtinError::IncontiguousSteps {
                    quadrant: StaircaseQuadrant::UpperRight,
                    next_step: ArtinGenerator::new(3, Sign::Negative).unwrap(),
                    previous_step: ArtinGenerator::new(1, Sign::Positive).unwrap(),
                }
                .into(),
            ),
            (
                vec![
                    ArtinGenerator::new(4, Sign::Positive).unwrap(),
                    ArtinGenerator::new(2, Sign::Negative).unwrap(),
                    ArtinGenerator::new(3, Sign::Positive).unwrap(),
                    ArtinGenerator::new(2, Sign::Positive).unwrap(),
                    ArtinGenerator::new(1, Sign::Positive).unwrap(),
                ],
                FromArtinError::ImbalancedStaircases(1).into(),
            ),
        ];

        for (invalid_artin_list, error) in invalid_artin_lists {
            expect_that!(BandGenerator::coalesce(&invalid_artin_list), err(eq(error)));
        }
    }
}
