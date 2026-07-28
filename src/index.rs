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

#[cfg(test)]
mod tests {
    use super::{BraidIndex, IndexValidationError};
    use googletest::assert_that;
    use googletest::matchers::{eq, ok};
    use std::assert_matches;

    #[test]
    fn a_valid_index_can_be_constructed() {
        let index = BraidIndex::new(3);
        assert_that!(index, ok(eq(&BraidIndex(3))));
    }

    #[test]
    fn braid_index_cannot_be_zero() {
        let index = BraidIndex::new(0);
        assert_matches!(index, Err(IndexValidationError::ZeroIndex));
    }
}
