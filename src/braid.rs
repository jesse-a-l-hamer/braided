use crate::{
    BraidIndex, IndexValidationError, Letter, Sign, StrandValidationError, Word,
    WordValidationError,
};

#[derive(Debug, thiserror::Error, PartialEq, Eq, Clone, Copy)]
pub enum BraidValidationError {
    #[error("Given index {index:?} less than minimal required index {minimal_required_index:?}.")]
    IndexTooSmall {
        index: BraidIndex,
        minimal_required_index: BraidIndex,
    },
    #[error("Attempt to multiply braids of unequal indices: {left:?} != {right:?}")]
    UnequalIndices { left: BraidIndex, right: BraidIndex },
    #[error(transparent)]
    IndexValidation(#[from] IndexValidationError),
    #[error(transparent)]
    WordValidation(#[from] WordValidationError),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Braid {
    index: BraidIndex,
    word: Word,
}

impl Braid {
    pub fn new<N>(index: Option<N>, word: Word) -> Result<Self, BraidValidationError>
    where
        N: TryInto<u16>,
        IndexValidationError: From<<N as TryInto<u16>>::Error>,
    {
        let minimal_required_index = word.minimal_required_braid_index();
        let index = if let Some(index) = index {
            BraidIndex::new(index)?
        } else {
            minimal_required_index
        };

        if index < minimal_required_index {
            Err(BraidValidationError::IndexTooSmall {
                index,
                minimal_required_index,
            })
        } else {
            Ok(Self { index, word })
        }
    }

    pub fn from_data<N, D, F, H>(
        index: Option<N>,
        word_data: D,
    ) -> Result<Self, BraidValidationError>
    where
        N: TryInto<u16>,
        IndexValidationError: From<<N as TryInto<u16>>::Error>,
        D: IntoIterator<Item = (F, Option<H>, Sign)>,
        F: TryInto<u16>,
        H: TryInto<u16>,
        StrandValidationError: From<<F as TryInto<u16>>::Error>
            + From<<H as TryInto<u16>>::Error>
            + From<std::convert::Infallible>,
    {
        let word: Word = Word::new(word_data)?;
        Self::new(index, word)
    }
    pub fn trivial<N>(index: N) -> Result<Self, BraidValidationError>
    where
        N: TryInto<u16>,
        IndexValidationError: From<<N as TryInto<u16>>::Error>,
    {
        Self::from_data(Some(index), Vec::<(u16, Option<u16>, Sign)>::new())
    }

    pub fn decompose(&self) -> Self {
        Self {
            index: self.index,
            word: self.word.decompose(),
        }
    }
    pub fn coalesce(&self) -> Self {
        Self {
            index: self.index,
            word: self.word.coalesce(),
        }
    }

    pub fn minimal_required_braid_index(&self) -> BraidIndex {
        self.word.minimal_required_braid_index()
    }
    pub fn writhe(&self) -> i32 {
        self.word.iter().fold(0, |a, b| {
            if b.sign() == Sign::Positive {
                a + 1
            } else {
                a - 1
            }
        })
    }
    pub fn letter_length(&self) -> u16 {
        // Length checks performed on underlying word: safe to unwrap
        self.word.len().try_into().unwrap()
    }
    pub fn artin_length(&self) -> u16 {
        // Length checks performed on underlying word: safe to unwrap
        self.word.iter().fold(0, |a, b| a + b.artin_length())
    }
    pub fn inverse(&self) -> Self {
        Self {
            index: self.index,
            word: self.word.inverse(),
        }
    }
    pub fn index(&self) -> BraidIndex {
        self.index
    }
    pub fn word(&self) -> Word {
        self.word.clone()
    }
    pub fn letters(&self) -> Vec<Letter> {
        self.word.letters()
    }
}

impl Default for Braid {
    fn default() -> Self {
        Self::trivial(1).unwrap()
    }
}

impl From<Word> for Braid {
    fn from(value: Word) -> Self {
        Self {
            index: value.minimal_required_braid_index(),
            word: value,
        }
    }
}
impl From<&Word> for Braid {
    fn from(value: &Word) -> Self {
        Self::from(value.clone())
    }
}

impl<L> TryFrom<Vec<L>> for Braid
where
    L: Into<Letter>,
{
    type Error = BraidValidationError;
    fn try_from(value: Vec<L>) -> Result<Self, Self::Error> {
        let word = Word::try_from(value)?;
        let index = word.minimal_required_braid_index();

        Ok(Self { index, word })
    }
}
impl<L> TryFrom<&[L]> for Braid
where
    L: Into<Letter> + std::clone::Clone,
{
    type Error = BraidValidationError;
    fn try_from(value: &[L]) -> Result<Self, Self::Error> {
        Self::try_from(value.to_vec())
    }
}

impl IntoIterator for Braid {
    type Item = (u16, Option<u16>, Sign);
    type IntoIter = <Word as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.word.into_iter()
    }
}
impl std::ops::Deref for Braid {
    type Target = [Letter];

    fn deref(&self) -> &Self::Target {
        &self.word
    }
}
impl AsRef<[Letter]> for Braid {
    fn as_ref(&self) -> &[Letter] {
        self
    }
}

impl std::ops::Mul<Letter> for Braid {
    type Output = Result<Self, BraidValidationError>;
    fn mul(self, rhs: Letter) -> Self::Output {
        if let required_index = rhs.minimal_required_braid_index()
            && self.index < required_index
        {
            Err(BraidValidationError::IndexTooSmall {
                index: self.index,
                minimal_required_index: required_index,
            })
        } else {
            Ok(Self {
                index: self.index,
                word: (self.word * rhs)?,
            })
        }
    }
}
impl std::ops::Mul<Braid> for Letter {
    type Output = Result<Braid, BraidValidationError>;

    fn mul(self, rhs: Braid) -> Self::Output {
        if let required_index = self.minimal_required_braid_index()
            && rhs.index() < required_index
        {
            Err(BraidValidationError::IndexTooSmall {
                index: rhs.index(),
                minimal_required_index: required_index,
            })
        } else {
            Ok(Braid {
                index: rhs.index,
                word: (self * rhs.word)?,
            })
        }
    }
}
impl std::ops::Mul<Word> for Braid {
    type Output = Result<Self, BraidValidationError>;
    fn mul(self, rhs: Word) -> Self::Output {
        if let required_index = rhs.minimal_required_braid_index()
            && self.index < required_index
        {
            Err(BraidValidationError::IndexTooSmall {
                index: self.index,
                minimal_required_index: required_index,
            })
        } else {
            Ok(Self {
                index: self.index,
                word: (self.word * rhs)?,
            })
        }
    }
}
impl std::ops::Mul<Braid> for Word {
    type Output = Result<Braid, BraidValidationError>;

    fn mul(self, rhs: Braid) -> Self::Output {
        if let required_index = self.minimal_required_braid_index()
            && rhs.index() < required_index
        {
            Err(BraidValidationError::IndexTooSmall {
                index: rhs.index(),
                minimal_required_index: required_index,
            })
        } else {
            Ok(Braid {
                index: rhs.index,
                word: (self * rhs.word)?,
            })
        }
    }
}
impl std::ops::Mul for Braid {
    type Output = Result<Self, BraidValidationError>;
    fn mul(self, rhs: Self) -> Self::Output {
        if self.index != rhs.index {
            Err(BraidValidationError::UnequalIndices {
                left: self.index,
                right: rhs.index,
            })
        } else {
            Ok(Self {
                index: self.index,
                word: (self.word * rhs.word)?,
            })
        }
    }
}
