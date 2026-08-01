use std::ops::{Add, Sub};

/// Error type for failed [`Strand`] construction or bad [`Strand`] arithmetic.
///
/// Validation of [`Strand`] construction mostly boils down to verifying that the strand index is
/// positive.
///
/// # Examples
///
/// ```
/// use braided::{Strand, StrandValidationError};
/// use std::assert_matches;
///
/// // Strands are 1-indexed:
/// let zero_strand = Strand::new(0);
/// assert_matches!(zero_strand, Err(StrandValidationError::Zero));
///
/// // Negative strand indices are not allowed:
/// let strand_1 = Strand::new(1).unwrap();
/// let strand_2 = Strand::new(2).unwrap();
/// assert_matches!(strand_1 - strand_2, Err(StrandValidationError::Negative { .. }));
/// ```
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum StrandValidationError {
    #[error("Strand index cannot be zero.")]
    Zero,
    #[error("Attempt to subtract {right:?} from {left:?} results in negative-indexed strand.")]
    Negative { left: Strand, right: u16 },
    #[error(
        "Attempt to add {left:?} to {right:?} results in strand index larger than {max}",
        max = u16::MAX,
    )]
    TooLarge { left: Strand, right: u16 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Strand(u16);

impl Strand {
    pub fn new(index: u16) -> Result<Self, StrandValidationError> {
        if index == 0 {
            return Err(StrandValidationError::Zero);
        }
        Ok(Self(index))
    }

    pub fn index(&self) -> u16 {
        self.0
    }
}

impl Sub for Strand {
    type Output = Result<Self, StrandValidationError>;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.0 < rhs.0 {
            return Err(StrandValidationError::Negative {
                left: self,
                right: rhs.0,
            });
        }
        Self::new(self.0 - rhs.0)
    }
}

impl Sub<u16> for Strand {
    type Output = Result<Self, StrandValidationError>;

    fn sub(self, rhs: u16) -> Self::Output {
        if self.0 < rhs {
            return Err(StrandValidationError::Negative {
                left: self,
                right: rhs,
            });
        }
        Self::new(self.0 - rhs)
    }
}

impl Add for Strand {
    type Output = Result<Self, StrandValidationError>;

    fn add(self, rhs: Self) -> Self::Output {
        if u16::MAX - self.0 < rhs.0 {
            Err(StrandValidationError::TooLarge {
                left: self,
                right: rhs.0,
            })
        } else {
            Ok(Self(self.0 + rhs.0))
        }
    }
}

impl Add<u16> for Strand {
    type Output = Result<Self, StrandValidationError>;

    fn add(self, rhs: u16) -> Self::Output {
        if u16::MAX - self.0 < rhs {
            Err(StrandValidationError::TooLarge {
                left: self,
                right: rhs,
            })
        } else {
            Ok(Self(self.0 + rhs))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Strand, StrandValidationError};
    use googletest::assert_that;
    use googletest::matchers::{anything, eq, err, ok};

    #[test]
    fn valid_strand_can_be_constructed() {
        let strand = Strand::new(1);
        assert_that!(strand, ok(anything()));
        assert_that!(strand.unwrap().index(), eq(1));
    }

    #[test]
    fn zero_strand_cannot_be_constructed() {
        let strand = Strand::new(0);
        assert_that!(strand, err(eq(&StrandValidationError::Zero)));
    }

    #[test]
    fn valid_subtraction_of_strands_succeeds() {
        let s1 = Strand::new(5).unwrap();
        let s2 = Strand::new(3).unwrap();
        let result = s1 - s2;
        assert_that!(result, ok(eq(&Strand(2))));
    }

    #[test]
    fn invalid_subtraction_of_strands_fails() {
        let s1 = Strand::new(3).unwrap();
        let s2 = Strand::new(5).unwrap();
        let result = s1 - s2;
        assert_that!(
            result,
            err(eq(&StrandValidationError::Negative {
                left: s1,
                right: s2.index(),
            }))
        );
    }

    #[test]
    fn valid_subtraction_of_u16_from_strand_succeeds() {
        let s1 = Strand::new(5).unwrap();
        let result = s1 - 3;
        assert_that!(result, ok(eq(&Strand(2))));
    }

    #[test]
    fn invalid_subtraction_of_u16_from_strand_fails() {
        let s1 = Strand::new(3).unwrap();
        let result = s1 - 5;
        assert_that!(
            result,
            err(eq(&StrandValidationError::Negative { left: s1, right: 5 }))
        );
    }

    #[test]
    fn two_strands_can_be_added() {
        let s1 = Strand::new(5).unwrap();
        let s2 = Strand::new(3).unwrap();
        let result = s1 + s2;
        assert_that!(result, ok(eq(&Strand(8))));
    }

    #[test]
    fn a_u16_can_be_added_to_a_strand() {
        let s1 = Strand::new(5).unwrap();
        let result = s1 + 3;
        assert_that!(result, ok(eq(&Strand(8))));
    }

    #[test]
    fn strand_index_cannot_exceed_max_u16() {
        let s1 = Strand::new(u16::MAX).unwrap();
        let s2 = Strand::new(1).unwrap();
        let result = s1 + s2;
        assert_that!(
            result,
            err(eq(&StrandValidationError::TooLarge {
                left: s1,
                right: s2.index()
            }))
        );

        let s3 = Strand::new(1).unwrap();
        assert_that!(
            s3 + u16::MAX,
            err(eq(&StrandValidationError::TooLarge {
                left: s3,
                right: u16::MAX
            }))
        );
    }
}
