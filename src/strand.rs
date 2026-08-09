use crate::{StrandResult, StrandValidationError};

/// A wrapper around a [`u16`] representing the index of a braid strand.
///
/// Braids encode weaving patterns among a collection of disjoint _strands_, or _strings_. Thus,
/// it seems reasonable that a library dedicated to braids should have a means of representing such
/// a fundamental concept. That is the purpose of this struct.
///
/// # Why [`u16`]?
///
/// The [`u16`] type was chosen as the inner type for both [`Strand`] and
/// [`BraidIndex`](crate::BraidIndex) in an attempt at balancing performance with practicality.
/// While the maximal size of the [`u8`] type is plenty large for any visualization purposes, there
/// may nevertheless be computational applications which can make use of the larger upper bound on
/// the number of strands. If it turns out that the performance gains of using a relatively small
/// integer type are negligible in comparison to the demand for greater computational freedom, then
/// a refactor should be straightforward to perform in the future.
///
/// # Construction
///
/// The recommended means of constructing a [`Strand`] is via the associated function
/// [`Strand::try_new`]. The reader is referred to the documentation for that function for further
/// detail.
///
/// # Interoperability with [`u16`]
///
/// To make [`Strand`] more ergonomic, a few traits are implemented to make it easy to use in place
/// of a [`u16`].
///
/// First, we implement [`From<Strand>`] for [`u16`]:
///
/// ```
/// use braided::Strand;
///
/// assert_eq!(u16::from(Strand::try_new(1).unwrap()), 1);
/// ```
///
/// Second, [`std::ops::Deref`] is iplemented on [`Strand`] to allow easy dereferencing into a
/// [`u16`]:
///
/// ```
/// use braided::Strand;
///
/// assert_eq!(*Strand::try_new(1).unwrap(), 1);
/// ```
///
/// Finally, [`AsRef<u16>`] is implemented on [`Strand`], allowing for [`Strand`] to be used in
/// generic function contexts where the input is any reference that can be converted into a [`u16`].
///
/// ```
/// use braided::Strand;
///
/// fn double<S: AsRef<u16>>(s: S) -> u16 { s.as_ref() * 2 }
///
/// assert_eq!(double(Strand::try_new(2).unwrap()), 4);
/// ```
/// Note that as [`BraidIndex`](crate::BraidIndex) also implements [`AsRef<u16>`], this allows for
/// defining generic functions which accept both [`Strand`] and [`BraidIndex`](crate::BraidIndex).
///
/// # Strand Arithmetic
///
/// We also implement all of the following to enable [`Strand`] to be used more ergonomically in
/// arithmetic contexts:
///
/// - [`std::ops::Add<Strand>`] and [`std::ops::Add<u16>`] for [`Strand`]:
///
/// ```
/// use braided::Strand;
///
/// assert_eq!(Strand::try_new(2).unwrap() + Strand::try_new(3).unwrap(), Strand::try_new(5));
///
/// assert_eq!(Strand::try_new(2).unwrap() + 3, Strand::try_new(5));
/// ```
///
/// - [`std::ops::Add<Strand>`] and for [`u16`]:
///
/// ```
/// use braided::Strand;
///
/// assert_eq!(2 + Strand::try_new(3).unwrap(), Strand::try_new(5));
/// ```
///
/// - [`std::ops::Sub<Strand>`] and [`std::ops::Sub<u16>`] for [`Strand`]:
///
/// ```
/// use braided::Strand;
///
/// assert_eq!(Strand::try_new(3).unwrap() - Strand::try_new(2).unwrap(), Strand::try_new(1));
///
/// assert_eq!(Strand::try_new(3).unwrap() - 2, Strand::try_new(1));
/// ```
///
/// - [`std::ops::Sub<Strand>`] for [`u16`]:
///
/// ```
/// use braided::Strand;
///
/// assert_eq!(3 - Strand::try_new(2).unwrap(), Strand::try_new(1));
/// ```
///
/// Note that each of the arithmetic operations above is fallible. See the documentation for
/// [`StrandValidationError`] for more information on the possible error results.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Strand(u16);

