use crate::arbitrary::valid;
use braided::{ArtinGenerator, ArtinResult, BandGenerator, Letter, Sign, Strand};
use proptest::prelude::*;

pub fn data(
    min_foot: Option<u16>,
    max_foot: Option<u16>,
) -> impl Strategy<Value = (valid::u16::Data, Sign)> {
    (
        valid::u16::data(min_foot, Some(max_foot.unwrap_or(u16::MAX - 1))),
        prop_oneof![Just(Sign::Positive), Just(Sign::Negative)],
    )
}

pub fn new(min_foot: Option<u16>, max_foot: Option<u16>) -> impl Strategy<Value = ArtinGenerator> {
    data(min_foot, max_foot).prop_map(|(foot, sign)| ArtinGenerator::try_new(foot, sign).unwrap())
}

pub fn data_with_given_head(head: u16) -> impl Strategy<Value = (valid::u16::Data, Sign)> {
    if head < 2 {
        panic!("Head index must be at least 2.");
    }

    (
        valid::u16::data(Some(head - 1), Some(head - 1)),
        prop_oneof![Just(Sign::Positive), Just(Sign::Negative)],
    )
}

pub fn with_given_head(head: u16) -> impl Strategy<Value = ArtinGenerator> {
    data_with_given_head(head).prop_map(|(foot, sign)| ArtinGenerator::try_new(foot, sign).unwrap())
}

pub fn vector_of_data_with_given_length(
    num_artins: usize,
    max_foot: Option<u16>,
) -> impl Strategy<Value = Vec<(valid::u16::Data, Sign)>> {
    prop::collection::vec(valid::artin::data(None, max_foot), num_artins..=num_artins)
}

pub mod test_cases {
    use super::*;

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct TryNewData {
        pub foot: valid::u16::Data,
        pub sign: Sign,
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct TryNew {
        pub data: TryNewData,
        pub expected_foot: Strand,
        pub expected_sign: Sign,
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct TryFromBandData {
        pub band: BandGenerator,
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct TryFromBand {
        pub data: TryFromBandData,
        pub expected: ArtinResult,
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct TryFromLetterData {
        pub letter: Letter,
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct TryFromLetter {
        pub data: TryFromLetterData,
        pub expected: ArtinResult,
    }

    pub fn try_new(min_foot: Option<u16>, max_foot: Option<u16>) -> impl Strategy<Value = TryNew> {
        data(min_foot, max_foot).prop_map(|(valid_u16_data, sign)| TryNew {
            data: TryNewData {
                foot: valid_u16_data,
                sign,
            },
            expected_foot: Strand::try_new(valid_u16_data).unwrap(),
            expected_sign: sign,
        })
    }

    pub fn try_from_band(max_head: Option<u16>) -> impl Strategy<Value = TryFromBand> {
        valid::band::new(max_head, Some(1), Some(1)).prop_map(|band| TryFromBand {
            data: TryFromBandData { band },
            expected: ArtinGenerator::try_new(band.foot(), band.sign()),
        })
    }

    pub fn try_from_letter(max_head: Option<u16>) -> impl Strategy<Value = TryFromLetter> {
        valid::letter::new(max_head, Some(1), Some(1)).prop_map(|letter| TryFromLetter {
            data: TryFromLetterData { letter },
            expected: ArtinGenerator::try_new(letter.foot(), letter.sign()),
        })
    }
}
