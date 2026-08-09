use crate::{
    Braid, BraidValidationError, Letter, LetterValidationError, Word, WordValidationError,
};

pub struct LetterResult(Result<Letter, LetterValidationError>);
pub struct WordResult(Result<Word, WordValidationError>);
pub struct BraidResult(Result<Braid, BraidValidationError>);

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
