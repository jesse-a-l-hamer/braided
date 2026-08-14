use crate::arbitrary::valid::{arbitrary_band_data, arbitrary_letter};
use braided::{ArtinGenerator, BandGenerator, BandResult, Letter, Sign, Word};
use proptest::prelude::*;

pub fn arbitrary_single_coalescence(
    max_head: Option<u16>,
    max_height: Option<u16>,
    max_artin_length: Option<u16>,
) -> impl Strategy<Value = (BandResult, Vec<ArtinGenerator>)> {
    arbitrary_band_data(max_head, max_height, max_artin_length)
        .prop_flat_map(|(foot, head, sign)| (Just(foot), Just(head), Just(sign), foot..(head - 1)))
        .prop_perturb(|(foot, head, sign, crossing_foot), mut rng| {
            let band = BandGenerator::try_new(foot, head, sign);
            let crossing = ArtinGenerator::try_new(crossing_foot, sign).unwrap();
            let mut left: Vec<ArtinGenerator> = Vec::new();
            let mut right: Vec<ArtinGenerator> = Vec::new();

            let mut lower_left = (foot..crossing_foot)
                .map(|f| ArtinGenerator::try_new(f, Sign::Negative).unwrap())
                .peekable();
            let mut upper_left = ((crossing_foot + 1)..head)
                .rev()
                .map(|f| ArtinGenerator::try_new(f, Sign::Positive).unwrap())
                .peekable();
            let mut lower_right = (foot..crossing_foot)
                .rev()
                .map(|f| ArtinGenerator::try_new(f, Sign::Positive).unwrap())
                .peekable();
            let mut upper_right = ((crossing_foot + 1)..head)
                .map(|f| ArtinGenerator::try_new(f, Sign::Negative).unwrap())
                .peekable();

            while lower_left.peek().is_some() && upper_left.peek().is_some() {
                if rng.random_bool(0.5) {
                    left.push(lower_left.next().unwrap());
                } else {
                    left.push(upper_left.next().unwrap());
                }
            }
            left.extend(lower_left);
            left.extend(upper_left);

            while lower_right.peek().is_some() && upper_right.peek().is_some() {
                if rng.random_bool(0.5) {
                    right.push(lower_right.next().unwrap());
                } else {
                    right.push(upper_right.next().unwrap());
                }
            }
            right.extend(lower_right);
            right.extend(upper_right);

            (band, [left, vec![crossing], right].concat())
        })
}

fn arbitrary_single_coalescence_as_letters(
    max_head: Option<u16>,
    max_height: Option<u16>,
    max_artin_length: Option<u16>,
) -> impl Strategy<Value = (Letter, Vec<Letter>)> {
    arbitrary_single_coalescence(max_head, max_height, max_artin_length).prop_perturb(
        |(band, artin_generators), mut rng| {
            let band_letter = Letter::Band(band.unwrap());
            let artin_letters: Vec<Letter> = artin_generators
                .iter()
                .map(|&a| {
                    if rng.random_bool(0.5) {
                        Letter::Artin(a)
                    } else {
                        Letter::try_new(a.foot(), Some((a.foot() + 1).unwrap()), a.sign()).unwrap()
                    }
                })
                .collect();

            (band_letter, artin_letters)
        },
    )
}

#[derive(Debug, Clone)]
struct BlockedCoalescence {
    band: Letter,
    decomposed_band: Vec<Letter>,
    left_blockage: Vec<Letter>,
    right_blockage: Vec<Letter>,
}

fn arbitrary_single_blocked_coalescence(
    max_head: Option<u16>,
    max_height: Option<u16>,
    max_artin_length: Option<u16>,
) -> impl Strategy<Value = BlockedCoalescence> {
    arbitrary_single_coalescence_as_letters(max_head, max_height, max_artin_length)
        .prop_flat_map(move |(coalesced_band, decomposed_band)| {
            let max_blockage_length =
                (max_artin_length.unwrap_or(u16::MAX) as usize - decomposed_band.len()).div_ceil(2);
            (
                Just(coalesced_band),
                Just(decomposed_band.clone()),
                arbitrary_letter(max_head, max_height, max_artin_length),
                1..max_blockage_length,
                arbitrary_letter(max_head, max_height, max_artin_length),
                1..max_blockage_length,
            )
        })
        .prop_filter_map(
            "The blockers must not extend the coalesced band.",
            |(
                coalesced_band,
                decomposed_band,
                left_blocker,
                left_blockage_length,
                right_blocker,
                right_blockage_length,
            )| {
                let foot = coalesced_band.foot();
                let head = coalesced_band.head();
                let is_lower_extension = left_blocker.sign() == Sign::Negative
                    && right_blocker.sign() == Sign::Positive
                    && left_blocker.head() == foot
                    && right_blocker.head() == foot;
                let is_upper_extension = left_blocker.sign() == Sign::Positive
                    && right_blocker.sign() == Sign::Negative
                    && left_blocker.foot() == head
                    && right_blocker.foot() == head;

                if is_lower_extension || is_upper_extension {
                    None
                } else {
                    Some(BlockedCoalescence {
                        band: coalesced_band,
                        decomposed_band,
                        left_blockage: vec![left_blocker; left_blockage_length],
                        right_blockage: vec![right_blocker; right_blockage_length],
                    })
                }
            },
        )
}

