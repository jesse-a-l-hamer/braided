use crate::arbitrary::valid;
use braided::{ArtinGenerator, ArtinResult, BandGenerator, Letter, Sign, Strand};
use proptest::prelude::*;

pub fn data(
    min_foot: Option<u16>,
    max_foot: Option<u16>,
) -> impl Strategy<Value = (valid::u16::Data, Sign)> {
    if let Some(min) = min_foot
        && min == 0
    {
        panic!("min_foot must be positive to generate this data.");
    }
    if let (Some(min), Some(max)) = (min_foot, max_foot)
        && min > max
    {
        panic!("min_foot may be no larger than max_foot.");
    }
    (
        valid::u16::data(min_foot, Some(max_foot.unwrap_or(u16::MAX - 1))),
        prop_oneof![Just(Sign::Positive), Just(Sign::Negative)],
    )
}

pub fn new(min_foot: Option<u16>, max_foot: Option<u16>) -> impl Strategy<Value = ArtinGenerator> {
    if let Some(min) = min_foot
        && min == 0
    {
        panic!("min_foot must be positive to generate this data.");
    }
    if let (Some(min), Some(max)) = (min_foot, max_foot)
        && min > max
    {
        panic!("min_foot may be no larger than max_foot.");
    }
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
    if head < 2 {
        panic!("Head index must be at least 2.");
    }

    data_with_given_head(head).prop_map(|(foot, sign)| ArtinGenerator::try_new(foot, sign).unwrap())
}

pub fn vector_of_data_with_given_length(
    num_generators: usize,
    max_foot: Option<u16>,
) -> impl Strategy<Value = Vec<(valid::u16::Data, Sign)>> {
    if let Some(max_foot) = max_foot
        && max_foot == 0
        && 0 < num_generators
    {
        panic!("max_foot must be positive to generate this data.");
    }
    prop::collection::vec(
        valid::artin::data(None, max_foot),
        num_generators..=num_generators,
    )
}

pub mod test_cases {
    use braided::BraidIndex;

    use super::*;

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct TryNewData {
        pub foot: valid::u16::Data,
        pub sign: Sign,
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct TryFromBandData(pub BandGenerator);

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct TryFromLetterData(pub Letter);

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct InverseData(pub ArtinGenerator);

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct TryNew {
        pub data: TryNewData,
        pub expected_foot: Strand,
        pub expected_head: Strand,
        pub expected_sign: Sign,
        pub expected_minimal_required_braid_index: BraidIndex,
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct TryFromBand {
        pub data: TryFromBandData,
        pub expected: ArtinResult,
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct TryFromLetter {
        pub data: TryFromLetterData,
        pub expected: ArtinResult,
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct Inverse {
        pub data: InverseData,
        pub expected: ArtinGenerator,
    }

    pub fn try_new(min_foot: Option<u16>, max_foot: Option<u16>) -> impl Strategy<Value = TryNew> {
        data(min_foot, max_foot).prop_map(|(valid_u16_data, sign)| {
            let expected_foot = Strand::try_new(valid_u16_data).unwrap();
            let expected_head = Strand::try_new(valid_u16_data + 1).unwrap();
            let expected_sign = sign;
            let expected_minimal_required_braid_index =
                BraidIndex::try_new(valid_u16_data + 1).unwrap();
            TryNew {
                data: TryNewData {
                    foot: valid_u16_data,
                    sign,
                },
                expected_foot,
                expected_head,
                expected_sign,
                expected_minimal_required_braid_index,
            }
        })
    }

    pub fn try_from_band(max_head: Option<u16>) -> impl Strategy<Value = TryFromBand> {
        valid::band::new(max_head, Some(1), Some(1)).prop_map(|band| TryFromBand {
            data: TryFromBandData(band),
            expected: ArtinGenerator::try_new(band.foot(), band.sign()),
        })
    }

    pub fn try_from_letter(max_head: Option<u16>) -> impl Strategy<Value = TryFromLetter> {
        valid::letter::new(max_head, Some(1), Some(1)).prop_map(|letter| TryFromLetter {
            data: TryFromLetterData(letter),
            expected: ArtinGenerator::try_new(letter.foot(), letter.sign()),
        })
    }

    pub fn inverse(min_foot: Option<u16>, max_foot: Option<u16>) -> impl Strategy<Value = Inverse> {
        data(min_foot, max_foot).prop_map(|(valid_u16_data, sign)| {
            let inverse_sign = -sign;
            Inverse {
                data: InverseData(ArtinGenerator::try_new(valid_u16_data, sign).unwrap()),
                expected: ArtinGenerator::try_new(valid_u16_data, inverse_sign).unwrap(),
            }
        })
    }
}
