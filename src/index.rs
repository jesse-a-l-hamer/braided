use crate::{IndexResult, IndexValidationError, Strand};

/// A wrapper around a [`u16`] representing the total number of [strands](crate::Strand) in a given
/// braid.
///
/// # A Bit About the Braid Index
///
/// Algebraically speaking, multiplication of braids is only defined for two braids with the same
/// [index](BraidIndex). Geometrically, too, "multiplication" of braids only makes sense if
/// two braids have the same [index](BraidIndex): thinking of a braid as a vertical stack of
/// curves in space which lie parallel in the same plane outside of small neighborhoods about the
/// crossings, then "multiplying" two braids together amounts to situating same-height strands
/// adjacent to one another and "gluing" the ends together so that the picture is once again a
/// vertical stack of parallel strands (except near the crossings). We require that each strand have
/// a single partner that it "glues" to, and this means that both of the original braids must have
/// the same [index](BraidIndex).
///
/// Of course, there are ways, both algebraic and geometric, to circumvent the requirement that two
/// braids must have the same [`index`](BraidIndex) for their product to be defined. For example,
/// every braid group embeds into every braid group of higher index (geometrically, this amounts to
/// adding some extra, unwoven strands above the smaller-index braid). In fact, one may construct
/// the [direct limit](https://en.wikipedia.org/wiki/Direct_limit) of braid groups, which serves as
/// a context in which any two braids _can_ be multiplied, regardless of their index (geometrically,
/// this corresponds to braids with infinitely many strands).
///
/// All this having been said, we will content ourselves in this library to assume that any
/// [`BraidIndex`] is finite.
///
/// # Why [`u16`]?
///
/// The [`u16`] type was chosen as the inner type for both [`Strand`][crate::Strand] and
/// [`BraidIndex`] in an attempt at balancing performance with practicality.
/// While the maximal size of the [`u8`] type is plenty large for any visualization purposes, there
/// may nevertheless be computational applications which can make use of the larger upper bound on
/// the number of strands. If it turns out that the performance gains of using a relatively small
/// integer type are negligible in comparison to the demand for greater computational freedom, then
/// a refactor should be straightforward to perform in the future.
///
/// # Construction
///
/// The recommended means of constructing a [`BraidIndex`] is via the associated function
/// [`BraidIndex::new`]. Please see the documentation for that function for more details and
/// examples.
///
/// # Interoperability with [`u16`]
///
/// [`BraidIndex`] implements the following traits to enable more ergonomic interaction in contexts
/// that need to access the wrapped [`u16`] type:
///
/// 1. [`From<BraidIndex>`] for [`u16`].
///
/// ```
/// use braided::BraidIndex;
///
/// assert_eq!(u16::from(BraidIndex::new(1).unwrap()), 1);
/// ```
///
/// 2. [`std::ops::Deref`] for [`BraidIndex`], with [`Target = u16`](u16).
///
/// ```
/// use braided::BraidIndex;
///
/// assert_eq!(*BraidIndex::new(1).unwrap(), 1);
/// ```
///
/// 3. [`AsRef<u16>`] for [`BraidIndex`].
///
/// ```
/// use braided::BraidIndex;
///
/// fn double<S: AsRef<u16>>(s: S) -> u16 { s.as_ref() * 2 }
///
/// assert_eq!(double(BraidIndex::new(2).unwrap()), 4);
/// ```
///
/// Note that as [`Strand`] also implements [`AsRef<u16>`], this allows for defining generic
/// functions which accept both [`Strand`] and [`BraidIndex`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BraidIndex(u16);

