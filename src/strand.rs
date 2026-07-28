use std::ops::{Add, Sub};

#[derive(Debug, thiserror::Error)]
pub enum StrandValidationError {
    #[error("Strand index cannot be negative.")]
    NegativeStrand,
    #[error("Strand index cannot be zero.")]
    ZeroStrand,
    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Strand(u16);

impl Strand {
    pub fn new(index: u16) -> Result<Self, StrandValidationError> {
        if index == 0 {
            return Err(StrandValidationError::ZeroStrand);
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
            return Err(StrandValidationError::NegativeStrand);
        }
        Self::new(self.0 - rhs.0)
    }
}

impl Sub<u16> for Strand {
    type Output = Result<Self, StrandValidationError>;

    fn sub(self, rhs: u16) -> Self::Output {
        if self.0 < rhs {
            return Err(StrandValidationError::NegativeStrand);
        }
        Self::new(self.0 - rhs)
    }
}

impl Add for Strand {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Add<u16> for Strand {
    type Output = Self;

    fn add(self, rhs: u16) -> Self::Output {
        Self(self.0 + rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::{Strand, StrandValidationError};
    use googletest::assert_that;
    use googletest::matchers::{anything, eq, ok};
    use std::assert_matches;

    #[test]
    fn valid_strand_can_be_constructed() {
        let strand = Strand::new(1);
        assert_that!(strand, ok(anything()));
        assert_that!(strand.unwrap().index(), eq(1));
    }

    #[test]
    fn zero_strand_cannot_be_constructed() {
        let strand = Strand::new(0);
        assert_matches!(strand, Err(StrandValidationError::ZeroStrand));
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
        assert_matches!(result, Err(StrandValidationError::NegativeStrand));
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
        assert_matches!(result, Err(StrandValidationError::NegativeStrand));
    }

    #[test]
    fn two_strands_can_be_added() {
        let s1 = Strand::new(5).unwrap();
        let s2 = Strand::new(3).unwrap();
        let result = s1 + s2;
        assert_that!(result, eq(Strand(8)));
    }

    #[test]
    fn a_u16_can_be_added_to_a_strand() {
        let s1 = Strand::new(5).unwrap();
        let result = s1 + 3;
        assert_that!(result, eq(Strand(8)));
    }
}
