#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum StrandValidationError {
    #[error("Strand index cannot be zero.")]
    Zero,
    #[error("Attempt to subtract {right:?} from {left:?} results in non-positive-indexed strand.")]
    Subtraction { left: u16, right: u16 },
    #[error(
        "Attempt to add {left:?} to {right:?} results in strand index larger than {max}",
        max = u16::MAX,
    )]
    Addition { left: u16, right: u16 },
    #[error(transparent)]
    FromInt(#[from] std::num::TryFromIntError),
    #[error(transparent)]
    Infallible(#[from] std::convert::Infallible),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Strand(u16);

impl Strand {
    pub fn new<K>(index: K) -> Result<Self, StrandValidationError>
    where
        K: TryInto<u16>,
        StrandValidationError: From<<K as TryInto<u16>>::Error> + From<std::convert::Infallible>,
    {
        Self::try_from(index.try_into()?)
    }
}

impl TryFrom<u16> for Strand {
    type Error = StrandValidationError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(StrandValidationError::Zero)
        } else {
            Ok(Self(value))
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
    type Output = Result<Self, StrandValidationError>;
    fn add(self, rhs: Self) -> Self::Output {
        if u16::MAX - self.0 < rhs.0 {
            Err(StrandValidationError::Addition {
                left: self.0,
                right: rhs.0,
            })
        } else {
            Ok(Self(self.0 + rhs.0))
        }
    }
}
impl std::ops::Add<u16> for Strand {
    type Output = Result<Self, StrandValidationError>;
    fn add(self, rhs: u16) -> Self::Output {
        if u16::MAX - self.0 < rhs {
            Err(StrandValidationError::Addition {
                left: self.0,
                right: rhs,
            })
        } else {
            Ok(Self(self.0 + rhs))
        }
    }
}
impl std::ops::Add<Strand> for u16 {
    type Output = Result<Strand, StrandValidationError>;
    fn add(self, rhs: Strand) -> Self::Output {
        if u16::MAX - self < rhs.0 {
            Err(StrandValidationError::Addition {
                left: self,
                right: rhs.0,
            })
        } else {
            Ok(Strand(self + rhs.0))
        }
    }
}

impl std::ops::Sub for Strand {
    type Output = Result<Self, StrandValidationError>;
    fn sub(self, rhs: Self) -> Self::Output {
        if self.0 <= rhs.0 {
            Err(StrandValidationError::Subtraction {
                left: self.0,
                right: rhs.0,
            })
        } else {
            Self::new(self.0 - rhs.0)
        }
    }
}
impl std::ops::Sub<u16> for Strand {
    type Output = Result<Self, StrandValidationError>;
    fn sub(self, rhs: u16) -> Self::Output {
        if self.0 <= rhs {
            Err(StrandValidationError::Subtraction {
                left: self.0,
                right: rhs,
            })
        } else {
            Self::new(self.0 - rhs)
        }
    }
}
impl std::ops::Sub<Strand> for u16 {
    type Output = Result<Strand, StrandValidationError>;
    fn sub(self, rhs: Strand) -> Self::Output {
        if self <= rhs.0 {
            Err(StrandValidationError::Subtraction {
                left: self,
                right: rhs.0,
            })
        } else {
            Strand::new(self - rhs.0)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Strand, StrandValidationError};
    use googletest::matchers::{anything, derefs_to, each, eq, err, ok, result_of_ref};
    use googletest::prelude::__internal_unstable_do_not_depend_on_these::result_of_ref;
    use googletest::{assert_that, expect_that, gtest};

    #[test]
    fn valid_input_yields_successful_construction() {
        let valid_strands = [
            Strand::try_from(1),
            Strand::new(1),
            Strand::new(Strand::try_from(1).unwrap()),
        ];

        assert_that!(valid_strands, each(ok(anything())));
    }

    #[test]
    fn derefs_to_u16() {
        let test_strands = [
            Strand::try_from(1),
            Strand::new(1),
            Strand::new(Strand::try_from(1).unwrap()),
        ];

        assert_that!(test_strands, each(ok(derefs_to(eq(&1)))));
    }

    #[test]
    fn can_coerce_strand_into_u16() {
        let coerced_strands: [u16; 3] = [
            Strand::try_from(1).unwrap().into(),
            Strand::new(1).unwrap().into(),
            Strand::new(Strand::try_from(1).unwrap()).unwrap().into(),
        ];

        assert_that!(coerced_strands, each(eq(1)));
    }

    #[test]
    fn can_be_used_as_reference_to_u16() {
        fn as_ref_tester<T: AsRef<u16>>(s: T) -> u16 {
            *s.as_ref()
        }

        let test_strands = [
            Strand::try_from(1).unwrap(),
            Strand::new(1).unwrap(),
            Strand::new(Strand::try_from(1).unwrap()).unwrap(),
        ];

        assert_that!(
            test_strands,
            each(result_of_ref(
                |s: Strand| as_ref_tester(s),
                eq(&1),
                "A dummy function that accepts an AsRef<u16>-bounded argument."
            ))
        );
    }

    #[test]
    fn valid_addition_succeeds() {
        let addition_examples = [
            Strand::new(2).unwrap() + Strand::new(3).unwrap(),
            Strand::new(2).unwrap() + 3,
            2 + Strand::new(3).unwrap(),
            Strand::new(5).unwrap() + 0,
            0 + Strand::new(5).unwrap(),
        ];

        assert_that!(addition_examples, each(ok(derefs_to(eq(&5)))));
    }

    #[test]
    fn valid_subtraction_succeeds() {
        let addition_examples = [
            Strand::new(3).unwrap() - Strand::new(2).unwrap(),
            Strand::new(3).unwrap() - 2,
            3 - Strand::new(2).unwrap(),
        ];

        assert_that!(addition_examples, each(ok(derefs_to(eq(&1)))));
    }

    #[gtest]
    fn invalid_constructor_input_fails() {
        let invalid_strands = [
            (Strand::new(0), StrandValidationError::Zero),
            (Strand::try_from(0), StrandValidationError::Zero),
            (
                Strand::new(u16::MAX as u32 + 1),
                StrandValidationError::FromInt(
                    <u32 as TryInto<u16>>::try_into(u16::MAX as u32 + 1)
                        .err()
                        .unwrap(),
                ),
            ),
        ];

        for (invalid_strand, error) in invalid_strands {
            expect_that!(invalid_strand, err(eq(&error)));
        }
    }

    #[gtest]
    fn invalid_addition_fails() {
        let invalid_addition_examples = [
            (
                Strand::new(u16::MAX - 1).unwrap() + Strand::new(2).unwrap(),
                StrandValidationError::Addition {
                    left: u16::MAX - 1,
                    right: 2,
                },
            ),
            (
                Strand::new(2).unwrap() + Strand::new(u16::MAX - 1).unwrap(),
                StrandValidationError::Addition {
                    left: 2,
                    right: u16::MAX - 1,
                },
            ),
            (
                Strand::new(u16::MAX - 1).unwrap() + 2,
                StrandValidationError::Addition {
                    left: u16::MAX - 1,
                    right: 2,
                },
            ),
            (
                Strand::new(2).unwrap() + (u16::MAX - 1),
                StrandValidationError::Addition {
                    left: 2,
                    right: u16::MAX - 1,
                },
            ),
            (
                (u16::MAX - 1) + Strand::new(2).unwrap(),
                StrandValidationError::Addition {
                    left: u16::MAX - 1,
                    right: 2,
                },
            ),
            (
                2 + Strand::new(u16::MAX - 1).unwrap(),
                StrandValidationError::Addition {
                    left: 2,
                    right: u16::MAX - 1,
                },
            ),
        ];

        for (invalid_addition, error) in invalid_addition_examples {
            expect_that!(invalid_addition, err(eq(&error)));
        }
    }

    #[gtest]
    fn invalid_subtraction_fails() {
        let invalid_subtraction_examples = [
            (
                Strand::new(2).unwrap() - Strand::new(2).unwrap(),
                StrandValidationError::Subtraction { left: 2, right: 2 },
            ),
            (
                Strand::new(2).unwrap() - 2,
                StrandValidationError::Subtraction { left: 2, right: 2 },
            ),
            (
                2 - Strand::new(2).unwrap(),
                StrandValidationError::Subtraction { left: 2, right: 2 },
            ),
            (
                Strand::new(2).unwrap() - Strand::new(3).unwrap(),
                StrandValidationError::Subtraction { left: 2, right: 3 },
            ),
            (
                Strand::new(2).unwrap() - 3,
                StrandValidationError::Subtraction { left: 2, right: 3 },
            ),
            (
                2 - Strand::new(3).unwrap(),
                StrandValidationError::Subtraction { left: 2, right: 3 },
            ),
        ];

        for (invalid_subtraction, error) in invalid_subtraction_examples {
            expect_that!(invalid_subtraction, err(eq(&error)));
        }
    }
}
