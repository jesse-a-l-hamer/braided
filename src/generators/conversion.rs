use crate::{ArtinGenerator, BandGenerator, Sign, Strand};
use std::{collections::BTreeSet, ops::Neg};

pub fn artin_to_band(generators: &[ArtinGenerator]) -> Vec<BandGenerator> {
    let mut bands = Vec::new();

    if generators.is_empty() {
        return bands;
    }

    let res = collect_generators(generators);

    bands.extend(artin_to_band(&res.0));
    bands.push(res.1);
    bands.extend(artin_to_band(&res.2));

    bands
}

#[derive(Debug, Clone, Copy)]
enum StaircaseDirection {
    Up,
    Down,
}

impl Neg for StaircaseDirection {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }
}

#[derive(Debug, Clone)]
struct Staircase {
    direction: StaircaseDirection,
    steps: BTreeSet<usize>,
}

fn collect_generators(
    generators: &[ArtinGenerator],
) -> (Vec<ArtinGenerator>, BandGenerator, Vec<ArtinGenerator>) {
    let mut pivot = if generators.len().is_multiple_of(2) {
        generators.len() / 2
    } else {
        generators.len().div_ceil(2)
    };

    while pivot > 1 {
        let primary_staircase = match construct_primary_staircase(generators, pivot) {
            Some(staircase) => staircase,
            None => {
                pivot -= 1;
                continue;
            }
        };
        let mirrored_primary_staircase =
            match mirror_primary_staircase(generators, pivot, &primary_staircase) {
                Some(staircase) => staircase,
                None => {
                    pivot -= 1;
                    continue;
                }
            };

        let secondary_staircase =
            construct_secondary_staircase(generators, pivot, -primary_staircase.direction);
        let mirrored_secondary_staircase =
            mirror_secondary_staircase(generators, pivot, &secondary_staircase);
        let secondary_staircase = Staircase {
            direction: secondary_staircase.direction,
            steps: secondary_staircase
                .steps
                .into_iter()
                .take(mirrored_secondary_staircase.steps.len())
                .collect(),
        };

        let skip_left_indices: Vec<&usize> = primary_staircase
            .steps
            .union(&secondary_staircase.steps)
            .collect();
        let skip_right_indices: Vec<&usize> = mirrored_primary_staircase
            .steps
            .union(&mirrored_secondary_staircase.steps)
            .collect();

        let skip_left = generators
            .iter()
            .enumerate()
            .filter_map(|(i, &v)| {
                if i < pivot && skip_left_indices.contains(&&i) {
                    Some(v)
                } else {
                    None
                }
            })
            .collect();
        let skip_right = generators
            .iter()
            .enumerate()
            .filter_map(|(i, &v)| {
                if pivot < i && skip_right_indices.contains(&&i) {
                    Some(v)
                } else {
                    None
                }
            })
            .collect();

        let band_parts: Vec<ArtinGenerator> = generators
            .iter()
            .enumerate()
            .filter_map(|(i, &v)| {
                if i == pivot
                    || primary_staircase.steps.contains(&i)
                    || secondary_staircase.steps.contains(&i)
                    || mirrored_primary_staircase.steps.contains(&i)
                    || mirrored_secondary_staircase.steps.contains(&i)
                {
                    Some(v)
                } else {
                    None
                }
            })
            .collect();

        return (
            skip_left,
            BandGenerator::from_artin(&band_parts)
                .expect("Failed to construct band generator from given parts."),
            skip_right,
        );
    }

    let skip_left = generators[..pivot].to_vec();
    let skip_right = generators[pivot + 1..].to_vec();
    (
        skip_left,
        BandGenerator::from_artin(&[generators[pivot]])
            .expect("Failed to construct band generator from given parts."),
        skip_right,
    )
}

enum DirectionSignMatchAction {
    Insert,
    Fail,
    Skip,
}

fn match_direction_and_sign(
    direction: StaircaseDirection,
    sign: Sign,
    next_foot: Strand,
    current_foot: Strand,
) -> DirectionSignMatchAction {
    match (direction, sign) {
        (StaircaseDirection::Up, Sign::Positive) => {
            if next_foot == current_foot + 1 {
                DirectionSignMatchAction::Insert
            } else if next_foot > current_foot + 1 {
                DirectionSignMatchAction::Fail
            } else {
                DirectionSignMatchAction::Skip
            }
        }
        (StaircaseDirection::Up, Sign::Negative) => {
            if next_foot > current_foot {
                DirectionSignMatchAction::Fail
            } else {
                DirectionSignMatchAction::Skip
            }
        }
        (StaircaseDirection::Down, Sign::Positive) => {
            if next_foot < current_foot {
                DirectionSignMatchAction::Fail
            } else {
                DirectionSignMatchAction::Skip
            }
        }
        (StaircaseDirection::Down, Sign::Negative) => {
            if next_foot + 1 == current_foot {
                DirectionSignMatchAction::Insert
            } else if next_foot + 1 < current_foot {
                DirectionSignMatchAction::Fail
            } else {
                DirectionSignMatchAction::Skip
            }
        }
    }
}

