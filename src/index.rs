#[derive(Debug, thiserror::Error)]
pub enum IndexValidationError {
    #[error("Braid index cannot be zero")]
    ZeroIndex,
    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BraidIndex(u16);

impl BraidIndex {
    pub fn new(index: u16) -> Result<Self, IndexValidationError> {
        if index == 0 {
            return Err(IndexValidationError::ZeroIndex);
        }
        Ok(Self(index))
    }
}