impl Strand {
    /// Attempts to construct a [`Strand`] from any type implementing [`TryInto<u16>`], failing if
    /// the input cannot be validated.
    ///
    /// This is the recommended means of constructing a new strand.
    ///
    /// <div class="warning">
    ///
    /// The return type is [`StrandResult`](StrandResult), which is a new-type wrapper around
    /// [`Result<Strand, StrandValidationError>`]. Use the dereference operator "*" for easy access to
    /// the inner value.
    ///
    /// </div>
    ///
    /// # Examples
    ///
    /// One may construct a new [`Strand`] directly from a [`u16`]:
    ///
    /// ```
    /// use braided::Strand;
    /// use std::assert_matches;
    ///
    /// assert_matches!(*Strand::try_new(1), Ok(_));
    /// ```
    ///
    /// or from anything that coerces into a [`u16`]:
    ///
    /// ```
    /// use braided::Strand;
    /// use std::assert_matches;
    ///
    /// assert_matches!(*Strand::try_new(i16::MAX), Ok(_));
    /// assert_matches!(*Strand::try_new(-(i16::MIN + 1)), Ok(_));
    /// assert_matches!(*Strand::try_new(1 as usize), Ok(_));
    /// assert_matches!(*Strand::try_new(-(-1 as isize)), Ok(_));
    /// ```
    ///
    /// including other [`Strand`]s:
    ///
    /// ```
    /// use braided::Strand;
    /// use std::assert_matches;
    ///
    /// assert_matches!(*Strand::try_new(Strand::try_new(1).unwrap()), Ok(_));
    /// ```
    ///
    /// as well as [`BraidIndex`](crate::BraidIndex):
    ///
    /// ```
    /// use braided::{BraidIndex, Strand};
    /// use std::assert_matches;
    ///
    /// assert_matches!(*Strand::try_new(BraidIndex::try_new(1).unwrap()), Ok(_));
    /// ```
    pub fn try_new<K>(index: K) -> StrandResult
    where
        K: TryInto<u16>,
        StrandValidationError: From<<K as TryInto<u16>>::Error> + From<std::convert::Infallible>,
    {
        let index = match index.try_into() {
            Ok(index) => index,
            Err(e) => return StrandResult::from(StrandValidationError::from(e)),
        };
        if index == 0 {
            StrandResult::from(StrandValidationError::Zero)
        } else {
            StrandResult::from(Self(index))
        }
    }
}

impl From<Strand> for u16 {
    fn from(value: Strand) -> Self {
        value.0
    }
}

