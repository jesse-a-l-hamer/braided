use crate::arbitrary::valid;
use braided::{ArtinGenerator, BandGenerator, Letter, Sign, Strand};
use proptest::prelude::*;

pub fn data_with_given_height(
    height: u16,
    max_head: Option<u16>,
) -> impl Strategy<Value = (valid::u16::Data, valid::u16::Data, Sign)> {
    if let Some(max_head) = max_head
        && max_head < height + 1
    {
        panic!("max_head must be at least height + 1.")
    }
    let min_head = 1 + height;
    (
        min_head..=max_head.unwrap_or(u16::MAX),
        prop_oneof![Just(Sign::Positive), Just(Sign::Negative)],
    )
        .prop_flat_map(move |(head_idx, sign)| {
            (
                valid::u16::data(Some(head_idx - height), Some(head_idx - height)),
                valid::u16::data(Some(head_idx), Some(head_idx)),
                Just(sign),
            )
        })
}

pub fn data(
    max_head: Option<u16>,
    max_height: Option<u16>,
    max_artin_length: Option<u16>,
) -> impl Strategy<Value = (valid::u16::Data, valid::u16::Data, Sign)> {
    if let Some(max_head) = max_head
        && max_head < 2
    {
        panic!("max_head must be at least 2.")
    }
    if let Some(max_artin_length) = max_artin_length
        && max_artin_length < 1
    {
        panic!("max_artin_length must be at least 1.")
    }
    (2u16..=max_head.unwrap_or(u16::MAX))
        .prop_flat_map(move |head_idx| {
            let max_height: u16 = *[
                head_idx - 1,
                max_height.unwrap_or(u16::MAX.div_ceil(2)),
                max_artin_length.unwrap_or(u16::MAX).div_ceil(2),
            ]
            .iter()
            .min()
            .unwrap();
            (Just(head_idx), 1u16..=max_height)
        })
        .prop_flat_map(|(head_idx, height)| {
            (
                valid::u16::data(Some(head_idx - height), Some(head_idx - height)),
                valid::u16::data(Some(head_idx), Some(head_idx)),
                prop_oneof![Just(Sign::Positive), Just(Sign::Negative)],
            )
        })
}

pub fn with_given_height(
    height: u16,
    max_head: Option<u16>,
) -> impl Strategy<Value = BandGenerator> {
    if let Some(max_head) = max_head
        && max_head < 2
    {
        panic!("max_head must be at least 2.")
    }
    data_with_given_height(height, max_head)
        .prop_map(|(foot, head, sign)| BandGenerator::try_new(foot, head, sign).unwrap())
}

pub fn data_with_given_head(
    head: u16,
    max_height: Option<u16>,
    max_artin_length: Option<u16>,
) -> impl Strategy<Value = (valid::u16::Data, valid::u16::Data, Sign)> {
    if head < 2 {
        panic!("Head index must be at least 2.");
    }
    let max_height = *[
        head - 1,
        max_height.unwrap_or(u16::MAX.div_ceil(2)),
        max_artin_length.unwrap_or(u16::MAX).div_ceil(2),
    ]
    .iter()
    .min()
    .unwrap();

    (
        Just(head),
        1..=max_height,
        prop_oneof![Just(Sign::Negative), Just(Sign::Positive)],
    )
        .prop_flat_map(|(head, height, sign)| {
            (
                valid::u16::data(Some(head - height), Some(head - height)),
                valid::u16::data(Some(head), Some(head)),
                Just(sign),
            )
        })
}

pub fn new(
    max_head: Option<u16>,
    max_height: Option<u16>,
    max_artin_length: Option<u16>,
) -> impl Strategy<Value = BandGenerator> {
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
        .prop_map(|(foot, head, sign)| BandGenerator::try_new(foot, head, sign).unwrap())
}

