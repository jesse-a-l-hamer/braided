use crate::{BraidIndex, Sign, Strand, StrandValidationError};

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum ArtinValidationError {
    #[error(transparent)]
    BadFoot(#[from] StrandValidationError),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ArtinGenerator {
    foot: Strand,
    sign: Sign,
}

impl ArtinGenerator {
    pub fn new(foot: u16, sign: Sign) -> Result<Self, ArtinValidationError> {
        if foot == u16::MAX {
            // The head strand is too large.
            Err(ArtinValidationError::BadFoot(
                StrandValidationError::TooLarge {
                    left: Strand::new(foot)?,
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
        BraidIndex::new(self.foot.index() + 1).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::assert_that;
    use googletest::matchers::{eq, err, ok};

    // Basic construction

    #[test]
    fn valid_positive_generator_can_be_constructed() {
        let generator = ArtinGenerator::new(3, Sign::Positive);
        assert_that!(
            generator,
            ok(eq(&ArtinGenerator {
                foot: Strand::new(3).unwrap(),
                sign: Sign::Positive
            }))
        );
    }

    #[test]
    fn valid_negative_generator_can_be_constructed() {
        let generator = ArtinGenerator::new(5, Sign::Negative);
        assert_that!(
            generator,
            ok(eq(&ArtinGenerator {
                foot: Strand::new(5).unwrap(),
                sign: Sign::Negative
            }))
        );
    }

    #[test]
    fn zero_foot_cannot_be_constructed() {
        let result = ArtinGenerator::new(0, Sign::Positive);
        assert_that!(
            result,
            err(eq(&ArtinValidationError::BadFoot(
                StrandValidationError::Zero
            )))
        );
    }

    #[test]
    fn too_large_foot_cannot_be_constructed() {
        let result = ArtinGenerator::new(u16::MAX, Sign::Positive);
        assert_that!(
            result,
            err(eq(&ArtinValidationError::BadFoot(
                StrandValidationError::TooLarge {
                    left: Strand::new(u16::MAX).unwrap(),
                    right: 1,
                }
            )))
        )
    }

    // Inversion

    #[test]
    fn inverting_a_positive_generator_flips_sign() {
        let orig = ArtinGenerator::new(4, Sign::Positive).unwrap();
        let inverse = orig.inverse();
        assert_that!(
            inverse,
            eq(ArtinGenerator {
                foot: Strand::new(4).unwrap(),
                sign: Sign::Negative
            })
        );
    }

    #[test]
    fn double_inverse_returns_original() {
        let orig = ArtinGenerator::new(9, Sign::Positive).unwrap();
        let double_inverse = orig.inverse().inverse();
        assert_that!(double_inverse, eq(orig));
    }

    // Minimal required braid index

    #[test]
    fn braid_index_of_positive_generator_is_foot_plus_one() {
        let generator = ArtinGenerator::new(5, Sign::Positive).unwrap();
        assert_that!(
            generator.minimal_required_braid_index(),
            eq(BraidIndex::new(6).unwrap())
        );
    }
}
