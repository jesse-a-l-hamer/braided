use crate::arbitrary::valid;
use braided::{ArtinGenerator, BandGenerator, Letter, Sign, Strand};
use proptest::prelude::*;

pub fn data_with_given_height(
    height: u16,
    max_head: Option<u16>,
) -> impl Strategy<Value = (valid::u16::Data, valid::u16::Data, Sign)> {
    let min_head = 1 + height.div_ceil(2);
    let max_head = [height + 1, max_head.unwrap_or(u16::MAX)]
        .iter()
        .min()
        .cloned();
    (
        min_head..=max_head.unwrap(),
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
    (2u16..max_head.unwrap_or(u16::MAX))
        .prop_flat_map(move |head_idx| {
            (
                Just(head_idx),
                1u16..=*[
                    head_idx - 1,
                    max_height.unwrap_or(u16::MAX.div_ceil(2)),
                    max_artin_length.unwrap_or(u16::MAX).div_ceil(2),
                ]
                .iter()
                .min()
                .unwrap(),
            )
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
    data(max_head, max_height, max_artin_length)
        .prop_map(|(foot, head, sign)| BandGenerator::try_new(foot, head, sign).unwrap())
}

pub fn vector_of_band_data_with_given_artin_length(
    artin_length: u16,
    max_head: Option<u16>,
) -> impl Strategy<Value = Vec<(valid::u16::Data, valid::u16::Data, Sign)>> {
    crate::arbitrary::utils::partition_into_odd_numbers(
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
                .push(valid::band::data_with_given_height(height, max_head));
        }
        band_generator_data_strategies
    })
}

pub mod test_cases {
    use braided::BandResult;

    use super::*;

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct TryNewData {
        pub foot: valid::u16::Data,
        pub head: valid::u16::Data,
        pub sign: Sign,
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct TryNew {
        pub data: TryNewData,
        pub expected_foot: Strand,
        pub expected_head: Strand,
        pub expected_sign: Sign,
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct FromArtinData {
        pub artin_generator: ArtinGenerator,
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct FromArtin {
        pub data: FromArtinData,
        pub expected: BandGenerator,
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct FromLetterData {
        pub letter: Letter,
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct FromLetter {
        pub data: FromLetterData,
        pub expected: BandGenerator,
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct CoalesceData {
        pub artin_generators: Vec<ArtinGenerator>,
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct Coalesce {
        pub data: CoalesceData,
        pub expected: BandResult,
    }

    pub fn try_new(
        max_head: Option<u16>,
        max_height: Option<u16>,
        max_artin_length: Option<u16>,
    ) -> impl Strategy<Value = TryNew> {
        data(max_head, max_height, max_artin_length).prop_map(|(foot, head, sign)| TryNew {
            data: TryNewData { foot, head, sign },
            expected_foot: Strand::try_new(foot).unwrap(),
            expected_head: Strand::try_new(head).unwrap(),
            expected_sign: sign,
        })
    }

    pub fn from_artin(max_head: Option<u16>) -> impl Strategy<Value = FromArtin> {
        valid::artin::new(None, max_head.map(|h| h - 1)).prop_map(|artin_generator| FromArtin {
            data: FromArtinData { artin_generator },
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
            data: FromLetterData { letter },
            expected: BandGenerator::try_new(letter.foot(), letter.head(), letter.sign()).unwrap(),
        })
    }

    pub fn coalesce(
        max_head: Option<u16>,
        max_height: Option<u16>,
        max_artin_length: Option<u16>,
    ) -> impl Strategy<Value = Coalesce> {
        valid::coalescence::of_band_generator(max_head, max_height, max_artin_length).prop_map(
            |(band_generator, artin_generators)| Coalesce {
                data: CoalesceData { artin_generators },
                expected: band_generator,
            },
        )
    }
}
