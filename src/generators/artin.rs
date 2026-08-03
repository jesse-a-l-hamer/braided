use crate::{BandGenerator, BraidIndex, Letter, Sign, Strand, StrandValidationError};

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum ArtinValidationError {
    #[error("Given band {0:?} cannot be coerced to Artin generator.")]
    BandIsNotArtin(BandGenerator),
    #[error(transparent)]
    StrandValidation(#[from] StrandValidationError),
    #[error(transparent)]
    Infallible(#[from] std::convert::Infallible),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ArtinGenerator {
    foot: Strand,
    sign: Sign,
}

impl ArtinGenerator {
    pub fn new<F>(foot: F, sign: Sign) -> Result<Self, ArtinValidationError>
    where
        F: TryInto<u16>,
        StrandValidationError: From<<F as TryInto<u16>>::Error> + From<std::convert::Infallible>,
    {
        let foot: u16 = foot.try_into().map_err(StrandValidationError::from)?;
        if foot == u16::MAX {
            // The head strand is too large.
            Err(ArtinValidationError::StrandValidation(
                StrandValidationError::Addition {
                    left: foot,
                    right: 1,
                },
            ))
        } else {
            Ok(Self {
                foot: Strand::new(foot)?,
                sign,
            })
        }
    }

    pub fn as_band(&self) -> BandGenerator {
        BandGenerator::new(self.foot, (self.foot + 1).unwrap(), self.sign).unwrap()
    }

    pub fn foot(&self) -> Strand {
        self.foot
    }
    pub fn sign(&self) -> Sign {
        self.sign
    }

    pub fn inverse(&self) -> Self {
        Self {
            foot: self.foot,
            sign: -self.sign,
        }
    }

    pub fn minimal_required_braid_index(&self) -> BraidIndex {
        BraidIndex::new((self.foot + 1).unwrap()).unwrap()
    }
}

impl TryFrom<BandGenerator> for ArtinGenerator {
    type Error = ArtinValidationError;

    fn try_from(value: BandGenerator) -> Result<Self, Self::Error> {
        if value.is_artin() {
            Ok(Self {
                foot: value.foot(),
                sign: value.sign(),
            })
        } else {
            Err(ArtinValidationError::BandIsNotArtin(value))
        }
    }
}
impl TryFrom<Letter> for ArtinGenerator {
    type Error = ArtinValidationError;

    fn try_from(value: Letter) -> Result<Self, Self::Error> {
        match value {
            Letter::Artin(artin) => Ok(artin),
            Letter::Band(band) => Self::try_from(band),
        }
    }
}
