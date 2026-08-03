use crate::{BraidIndex, IndexValidationError, Letter, Sign, Word, WordValidationError};

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum BraidValidationError {
    #[error("Braid index {index:?} too small for word requiring minimal index {required_index:?}.")]
    IndexTooSmall {
        index: BraidIndex,
        required_index: BraidIndex,
    },
    #[error("Attempt to multiply braids of unequal indices: {left:?} != {right:?}")]
    UnequalIndices { left: BraidIndex, right: BraidIndex },
    #[error(transparent)]
    IndexValidation(#[from] IndexValidationError),
    #[error(transparent)]
    WordValidation(#[from] WordValidationError),
    #[error(transparent)]
    Infallible(#[from] std::convert::Infallible),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Braid {
    index: BraidIndex,
    word: Word,
}

impl Braid {
    pub fn new<I, W>(index: I, word: W) -> Result<Self, BraidValidationError>
    where
        I: TryInto<u16>,
        W: TryInto<Word>,
        BraidValidationError: From<<W as TryInto<Word>>::Error>,
        IndexValidationError: From<<I as TryInto<u16>>::Error>,
    {
        let index = BraidIndex::new(index)?;
        let word: Word = word.try_into()?;

        if let required_index = word.minimal_required_braid_index()
            && index < required_index
        {
            Err(BraidValidationError::IndexTooSmall {
                index,
                required_index,
            })
        } else {
            Ok(Self { index, word })
        }
    }
    pub fn trivial<I>(index: I) -> Result<Self, BraidValidationError>
    where
        I: TryInto<u16>,
        IndexValidationError: From<<I as TryInto<u16>>::Error>,
    {
        Self::new(index, Word::trivial())
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
        Ok(Self::from(Word::try_from(value)?))
    }
}
impl<L> TryFrom<&[L]> for Braid
where
    L: Into<Letter> + Clone,
{
    type Error = BraidValidationError;
    fn try_from(value: &[L]) -> Result<Self, Self::Error> {
        Ok(Self::from(Word::try_from(value)?))
    }
}

impl Default for Braid {
    fn default() -> Self {
        Self::trivial(1).unwrap()
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
                required_index,
            })
        } else {
            Self::new(self.index, (self.word * rhs)?)
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
                required_index,
            })
        } else {
            Self::new(self.index, (self.word * rhs)?)
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
            Self::new(self.index, (self.word * rhs.word)?)
        }
    }
}

impl IntoIterator for Braid {
    type Item = Letter;
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
impl std::ops::DerefMut for Braid {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.word
    }
}
