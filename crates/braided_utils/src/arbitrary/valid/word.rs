use crate::arbitrary::valid::band::{
    arbitrary_band_data_with_given_height, arbitrary_band_with_given_height,
};
use crate::arbitrary::valid::{arbitrary_artin_data, arbitrary_artin_generator};
use braided::{BandGenerator, Letter, Sign, Word};
use proptest::prelude::*;

// Returns a partition of `partition_value` into a vector of odd numbers no greater than
// `max_partition_elem`.
fn arbitrary_partition_into_odd_numbers(
    partition_value: u16,
    max_partition_elem: u16,
) -> impl Strategy<Value = Vec<u16>> {
    let partition_length_strategy = if partition_value == 0 {
        (0u16..=0).boxed()
    } else if partition_value.is_multiple_of(2) {
        (1..=partition_value / 2).prop_map(|k| 2 * k).boxed()
    } else {
        (1..=partition_value.div_euclid(2))
            .prop_map(|k| 2 * k + 1)
            .boxed()
    };
    partition_length_strategy
        .prop_flat_map(move |partition_length| {
            let elem_upper_bound = if max_partition_elem.is_multiple_of(2) {
                max_partition_elem / 2 - 1
            } else {
                max_partition_elem.div_euclid(2)
            };
            vec![1..=elem_upper_bound; partition_length as usize]
        })
        .prop_map(move |partition| {
            let mut partition: Vec<usize> =
                partition.iter().map(|k| (2 * k + 1) as usize).collect();
            let mut sum = partition.iter().sum::<usize>();

            if sum > partition_value.into() {
                let mut partition_iter = partition.iter_mut();
                while sum > partition_value.into() {
                    match partition_iter.next() {
                        None => partition_iter = partition.iter_mut(),
                        Some(elem) => {
                            if *elem > 1usize {
                                *elem -= 2;
                                sum -= 2;
                            }
                        }
                    }
                }
            } else if sum < partition_value.into() {
                let elem_upper_bound = if max_partition_elem.is_multiple_of(2) {
                    max_partition_elem - 1
                } else {
                    max_partition_elem
                };
                let mut partition_iter = partition.iter_mut();
                while sum < partition_value.into() {
                    match partition_iter.next() {
                        None => partition_iter = partition.iter_mut(),
                        Some(elem) => {
                            if *elem < elem_upper_bound.into() {
                                *elem += 2;
                                sum += 2;
                            }
                        }
                    }
                }
            }

            partition
                .iter()
                .map(|&elem| <usize as TryInto<u16>>::try_into(elem).unwrap())
                .collect()
        })
}

pub fn arbitrary_vector_of_artin_data_with_given_length(
    num_artins: usize,
    max_foot: Option<u16>,
) -> impl Strategy<Value = Vec<(u16, Sign)>> {
    prop::collection::vec(arbitrary_artin_data(max_foot), num_artins..=num_artins)
}

pub fn arbitrary_vector_of_artin_letters_with_given_length(
    num_artins: usize,
    max_foot: Option<u16>,
) -> impl Strategy<Value = Vec<Letter>> {
    prop::collection::vec(arbitrary_artin_generator(max_foot), num_artins..=num_artins)
        .prop_map(|artin_generators| artin_generators.iter().map(|&a| Letter::from(a)).collect())
}

pub fn arbitrary_vector_of_band_data_with_given_artin_length(
    artin_length: u16,
    max_head: Option<u16>,
) -> impl Strategy<Value = Vec<(u16, u16, Sign)>> {
    arbitrary_partition_into_odd_numbers(
        artin_length,
        *[artin_length, 2 * max_head.unwrap_or(u16::MAX) - 3]
            .iter()
            .min()
            .unwrap(),
    )
    .prop_flat_map(move |partition| {
        let mut band_generator_data_strategies = Vec::new();
        for height in partition {
            band_generator_data_strategies
                .push(arbitrary_band_data_with_given_height(height, max_head));
        }
        band_generator_data_strategies
    })
}

pub fn arbitrary_vector_of_band_letters_with_given_artin_length(
    artin_length: u16,
    max_head: Option<u16>,
) -> impl Strategy<Value = Vec<Letter>> {
    arbitrary_partition_into_odd_numbers(
        artin_length,
        *[artin_length, 2 * max_head.unwrap_or(u16::MAX) - 3]
            .iter()
            .min()
            .unwrap(),
    )
    .prop_flat_map(move |partition| {
        let mut bg_strategy_vec = Vec::new();
        for height in partition {
            bg_strategy_vec.push(arbitrary_band_with_given_height(height, max_head));
        }
        bg_strategy_vec
    })
    .prop_map(|band_generators| {
        let band_generators: Vec<BandGenerator> = band_generators;
        band_generators.iter().map(|b| Letter::from(*b)).collect()
    })
}