impl std::ops::Deref for Strand {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl AsRef<u16> for Strand {
    fn as_ref(&self) -> &u16 {
        self
    }
}

impl std::ops::Add for Strand {
    type Output = StrandResult;
    fn add(self, rhs: Self) -> Self::Output {
        if u16::MAX - self.0 < rhs.0 {
            StrandResult::from(StrandValidationError::Addition {
                left: self.0,
                right: rhs.0,
            })
        } else {
            Self::try_new(self.0 + rhs.0)
        }
    }
}
impl std::ops::Add<u16> for Strand {
    type Output = StrandResult;
    fn add(self, rhs: u16) -> Self::Output {
        if u16::MAX - self.0 < rhs {
            StrandResult::from(StrandValidationError::Addition {
                left: self.0,
                right: rhs,
            })
        } else {
            Self::try_new(self.0 + rhs)
        }
    }
}
impl std::ops::Add<Strand> for u16 {
    type Output = StrandResult;
    fn add(self, rhs: Strand) -> Self::Output {
        if u16::MAX - self < rhs.0 {
            StrandResult::from(StrandValidationError::Addition {
                left: self,
                right: rhs.0,
            })
        } else {
            Strand::try_new(self + rhs.0)
        }
    }
}

impl std::ops::Sub for Strand {
    type Output = StrandResult;
    fn sub(self, rhs: Self) -> Self::Output {
        if self.0 <= rhs.0 {
            StrandResult::from(StrandValidationError::Subtraction {
                left: self.0,
                right: rhs.0,
            })
        } else {
            Self::try_new(self.0 - rhs.0)
        }
    }
}
impl std::ops::Sub<u16> for Strand {
    type Output = StrandResult;
    fn sub(self, rhs: u16) -> Self::Output {
        if self.0 <= rhs {
            StrandResult::from(StrandValidationError::Subtraction {
                left: self.0,
                right: rhs,
            })
        } else {
            Self::try_new(self.0 - rhs)
        }
    }
}
impl std::ops::Sub<Strand> for u16 {
    type Output = StrandResult;
    fn sub(self, rhs: Strand) -> Self::Output {
        if self <= rhs.0 {
            StrandResult::from(StrandValidationError::Subtraction {
                left: self,
                right: rhs.0,
            })
        } else {
            Strand::try_new(self - rhs.0)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{BraidIndex, Strand, StrandValidationError};
    use googletest::matchers::{anything, derefs_to, each, eq, err, ok, result_of_ref};
    use googletest::{assert_that, expect_that, gtest};

    #[test]
    fn valid_input_yields_successful_construction() {
        let valid_strands = [
            Strand::try_new(1),
            Strand::try_new(Strand::try_new(1).unwrap()),
            Strand::try_new(BraidIndex::try_new(1).unwrap()),
        ];

        assert_that!(&valid_strands, each(derefs_to(ok(anything()))));
    }

    #[test]
    fn derefs_to_u16() {
        let test_strands = [
            &Strand::try_new(1),
            &Strand::try_new(Strand::try_new(1).unwrap()),
            &Strand::try_new(BraidIndex::try_new(1).unwrap()),
        ];

        assert_that!(test_strands, each(derefs_to(ok(derefs_to(eq(&1))))));
    }

    #[test]
    fn can_coerce_strand_into_u16() {
        let coerced_strands: [u16; 3] = [
            Strand::try_new(1).unwrap().into(),
            Strand::try_new(Strand::try_new(1).unwrap()).unwrap().into(),
            Strand::try_new(BraidIndex::try_new(1).unwrap())
                .unwrap()
                .into(),
        ];

        assert_that!(coerced_strands, each(eq(1)));
    }

    #[test]
    fn can_be_used_as_reference_to_u16() {
        fn as_ref_tester<T: AsRef<u16>>(s: T) -> u16 {
            *s.as_ref()
        }

        let test_strands = [
            Strand::try_new(1).unwrap(),
            Strand::try_new(Strand::try_new(1).unwrap()).unwrap(),
            Strand::try_new(BraidIndex::try_new(1).unwrap()).unwrap(),
        ];

        assert_that!(
            test_strands,
            each(result_of_ref!(|s: Strand| as_ref_tester(s), eq(&1)))
        );
    }

    #[test]
    fn valid_addition_succeeds() {
        let addition_examples = [
            &(Strand::try_new(2).unwrap() + Strand::try_new(3).unwrap()),
            &(Strand::try_new(2).unwrap() + 3),
            &(2 + Strand::try_new(3).unwrap()),
            &(Strand::try_new(5).unwrap() + 0),
            &(0 + Strand::try_new(5).unwrap()),
        ];

        assert_that!(addition_examples, each(derefs_to(ok(derefs_to(eq(&5))))));
    }

    #[test]
    fn valid_subtraction_succeeds() {
        let addition_examples = [
            &(Strand::try_new(3).unwrap() - Strand::try_new(2).unwrap()),
            &(Strand::try_new(3).unwrap() - 2),
            &(3 - Strand::try_new(2).unwrap()),
        ];

        assert_that!(addition_examples, each(derefs_to(ok(derefs_to(eq(&1))))));
    }

    #[gtest]
    fn invalid_constructor_input_fails() {
        let invalid_strands = [
            (Strand::try_new(0), StrandValidationError::Zero),
            (
                Strand::try_new(-1),
                StrandValidationError::FromInt(<i16 as TryInto<u16>>::try_into(-1).err().unwrap()),
            ),
            (
                Strand::try_new(u16::MAX as u32 + 1),
                StrandValidationError::FromInt(
                    <u32 as TryInto<u16>>::try_into(u16::MAX as u32 + 1)
                        .err()
                        .unwrap(),
                ),
            ),
        ];

        for (invalid_strand, error) in invalid_strands {
            expect_that!(*invalid_strand, err(eq(error)));
        }
    }

    #[gtest]
    fn invalid_addition_fails() {
        let invalid_addition_examples = [
            (
                Strand::try_new(u16::MAX - 1).unwrap() + Strand::try_new(2).unwrap(),
                StrandValidationError::Addition {
                    left: u16::MAX - 1,
                    right: 2,
                },
            ),
            (
                Strand::try_new(2).unwrap() + Strand::try_new(u16::MAX - 1).unwrap(),
                StrandValidationError::Addition {
                    left: 2,
                    right: u16::MAX - 1,
                },
            ),
            (
                Strand::try_new(u16::MAX - 1).unwrap() + 2,
                StrandValidationError::Addition {
                    left: u16::MAX - 1,
                    right: 2,
                },
            ),
            (
                Strand::try_new(2).unwrap() + (u16::MAX - 1),
                StrandValidationError::Addition {
                    left: 2,
                    right: u16::MAX - 1,
                },
            ),
            (
                (u16::MAX - 1) + Strand::try_new(2).unwrap(),
                StrandValidationError::Addition {
                    left: u16::MAX - 1,
                    right: 2,
                },
            ),
            (
                2 + Strand::try_new(u16::MAX - 1).unwrap(),
                StrandValidationError::Addition {
                    left: 2,
                    right: u16::MAX - 1,
                },
            ),
        ];

        for (invalid_addition, error) in invalid_addition_examples {
            expect_that!(*invalid_addition, err(eq(error)));
        }
    }

    #[gtest]
    fn invalid_subtraction_fails() {
        let invalid_subtraction_examples = [
            (
                Strand::try_new(2).unwrap() - Strand::try_new(2).unwrap(),
                StrandValidationError::Subtraction { left: 2, right: 2 },
            ),
            (
                Strand::try_new(2).unwrap() - 2,
                StrandValidationError::Subtraction { left: 2, right: 2 },
            ),
            (
                2 - Strand::try_new(2).unwrap(),
                StrandValidationError::Subtraction { left: 2, right: 2 },
            ),
            (
                Strand::try_new(2).unwrap() - Strand::try_new(3).unwrap(),
                StrandValidationError::Subtraction { left: 2, right: 3 },
            ),
            (
                Strand::try_new(2).unwrap() - 3,
                StrandValidationError::Subtraction { left: 2, right: 3 },
            ),
            (
                2 - Strand::try_new(3).unwrap(),
                StrandValidationError::Subtraction { left: 2, right: 3 },
            ),
        ];

        for (invalid_subtraction, error) in invalid_subtraction_examples {
            expect_that!(*invalid_subtraction, err(eq(error)));
        }
    }
}