impl BraidIndex {
    /// Attempts to construct a [`BraidIndex`] from any type implementing [`TryInto<u16>`], failing
    /// if the input cannot be validated.
    ///
    /// This is the recommended way of constructing a [`BraidIndex`].
    ///
    /// # Examples
    ///
    /// One may construct a new [`BraidIndex`] directly from a [`u16`]:
    ///
    /// ```
    /// use braided::BraidIndex;
    /// use std::assert_matches;
    ///
    /// assert_matches!(BraidIndex::new(1), Ok(_));
    /// ```
    ///
    /// or from anything that coerces into a [`u16`]:
    ///
    /// ```
    /// use braided::BraidIndex;
    /// use std::assert_matches;
    ///
    /// assert_matches!(BraidIndex::new(i16::MAX), Ok(_));
    /// assert_matches!(BraidIndex::new(-(i16::MIN + 1)), Ok(_));
    /// assert_matches!(BraidIndex::new(1 as usize), Ok(_));
    /// assert_matches!(BraidIndex::new(-(-1 as isize)), Ok(_));
    /// ```
    ///
    /// including other [`BraidIndex`]:
    ///
    /// ```
    /// use braided::BraidIndex;
    /// use std::assert_matches;
    ///
    /// assert_matches!(BraidIndex::new(BraidIndex::new(1).unwrap()), Ok(_));
    /// ```
    ///
    /// as well as [`Strand`]:
    ///
    /// ```
    /// use braided::{BraidIndex, Strand};
    /// use std::assert_matches;
    ///
    /// assert_matches!(BraidIndex::new(Strand::new(1).unwrap()), Ok(_));
    /// ```
    pub fn try_new<N>(index: N) -> IndexResult
    where
        N: TryInto<u16>,
        IndexValidationError: From<<N as TryInto<u16>>::Error>,
    {
        let index = match index.try_into() {
            Ok(index) => index,
            Err(e) => return IndexResult::from(IndexValidationError::from(e)),
        };
        if index == 0 {
            IndexResult::from(IndexValidationError::Zero)
        } else {
            IndexResult::from(Self(index))
        }
    }
}

impl From<Strand> for BraidIndex {
    fn from(value: Strand) -> Self {
        Self(value.into())
    }
}
impl From<BraidIndex> for u16 {
    fn from(value: BraidIndex) -> Self {
        value.0
    }
}

impl std::ops::Deref for BraidIndex {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl AsRef<u16> for BraidIndex {
    fn as_ref(&self) -> &u16 {
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::{BraidIndex, IndexResult, IndexValidationError, Strand};
    use googletest::matchers::{anything, derefs_to, each, eq, err, ok, result_of_ref};
    use googletest::{assert_that, expect_that, gtest};

    #[test]
    fn valid_input_yields_successful_construction() {
        let valid_indices = [
            IndexResult::from(BraidIndex::from(Strand::try_new(1).unwrap())),
            BraidIndex::try_new(1),
            BraidIndex::try_new(Strand::try_new(1).unwrap()),
            BraidIndex::try_new(BraidIndex::try_new(1).unwrap()),
        ];

        assert_that!(&valid_indices, each(derefs_to(ok(anything()))));
    }

    #[test]
    fn derefs_to_u16() {
        let test_indices = [
            &IndexResult::from(BraidIndex::from(Strand::try_new(1).unwrap())),
            &BraidIndex::try_new(1),
            &BraidIndex::try_new(Strand::try_new(1).unwrap()),
            &BraidIndex::try_new(BraidIndex::try_new(1).unwrap()),
        ];

        assert_that!(test_indices, each(derefs_to(ok(derefs_to(eq(&1))))));
    }

    #[test]
    fn can_coerce_index_into_u16() {
        let coerced_indices: [u16; 4] = [
            BraidIndex::from(Strand::try_new(1).unwrap()).into(),
            BraidIndex::try_new(1).unwrap().into(),
            BraidIndex::try_new(Strand::try_new(1).unwrap())
                .unwrap()
                .into(),
            BraidIndex::try_new(BraidIndex::try_new(1).unwrap())
                .unwrap()
                .into(),
        ];

        assert_that!(coerced_indices, each(eq(1)));
    }

    #[test]
    fn can_be_used_as_reference_to_u16() {
        fn as_ref_tester<T: AsRef<u16>>(n: T) -> u16 {
            *n.as_ref()
        }

        let test_indices = [
            BraidIndex::from(Strand::try_new(1).unwrap()),
            BraidIndex::try_new(1).unwrap(),
            BraidIndex::try_new(Strand::try_new(1).unwrap()).unwrap(),
            BraidIndex::try_new(BraidIndex::try_new(1).unwrap()).unwrap(),
        ];

        assert_that!(
            test_indices,
            each(result_of_ref!(|n: BraidIndex| as_ref_tester(n), eq(&1)))
        );
    }

    #[gtest]
    fn invalid_constructor_input_fails() {
        let invalid_indices = [
            (BraidIndex::try_new(0), IndexValidationError::Zero),
            (
                BraidIndex::try_new(-1),
                IndexValidationError::FromInt(<i16 as TryInto<u16>>::try_into(-1).err().unwrap()),
            ),
            (
                BraidIndex::try_new(u16::MAX as u32 + 1),
                IndexValidationError::FromInt(
                    <u32 as TryInto<u16>>::try_into(u16::MAX as u32 + 1)
                        .err()
                        .unwrap(),
                ),
            ),
        ];

        for (invalid_index, error) in invalid_indices {
            expect_that!(*invalid_index, err(eq(error)));
        }
    }
}
