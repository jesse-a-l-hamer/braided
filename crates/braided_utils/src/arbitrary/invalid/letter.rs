use crate::arbitrary::invalid;
use braided::LetterValidationError;
use proptest::prelude::*;

pub mod test_cases {
    use super::*;

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum TryNewData {
        InvalidArtinGenerator(invalid::artin::test_cases::TryNewData),
        InvalidBand(invalid::band::test_cases::TryNewData),
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum MacroData {
        InvalidArtinGenerator(invalid::artin::test_cases::TryNewData),
        InvalidBandGenerator(invalid::band::test_cases::TryNewData),
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct TryNew {
        pub data: TryNewData,
        pub error: LetterValidationError,
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct Macro {
        pub data: MacroData,
        pub error: LetterValidationError,
    }

    pub fn try_new() -> impl Strategy<Value = TryNew> {
        prop_oneof![
            invalid::artin::test_cases::try_new().prop_map(|invalid_artin_generator| {
                TryNew {
                    data: TryNewData::InvalidArtinGenerator(invalid_artin_generator.data),
                    error: LetterValidationError::ArtinValidation(invalid_artin_generator.error),
                }
            }),
            invalid::band::test_cases::try_new().prop_map(|invalid_band| TryNew {
                data: TryNewData::InvalidBand(invalid_band.data),
                error: LetterValidationError::BandValidation(invalid_band.error),
            }),
        ]
    }

    pub fn letter_macro() -> impl Strategy<Value = Macro> {
        prop_oneof![
            invalid::artin::test_cases::try_new().prop_map(|invalid_artin_generator| {
                Macro {
                    data: MacroData::InvalidArtinGenerator(invalid_artin_generator.data),
                    error: LetterValidationError::ArtinValidation(invalid_artin_generator.error),
                }
            }),
            invalid::band::test_cases::try_new().prop_map(|invalid_band| Macro {
                data: MacroData::InvalidBandGenerator(invalid_band.data),
                error: LetterValidationError::BandValidation(invalid_band.error),
            }),
        ]
    }
}
