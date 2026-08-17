use crate::arbitrary::valid;
use braided::{Sign, Word};
use proptest::prelude::*;

pub fn with_given_artin_length(
    artin_length: u16,
    max_head: Option<u16>,
) -> impl Strategy<Value = Word> {
    valid::letter::vector_of_letters_with_given_artin_length(artin_length, max_head)
        .prop_map(|letters| Word::try_from_letters(&letters[..]).clone_unwrap())
}

pub fn data_where_single_letter_has_given_head(
    head: u16,
    max_height: Option<u16>,
    max_artin_length: Option<u16>,
) -> impl Strategy<Value = Vec<(valid::u16::Data, Option<valid::u16::Data>, Sign)>> {
    if head < 3 {
        panic!("Head must be at least 3 to generate the appropriate data.");
    }

    valid::letter::data_with_given_head(head, max_height, max_artin_length)
        .prop_flat_map(move |(foot_idx, head_idx, sign)| {
            let height: u16 =
                head - <valid::u16::Data as TryInto<u16>>::try_into(foot_idx).unwrap();
            let max_artin_length = max_artin_length.map(|max| max - (2 * height - 1));
            (
                Just((foot_idx, head_idx, sign)),
                valid::word::data(Some(head - 1), max_artin_length),
            )
        })
        .prop_map(|(fixed_head_letter, word_data)| [vec![fixed_head_letter], word_data].concat())
        .prop_shuffle()
}

pub fn macro_data_where_one_factor_has_given_head(
    head: u16,
    max_artin_length: Option<u16>,
) -> impl Strategy<Value = (valid::u16::Data, Option<valid::u16::Data>, isize)> {
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
                .prop_map(|(head, height, sign, exponent)| {
                    let foot = head - height;
                    match sign {
                        Sign::Negative => (height, foot, head, -exponent),
                        Sign::Positive => (height, foot, head, exponent),
                    }
                })
        })
        .prop_flat_map(|(height, foot, head, exponent)| {
            (
                Just(height),
                valid::u16::data(Some(foot), Some(foot)),
                valid::u16::data(Some(head), Some(head)),
                Just(exponent),
            )
        })
        .prop_perturb(|(height, foot, head, exponent), mut rng| {
            if height > 1 || rng.random_bool(0.5) {
                (foot, Some(head), exponent)
            } else {
                (foot, None, exponent)
            }
        })
}

pub fn macro_data_with_single_factor(
    max_head: Option<u16>,
    max_artin_length: Option<u16>,
) -> impl Strategy<Value = (valid::u16::Data, Option<valid::u16::Data>, isize)> {
    (2..=max_head.unwrap_or(u16::MAX)).prop_flat_map(move |head| {
        macro_data_where_one_factor_has_given_head(head, max_artin_length)
    })
}

pub fn macro_data(
    max_head: Option<u16>,
    max_artin_length: Option<u16>,
) -> impl Strategy<Value = [(valid::u16::Data, Option<valid::u16::Data>, isize); 5]> {
    if max_artin_length.unwrap_or(u16::MAX) < 5 {
        panic!("Max Artin length must be at least 5 to generate this data.")
    }
    if max_head.unwrap_or(u16::MAX) < 2 {
        panic!("Max head must be at least 2 to generate this data.")
    }
    [
        macro_data_with_single_factor(
            max_head,
            Some(max_artin_length.unwrap_or(u16::MAX).div_euclid(5)),
        ),
        macro_data_with_single_factor(
            max_head,
            Some(max_artin_length.unwrap_or(u16::MAX).div_euclid(5)),
        ),
        macro_data_with_single_factor(
            max_head,
            Some(max_artin_length.unwrap_or(u16::MAX).div_euclid(5)),
        ),
        macro_data_with_single_factor(
            max_head,
            Some(max_artin_length.unwrap_or(u16::MAX).div_euclid(5)),
        ),
        macro_data_with_single_factor(
            max_head,
            Some(max_artin_length.unwrap_or(u16::MAX).div_euclid(5)),
        ),
    ]
}

pub fn data(
    max_head: Option<u16>,
    max_artin_length: Option<u16>,
) -> impl Strategy<Value = Vec<(valid::u16::Data, Option<valid::u16::Data>, Sign)>> {
    (0..=max_artin_length.unwrap_or(u16::MAX)).prop_flat_map(move |artin_length| {
        valid::letter::vector_of_data_with_given_artin_length(artin_length, max_head)
    })
}

pub fn new(max_head: Option<u16>, max_artin_length: Option<u16>) -> impl Strategy<Value = Word> {
    (0..=max_artin_length.unwrap_or(u16::MAX))
        .prop_flat_map(move |artin_length| with_given_artin_length(artin_length, max_head))
}
