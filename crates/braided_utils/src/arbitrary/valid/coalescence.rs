use crate::arbitrary::valid;
use braided::{ArtinGenerator, BandGenerator, BandResult, Letter, Sign, Word};
use proptest::prelude::*;

pub fn of_band_generator(
    max_head: Option<u16>,
    max_height: Option<u16>,
    max_artin_length: Option<u16>,
) -> impl Strategy<Value = (BandResult, Vec<ArtinGenerator>)> {
    if let Some(max_head) = max_head
        && max_head < 2
    {
        panic!("max_head must be at least 2.")
    }
    if let Some(max_artin_length) = max_artin_length
        && max_artin_length < 1
    {
        panic!("max_artin_length must be positive.")
    }
    valid::band::data(max_head, max_height, max_artin_length)
        .prop_flat_map(|(foot, head, sign)| {
            let foot: u16 = foot.into();
            let head: u16 = head.into();
            (Just(foot), Just(head), Just(sign), foot..=(head - 1))
        })
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

fn of_band_letter(
    max_head: Option<u16>,
    max_height: Option<u16>,
    max_artin_length: Option<u16>,
) -> impl Strategy<Value = (Letter, Vec<Letter>)> {
    if let Some(max_head) = max_head
        && max_head < 2
    {
        panic!("max_head must be at least 2.")
    }
    of_band_generator(max_head, max_height, max_artin_length).prop_perturb(
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
struct WalledBandLetterCoalescence {
    band: Letter,
    decomposed_band: Vec<Letter>,
    left_wall: Vec<Letter>,
    right_wall: Vec<Letter>,
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

fn of_band_letter_walled(
    max_head: Option<u16>,
    max_height: Option<u16>,
    max_artin_length: Option<u16>,
) -> impl Strategy<Value = WalledBandLetterCoalescence> {
    if let Some(max_head) = max_head
        && max_head < 2
    {
        panic!("max_head must be at least 2.")
    }
    let max_band_artin_length = if let Some(max) = max_artin_length {
        if max <= 2 {
            panic!("The max_artin_length for a walled coalescence must be greater than 2.");
        } else {
            max - 2
        }
    } else {
        u16::MAX - 2
    };
    of_band_letter(max_head, max_height, Some(max_band_artin_length))
        .prop_flat_map(move |(band, decomposed_band)| {
            let max_wall_length = max_artin_length.unwrap_or(u16::MAX)
                - <usize as TryInto<u16>>::try_into(decomposed_band.len()).unwrap();
            if max_wall_length < 2 {
                panic!("max_wall_length must be at least 2.")
            }
            (
                Just(band),
                Just(decomposed_band),
                Just(max_wall_length),
                1..=(max_wall_length - 1),
            )
        })
        .prop_flat_map(
            move |(band, decomposed_band, max_total_wall_length, max_left_wall_length)| {
                let left_wall_letter = *decomposed_band.first().unwrap();
                let right_wall_letter = *decomposed_band.last().unwrap();
                if max_total_wall_length == 0 {
                    panic!("max_total_wall_length must be positive")
                }
                if max_left_wall_length == 0 {
                    panic!("max_left_wall_length must be positive")
                }
                let max_right_wall_length = max_total_wall_length - max_left_wall_length;
                if max_right_wall_length == 0 {
                    panic!("max_right_wall_length must be positive")
                }
                (
                    Just(band),
                    Just(decomposed_band),
                    Just(max_left_wall_length),
                    Just(max_right_wall_length),
                    Just(left_wall_letter),
                    Just(right_wall_letter),
                    // valid::letter::new(max_head, max_height, Some(max_left_wall_length)),
                    // valid::letter::new(max_head, max_height, Some(max_right_wall_length)),
                )
            },
        )
        .prop_filter(
            "The wall letters must not extend the coalesced band.",
            |(band, _, _, _, left_wall_letter, right_wall_letter)| {
                !pair_extends_band(*band, *left_wall_letter, *right_wall_letter)
            },
        )
        .prop_flat_map(
            |(
                band,
                decomposed_band,
                max_left_wall_length,
                max_right_wall_length,
                left_wall_letter,
                right_wall_letter,
            )| {
                let max_left_wall_letter_reps =
                    max_left_wall_length.div_euclid(left_wall_letter.artin_length());
                let max_right_wall_letter_reps =
                    max_right_wall_length.div_euclid(right_wall_letter.artin_length());
                (
                    Just(band),
                    Just(decomposed_band),
                    Just(left_wall_letter),
                    Just(right_wall_letter),
                    1..=max_left_wall_letter_reps,
                    1..=max_right_wall_letter_reps,
                )
            },
        )
        .prop_map(
            |(
                band,
                decomposed_band,
                left_wall_letter,
                right_wall_letter,
                left_wall_letter_reps,
                right_wall_letter_reps,
            )| WalledBandLetterCoalescence {
                band,
                decomposed_band,
                left_wall: vec![left_wall_letter; left_wall_letter_reps as usize],
                right_wall: vec![right_wall_letter; right_wall_letter_reps as usize],
            },
        )
}

fn of_band_letter_walled_multiple(
    max_head: Option<u16>,
    max_height: Option<u16>,
    max_artin_length: Option<u16>,
) -> impl Strategy<Value = Vec<WalledBandLetterCoalescence>> {
    if let Some(max_head) = max_head
        && max_head < 2
    {
        panic!("max_head must be at least 2.")
    }
    let max_walled_coalescences = if let Some(max) = max_artin_length {
        if max < 3 {
            panic!("Max artin length must be at least 3 for generation of arbitrary coalescences.");
        } else {
            max.div_euclid(3)
        }
    } else {
        u16::MAX.div_euclid(3)
    };
    (1..=max_walled_coalescences).prop_flat_map(move |num_walled_coalescences| {
        let max_walled_coalescence_artin_length = max_artin_length
            .unwrap_or(u16::MAX)
            .div_euclid(num_walled_coalescences);
        let mut strategies = Vec::new();
        for _ in 0..num_walled_coalescences {
            strategies.push(of_band_letter_walled(
                max_head,
                max_height,
                Some(max_walled_coalescence_artin_length),
            ));
        }
        strategies
    })
}

enum PruningSafety {
    None,
    Partial,
    Full,
}

fn get_pruning_safety(
    first: Letter,
    first_left_letter: Option<Letter>,
    first_right_wall: &[Letter],
    second: Letter,
    second_left_wall: &[Letter],
    second_right_wall: &[Letter],
) -> PruningSafety {
    let check_first = first_left_letter.is_some();
    let first_right_letter = first_right_wall.first().unwrap();
    let second_left_letter = second_left_wall.first().unwrap();
    let second_right_letter = second_right_wall.first().unwrap();

    if (check_first && pair_extends_band(first, first_left_letter.unwrap(), *second_left_letter))
        || pair_extends_band(second, *first_right_letter, *second_right_letter)
    {
        PruningSafety::None
    } else if (check_first && pair_extends_band(first, first_left_letter.unwrap(), second))
        || pair_extends_band(second, first, *second_right_letter)
    {
        PruningSafety::Partial
    } else {
        PruningSafety::Full
    }
}

pub fn of_word(
    max_head: Option<u16>,
    max_height: Option<u16>,
    max_artin_length: Option<u16>,
    prune: bool,
) -> impl Strategy<Value = (Word, Word)> {
    if let Some(max_head) = max_head
        && max_head < 2
    {
        panic!("max_head must be at least 2.")
    }
    of_band_letter_walled_multiple(max_head, max_height, max_artin_length).prop_perturb(
        move |mut walled_coalescences, mut rng| {
            let mut walled_coalescences = walled_coalescences.iter_mut().peekable();
            let mut coalescence: Vec<Letter> = Vec::new();
            let mut decomposed: Vec<Letter> = Vec::new();
            let mut first_left_letter: Option<Letter> = None;

            let initial = walled_coalescences.peek_mut();

            if rng.random_bool(0.5)
                && let Some(WalledBandLetterCoalescence {
                    band: _,
                    decomposed_band: _,
                    left_wall: initial_left_wall,
                    right_wall: _,
                }) = initial
            {
                *initial_left_wall = Vec::new();
            } else if let Some(WalledBandLetterCoalescence {
                band: _,
                decomposed_band: _,
                left_wall: initial_left_wall,
                right_wall: _,
            }) = initial
            {
                first_left_letter = Some(*initial_left_wall.first().unwrap());
            }

            while let Some(WalledBandLetterCoalescence {
                band,
                decomposed_band,
                left_wall,
                right_wall,
            }) = walled_coalescences.next()
            {
                if prune
                    && let Some(WalledBandLetterCoalescence {
                        band: next_band,
                        decomposed_band: _,
                        left_wall: next_left_wall,
                        right_wall: next_right_wall,
                    }) = walled_coalescences.peek_mut()
                {
                    match get_pruning_safety(
                        *band,
                        first_left_letter,
                        right_wall,
                        *next_band,
                        next_left_wall,
                        next_right_wall,
                    ) {
                        PruningSafety::None => {
                            first_left_letter = next_left_wall.first().cloned();
                        }
                        PruningSafety::Partial => {
                            if rng.random_bool(0.5) && rng.random_bool(0.5) {
                                *right_wall = Vec::new();
                                first_left_letter = next_left_wall.first().cloned();
                            } else if rng.random_bool(0.5) && rng.random_bool(0.5) {
                                *next_left_wall = Vec::new();
                                first_left_letter = right_wall.first().cloned();
                            }
                        }
                        PruningSafety::Full => {
                            if rng.random_bool(0.5) {
                                *right_wall = Vec::new();
                                *next_left_wall = Vec::new();
                                first_left_letter = Some(*band);
                            }
                        }
                    }
                } else if prune && rng.random_bool(0.5) {
                    *right_wall = Vec::new();
                }

                coalescence.extend(left_wall.clone());
                coalescence.push(*band);
                coalescence.extend(right_wall.clone());

                decomposed.extend(left_wall.clone());
                decomposed.extend(decomposed_band.clone());
                decomposed.extend(right_wall.clone());
            }

            (
                Word::try_from_letters(&decomposed[..]).clone_unwrap(),
                Word::try_from_letters(&coalescence[..]).clone_unwrap(),
            )
        },
    )
}

pub mod test_cases {
    use super::*;
    use braided::Braid;

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct CoalesceBandData(pub Vec<ArtinGenerator>);

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct DecomposeBandData(pub BandGenerator);

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct DecomposeLetterData(pub Letter);

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct CoalesceWordData(pub Word);

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct DecomposeWordData(pub Word);

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct CoalesceBraidData(pub Braid);

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct DecomposeBraidData(pub Braid);

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct CoalesceBand {
        pub data: CoalesceBandData,
        pub expected: BandResult,
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct DecomposeBand {
        pub data: DecomposeBandData,
        pub expected: Vec<ArtinGenerator>,
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct DecomposeLetter {
        pub data: DecomposeLetterData,
        pub expected: Vec<Letter>,
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct CoalesceWord {
        pub data: CoalesceWordData,
        pub expected: Word,
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct DecomposeWord {
        pub data: DecomposeWordData,
        pub expected: Word,
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct CoalesceBraid {
        pub data: CoalesceBraidData,
        pub expected: Braid,
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct DecomposeBraid {
        pub data: DecomposeBraidData,
        pub expected: Braid,
    }

    pub fn coalesce_band(
        max_head: Option<u16>,
        max_height: Option<u16>,
        max_artin_length: Option<u16>,
    ) -> impl Strategy<Value = CoalesceBand> {
        of_band_generator(max_head, max_height, max_artin_length).prop_map(
            |(band_generator, artin_generators)| CoalesceBand {
                data: CoalesceBandData(artin_generators),
                expected: band_generator,
            },
        )
    }

    pub fn decompose_band(
        max_head: Option<u16>,
        max_height: Option<u16>,
        max_artin_length: Option<u16>,
    ) -> impl Strategy<Value = DecomposeBand> {
        of_band_generator(max_head, max_height, max_artin_length).prop_map(
            |(band_generator, artin_generators)| DecomposeBand {
                data: DecomposeBandData(band_generator.unwrap()),
                expected: artin_generators,
            },
        )
    }

    pub fn decompose_letter(
        max_head: Option<u16>,
        max_height: Option<u16>,
        max_artin_length: Option<u16>,
    ) -> impl Strategy<Value = DecomposeLetter> {
        prop_oneof![
            of_band_generator(max_head, max_height, max_artin_length).prop_map(
                |(band_generator, artin_generators)| DecomposeLetter {
                    data: DecomposeLetterData(Letter::Band(band_generator.unwrap())),
                    expected: artin_generators
                        .iter()
                        .map(|&artin_generator| Letter::Artin(artin_generator))
                        .collect(),
                },
            ),
            valid::artin::new(None, max_head.map(|h| h - 1)).prop_map(|artin_generator| {
                DecomposeLetter {
                    data: DecomposeLetterData(Letter::Artin(artin_generator)),
                    expected: vec![Letter::Artin(artin_generator)],
                }
            })
        ]
    }

    pub fn coalesce_word(
        max_head: Option<u16>,
        max_height: Option<u16>,
        max_artin_length: Option<u16>,
        prune: bool,
    ) -> impl Strategy<Value = CoalesceWord> {
        of_word(max_head, max_height, max_artin_length, prune).prop_map(|(word, expected)| {
            CoalesceWord {
                data: CoalesceWordData(word),
                expected,
            }
        })
    }

    pub fn decompose_word(
        max_head: Option<u16>,
        max_height: Option<u16>,
        max_artin_length: Option<u16>,
        prune: bool,
    ) -> impl Strategy<Value = DecomposeWord> {
        of_word(max_head, max_height, max_artin_length, prune).prop_map(|(expected, word)| {
            DecomposeWord {
                data: DecomposeWordData(word),
                expected,
            }
        })
    }

    pub fn coalesce_braid(
        max_braid_index: Option<u16>,
        max_height: Option<u16>,
        max_artin_length: Option<u16>,
        prune: bool,
    ) -> impl Strategy<Value = CoalesceBraid> {
        of_word(max_braid_index, max_height, max_artin_length, prune).prop_map(
            |(decomposed_word, expected_word)| {
                let braid_index = decomposed_word.minimal_required_braid_index();
                CoalesceBraid {
                    data: CoalesceBraidData(
                        Braid::try_new(braid_index, decomposed_word).clone_unwrap(),
                    ),
                    expected: Braid::try_new(braid_index, expected_word).clone_unwrap(),
                }
            },
        )
    }

    pub fn decompose_braid(
        max_braid_index: Option<u16>,
        max_height: Option<u16>,
        max_artin_length: Option<u16>,
        prune: bool,
    ) -> impl Strategy<Value = DecomposeBraid> {
        of_word(max_braid_index, max_height, max_artin_length, prune).prop_map(
            |(expected_word, coalesced_word)| {
                let braid_index = coalesced_word.minimal_required_braid_index();
                DecomposeBraid {
                    data: DecomposeBraidData(
                        Braid::try_new(braid_index, coalesced_word).clone_unwrap(),
                    ),
                    expected: Braid::try_new(braid_index, expected_word).clone_unwrap(),
                }
            },
        )
    }
}
