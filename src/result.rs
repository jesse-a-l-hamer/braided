use crate::{
    ArtinGenerator, ArtinValidationError, BandGenerator, BandValidationError, Braid, BraidIndex,
    BraidValidationError, IndexValidationError, Letter, LetterValidationError, Strand,
    StrandValidationError, Word, WordValidationError,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ArtinResult(Result<ArtinGenerator, ArtinValidationError>);
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct BandResult(Result<BandGenerator, BandValidationError>);
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BraidResult(Result<Braid, BraidValidationError>);
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct IndexResult(Result<BraidIndex, IndexValidationError>);
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct LetterResult(Result<Letter, LetterValidationError>);
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct StrandResult(Result<Strand, StrandValidationError>);
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct WordResult(Result<Word, WordValidationError>);

impl From<Result<ArtinGenerator, ArtinValidationError>> for ArtinResult {
    fn from(value: Result<ArtinGenerator, ArtinValidationError>) -> Self {
        Self(value)
    }
}
impl From<ArtinGenerator> for ArtinResult {
    fn from(value: ArtinGenerator) -> Self {
        Self(Ok(value))
    }
}
impl From<ArtinValidationError> for ArtinResult {
    fn from(value: ArtinValidationError) -> Self {
        Self(Err(value))
    }
}
impl std::ops::Deref for ArtinResult {
    type Target = Result<ArtinGenerator, ArtinValidationError>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for ArtinResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Result<BandGenerator, BandValidationError>> for BandResult {
    fn from(value: Result<BandGenerator, BandValidationError>) -> Self {
        Self(value)
    }
}
impl From<BandGenerator> for BandResult {
    fn from(value: BandGenerator) -> Self {
        Self(Ok(value))
    }
}
impl From<BandValidationError> for BandResult {
    fn from(value: BandValidationError) -> Self {
        Self(Err(value))
    }
}
impl std::ops::Deref for BandResult {
    type Target = Result<BandGenerator, BandValidationError>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for BandResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Result<Braid, BraidValidationError>> for BraidResult {
    fn from(value: Result<Braid, BraidValidationError>) -> Self {
        Self(value)
    }
}
impl From<Braid> for BraidResult {
    fn from(value: Braid) -> Self {
        Self(Ok(value))
    }
}
impl From<BraidValidationError> for BraidResult {
    fn from(value: BraidValidationError) -> Self {
        Self(Err(value))
    }
}
impl std::ops::Deref for BraidResult {
    type Target = Result<Braid, BraidValidationError>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for BraidResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Result<BraidIndex, IndexValidationError>> for IndexResult {
    fn from(value: Result<BraidIndex, IndexValidationError>) -> Self {
        Self(value)
    }
}
impl From<BraidIndex> for IndexResult {
    fn from(value: BraidIndex) -> Self {
        Self(Ok(value))
    }
}
impl From<IndexValidationError> for IndexResult {
    fn from(value: IndexValidationError) -> Self {
        Self(Err(value))
    }
}
impl std::ops::Deref for IndexResult {
    type Target = Result<BraidIndex, IndexValidationError>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for IndexResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Result<Letter, LetterValidationError>> for LetterResult {
    fn from(value: Result<Letter, LetterValidationError>) -> Self {
        Self(value)
    }
}
impl From<Letter> for LetterResult {
    fn from(value: Letter) -> Self {
        Self(Ok(value))
    }
}
impl From<LetterValidationError> for LetterResult {
    fn from(value: LetterValidationError) -> Self {
        Self(Err(value))
    }
}
impl std::ops::Deref for LetterResult {
    type Target = Result<Letter, LetterValidationError>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for LetterResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Result<Strand, StrandValidationError>> for StrandResult {
    fn from(value: Result<Strand, StrandValidationError>) -> Self {
        Self(value)
    }
}
impl From<Strand> for StrandResult {
    fn from(value: Strand) -> Self {
        Self(Ok(value))
    }
}
impl From<StrandValidationError> for StrandResult {
    fn from(value: StrandValidationError) -> Self {
        Self(Err(value))
    }
}
impl std::ops::Deref for StrandResult {
    type Target = Result<Strand, StrandValidationError>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for StrandResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Result<Word, WordValidationError>> for WordResult {
    fn from(value: Result<Word, WordValidationError>) -> Self {
        Self(value)
    }
}
impl From<Word> for WordResult {
    fn from(value: Word) -> Self {
        Self(Ok(value))
    }
}
impl From<WordValidationError> for WordResult {
    fn from(value: WordValidationError) -> Self {
        Self(Err(value))
    }
}
impl std::ops::Deref for WordResult {
    type Target = Result<Word, WordValidationError>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for WordResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
