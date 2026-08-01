use crate::{ArtinGenerator, BraidIndex, Sign, Strand};
use anyhow::Context;

#[derive(thiserror::Error, Debug)]
pub enum BandValidationError {
    #[error("foot strand and head strand are the same ({0:?})")]
    FootOnHead(Strand),
    #[error("foot strand ({foot:?}) is over head strand ({head:?})")]
    FootOverHead { foot: Strand, head: Strand },
    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}

#[derive(Debug, PartialEq, Eq)]
pub enum StaircaseQuadrant {
    UpperLeft,
    LowerLeft,
    LowerRight,
    UpperRight,
}

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum FromArtinError {
    #[error("No Artin generators provided.")]
    NoGenerators,
    #[error("Even number of Artin generators provided.")]
    EvenGenerators,
    #[error("Could not append {next_step:?} to {previous_step:?} in {quadrant:?} staircase.")]
    IncompatibleSteps {
        quadrant: StaircaseQuadrant,
        next_step: ArtinGenerator,
        previous_step: ArtinGenerator,
    },
    #[error("Staircases are not balanced: difference of {0} steps found.")]
    ImbalancedStaircases(usize),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct BandGenerator {
    foot: Strand,
    head: Strand,
    sign: Sign,
}

impl BandGenerator {
    pub fn new(foot: u16, head: u16, sign: Sign) -> Result<Self, BandValidationError> {
        let foot = Strand::new(foot).context("Failed to construct foot strand.")?;
        let head = Strand::new(head).context("Failed to construct head strand.")?;
        if foot == head {
            return Err(BandValidationError::FootOnHead(foot));
        }
        if foot > head {
            return Err(BandValidationError::FootOverHead { foot, head });
        }
        Ok(Self { foot, head, sign })
    }
    pub fn from_artin(band_parts: &[ArtinGenerator]) -> Result<Self, FromArtinError> {
        let num_parts = band_parts.len();

        if num_parts == 0 {
            return Err(FromArtinError::NoGenerators);
        } else if num_parts == 1 {
            let generator = band_parts.last().unwrap();
            return Ok(BandGenerator {
                foot: generator.foot(),
                head: generator.foot() + 1,
                sign: generator.sign(),
            });
        } else if num_parts.is_multiple_of(2) {
            return Err(FromArtinError::EvenGenerators);
        }

        let mut upper_left_staircase = Vec::new();
        let mut lower_left_staircase = Vec::new();
        let mut upper_right_staircase = Vec::new();
        let mut lower_right_staircase = Vec::new();

        let (left_parts, right_parts) = band_parts.split_at(num_parts.div_euclid(2));
        let crossing = right_parts.first().unwrap();
        let right_parts = &right_parts[1..];

        for (left_part, right_part) in left_parts.iter().rev().zip(right_parts.iter()) {
            // Add new parts to staircases, and check for "contiguity" and "mirroring"
            match left_part.sign() {
                Sign::Positive => {
                    let previous_step = upper_left_staircase.last().unwrap_or(crossing);
                    if left_part.foot() == previous_step.foot() + 1 {
                        upper_left_staircase.push(*left_part);
                    } else {
                        return Err(FromArtinError::IncompatibleSteps {
                            quadrant: StaircaseQuadrant::UpperLeft,
                            next_step: *left_part,
                            previous_step: *previous_step,
                        });
                    }
                }
                Sign::Negative => {
                    let previous_step = lower_left_staircase.last().unwrap_or(crossing);
                    if left_part.foot() + 1 == previous_step.foot() {
                        lower_left_staircase.push(*left_part);
                    } else {
                        return Err(FromArtinError::IncompatibleSteps {
                            quadrant: StaircaseQuadrant::LowerLeft,
                            next_step: *left_part,
                            previous_step: *previous_step,
                        });
                    }
                }
            };
            match right_part.sign() {
                Sign::Positive => {
                    let previous_step = lower_right_staircase.last().unwrap_or(crossing);
                    if right_part.foot() + 1 == previous_step.foot() {
                        lower_right_staircase.push(*right_part);
                    } else {
                        return Err(FromArtinError::IncompatibleSteps {
                            quadrant: StaircaseQuadrant::LowerRight,
                            next_step: *right_part,
                            previous_step: *previous_step,
                        });
                    }
                }
                Sign::Negative => {
                    let previous_step = upper_right_staircase.last().unwrap_or(crossing);
                    if right_part.foot() == previous_step.foot() + 1 {
                        upper_right_staircase.push(*right_part);
                    } else {
                        return Err(FromArtinError::IncompatibleSteps {
                            quadrant: StaircaseQuadrant::UpperRight,
                            next_step: *right_part,
                            previous_step: *previous_step,
                        });
                    }
                }
            };
        }

        // If one set of staircases is imbalanced, then both are.
        if let difference = lower_left_staircase
            .len()
            .abs_diff(lower_right_staircase.len())
            && difference > 0
        {
            return Err(FromArtinError::ImbalancedStaircases(difference));
        }

        let foot = lower_left_staircase.last().unwrap_or(crossing).foot();
        let head = upper_left_staircase.last().unwrap_or(crossing).foot() + 1;
        let sign = crossing.sign();

        Ok(Self { foot, head, sign })
    }

    pub fn foot(&self) -> Strand {
        self.foot
    }
    pub fn head(&self) -> Strand {
        self.head
    }
    pub fn sign(&self) -> Sign {
        self.sign
    }

    pub fn inverse(&self) -> Self {
        Self {
            foot: self.foot,
            head: self.head,
            sign: -self.sign,
        }
    }

    pub fn height(&self) -> u16 {
        self.head.index() - self.foot.index()
    }
    pub fn is_artin(&self) -> bool {
        self.height() == 1
    }
    pub fn minimal_required_braid_index(&self) -> BraidIndex {
        BraidIndex::new(self.head.index()).unwrap()
    }
    pub fn artin_length(&self) -> u16 {
        1 + (self.height() - 1) * 2
    }
}

#[cfg(test)]
mod tests {
    use super::{FromArtinError, StaircaseQuadrant};
    use crate::generators::BandValidationError;
    use crate::{BandGenerator, BraidIndex, Sign, Strand, artin};
    use googletest::assert_that;
    use googletest::matchers::{eq, err, is_false, is_true, ok};
    use std::assert_matches;

    // Basic Construction

    #[test]
    fn valid_band_generator_is_constructed() {
        let positive_band = BandGenerator::new(1, 4, Sign::Positive);
        let negative_band = BandGenerator::new(3, 7, Sign::Negative);

        assert_that!(
            positive_band,
            ok(eq(&BandGenerator {
                foot: Strand::new(1).unwrap(),
                head: Strand::new(4).unwrap(),
                sign: Sign::Positive,
            }))
        );

        assert_that!(
            negative_band,
            ok(eq(&BandGenerator {
                foot: Strand::new(3).unwrap(),
                head: Strand::new(7).unwrap(),
                sign: Sign::Negative,
            }))
        );
    }

    #[test]
    fn validation_error_when_head_equals_foot() {
        let band = BandGenerator::new(3, 3, Sign::Positive);

        assert_matches!(band, Err(BandValidationError::FootOnHead(_)))
    }

    #[test]
    fn validation_error_when_foot_over_head() {
        let band = BandGenerator::new(5, 3, Sign::Positive);

        assert_matches!(band, Err(BandValidationError::FootOverHead { .. }))
    }

    // Construction from Artin generators

    #[test]
    fn valid_collection_of_artin_generators_successfully_construct_band_generator() {
        let good_bands = [
            (
                artin![3; -1].unwrap(),
                BandGenerator::new(3, 4, Sign::Negative).unwrap(),
            ),
            (
                [
                    artin![1; -1].unwrap(),
                    artin![2; -1].unwrap(),
                    artin![3; -1].unwrap(),
                    artin![4; -1].unwrap(),
                    artin![3; 1].unwrap(),
                    artin![2; 1].unwrap(),
                    artin![1; 1].unwrap(),
                ]
                .concat(),
                BandGenerator::new(1, 5, Sign::Negative).unwrap(),
            ),
            (
                [
                    artin![4; 1].unwrap(),
                    artin![3; 1].unwrap(),
                    artin![2; 1].unwrap(),
                    artin![1; -1].unwrap(),
                    artin![2; -1].unwrap(),
                    artin![3; -1].unwrap(),
                    artin![4; -1].unwrap(),
                ]
                .concat(),
                BandGenerator::new(1, 5, Sign::Negative).unwrap(),
            ),
            (
                [
                    artin![1; -1].unwrap(),
                    artin![4; 1].unwrap(),
                    artin![2; -1].unwrap(),
                    artin![3; -1].unwrap(),
                    artin![2; 1].unwrap(),
                    artin![1; 1].unwrap(),
                    artin![4; -1].unwrap(),
                ]
                .concat(),
                BandGenerator::new(1, 5, Sign::Negative).unwrap(),
            ),
        ];

        for (artin_list, expected) in good_bands {
            let band = BandGenerator::from_artin(&artin_list);
            assert_that!(band, ok(eq(&expected)))
        }
    }

    #[test]
    fn invalid_collections_of_artin_generators_fail_to_construct_band_generator() {
        let bad_artin_words = [
            (vec![], FromArtinError::NoGenerators),
            (
                [
                    artin![1; -1].unwrap(),
                    artin![4; 1].unwrap(),
                    artin![2; -1].unwrap(),
                    artin![2; 1].unwrap(),
                    artin![1; -1].unwrap(),
                    artin![4; -1].unwrap(),
                ]
                .concat(),
                FromArtinError::EvenGenerators,
            ),
            (
                [
                    artin![1; -1].unwrap(),
                    artin![4; 1].unwrap(),
                    artin![2; -1].unwrap(),
                    artin![3; -1].unwrap(),
                    artin![2; 1].unwrap(),
                    artin![1; 1].unwrap(),
                    artin![3; -1].unwrap(),
                ]
                .concat(),
                FromArtinError::IncompatibleSteps {
                    quadrant: StaircaseQuadrant::UpperRight,
                    next_step: *artin![3; -1].unwrap().last().unwrap(),
                    previous_step: *artin![3; -1].unwrap().last().unwrap(),
                },
            ),
            (
                [
                    artin![1; -1].unwrap(),
                    artin![2; -1].unwrap(),
                    artin![4; 1].unwrap(),
                    artin![3; -1].unwrap(),
                    artin![2; 1].unwrap(),
                    artin![4; -1].unwrap(),
                    artin![5; -1].unwrap(),
                ]
                .concat(),
                FromArtinError::ImbalancedStaircases(1),
            ),
        ];

        for (bad_artin_word, error) in bad_artin_words {
            let bad_band = BandGenerator::from_artin(&bad_artin_word);
            assert_that!(bad_band, err(eq(&error)))
        }
    }

    // Computable properties

    #[test]
    fn height_is_computable() {
        let band = BandGenerator::new(1, 5, Sign::Positive).unwrap();
        assert_that!(band.height(), eq(4));
    }

    #[test]
    fn is_artin_detects_artin_generators() {
        let artin_generator = BandGenerator::new(2, 3, Sign::Negative).unwrap();
        let not_artin_generator = BandGenerator::new(2, 4, Sign::Negative).unwrap();

        assert_that!(artin_generator.is_artin(), is_true());
        assert_that!(not_artin_generator.is_artin(), is_false());
    }

    #[test]
    fn minimal_required_braid_index_is_head() {
        let band = BandGenerator::new(2, 17, Sign::Positive).unwrap();
        assert_that!(
            band.minimal_required_braid_index(),
            eq(BraidIndex::new(17).unwrap())
        );
    }

    #[test]
    fn artin_length_is_accurate() {
        let artin_word = [
            artin![1; -1].unwrap(),
            artin![4; 1].unwrap(),
            artin![2; -1].unwrap(),
            artin![3; -1].unwrap(),
            artin![2; 1].unwrap(),
            artin![1; 1].unwrap(),
            artin![4; -1].unwrap(),
        ]
        .concat();
        let band = BandGenerator::from_artin(&artin_word).unwrap();

        assert_that!(band.artin_length(), eq(artin_word.len() as u16));
    }

    // Negation

    #[test]
    fn band_can_be_inverted() {
        let band = BandGenerator::new(9, 16, Sign::Positive).unwrap();
        let inverse_band = band.inverse();

        assert_that!(
            inverse_band,
            eq(BandGenerator::new(9, 16, Sign::Negative).unwrap())
        );

        let double_inverse_band = inverse_band.inverse();
        assert_that!(double_inverse_band, eq(band));
    }
}