pub fn arbitrary_vector_of_letter_data_with_given_artin_length(
    artin_length: u16,
    max_head: Option<u16>,
) -> impl Strategy<Value = Vec<(u16, Option<u16>, Sign)>> {
    (0..=artin_length)
        .prop_flat_map(move |num_artins| {
            (
                arbitrary_vector_of_artin_data_with_given_length(
                    num_artins as usize,
                    max_head.map(|h| h - 1),
                )
                .prop_flat_map(|artin_data_vector| {
                    let mut letter_data_strategies = Vec::new();
                    for (foot, sign) in artin_data_vector {
                        letter_data_strategies.push((Just(foot), Just(None), Just(sign)));
                    }
                    letter_data_strategies
                }),
                arbitrary_vector_of_band_data_with_given_artin_length(
                    artin_length - num_artins,
                    max_head,
                )
                .prop_flat_map(|band_data_vector| {
                    let mut letter_data_strategies = Vec::new();
                    for (foot, head, sign) in band_data_vector {
                        letter_data_strategies.push((Just(foot), Just(Some(head)), Just(sign)));
                    }
                    letter_data_strategies
                }),
            )
        })
        .prop_map(|(artin_letters, band_letters)| [artin_letters, band_letters].concat())
        .prop_shuffle()
}

pub fn arbitrary_vector_of_letters_with_given_artin_length(
    artin_length: u16,
    max_head: Option<u16>,
) -> impl Strategy<Value = Vec<Letter>> {
    (0..=artin_length)
        .prop_flat_map(move |num_artins| {
            (
                arbitrary_vector_of_artin_letters_with_given_length(
                    num_artins as usize,
                    max_head.map(|h| h - 1),
                ),
                arbitrary_vector_of_band_letters_with_given_artin_length(
                    artin_length - num_artins,
                    max_head,
                ),
            )
        })
        .prop_map(|(artin_letters, band_letters)| [artin_letters, band_letters].concat())
        .prop_shuffle()
}

pub fn arbitrary_word_with_given_artin_length(
    artin_length: u16,
    max_head: Option<u16>,
) -> impl Strategy<Value = Word> {
    arbitrary_vector_of_letters_with_given_artin_length(artin_length, max_head)
        .prop_map(|letters| Word::try_from_letters(&letters[..]).clone_unwrap())
}

pub fn arbitrary_word_macro_data_single_factor_with_given_head(
    head: u16,
    max_artin_length: Option<u16>,
) -> impl Strategy<Value = (u16, Option<u16>, isize)> {
    (
        Just(head),
        1..=*[head - 1, max_artin_length.unwrap_or(u16::MAX).div_ceil(2)]
            .iter()
            .min()
            .unwrap(),
    )
        .prop_flat_map(move |(head, height)| {
            (
                Just(head),
                Just(height),
                prop_oneof![Just(Sign::Negative), Just(Sign::Positive)],
                1isize
                    ..=(max_artin_length
                        .unwrap_or(u16::MAX)
                        .div_euclid(2 * height - 1) as isize),
            )
                .prop_perturb(|(head, height, sign, exponent), mut rng| {
                    let foot = head - height;
                    let head = if height > 1 || rng.random_bool(0.5) {
                        Some(head)
                    } else {
                        None
                    };
                    match sign {
                        Sign::Negative => (foot, head, -exponent),
                        Sign::Positive => (foot, head, exponent),
                    }
                })
        })
}

pub fn arbitrary_word_macro_data_single_factor(
    max_head: Option<u16>,
    max_artin_length: Option<u16>,
) -> impl Strategy<Value = (u16, Option<u16>, isize)> {
    (2..=max_head.unwrap_or(u16::MAX)).prop_flat_map(move |head| {
        arbitrary_word_macro_data_single_factor_with_given_head(head, max_artin_length)
    })
}

pub fn arbitrary_word_macro_data(
    max_head: Option<u16>,
    max_artin_length: Option<u16>,
) -> impl Strategy<Value = [(u16, Option<u16>, isize); 5]> {
    if max_artin_length.unwrap_or(u16::MAX) < 5 {
        panic!("Max Artin length must be at least 5 to generate this data.")
    }
    if max_head.unwrap_or(u16::MAX) < 2 {
        panic!("Max head must be at least 2 to generate this data.")
    }
    [
        arbitrary_word_macro_data_single_factor(
            max_head,
            Some(max_artin_length.unwrap_or(u16::MAX).div_euclid(5)),
        ),
        arbitrary_word_macro_data_single_factor(
            max_head,
            Some(max_artin_length.unwrap_or(u16::MAX).div_euclid(5)),
        ),
        arbitrary_word_macro_data_single_factor(
            max_head,
            Some(max_artin_length.unwrap_or(u16::MAX).div_euclid(5)),
        ),
        arbitrary_word_macro_data_single_factor(
            max_head,
            Some(max_artin_length.unwrap_or(u16::MAX).div_euclid(5)),
        ),
        arbitrary_word_macro_data_single_factor(
            max_head,
            Some(max_artin_length.unwrap_or(u16::MAX).div_euclid(5)),
        ),
    ]
}

pub fn arbitrary_word_data(
    max_head: Option<u16>,
    max_artin_length: Option<u16>,
) -> impl Strategy<Value = Vec<(u16, Option<u16>, Sign)>> {
    (0..=max_artin_length.unwrap_or(u16::MAX)).prop_flat_map(move |artin_length| {
        arbitrary_vector_of_letter_data_with_given_artin_length(artin_length, max_head)
    })
}

pub fn arbitrary_word(
    max_head: Option<u16>,
    max_artin_length: Option<u16>,
) -> impl Strategy<Value = Word> {
    (0..=max_artin_length.unwrap_or(u16::MAX)).prop_flat_map(move |artin_length| {
        arbitrary_word_with_given_artin_length(artin_length, max_head)
    })
}
