use crate::arbitrary::valid;
use braided::{Letter, Sign};
use proptest::prelude::*;

pub fn data(
    max_head: Option<u16>,
    max_height: Option<u16>,
    max_artin_length: Option<u16>,
) -> impl Strategy<Value = (valid::u16::Data, Option<valid::u16::Data>, Sign)> {
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
    prop_oneof![
        valid::artin::data(None, max_head.map(|h| h - 1))
            .prop_map(|(foot_idx, sign)| (foot_idx, None, sign)),
        valid::band::data(max_head, max_height, max_artin_length)
            .prop_map(|(foot_idx, head_idx, sign)| (foot_idx, Some(head_idx), sign)),
    ]
}

pub fn data_with_given_head(
    head: u16,
    max_height: Option<u16>,
    max_artin_length: Option<u16>,
) -> impl Strategy<Value = (valid::u16::Data, Option<valid::u16::Data>, Sign)> {
    if head < 2 {
        panic!("head must be at least 2 to generate this data.");
    }
    prop_oneof![
        valid::artin::data_with_given_head(head).prop_map(|(foot, sign)| (foot, None, sign)),
        valid::band::data_with_given_head(head, max_height, max_artin_length)
            .prop_map(|(foot, head, sign)| (foot, Some(head), sign))
    ]
}

pub fn data_with_given_height(
    max_head: Option<u16>,
    height: u16,
) -> impl Strategy<Value = (valid::u16::Data, Option<valid::u16::Data>, Sign)> {
    if let Some(max_head) = max_head
        && max_head < 2
    {
        panic!("max_head must be at least 2.")
    }
    if height == 1 {
        prop_oneof![
            3 => valid::artin::data(None, max_head.map(|h| h - 1))
                .prop_map(|(foot_idx, sign)| (foot_idx, None, sign)),
            1 => valid::band::data_with_given_height(1, max_head).prop_map(|(foot_idx, head_idx, sign)| (foot_idx, Some(head_idx), sign)),
        ].boxed()
    } else {
        valid::band::data_with_given_height(height, max_head)
            .prop_map(|(foot_idx, head_idx, sign)| (foot_idx, Some(head_idx), sign))
            .boxed()
    }
}

pub fn new(
    max_head: Option<u16>,
    max_height: Option<u16>,
    max_artin_length: Option<u16>,
) -> impl Strategy<Value = Letter> {
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
    data(max_head, max_height, max_artin_length)
        .prop_map(|(foot, head, sign)| Letter::try_new(foot, head, sign).unwrap())
}

pub fn with_given_head(
    head: u16,
    max_height: Option<u16>,
    max_artin_length: Option<u16>,
) -> impl Strategy<Value = Letter> {
    if head < 2 {
        panic!("head must be at least 2 to generate this data.");
    }
    data_with_given_head(head, max_height, max_artin_length)
        .prop_map(|(foot, head, sign)| Letter::try_new(foot, head, sign).unwrap())
}

pub fn vector_with_given_length_artin_only(
    num_generators: usize,
    max_foot: Option<u16>,
) -> impl Strategy<Value = Vec<Letter>> {
    if let Some(max_foot) = max_foot
        && max_foot < 1
        && 0 < num_generators
    {
        panic!("max_foot must be at least 1.");
    }
    let mut artin_generator_strategies = Vec::new();
    for _ in 0..num_generators {
        artin_generator_strategies.push(valid::artin::new(None, max_foot));
    }
    artin_generator_strategies
        .prop_map(|artin_generators| artin_generators.iter().map(|&a| Letter::from(a)).collect())
}

pub fn vector_with_given_artin_length_band_only(
    artin_length: u16,
    max_head: Option<u16>,
) -> impl Strategy<Value = Vec<Letter>> {
    if let Some(max_head) = max_head
        && max_head < 2
        && 0 < artin_length
    {
        panic!("max_head must be at least 2.")
    }
    if artin_length == 0 {
        return Just(Vec::new()).boxed();
    }
    crate::arbitrary::utils::partition_into_odd_numbers(
        artin_length,
        (*[
            artin_length as usize,
            2usize * (max_head.unwrap_or(u16::MAX) as usize) - 3,
        ]
        .iter()
        .min()
        .unwrap())
        .try_into()
        .unwrap(),
    )
    .prop_flat_map(move |partition| {
        let mut band_generator_strategies = Vec::new();
        for artin_length in partition {
            let height = artin_length.div_ceil(2);
            band_generator_strategies.push(valid::band::with_given_height(height, max_head));
        }
        band_generator_strategies
    })
    .prop_map(|band_generators| band_generators.iter().map(|b| Letter::from(*b)).collect())
    .boxed()
}

