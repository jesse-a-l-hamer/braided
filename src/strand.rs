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
