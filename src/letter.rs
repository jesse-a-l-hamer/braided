use crate::{
    ArtinGenerator, ArtinValidationError, BandGenerator, BandValidationError, BraidIndex, Sign,
    Strand, StrandValidationError, Word, WordValidationError,
};

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum LetterValidationError {
    #[error(transparent)]
    ArtinValidation(#[from] ArtinValidationError),
    #[error(transparent)]
    BandValidation(#[from] BandValidationError),
}

#[derive(Debug, Clone, Copy, Eq)]
pub enum Letter {
    Artin(ArtinGenerator),
    Band(BandGenerator),
}

impl Letter {
    pub fn new<F, H>(foot: F, head: Option<H>, sign: Sign) -> Result<Self, LetterValidationError>
    where
        F: TryInto<u16>,
        H: TryInto<u16>,
        StrandValidationError: From<<F as TryInto<u16>>::Error>
            + From<<H as TryInto<u16>>::Error>
            + From<std::convert::Infallible>,
    {
        match head {
            Some(head) => Ok(Self::Band(BandGenerator::new(foot, head, sign)?)),
            None => Ok(Self::Artin(ArtinGenerator::new(foot, sign)?)),
        }
    }

    pub fn sign(&self) -> Sign {
        match self {
            Self::Artin(artin) => artin.sign(),
            Self::Band(band) => band.sign(),
        }
    }
    pub fn foot(&self) -> Strand {
        match self {
            Self::Artin(artin) => artin.foot(),
            Self::Band(band) => band.foot(),
        }
    }
    pub fn head(&self) -> Strand {
        match self {
            Self::Artin(artin) => artin.head(),
            Self::Band(band) => band.head(),
        }
    }

    pub fn inverse(&self) -> Self {
        match self {
            Self::Artin(artin) => Self::Artin(artin.inverse()),
            Self::Band(band) => Self::Band(band.inverse()),
        }
    }

    pub fn is_artin(&self) -> bool {
        match self {
            Self::Artin(_) => true,
            Self::Band(band) => band.is_artin(),
        }
    }

    pub fn artin_length(&self) -> u16 {
        match self {
            Self::Artin(_) => 1,
            Self::Band(band) => band.artin_length(),
        }
    }
    pub fn minimal_required_braid_index(&self) -> BraidIndex {
        match self {
            Self::Artin(artin) => artin.minimal_required_braid_index(),
            Self::Band(band) => band.minimal_required_braid_index(),
        }
    }
}

impl From<ArtinGenerator> for Letter {
    fn from(value: ArtinGenerator) -> Self {
        Self::Artin(value)
    }
}
impl From<BandGenerator> for Letter {
    fn from(value: BandGenerator) -> Self {
        Self::Band(value)
    }
}

impl std::cmp::PartialEq for Letter {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Artin(lhs), Self::Artin(rhs)) => lhs == rhs,
            (Self::Artin(lhs), Self::Band(rhs)) => {
                rhs.is_artin() && *lhs == ArtinGenerator::new(rhs.foot(), rhs.sign()).unwrap()
            }
            (Self::Band(lhs), Self::Artin(rhs)) => {
                lhs.is_artin() && ArtinGenerator::new(lhs.foot(), rhs.sign()).unwrap() == *rhs
            }
            (Self::Band(lhs), Self::Band(rhs)) => lhs == rhs,
        }
    }
}

impl std::ops::Mul for Letter {
    type Output = Result<Word, WordValidationError>;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Artin(lhs), Self::Artin(rhs)) => {
                if lhs == rhs.inverse() {
                    Ok(Word::trivial())
                } else {
                    Word::try_from(vec![lhs, rhs])
                }
            }
            (Self::Artin(lhs), Self::Band(rhs)) => {
                if rhs.inverse() == lhs.into() {
                    Ok(Word::trivial())
                } else {
                    Word::try_from(vec![Self::Artin(lhs), Self::Band(rhs)])
                }
            }
            (Self::Band(lhs), Self::Artin(rhs)) => {
                if lhs.inverse() == rhs.into() {
                    Ok(Word::trivial())
                } else {
                    Word::try_from(vec![Self::Band(lhs), Self::Artin(rhs)])
                }
            }
            (Self::Band(lhs), Self::Band(rhs)) => {
                if lhs == rhs.inverse() {
                    Ok(Word::trivial())
                } else {
                    Word::try_from(vec![lhs, rhs])
                }
            }
        }
    }
}
