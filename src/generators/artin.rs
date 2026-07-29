use crate::{BraidIndex, Sign, Strand};
use anyhow::Context;
use std::ops::Neg;

#[derive(Debug, thiserror::Error)]
pub enum ArtinValidationError {
    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}

/// Struct representing a generator of the standard Artin braid group.
///
/// Geometrically, a positive (negative) Artin generator represents a positive (negative) crossing
/// of two adjacent strands. Thus if we think of the braid strands as stacked vertically and
/// oriented left-to-right, then a positive (negative) Artin generator corresponds to the crossing
/// of a strand under (over) the strand immediately above it.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ArtinGenerator {
    foot: Strand,
    sign: Sign,
}

impl ArtinGenerator {
    /// Constructor for ArtinGenerator
    pub fn new(foot: u16, sign: Sign) -> Result<Self, ArtinValidationError> {
        let foot = Strand::new(foot).context("Failed to construct foot strand.")?;
        Ok(Self { foot, sign })
    }

    /// Accessor for `foot` strand field
    pub fn foot(&self) -> Strand {
        self.foot
    }
    /// Accessor for `sign` field
    pub fn sign(&self) -> Sign {
        self.sign
    }

    /// Compute minimal required braid index for this generator
    pub fn minimal_required_braid_index(&self) -> BraidIndex {
        BraidIndex::new(self.foot.index() + 1).unwrap()
    }
}

impl Neg for ArtinGenerator {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(self.foot.index(), -self.sign).unwrap()
    }
}

#[macro_export]
macro_rules! artin {
    ($foot:expr; +) => {
        $crate::ArtinGenerator::new($foot, $crate::Sign::Positive)
    };
    ($foot:expr; -) => {
        $crate::ArtinGenerator::new($foot, $crate::Sign::Negative)
    };
    ($foot:expr; $power:expr) => {
        {
            let letter = if $power < 0 {
                artin![$foot; -]
            } else {
                artin![$foot; +]
            };
            let repetitions: usize = ($power as i16).abs().try_into().unwrap();
            let result: Result<
                Vec<$crate::ArtinGenerator>, $crate::generators::artin::ArtinValidationError
            > = match letter {
                Ok(generator) => Ok(vec![generator; repetitions]),
                Err(e) => Err(e),
            };
            result
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::assert_that;
    use googletest::matchers::{each, eq, is_empty, len, ok};
    use std::assert_matches;

    // --- Constructor & basic construction ----------------------------------

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
        // The constructor calls Strand::new, which returns Err when foot == 0.
        assert_matches!(result, Err(ArtinValidationError::Unexpected(_)));
    }

    // --- Negation -----------------------------------------------------------

    #[test]
    fn negating_a_positive_generator_flips_sign() {
        let orig = ArtinGenerator::new(4, Sign::Positive).unwrap();
        let negated = -orig;
        assert_that!(
            negated,
            eq(ArtinGenerator {
                foot: Strand::new(4).unwrap(),
                sign: Sign::Negative
            })
        );
    }

    #[test]
    fn double_negation_returns_original() {
        let orig = ArtinGenerator::new(9, Sign::Positive).unwrap();
        let twice_negated = -(-orig);
        assert_that!(twice_negated, eq(orig));
    }

    // --- minimal_required_braid_index --------------------------------------

    #[test]
    fn braid_index_of_positive_generator_is_foot_plus_one() {
        let generator = ArtinGenerator::new(5, Sign::Positive).unwrap();
        assert_that!(
            generator.minimal_required_braid_index(),
            eq(BraidIndex::new(6).unwrap())
        );
    }

    // --- Macro-based constructors ------------------------------------------

    #[test]
    fn macro_artin_with_positive_sign_returns_positive_artin_generator() {
        let generator = artin![7; +].expect("Failed to construct Artin generator.");
        assert_that!(
            generator,
            eq(ArtinGenerator {
                foot: Strand::new(7).unwrap(),
                sign: Sign::Positive
            })
        )
    }

    #[test]
    fn macro_artin_with_negative_sign_returns_negative_artin_generator() {
        let generator = artin![2; -].expect("Failed to construct Artin generator.");
        assert_that!(
            generator,
            eq(ArtinGenerator {
                foot: Strand::new(2).unwrap(),
                sign: Sign::Negative
            })
        )
    }

    #[test]
    fn macro_artin_with_zero_returns_empty_vector() {
        let trivial = artin![9; 0].expect("Failed to construct zero-power Artin generator.");
        assert_that!(trivial, is_empty());
    }

    #[test]
    fn macro_artin_with_power_returns_vector_of_positive_artin_generators() {
        let power_generator = artin![3; 4].expect("Failed to construct power of Artin generator.");
        assert_that!(power_generator, len(eq(4)));
        assert_that!(
            power_generator,
            each(eq(&ArtinGenerator {
                foot: Strand::new(3).unwrap(),
                sign: Sign::Positive
            }))
        );
    }

    #[test]
    fn macro_artin_with_negative_power_returns_vector_of_negative_artin_generators() {
        let power_generator = artin![7; -5].expect("Failed to construct power of Artin generator.");
        assert_that!(power_generator, len(eq(5)));
        assert_that!(
            power_generator,
            each(eq(&ArtinGenerator {
                foot: Strand::new(7).unwrap(),
                sign: Sign::Negative
            }))
        );
    }
}
