use crate::arbitrary::invalid;
use crate::arbitrary::valid;
use braided::{ArtinValidationError, BandGenerator, Letter, Sign};
use proptest::prelude::*;

pub mod test_cases {
    use super::*;

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum TryNewData {
        InvalidHead(u16, Sign),
        InvalidStrand(invalid::strand::test_cases::TryNewData, Sign),
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct TryFromBandData(pub BandGenerator);

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct TryFromLetterData(pub Letter);

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct TryNew {
        pub data: TryNewData,
        pub error: ArtinValidationError,
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct TryFromBand {
        pub data: TryFromBandData,
        pub error: ArtinValidationError,
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct TryFromLetter {
        pub data: TryFromLetterData,
        pub error: ArtinValidationError,
    }

    pub fn try_new() -> impl Strategy<Value = TryNew> {
        prop_oneof![
            (
                Just(u16::MAX),
                Just(Sign::Negative).prop_union(Just(Sign::Positive))
            )
                .prop_map(|(max, sign)| TryNew {
                    data: TryNewData::InvalidHead(max, sign),
                    error: ArtinValidationError::InvalidHead
                }),
            (
                invalid::strand::test_cases::try_new(),
                Just(Sign::Negative).prop_union(Just(Sign::Positive))
            )
                .prop_map(|(invalid_strand, sign)| {
                    TryNew {
                        data: TryNewData::InvalidStrand(invalid_strand.data, sign),
                        error: ArtinValidationError::StrandValidation(invalid_strand.error),
                    }
                })
        ]
    }

    pub fn try_from_band() -> impl Strategy<Value = TryFromBand> {
        (2..=u16::MAX.div_ceil(2)).prop_flat_map(|height| {
            valid::band::with_given_height(height, None).prop_map(|band| TryFromBand {
                data: TryFromBandData(band),
                error: ArtinValidationError::FromBand(band),
            })
        })
    }

    pub fn try_from_letter() -> impl Strategy<Value = TryFromLetter> {
        (2..=u16::MAX.div_ceil(2)).prop_flat_map(|height| {
            valid::band::with_given_height(height, None).prop_map(|band| TryFromLetter {
                data: TryFromLetterData(Letter::Band(band)),
                error: ArtinValidationError::FromBand(band),
            })
        })
    }
}
