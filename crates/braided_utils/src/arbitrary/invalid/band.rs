use crate::arbitrary::invalid::strand::{
    InvalidStrandTryNewData, arbitrary_invalid_strand_try_new,
};
use crate::arbitrary::invalid::u16;
use crate::arbitrary::valid::arbitrary_artin_generator;
use crate::arbitrary::valid::coalescence::arbitrary_single_coalescence;
use braided::generators::band::{FromArtinError, MAX_BAND_HEIGHT, StaircaseQuadrant};
use braided::{ArtinGenerator, BandValidationError, Sign, Strand};
use proptest::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum InvalidBandTryNewData {
    FootOnHead(u16, Sign),
    FootOverHead {
        foot: u16,
        head: u16,
        sign: Sign,
    },
    TooTall {
        foot: u16,
        head: u16,
        sign: Sign,
    },
    InvalidFoot {
        foot: InvalidStrandTryNewData,
        head: u16,
        sign: Sign,
    },
    InvalidHead {
        foot: u16,
        head: InvalidStrandTryNewData,
        sign: Sign,
    },
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum InvalidBandCoalescenceData {
    NoGenerators(Vec<ArtinGenerator>),
    EvenGenerators(Vec<ArtinGenerator>),
    TooManyGenerators(Vec<ArtinGenerator>),
    IncontiguousSteps(Vec<ArtinGenerator>),
    ImbalancedStaircases(Vec<ArtinGenerator>),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct InvalidBandTryNew {
    pub data: InvalidBandTryNewData,
    pub error: BandValidationError,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct InvalidBandCoalescence {
    pub data: InvalidBandCoalescenceData,
    pub error: BandValidationError,
}

fn arbitrary_invalid_band_foot_on_head() -> impl Strategy<Value = InvalidBandTryNew> {
    (
        1..=u16::MAX,
        Just(Sign::Negative).prop_union(Just(Sign::Positive)),
    )
        .prop_map(|(strand_idx, sign)| InvalidBandTryNew {
            data: InvalidBandTryNewData::FootOnHead(strand_idx, sign),
            error: BandValidationError::FootOnHead(Strand::try_new(strand_idx).unwrap()),
        })
}
fn arbitrary_invalid_band_head_over_foot() -> impl Strategy<Value = InvalidBandTryNew> {
    (
        (2..=u16::MAX).prop_flat_map(|foot_idx| (Just(foot_idx), 1..foot_idx)),
        Just(Sign::Negative).prop_union(Just(Sign::Positive)),
    )
        .prop_map(|((foot_idx, head_idx), sign)| InvalidBandTryNew {
            data: InvalidBandTryNewData::FootOverHead {
                foot: foot_idx,
                head: head_idx,
                sign,
            },
            error: BandValidationError::FootOverHead {
                foot: Strand::try_new(foot_idx).unwrap(),
                head: Strand::try_new(head_idx).unwrap(),
            },
        })
}
fn arbitrary_invalid_band_too_tall() -> impl Strategy<Value = InvalidBandTryNew> {
    (
        ((MAX_BAND_HEIGHT + 1)..(u16::MAX - 1))
            .prop_flat_map(|height| (Just(height), (1..(u16::MAX - height)))),
        Just(Sign::Negative).prop_union(Just(Sign::Positive)),
    )
        .prop_map(|((height, foot_idx), sign)| InvalidBandTryNew {
            data: InvalidBandTryNewData::TooTall {
                foot: foot_idx,
                head: foot_idx + height,
                sign,
            },
            error: BandValidationError::TooTall(height),
        })
}
fn arbitrary_invalid_band_invalid_foot() -> impl Strategy<Value = InvalidBandTryNew> {
    (
        arbitrary_invalid_strand_try_new(),
        1..u16::MAX,
        Just(Sign::Negative).prop_union(Just(Sign::Positive)),
    )
        .prop_map(|(invalid_foot, head_idx, sign)| InvalidBandTryNew {
            data: InvalidBandTryNewData::InvalidFoot {
                foot: invalid_foot.data,
                head: head_idx,
                sign,
            },
            error: BandValidationError::StrandValidation(invalid_foot.error),
        })
}
fn arbitrary_invalid_band_invalid_head() -> impl Strategy<Value = InvalidBandTryNew> {
    (
        1..u16::MAX,
        arbitrary_invalid_strand_try_new(),
        Just(Sign::Negative).prop_union(Just(Sign::Positive)),
    )
        .prop_map(|(foot_idx, invalid_head, sign)| InvalidBandTryNew {
            data: InvalidBandTryNewData::InvalidHead {
                foot: foot_idx,
                head: invalid_head.data,
                sign,
            },
            error: BandValidationError::StrandValidation(invalid_head.error),
        })
}

pub fn arbitrary_invalid_band_try_new() -> impl Strategy<Value = InvalidBandTryNew> {
    prop_oneof![
        arbitrary_invalid_band_foot_on_head(),
        arbitrary_invalid_band_head_over_foot(),
        arbitrary_invalid_band_too_tall(),
        arbitrary_invalid_band_invalid_head(),
        arbitrary_invalid_band_invalid_foot(),
    ]
}

fn arbitrary_invalid_band_coalescence_no_generators()
-> impl Strategy<Value = InvalidBandCoalescence> {
    Just(InvalidBandCoalescence {
        data: InvalidBandCoalescenceData::NoGenerators(Vec::new()),
        error: BandValidationError::FromArtin(FromArtinError::NoGenerators),
    })
}
fn arbitrary_invalid_band_coalescence_even_generators()
-> impl Strategy<Value = InvalidBandCoalescence> {
    (1..u16::MAX.div_ceil(2))
        .prop_flat_map(|half_length| {
            let mut strategies = Vec::new();
            for _ in 0..(2 * half_length) {
                strategies.push(arbitrary_artin_generator(None, None));
            }
            strategies
        })
        .prop_map(|artin_generators| InvalidBandCoalescence {
            data: InvalidBandCoalescenceData::EvenGenerators(artin_generators),
            error: BandValidationError::FromArtin(FromArtinError::EvenGenerators),
        })
}
fn arbitrary_invalid_band_coalescence_too_many_generators()
-> impl Strategy<Value = InvalidBandCoalescence> {
    ((u16::MAX as usize + 1)..(2 * (u16::MAX as usize)))
        .prop_flat_map(|num_generators| {
            let mut strategies = Vec::new();
            for _ in 0..num_generators {
                strategies.push(arbitrary_artin_generator(None, None));
            }
            (Just(num_generators), strategies)
        })
        .prop_map(
            |(num_generators, artin_generators)| InvalidBandCoalescence {
                data: InvalidBandCoalescenceData::TooManyGenerators(artin_generators),
                error: BandValidationError::FromArtin(FromArtinError::TooManyGenerators(
                    num_generators,
                )),
            },
        )
}
fn arbitrary_invalid_band_coalescence_incontiguous_steps()
-> impl Strategy<Value = InvalidBandCoalescence> {
    arbitrary_single_coalescence(None, None, None).prop_perturb(
        |(band, decomposed_band), mut rng| {
            let height = band.unwrap().height();
            let artin_length = band.unwrap().artin_length();
            let crossing = decomposed_band.get(height as usize).unwrap();
            let mut decomposed_band = decomposed_band.clone();

            let (quadrant, next_step_idx, next_step, previous_step) = if rng.random_bool(0.5) {
                let next_step_idx = rng.random_range(..height) as usize;
                let next_step = decomposed_band.get(next_step_idx).unwrap();
                if next_step.sign() == Sign::Negative {
                    let perturbed_next_step =
                        ArtinGenerator::try_new((next_step.foot() + 1).unwrap(), next_step.sign())
                            .unwrap();
                    if (next_step.foot() + 1).unwrap() == crossing.foot() {
                        (
                            StaircaseQuadrant::LowerLeft,
                            next_step_idx,
                            perturbed_next_step,
                            *crossing,
                        )
                    } else {
                        (
                            StaircaseQuadrant::LowerLeft,
                            next_step_idx,
                            perturbed_next_step,
                            perturbed_next_step,
                        )
                    }
                } else {
                    let perturbed_next_step =
                        ArtinGenerator::try_new((next_step.foot() - 1).unwrap(), next_step.sign())
                            .unwrap();
                    if (next_step.foot() - 1).unwrap() == crossing.foot() {
                        (
                            StaircaseQuadrant::UpperLeft,
                            next_step_idx,
                            perturbed_next_step,
                            *crossing,
                        )
                    } else {
                        (
                            StaircaseQuadrant::UpperLeft,
                            next_step_idx,
                            perturbed_next_step,
                            perturbed_next_step,
                        )
                    }
                }
            } else {
                let next_step_idx = rng.random_range((height + 1)..artin_length) as usize;
                let next_step = decomposed_band.get(next_step_idx).unwrap();
                if next_step.sign() == Sign::Negative {
                    let perturbed_next_step =
                        ArtinGenerator::try_new((next_step.foot() - 1).unwrap(), next_step.sign())
                            .unwrap();
                    if (next_step.foot() - 1).unwrap() == crossing.foot() {
                        (
                            StaircaseQuadrant::UpperRight,
                            next_step_idx,
                            perturbed_next_step,
                            *crossing,
                        )
                    } else {
                        (
                            StaircaseQuadrant::UpperRight,
                            next_step_idx,
                            perturbed_next_step,
                            perturbed_next_step,
                        )
                    }
                } else {
                    let perturbed_next_step =
                        ArtinGenerator::try_new((next_step.foot() + 1).unwrap(), next_step.sign())
                            .unwrap();
                    if (next_step.foot() + 1).unwrap() == crossing.foot() {
                        (
                            StaircaseQuadrant::LowerRight,
                            next_step_idx,
                            perturbed_next_step,
                            *crossing,
                        )
                    } else {
                        (
                            StaircaseQuadrant::LowerRight,
                            next_step_idx,
                            perturbed_next_step,
                            perturbed_next_step,
                        )
                    }
                }
            };
            decomposed_band[next_step_idx] = next_step;
            InvalidBandCoalescence {
                data: InvalidBandCoalescenceData::IncontiguousSteps(decomposed_band),
                error: BandValidationError::FromArtin(FromArtinError::IncontiguousSteps {
                    quadrant,
                    next_step,
                    previous_step,
                }),
            }
        },
    )
}
fn arbitrary_invalid_band_coalescence_imbalanced_staircases()
-> impl Strategy<Value = InvalidBandCoalescence> {
    arbitrary_single_coalescence(None, None, None).prop_perturb(
        |(band, decomposed_band), mut rng| {
            let height = band.unwrap().height();
            let (left, right) = decomposed_band.split_at(height as usize);
            let (crossing, right) = right.split_first().unwrap();

            let (lower_left_idxs, upper_left_idxs) =
                left.iter()
                    .enumerate()
                    .fold((Vec::new(), Vec::new()), |acc, (idx, artin)| {
                        if artin.sign() == Sign::Negative {
                            ([acc.0, vec![idx]].concat(), acc.1)
                        } else {
                            (acc.0, [acc.1, vec![idx]].concat())
                        }
                    });

            let (take_quadrant, num_to_take, outer_opposite_artin) = if lower_left_idxs.is_empty() {
                (
                    StaircaseQuadrant::UpperLeft,
                    rng.random_range(1..=upper_left_idxs.len()),
                    crossing,
                )
            } else if upper_left_idxs.is_empty() {
                (
                    StaircaseQuadrant::LowerLeft,
                    rng.random_range(1..=lower_left_idxs.len()),
                    crossing,
                )
            } else if rng.random_bool(0.5) {
                (
                    StaircaseQuadrant::UpperLeft,
                    rng.random_range(1..=upper_left_idxs.len()),
                    left.get(*lower_left_idxs.first().unwrap()).unwrap(),
                )
            } else {
                (
                    StaircaseQuadrant::LowerLeft,
                    rng.random_range(1..=lower_left_idxs.len()),
                    left.get(*upper_left_idxs.first().unwrap()).unwrap(),
                )
            };

            let mut left: Vec<ArtinGenerator> = left
                .iter()
                .enumerate()
                .filter_map(|(idx, &artin)| {
                    if (take_quadrant == StaircaseQuadrant::UpperLeft
                        && upper_left_idxs.contains(&idx))
                        || (take_quadrant == StaircaseQuadrant::LowerLeft
                            && lower_left_idxs.contains(&idx))
                    {
                        None
                    } else {
                        Some(artin)
                    }
                })
                .rev()
                .collect();

            if take_quadrant == StaircaseQuadrant::UpperLeft {
                for i in 1u16..=(*[
                    <Strand as Into<u16>>::into(outer_opposite_artin.foot()) - 1,
                    num_to_take.try_into().unwrap(),
                ]
                .iter()
                .min()
                .unwrap())
                {
                    left.push(
                        ArtinGenerator::try_new(
                            (outer_opposite_artin.foot() - i).unwrap(),
                            Sign::Negative,
                        )
                        .unwrap(),
                    );
                }
            } else {
                for i in 1u16..=(*[
                    (u16::MAX - <Strand as Into<u16>>::into(outer_opposite_artin.foot())) - 1,
                    num_to_take.try_into().unwrap(),
                ]
                .iter()
                .min()
                .unwrap())
                {
                    left.push(
                        ArtinGenerator::try_new(
                            (outer_opposite_artin.foot() + i).unwrap(),
                            Sign::Positive,
                        )
                        .unwrap(),
                    );
                }
            }

            left.reverse();

            let right: Vec<_> = right.iter().rev().collect();

            let (lower_right_idxs, upper_right_idxs) =
                right
                    .iter()
                    .enumerate()
                    .fold((Vec::new(), Vec::new()), |acc, (idx, artin)| {
                        if artin.sign() == Sign::Positive {
                            ([acc.0, vec![idx]].concat(), acc.1)
                        } else {
                            (acc.0, [acc.1, vec![idx]].concat())
                        }
                    });

            let (take_quadrant, outer_opposite_artin) = if upper_right_idxs.is_empty() {
                (StaircaseQuadrant::LowerRight, crossing)
            } else if lower_right_idxs.is_empty() {
                (StaircaseQuadrant::UpperRight, crossing)
            } else if take_quadrant == StaircaseQuadrant::UpperLeft {
                (
                    StaircaseQuadrant::LowerRight,
                    *right.get(*upper_right_idxs.first().unwrap()).unwrap(),
                )
            } else {
                (
                    StaircaseQuadrant::UpperRight,
                    *right.get(*lower_right_idxs.first().unwrap()).unwrap(),
                )
            };

            let mut right: Vec<ArtinGenerator> = right
                .iter()
                .enumerate()
                .filter_map(|(idx, &artin)| {
                    if (take_quadrant == StaircaseQuadrant::UpperRight
                        && upper_right_idxs.contains(&idx))
                        || (take_quadrant == StaircaseQuadrant::LowerRight
                            && lower_right_idxs.contains(&idx))
                    {
                        None
                    } else {
                        Some(*artin)
                    }
                })
                .rev()
                .collect();

            if take_quadrant == StaircaseQuadrant::UpperRight {
                for i in 1u16..=(*[
                    <Strand as Into<u16>>::into(outer_opposite_artin.foot()) - 1,
                    num_to_take.try_into().unwrap(),
                ]
                .iter()
                .min()
                .unwrap())
                {
                    right.push(
                        ArtinGenerator::try_new(
                            (outer_opposite_artin.foot() - i).unwrap(),
                            Sign::Positive,
                        )
                        .unwrap(),
                    );
                }
            } else {
                for i in 1u16..=(*[
                    (u16::MAX - <Strand as Into<u16>>::into(outer_opposite_artin.foot())) - 1,
                    num_to_take.try_into().unwrap(),
                ]
                .iter()
                .min()
                .unwrap())
                {
                    right.push(
                        ArtinGenerator::try_new(
                            (outer_opposite_artin.foot() + i).unwrap(),
                            Sign::Negative,
                        )
                        .unwrap(),
                    );
                }
            }
            InvalidBandCoalescence {
                data: InvalidBandCoalescenceData::ImbalancedStaircases(
                    [left, vec![*crossing], right].concat(),
                ),
                error: BandValidationError::FromArtin(FromArtinError::ImbalancedStaircases(
                    2 * num_to_take,
                )),
            }
        },
    )
}

pub fn arbitrary_invalid_band_coalescence() -> impl Strategy<Value = InvalidBandCoalescence> {
    prop_oneof![
        arbitrary_invalid_band_coalescence_no_generators(),
        arbitrary_invalid_band_coalescence_even_generators(),
        arbitrary_invalid_band_coalescence_too_many_generators(),
        arbitrary_invalid_band_coalescence_incontiguous_steps(),
        arbitrary_invalid_band_coalescence_imbalanced_staircases(),
    ]
}