fn arbitrary_multiple_blocked_coalescence(
    max_head: Option<u16>,
    max_height: Option<u16>,
    max_artin_length: Option<u16>,
) -> impl Strategy<Value = Vec<BlockedCoalescence>> {
    prop::collection::vec(
        arbitrary_single_blocked_coalescence(max_head, max_height, max_artin_length),
        1..(max_artin_length.unwrap_or(u16::MAX) as usize),
    )
}

enum PruningSafety {
    None,
    Partial,
    Full,
}

fn pair_extends_band(band: Letter, left: Letter, right: Letter) -> bool {
    let extends_up = left.sign() == Sign::Negative
        && right.sign() == Sign::Positive
        && left.head() == band.foot()
        && right.head() == band.foot();
    let extends_down = left.sign() == Sign::Positive
        && right.sign() == Sign::Negative
        && left.foot() == band.head()
        && right.foot() == band.head();

    extends_up || extends_down
}

fn get_pruning_safety(
    first: Letter,
    first_left_blockage: &[Letter],
    first_right_blockage: &[Letter],
    second: Letter,
    second_left_blockage: &[Letter],
    second_right_blockage: &[Letter],
) -> PruningSafety {
    let first_left_letter = first_left_blockage.first().unwrap();
    let first_right_letter = first_right_blockage.first().unwrap();
    let second_left_letter = second_left_blockage.first().unwrap();
    let second_right_letter = second_right_blockage.first().unwrap();

    if pair_extends_band(first, *first_left_letter, *second_left_letter)
        || pair_extends_band(second, *first_right_letter, *second_right_letter)
    {
        PruningSafety::None
    } else if pair_extends_band(first, *first_left_letter, second)
        || pair_extends_band(second, first, *second_right_letter)
    {
        PruningSafety::Partial
    } else {
        PruningSafety::Full
    }
}

pub fn arbitrary_coalescence(
    max_head: Option<u16>,
    max_height: Option<u16>,
    max_artin_length: Option<u16>,
) -> impl Strategy<Value = (Word, Word)> {
    arbitrary_multiple_blocked_coalescence(max_head, max_height, max_artin_length).prop_perturb(
        |mut blocked_coalescences, mut rng| {
            let mut blocked_coalescences = blocked_coalescences.iter_mut().peekable();
            let mut coalescence: Vec<Letter> = Vec::new();
            let mut decomposed: Vec<Letter> = Vec::new();

            if rng.random_bool(0.5)
                && let Some(BlockedCoalescence {
                    band: _,
                    decomposed_band: _,
                    left_blockage: initial_left_blockage,
                    right_blockage: _,
                }) = blocked_coalescences.peek_mut()
            {
                *initial_left_blockage = Vec::new();
            }

            while let Some(BlockedCoalescence {
                band,
                decomposed_band,
                left_blockage,
                right_blockage,
            }) = blocked_coalescences.next()
            {
                if let Some(BlockedCoalescence {
                    band: next_band,
                    decomposed_band: _,
                    left_blockage: next_left_blockage,
                    right_blockage: next_right_blockage,
                }) = blocked_coalescences.peek_mut()
                {
                    match get_pruning_safety(
                        *band,
                        left_blockage,
                        right_blockage,
                        *next_band,
                        next_left_blockage,
                        next_right_blockage,
                    ) {
                        PruningSafety::None => {}
                        PruningSafety::Partial => {
                            if rng.random_bool(0.5) && rng.random_bool(0.5) {
                                *right_blockage = Vec::new();
                            } else if rng.random_bool(0.5) && rng.random_bool(0.5) {
                                *next_left_blockage = Vec::new();
                            }
                        }
                        PruningSafety::Full => {
                            if rng.random_bool(0.5) {
                                *right_blockage = Vec::new();
                                *next_left_blockage = Vec::new();
                            }
                        }
                    }
                } else {
                    if rng.random_bool(0.5) {
                        *right_blockage = Vec::new();
                    }
                }

                coalescence.extend(left_blockage.clone());
                coalescence.push(*band);
                coalescence.extend(right_blockage.clone());

                decomposed.extend(left_blockage.clone());
                decomposed.extend(decomposed_band.clone());
                decomposed.extend(right_blockage.clone());
            }

            (
                Word::try_from_letters(&coalescence[..]).clone_unwrap(),
                Word::try_from_letters(&decomposed[..]).clone_unwrap(),
            )
        },
    )
}
