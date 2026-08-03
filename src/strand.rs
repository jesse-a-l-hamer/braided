#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum StrandValidationError {
    #[error("Strand index cannot be zero.")]
    Zero,
    #[error("Attempt to subtract {right:?} from {left:?} results in non-positive-indexed strand.")]
    Subtraction { left: Strand, right: u16 },
    #[error(
        "Attempt to add {left:?} to {right:?} results in strand index larger than {max}",
        max = u16::MAX,
    )]
    Addition { left: Strand, right: u16 },
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

impl std::ops::Sub for Strand {
    type Output = Result<Self, StrandValidationError>;
    fn sub(self, rhs: Self) -> Self::Output {
        if self.0 < rhs.0 {
            return Err(StrandValidationError::Subtraction {
                left: self,
                right: rhs.0,
            });
        }
        Self::new(self.0 - rhs.0)
    }
}
impl std::ops::Sub<u16> for Strand {
    type Output = Result<Self, StrandValidationError>;
    fn sub(self, rhs: u16) -> Self::Output {
        if self.0 < rhs {
            return Err(StrandValidationError::Subtraction {
                left: self,
                right: rhs,
            });
        }
        Self::new(self.0 - rhs)
    }
}

impl std::ops::Add for Strand {
    type Output = Result<Self, StrandValidationError>;
    fn add(self, rhs: Self) -> Self::Output {
        if u16::MAX - self.0 < rhs.0 {
            Err(StrandValidationError::Addition {
                left: self,
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
                left: self,
                right: rhs,
            })
        } else {
            Ok(Self(self.0 + rhs))
        }
    }
}