fn construct_primary_staircase(generators: &[ArtinGenerator], pivot: usize) -> Option<Staircase> {
    let start = generators.first().unwrap();
    let crossing = generators[pivot];

    let direction = match start.foot().cmp(&crossing.foot()) {
        std::cmp::Ordering::Less => StaircaseDirection::Down,
        std::cmp::Ordering::Equal => return None,
        std::cmp::Ordering::Greater => StaircaseDirection::Up,
    };

    let mut steps = BTreeSet::new();

    for (i, next) in generators[..pivot].iter().rev().enumerate() {
        let current = generators.get(*steps.last().unwrap_or(&pivot)).unwrap();
        if next.foot() == current.foot() {
            return None;
        }

        match match_direction_and_sign(direction, next.sign(), next.foot(), current.foot()) {
            DirectionSignMatchAction::Insert => steps.insert(pivot - (i + 1)),
            DirectionSignMatchAction::Fail => return None,
            DirectionSignMatchAction::Skip => continue,
        };
    }

    Some(Staircase { direction, steps })
}

fn mirror_primary_staircase(
    generators: &[ArtinGenerator],
    pivot: usize,
    primary_staircase: &Staircase,
) -> Option<Staircase> {
    let direction = primary_staircase.direction;
    let target_length = primary_staircase.steps.len();

    let mut steps = BTreeSet::new();

    if pivot + 1 + target_length > generators.len() {
        return None;
    }

    for (i, next) in generators[pivot + 1..].iter().enumerate() {
        let current = generators.get(*steps.last().unwrap_or(&pivot)).unwrap();
        if next.foot() == current.foot() {
            return None;
        }

        match match_direction_and_sign(direction, next.sign(), next.foot(), current.foot()) {
            DirectionSignMatchAction::Insert => steps.insert(i),
            DirectionSignMatchAction::Fail => return None,
            DirectionSignMatchAction::Skip => continue,
        };

        if steps.len() == target_length {
            break;
        }
    }

    Some(Staircase { direction, steps })
}

fn construct_secondary_staircase(
    generators: &[ArtinGenerator],
    pivot: usize,
    direction: StaircaseDirection,
) -> Staircase {
    let mut steps = BTreeSet::new();

    for (i, next) in generators[..pivot].iter().rev().enumerate() {
        let current = generators.get(*steps.last().unwrap_or(&pivot)).unwrap();
        if next.foot() == current.foot() {
            return Staircase { direction, steps };
        }

        match match_direction_and_sign(direction, next.sign(), next.foot(), current.foot()) {
            DirectionSignMatchAction::Insert => steps.insert(pivot - (i + 1)),
            DirectionSignMatchAction::Fail => return Staircase { direction, steps },
            DirectionSignMatchAction::Skip => continue,
        };
    }

    Staircase { direction, steps }
}

fn mirror_secondary_staircase(
    generators: &[ArtinGenerator],
    pivot: usize,
    secondary_staircase: &Staircase,
) -> Staircase {
    let direction = secondary_staircase.direction;
    let target_length = secondary_staircase.steps.len();

    let mut steps = BTreeSet::new();

    if pivot + 1 + target_length > generators.len() {
        return Staircase { direction, steps };
    }

    for (i, next) in generators[pivot + 1..].iter().enumerate() {
        let current = generators.get(*steps.last().unwrap_or(&pivot)).unwrap();
        if next.foot() == current.foot() {
            return Staircase { direction, steps };
        }

        match match_direction_and_sign(direction, next.sign(), next.foot(), current.foot()) {
            DirectionSignMatchAction::Insert => steps.insert(i),
            DirectionSignMatchAction::Fail => return Staircase { direction, steps },
            DirectionSignMatchAction::Skip => continue,
        };

        if steps.len() == target_length {
            break;
        }
    }

    Staircase { direction, steps }
}

pub fn band_to_artin(bands: &[BandGenerator]) -> Vec<ArtinGenerator> {
    bands
        .iter()
        .fold(Vec::new(), |w, b| [w, decompose_band(b)].concat())
}

fn decompose_band(band: &BandGenerator) -> Vec<ArtinGenerator> {
    // Band decomposition is infallible, so it's safe to unwrap any intermediate results
    let crossing = ArtinGenerator::new(band.head().index() - 1, band.sign()).unwrap();
    let mut left = Vec::new();
    for foot_idx in band.foot().index()..(band.head().index() - 1) {
        left.push(ArtinGenerator::new(foot_idx, Sign::Negative).unwrap());
    }
    let right = left.iter().rev().map(|a| -*a).collect();
    [left, vec![crossing], right].concat()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct ConversionTestFixture {
        artin_word: Vec<ArtinGenerator>,
        band_word: Vec<BandGenerator>,
    }
}