pub fn vector_of_data_with_given_artin_length(
    artin_length: u16,
    max_head: Option<u16>,
) -> impl Strategy<Value = Vec<(valid::u16::Data, Option<valid::u16::Data>, Sign)>> {
    if let Some(max_head) = max_head
        && max_head < 2
        && 0 < artin_length
    {
        panic!("max_head must be at least 2.")
    }
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

pub fn vector_with_given_artin_length(
    artin_length: u16,
    max_head: Option<u16>,
) -> impl Strategy<Value = Vec<Letter>> {
    if let Some(max_head) = max_head
        && max_head < 2
        && 0 < artin_length
    {
        panic!("max_head must be at least 2.")
    }
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

pub fn vector(
    max_head: Option<u16>,
    max_artin_length: Option<u16>,
) -> impl Strategy<Value = Vec<Letter>> {
    if let Some(max_head) = max_head
        && max_head < 2
        && max_artin_length.is_none_or(|artin_length| 0 < artin_length)
    {
        panic!("max_head must be at least 2.")
    }
    (0..=max_artin_length.unwrap_or(u16::MAX))
        .prop_flat_map(move |artin_length| vector_with_given_artin_length(artin_length, max_head))
}

pub fn equal_pair(
    max_head: Option<u16>,
    max_height: Option<u16>,
    max_artin_length: Option<u16>,
) -> impl Strategy<Value = (Letter, Letter)> {
    if let Some(max_head) = max_head
        && max_head < 2
    {
        panic!("max_head must be at least 2.")
    }
    data(max_head, max_height, max_artin_length).prop_perturb(|(foot, head, sign), mut rng| {
        let first = Letter::try_new(foot, head, sign).unwrap();
        let second: Letter = if head.is_none() && rng.random_bool(0.5) {
            Letter::try_new(foot, Some(foot + 1u16), sign).unwrap()
        } else {
            Letter::try_new(foot, head, sign).unwrap()
        };

        if rng.random_bool(0.5) {
            (first, second)
        } else {
            (second, first)
        }
    })
}

pub fn unequal_pair(
    max_head: Option<u16>,
    max_height: Option<u16>,
    max_artin_length: Option<u16>,
) -> impl Strategy<Value = (Letter, Letter)> {
    if let Some(max_head) = max_head
        && max_head < 2
    {
        panic!("max_head must be at least 2.")
    }
    (
        data(max_head, max_height, max_artin_length),
        data(max_head, max_height, max_artin_length),
    )
        .prop_filter_map(
            "Data from this branch must produce unequal letters.",
            |(left, right)| {
                let left_foot: u16 = left.0.try_into().unwrap();
                let left_head: u16 = left
                    .1
                    .map(|h| h.try_into().unwrap())
                    .unwrap_or(left_foot + 1);
                let right_foot: u16 = right.0.try_into().unwrap();
                let right_head: u16 = right
                    .1
                    .map(|h| h.try_into().unwrap())
                    .unwrap_or(right_foot + 1);

                if left_foot == right_foot && left_head == right_head && left.2 == right.2 {
                    None
                } else {
                    Some((
                        Letter::try_new(left.0, left.1, left.2).unwrap(),
                        Letter::try_new(right.0, right.1, right.2).unwrap(),
                    ))
                }
            },
        )
}

pub mod test_cases {
    use super::*;
    use braided::{ArtinGenerator, BandGenerator, BraidIndex, LetterResult, Strand};

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct TryNewData {
        pub foot: valid::u16::Data,
        pub head: Option<valid::u16::Data>,
        pub sign: Sign,
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum FromData {
        ArtinGenerator(ArtinGenerator),
        BandGenerator(BandGenerator),
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct MacroData {
        pub foot: valid::u16::Data,
        pub head: Option<valid::u16::Data>,
        pub sign: Sign,
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct EqualityData {
        pub left: Letter,
        pub right: Letter,
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct InverseData(pub Letter);

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct TryNew {
        pub data: TryNewData,
        pub expected_foot: Strand,
        pub expected_head: Strand,
        pub expected_sign: Sign,
        pub expected_is_artin: bool,
        pub expected_height: u16,
        pub expected_artin_length: u16,
        pub expected_minimal_required_braid_index: BraidIndex,
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct From {
        pub data: FromData,
        pub expected: Letter,
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct Macro {
        pub data: MacroData,
        pub expected: LetterResult,
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct Equality {
        pub data: EqualityData,
        pub expected: bool,
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct Inverse {
        pub data: InverseData,
        pub expected: Letter,
    }

    pub fn try_new(
        max_head: Option<u16>,
        max_height: Option<u16>,
        max_artin_length: Option<u16>,
    ) -> impl Strategy<Value = TryNew> {
        data(max_head, max_height, max_artin_length).prop_map(|(foot, head, sign)| {
            let expected_foot = Strand::try_new(foot).unwrap();
            let expected_head = Strand::try_new(head.unwrap_or(foot + 1u16)).unwrap();
            let expected_sign = sign;
            let expected_height: u16 = (expected_head - expected_foot).unwrap().into();
            let expected_is_artin = expected_height == 1;
            let expected_artin_length = 2 * expected_height - 1;
            let expected_minimal_required_braid_index = BraidIndex::try_new(expected_head).unwrap();
            TryNew {
                data: TryNewData { foot, head, sign },
                expected_foot,
                expected_head,
                expected_sign,
                expected_is_artin,
                expected_height,
                expected_artin_length,
                expected_minimal_required_braid_index,
            }
        })
    }

    pub fn from(
        max_head: Option<u16>,
        max_height: Option<u16>,
        max_artin_length: Option<u16>,
    ) -> impl Strategy<Value = From> {
        prop_oneof![
            valid::artin::new(None, max_head.map(|h| h - 1)).prop_map(|artin_generator| From {
                data: FromData::ArtinGenerator(artin_generator),
                expected: Letter::try_new(
                    artin_generator.foot(),
                    None::<u16>,
                    artin_generator.sign()
                )
                .unwrap()
            }),
            valid::band::new(max_head, max_height, max_artin_length).prop_map(|band_generator| {
                From {
                    data: FromData::BandGenerator(band_generator),
                    expected: Letter::try_new(
                        band_generator.foot(),
                        Some(band_generator.head()),
                        band_generator.sign(),
                    )
                    .unwrap(),
                }
            }),
        ]
    }

    pub fn letter_macro(
        max_head: Option<u16>,
        max_height: Option<u16>,
        max_artin_length: Option<u16>,
    ) -> impl Strategy<Value = Macro> {
        data(max_head, max_height, max_artin_length).prop_map(|(foot, head, sign)| Macro {
            data: MacroData { foot, head, sign },
            expected: Letter::try_new(foot, head, sign),
        })
    }

    pub fn equality(
        max_head: Option<u16>,
        max_height: Option<u16>,
        max_artin_length: Option<u16>,
    ) -> impl Strategy<Value = Equality> {
        prop_oneof![
            unequal_pair(max_head, max_height, max_artin_length).prop_map(|(left, right)| {
                Equality {
                    data: EqualityData { left, right },
                    expected: false,
                }
            }),
            equal_pair(max_head, max_height, max_artin_length).prop_map(|(left, right)| Equality {
                data: EqualityData { left, right },
                expected: true,
            }),
        ]
    }

    pub fn inverse(
        max_head: Option<u16>,
        max_height: Option<u16>,
        max_artin_length: Option<u16>,
    ) -> impl Strategy<Value = Inverse> {
        data(max_head, max_height, max_artin_length).prop_map(|(foot, head, sign)| Inverse {
            data: InverseData(Letter::try_new(foot, head, sign).unwrap()),
            expected: Letter::try_new(foot, head, -sign).unwrap(),
        })
    }
}
