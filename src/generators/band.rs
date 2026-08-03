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
    IncompatibleSteps {
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
                            FromArtinError::IncompatibleSteps {
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
                            FromArtinError::IncompatibleSteps {
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
                            FromArtinError::IncompatibleSteps {
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
                            FromArtinError::IncompatibleSteps {
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
        1 + (self.height() - 1) * 2
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
