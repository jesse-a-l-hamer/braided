use crate::arbitrary::valid;
use braided::{Letter, Sign};
use proptest::prelude::*;

pub fn data(
    max_head: Option<u16>,
    max_height: Option<u16>,
    max_artin_length: Option<u16>,
) -> impl Strategy<Value = (valid::u16::Data, Option<valid::u16::Data>, Sign)> {
    prop_oneof![
        valid::artin::data(None, max_head.map(|h| h - 1))
            .prop_map(|(foot_idx, sign)| (foot_idx, None, sign)),
        valid::band::data(max_head, max_height, max_artin_length)
            .prop_map(|(foot_idx, head_idx, sign)| (foot_idx, Some(head_idx), sign)),
    ]
}

pub fn new(
    max_head: Option<u16>,
    max_height: Option<u16>,
    max_artin_length: Option<u16>,
) -> impl Strategy<Value = Letter> {
    data(max_head, max_height, max_artin_length)
        .prop_map(|(foot, head, sign)| Letter::try_new(foot, head, sign).unwrap())
}

pub fn data_with_given_head(
    head: u16,
    max_height: Option<u16>,
    max_artin_length: Option<u16>,
) -> impl Strategy<Value = (valid::u16::Data, Option<valid::u16::Data>, Sign)> {
    prop_oneof![
        valid::artin::data_with_given_head(head).prop_map(|(foot, sign)| (foot, None, sign)),
        valid::band::data_with_given_head(head, max_height, max_artin_length)
            .prop_map(|(foot, head, sign)| (foot, Some(head), sign))
    ]
}

pub fn with_given_head(
    head: u16,
    max_height: Option<u16>,
    max_artin_length: Option<u16>,
) -> impl Strategy<Value = Letter> {
    data_with_given_head(head, max_height, max_artin_length)
        .prop_map(|(foot, head, sign)| Letter::try_new(foot, head, sign).unwrap())
}

pub fn vector_with_given_length_artin_only(
    num_artins: usize,
    max_foot: Option<u16>,
) -> impl Strategy<Value = Vec<Letter>> {
    prop::collection::vec(valid::artin::new(None, max_foot), num_artins..=num_artins)
        .prop_map(|artin_generators| artin_generators.iter().map(|&a| Letter::from(a)).collect())
}

pub fn vector_with_given_artin_length_band_only(
    artin_length: u16,
    max_head: Option<u16>,
) -> impl Strategy<Value = Vec<Letter>> {
    crate::arbitrary::utils::partition_into_odd_numbers(
        artin_length,
        *[artin_length, 2 * max_head.unwrap_or(u16::MAX) - 3]
            .iter()
            .min()
            .unwrap(),
    )
    .prop_flat_map(move |partition| {
        let mut bg_strategy_vec = Vec::new();
        for height in partition {
            bg_strategy_vec.push(valid::band::with_given_height(height, max_head));
        }
        bg_strategy_vec
    })
    .prop_map(|band_generators| band_generators.iter().map(|b| Letter::from(*b)).collect())
}

pub fn vector_of_data_with_given_artin_length(
    artin_length: u16,
    max_head: Option<u16>,
) -> impl Strategy<Value = Vec<(valid::u16::Data, Option<valid::u16::Data>, Sign)>> {
    (0..=artin_length)
        .prop_flat_map(move |num_artins| {
            (
                valid::artin::vector_of_data_with_given_length(
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
                valid::band::vector_of_band_data_with_given_artin_length(
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

pub fn vector_of_letters_with_given_artin_length(
    artin_length: u16,
    max_head: Option<u16>,
) -> impl Strategy<Value = Vec<Letter>> {
    (0..=artin_length)
        .prop_flat_map(move |num_artins| {
            (
                valid::letter::vector_with_given_length_artin_only(
                    num_artins as usize,
                    max_head.map(|h| h - 1),
                ),
                valid::letter::vector_with_given_artin_length_band_only(
                    artin_length - num_artins,
                    max_head,
                ),
            )
        })
        .prop_map(|(artin_letters, band_letters)| [artin_letters, band_letters].concat())
        .prop_shuffle()
}