pub fn vector_of_band_data_with_given_artin_length(
    artin_length: u16,
    max_head: Option<u16>,
) -> impl Strategy<Value = Vec<(valid::u16::Data, valid::u16::Data, Sign)>> {
    if let Some(max_head) = max_head
        && max_head < 2
        && 0 < artin_length
    {
        panic!("max_head must be at least 2.")
    }
    let max_band_artin_length = max_head
        .map(|h| 2 * (h as usize).max(2) - 3)
        .unwrap_or(artin_length as usize);
    crate::arbitrary::utils::partition_into_odd_numbers(artin_length, max_band_artin_length)
        .prop_flat_map(move |partition| {
            let mut band_generator_data_strategies = Vec::new();
            for band_artin_length in partition {
                let height = band_artin_length.div_ceil(2);
                if let Some(max_head) = max_head
                    && max_head < height + 1
                {
                    panic!("max_head = {max_head} < {} = height + 1", height + 1)
                }
                band_generator_data_strategies
                    .push(valid::band::data_with_given_height(height, max_head));
            }
            band_generator_data_strategies
        })
}

pub mod test_cases {
    use braided::BraidIndex;

    use super::*;

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct TryNewData {
        pub foot: valid::u16::Data,
        pub head: valid::u16::Data,
        pub sign: Sign,
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct FromArtinData(pub ArtinGenerator);

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct FromLetterData(pub Letter);

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct InverseData(pub BandGenerator);

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct TryNew {
        pub data: TryNewData,
        pub expected_foot: Strand,
        pub expected_head: Strand,
        pub expected_sign: Sign,
        pub expected_height: u16,
        pub expected_is_artin: bool,
        pub expected_minimal_required_braid_index: BraidIndex,
        pub expected_artin_length: u16,
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct FromArtin {
        pub data: FromArtinData,
        pub expected: BandGenerator,
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct FromLetter {
        pub data: FromLetterData,
        pub expected: BandGenerator,
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct Inverse {
        pub data: InverseData,
        pub expected: BandGenerator,
    }

    pub fn try_new(
        max_head: Option<u16>,
        max_height: Option<u16>,
        max_artin_length: Option<u16>,
    ) -> impl Strategy<Value = TryNew> {
        data(max_head, max_height, max_artin_length).prop_map(|(foot, head, sign)| {
            let expected_foot = Strand::try_new(foot).unwrap();
            let expected_head = Strand::try_new(head).unwrap();
            let expected_sign = sign;
            let expected_height: u16 = (expected_head - expected_foot).unwrap().into();
            let expected_is_artin = expected_height == 1;
            let expected_minimal_required_braid_index = BraidIndex::try_new(expected_head).unwrap();
            let expected_artin_length = 2 * expected_height - 1;
            TryNew {
                data: TryNewData { foot, head, sign },
                expected_foot,
                expected_head,
                expected_sign,
                expected_height,
                expected_is_artin,
                expected_minimal_required_braid_index,
                expected_artin_length,
            }
        })
    }

    pub fn from_artin(max_head: Option<u16>) -> impl Strategy<Value = FromArtin> {
        valid::artin::new(None, max_head.map(|h| h - 1)).prop_map(|artin_generator| FromArtin {
            data: FromArtinData(artin_generator),
            expected: BandGenerator::try_new(
                artin_generator.foot(),
                artin_generator.head(),
                artin_generator.sign(),
            )
            .unwrap(),
        })
    }

    pub fn from_letter(
        max_head: Option<u16>,
        max_height: Option<u16>,
        max_artin_length: Option<u16>,
    ) -> impl Strategy<Value = FromLetter> {
        valid::letter::new(max_head, max_height, max_artin_length).prop_map(|letter| FromLetter {
            data: FromLetterData(letter),
            expected: BandGenerator::try_new(letter.foot(), letter.head(), letter.sign()).unwrap(),
        })
    }

    pub fn inverse(
        max_head: Option<u16>,
        max_height: Option<u16>,
        max_artin_length: Option<u16>,
    ) -> impl Strategy<Value = Inverse> {
        data(max_head, max_height, max_artin_length).prop_map(|(foot, head, sign)| Inverse {
            data: InverseData(BandGenerator::try_new(foot, head, sign).unwrap()),
            expected: BandGenerator::try_new(foot, head, -sign).unwrap(),
        })
    }
}
