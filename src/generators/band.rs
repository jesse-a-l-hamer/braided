use crate::{ArtinGenerator, BraidIndex, Letter, Sign, Strand, StrandValidationError};

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum BandValidationError {
    #[error("foot strand and head strand are the same ({0:?})")]
    FootOnHead(Strand),
    #[error("foot strand ({foot:?}) is over head strand ({head:?})")]
    FootOverHead { foot: Strand, head: Strand },
    #[error(transparent)]
    StrandValidation(#[from] StrandValidationError),
    #[error(transparent)]
    FromArtin(#[from] FromArtinError),
}

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum FromArtinError {
    #[error("No Artin generators provided.")]
    NoGenerators,
    #[error("Even number of Artin generators provided.")]
    EvenGenerators,
    #[error("Could not append {next_step:?} to {previous_step:?} in {quadrant:?} staircase.")]
    IncontiguousSteps {
        quadrant: StaircaseQuadrant,
        next_step: ArtinGenerator,
        previous_step: ArtinGenerator,
    },
    #[error("Staircases are not balanced: difference of {0} steps found.")]
    ImbalancedStaircases(usize),
}

#[derive(Debug, PartialEq, Eq)]
pub enum StaircaseQuadrant {
    UpperLeft,
    LowerLeft,
    LowerRight,
    UpperRight,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct BandGenerator {
    foot: Strand,
    head: Strand,
    sign: Sign,
}

impl BandGenerator {
    pub fn new<F, H>(foot: F, head: H, sign: Sign) -> Result<Self, BandValidationError>
    where
        F: TryInto<u16>,
        H: TryInto<u16>,
        StrandValidationError: From<<F as TryInto<u16>>::Error>
            + From<<H as TryInto<u16>>::Error>
            + From<std::convert::Infallible>,
    {
        let foot = Strand::new(foot)?;
        let head = Strand::new(head)?;
        match foot.cmp(&head) {
            std::cmp::Ordering::Less => Ok(Self { foot, head, sign }),
            std::cmp::Ordering::Equal => Err(BandValidationError::FootOnHead(foot)),
            std::cmp::Ordering::Greater => Err(BandValidationError::FootOverHead { foot, head }),
        }
    }
    pub fn coalesce(band_parts: &[ArtinGenerator]) -> Result<Self, BandValidationError> {
        let num_parts = band_parts.len();

        if num_parts == 0 {
            return Err(BandValidationError::from(FromArtinError::NoGenerators));
        } else if num_parts == 1 {
            let generator = band_parts.last().unwrap();
            return Ok(BandGenerator {
                foot: generator.foot(),
                head: (generator.foot() + 1).unwrap(),
                sign: generator.sign(),
            });
        } else if num_parts.is_multiple_of(2) {
            return Err(BandValidationError::from(FromArtinError::EvenGenerators));
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
                    if left_part.foot() == (previous_step.foot() + 1).unwrap() {
                        upper_left_staircase.push(*left_part);
                    } else {
                        return Err(BandValidationError::from(
                            FromArtinError::IncontiguousSteps {
                                quadrant: StaircaseQuadrant::UpperLeft,
                                next_step: *left_part,
                                previous_step: *previous_step,
                            },
                        ));
                    }
                }
                Sign::Negative => {
                    let previous_step = lower_left_staircase.last().unwrap_or(crossing);
                    if (left_part.foot() + 1).unwrap() == previous_step.foot() {
                        lower_left_staircase.push(*left_part);
                    } else {
                        return Err(BandValidationError::from(
                            FromArtinError::IncontiguousSteps {
                                quadrant: StaircaseQuadrant::LowerLeft,
                                next_step: *left_part,
                                previous_step: *previous_step,
                            },
                        ));
                    }
                }
            };
            match right_part.sign() {
                Sign::Positive => {
                    let previous_step = lower_right_staircase.last().unwrap_or(crossing);
                    if (right_part.foot() + 1).unwrap() == previous_step.foot() {
                        lower_right_staircase.push(*right_part);
                    } else {
                        return Err(BandValidationError::from(
                            FromArtinError::IncontiguousSteps {
                                quadrant: StaircaseQuadrant::LowerRight,
                                next_step: *right_part,
                                previous_step: *previous_step,
                            },
                        ));
                    }
                }
                Sign::Negative => {
                    let previous_step = upper_right_staircase.last().unwrap_or(crossing);
                    if right_part.foot() == (previous_step.foot() + 1).unwrap() {
                        upper_right_staircase.push(*right_part);
                    } else {
                        return Err(BandValidationError::from(
                            FromArtinError::IncontiguousSteps {
                                quadrant: StaircaseQuadrant::UpperRight,
                                next_step: *right_part,
                                previous_step: *previous_step,
                            },
                        ));
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
            return Err(BandValidationError::from(
                FromArtinError::ImbalancedStaircases(difference),
            ));
        }

        let foot = lower_left_staircase.last().unwrap_or(crossing).foot();
        let head = (upper_left_staircase.last().unwrap_or(crossing).foot() + 1).unwrap();
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
        (self.head - self.foot).unwrap().into()
    }
    pub fn is_artin(&self) -> bool {
        self.height() == 1
    }
    pub fn minimal_required_braid_index(&self) -> BraidIndex {
        BraidIndex::new(self.head).unwrap()
    }
    pub fn artin_length(&self) -> u16 {
        2 * self.height() - 1
    }

    pub fn decompose(&self) -> Vec<ArtinGenerator> {
        // Band decomposition is infallible, so it's safe to unwrap any intermediate results
        let crossing = ArtinGenerator::new((self.head() - 1).unwrap(), self.sign()).unwrap();
        let mut left = Vec::new();
        let min_foot: u16 = self.foot.into();
        let max_head: u16 = (self.head - 1).unwrap().into();
        for foot_idx in min_foot..max_head {
            left.push(ArtinGenerator::new(foot_idx, Sign::Negative).unwrap());
        }
        let right = left.iter().rev().map(|a| a.inverse()).collect();
        [left, vec![crossing], right].concat()
    }
}

impl From<ArtinGenerator> for BandGenerator {
    fn from(value: ArtinGenerator) -> Self {
        Self {
            foot: value.foot(),
            head: (value.foot() + 1).unwrap(),
            sign: value.sign(),
        }
    }
}
impl From<Letter> for BandGenerator {
    fn from(value: Letter) -> Self {
        match value {
            Letter::Artin(artin) => Self::from(artin),
            Letter::Band(band) => band,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{FromArtinError, StaircaseQuadrant};
    use crate::{
        ArtinGenerator, BandGenerator, BandValidationError, BraidIndex, Letter, Sign, Strand,
    };
    use googletest::matchers::{anything, each, eq, err, ok};
    use googletest::{assert_that, expect_that, gtest};

    #[test]
    fn valid_inputs_to_new_yield_successful_construction() {
        let valid_bands = [
            BandGenerator::new(3, 4, Sign::Positive),
            BandGenerator::new(1, u16::MAX, Sign::Negative),
            BandGenerator::new(2_usize, 5_isize, Sign::Positive),
            BandGenerator::new(Strand::new(9).unwrap(), 40_u32, Sign::Negative),
            BandGenerator::new(-(-3), Strand::new(10).unwrap(), Sign::Positive),
        ];

        assert_that!(valid_bands, each(ok(anything())));
    }

    #[test]
    fn valid_inputs_to_from_yield_expected_construction() {
        let expected_band = BandGenerator::new(1, 2, Sign::Positive).unwrap();
        let test_bands = [
            BandGenerator::from(ArtinGenerator::new(1, Sign::Positive).unwrap()),
            BandGenerator::from(Letter::new(1, None::<u16>, Sign::Positive).unwrap()),
            BandGenerator::from(Letter::new(1, Some(2), Sign::Positive).unwrap()),
        ];

        assert_that!(test_bands, each(eq(expected_band)));
    }

    #[gtest]
    fn valid_inputs_to_coalesce_yield_successful_construction() {
        expect_that!(
            BandGenerator::coalesce(&[ArtinGenerator::new(1, Sign::Negative).unwrap()]),
            eq(&BandGenerator::new(1, 2, Sign::Negative))
        );

        let test_band = BandGenerator::new(1, 4, Sign::Positive);
        let valid_bands = [
            BandGenerator::coalesce(&[
                ArtinGenerator::new(1, Sign::Negative).unwrap(),
                ArtinGenerator::new(2, Sign::Negative).unwrap(),
                ArtinGenerator::new(3, Sign::Positive).unwrap(),
                ArtinGenerator::new(2, Sign::Positive).unwrap(),
                ArtinGenerator::new(1, Sign::Positive).unwrap(),
            ]),
            BandGenerator::coalesce(&[
                ArtinGenerator::new(3, Sign::Positive).unwrap(),
                ArtinGenerator::new(2, Sign::Positive).unwrap(),
                ArtinGenerator::new(1, Sign::Positive).unwrap(),
                ArtinGenerator::new(2, Sign::Negative).unwrap(),
                ArtinGenerator::new(3, Sign::Negative).unwrap(),
            ]),
            BandGenerator::coalesce(&[
                ArtinGenerator::new(3, Sign::Positive).unwrap(),
                ArtinGenerator::new(1, Sign::Negative).unwrap(),
                ArtinGenerator::new(2, Sign::Positive).unwrap(),
                ArtinGenerator::new(1, Sign::Positive).unwrap(),
                ArtinGenerator::new(3, Sign::Negative).unwrap(),
            ]),
            BandGenerator::coalesce(&[
                ArtinGenerator::new(1, Sign::Negative).unwrap(),
                ArtinGenerator::new(3, Sign::Positive).unwrap(),
                ArtinGenerator::new(2, Sign::Positive).unwrap(),
                ArtinGenerator::new(1, Sign::Positive).unwrap(),
                ArtinGenerator::new(3, Sign::Negative).unwrap(),
            ]),
        ];

        expect_that!(valid_bands, each(eq(&test_band)));
    }

    #[gtest]
    fn properties_compute_as_expected() {
        let input_data = [(1, 2, Sign::Positive), (2, 5, Sign::Negative)];

        for (foot, head, sign) in input_data {
            let band = BandGenerator::new(foot, head, sign).unwrap();

            expect_that!(band.foot(), eq(Strand::new(foot).unwrap()));
            expect_that!(band.head(), eq(Strand::new(head).unwrap()));
            expect_that!(band.sign(), eq(sign));
            expect_that!(
                band.inverse(),
                eq(BandGenerator::new(foot, head, -sign).unwrap())
            );
            expect_that!(band.height(), eq(head - foot));
            expect_that!(band.is_artin(), eq(head - foot == 1));
            expect_that!(
                band.minimal_required_braid_index(),
                eq(BraidIndex::new(head).unwrap())
            );
            expect_that!(band.artin_length(), eq(2 * (head - foot) - 1));
        }
    }

    #[gtest]
    fn decomposition_works_as_expected() {
        let expected_results = [
            (
                BandGenerator::new(1, 2, Sign::Positive),
                vec![ArtinGenerator::new(1, Sign::Positive).unwrap()],
            ),
            (
                BandGenerator::new(1, 4, Sign::Positive),
                vec![
                    ArtinGenerator::new(1, Sign::Negative).unwrap(),
                    ArtinGenerator::new(2, Sign::Negative).unwrap(),
                    ArtinGenerator::new(3, Sign::Positive).unwrap(),
                    ArtinGenerator::new(2, Sign::Positive).unwrap(),
                    ArtinGenerator::new(1, Sign::Positive).unwrap(),
                ],
            ),
            (
                BandGenerator::coalesce(&[
                    ArtinGenerator::new(1, Sign::Negative).unwrap(),
                    ArtinGenerator::new(2, Sign::Negative).unwrap(),
                    ArtinGenerator::new(3, Sign::Positive).unwrap(),
                    ArtinGenerator::new(2, Sign::Positive).unwrap(),
                    ArtinGenerator::new(1, Sign::Positive).unwrap(),
                ]),
                vec![
                    ArtinGenerator::new(1, Sign::Negative).unwrap(),
                    ArtinGenerator::new(2, Sign::Negative).unwrap(),
                    ArtinGenerator::new(3, Sign::Positive).unwrap(),
                    ArtinGenerator::new(2, Sign::Positive).unwrap(),
                    ArtinGenerator::new(1, Sign::Positive).unwrap(),
                ],
            ),
            (
                BandGenerator::coalesce(&[
                    ArtinGenerator::new(1, Sign::Negative).unwrap(),
                    ArtinGenerator::new(3, Sign::Positive).unwrap(),
                    ArtinGenerator::new(2, Sign::Positive).unwrap(),
                    ArtinGenerator::new(1, Sign::Positive).unwrap(),
                    ArtinGenerator::new(3, Sign::Negative).unwrap(),
                ]),
                vec![
                    ArtinGenerator::new(1, Sign::Negative).unwrap(),
                    ArtinGenerator::new(2, Sign::Negative).unwrap(),
                    ArtinGenerator::new(3, Sign::Positive).unwrap(),
                    ArtinGenerator::new(2, Sign::Positive).unwrap(),
                    ArtinGenerator::new(1, Sign::Positive).unwrap(),
                ],
            ),
        ];

        for (band, expected_decomposition) in expected_results {
            expect_that!(band.unwrap().decompose(), eq(&expected_decomposition));
        }
    }

    #[gtest]
    fn invalid_inputs_to_new_yield_failure() {
        let invalid_bands = [
            (
                BandGenerator::new(1, 1, Sign::Positive),
                BandValidationError::FootOnHead(Strand::new(1).unwrap()),
            ),
            (
                BandGenerator::new(4, 1, Sign::Negative),
                BandValidationError::FootOverHead {
                    foot: Strand::new(4).unwrap(),
                    head: Strand::new(1).unwrap(),
                },
            ),
            (
                BandGenerator::new(0, 4, Sign::Negative),
                BandValidationError::from(Strand::new(0).err().unwrap()),
            ),
            (
                BandGenerator::new(-1, 4, Sign::Positive),
                BandValidationError::from(Strand::new(-1).err().unwrap()),
            ),
            (
                BandGenerator::new(1, u16::MAX as u32 + 1, Sign::Negative),
                BandValidationError::from(Strand::new(u16::MAX as u32 + 1).err().unwrap()),
            ),
        ];

        for (invalid_band, error) in invalid_bands {
            expect_that!(invalid_band, err(eq(&error)));
        }
    }

    #[gtest]
    fn invalid_inputs_to_coalesce_yield_failure() {
        let invalid_artin_lists: [(Vec<ArtinGenerator>, BandValidationError); 7] = [
            (vec![], FromArtinError::NoGenerators.into()),
            (
                vec![
                    ArtinGenerator::new(1, Sign::Positive).unwrap(),
                    ArtinGenerator::new(2, Sign::Negative).unwrap(),
                ],
                FromArtinError::EvenGenerators.into(),
            ),
            (
                vec![
                    ArtinGenerator::new(2, Sign::Negative).unwrap(),
                    ArtinGenerator::new(1, Sign::Negative).unwrap(),
                    ArtinGenerator::new(3, Sign::Positive).unwrap(),
                    ArtinGenerator::new(2, Sign::Positive).unwrap(),
                    ArtinGenerator::new(1, Sign::Positive).unwrap(),
                ],
                FromArtinError::IncontiguousSteps {
                    quadrant: StaircaseQuadrant::LowerLeft,
                    next_step: ArtinGenerator::new(1, Sign::Negative).unwrap(),
                    previous_step: ArtinGenerator::new(3, Sign::Positive).unwrap(),
                }
                .into(),
            ),
            (
                vec![
                    ArtinGenerator::new(2, Sign::Positive).unwrap(),
                    ArtinGenerator::new(3, Sign::Positive).unwrap(),
                    ArtinGenerator::new(1, Sign::Positive).unwrap(),
                    ArtinGenerator::new(2, Sign::Negative).unwrap(),
                    ArtinGenerator::new(3, Sign::Negative).unwrap(),
                ],
                FromArtinError::IncontiguousSteps {
                    quadrant: StaircaseQuadrant::UpperLeft,
                    next_step: ArtinGenerator::new(3, Sign::Positive).unwrap(),
                    previous_step: ArtinGenerator::new(1, Sign::Positive).unwrap(),
                }
                .into(),
            ),
            (
                vec![
                    ArtinGenerator::new(1, Sign::Negative).unwrap(),
                    ArtinGenerator::new(2, Sign::Negative).unwrap(),
                    ArtinGenerator::new(3, Sign::Positive).unwrap(),
                    ArtinGenerator::new(1, Sign::Positive).unwrap(),
                    ArtinGenerator::new(2, Sign::Positive).unwrap(),
                ],
                FromArtinError::IncontiguousSteps {
                    quadrant: StaircaseQuadrant::LowerRight,
                    next_step: ArtinGenerator::new(1, Sign::Positive).unwrap(),
                    previous_step: ArtinGenerator::new(3, Sign::Positive).unwrap(),
                }
                .into(),
            ),
            (
                vec![
                    ArtinGenerator::new(3, Sign::Positive).unwrap(),
                    ArtinGenerator::new(2, Sign::Positive).unwrap(),
                    ArtinGenerator::new(1, Sign::Positive).unwrap(),
                    ArtinGenerator::new(3, Sign::Negative).unwrap(),
                    ArtinGenerator::new(2, Sign::Negative).unwrap(),
                ],
                FromArtinError::IncontiguousSteps {
                    quadrant: StaircaseQuadrant::UpperRight,
                    next_step: ArtinGenerator::new(3, Sign::Negative).unwrap(),
                    previous_step: ArtinGenerator::new(1, Sign::Positive).unwrap(),
                }
                .into(),
            ),
            (
                vec![
                    ArtinGenerator::new(4, Sign::Positive).unwrap(),
                    ArtinGenerator::new(2, Sign::Negative).unwrap(),
                    ArtinGenerator::new(3, Sign::Positive).unwrap(),
                    ArtinGenerator::new(2, Sign::Positive).unwrap(),
                    ArtinGenerator::new(1, Sign::Positive).unwrap(),
                ],
                FromArtinError::ImbalancedStaircases(1).into(),
            ),
        ];

        for (invalid_artin_list, error) in invalid_artin_lists {
            expect_that!(
                BandGenerator::coalesce(&invalid_artin_list),
                err(eq(&error))
            );
        }
    }
}
