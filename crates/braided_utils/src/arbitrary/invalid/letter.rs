use crate::arbitrary::invalid::artin::{
    InvalidArtinGeneratorTryNewData, arbitrary_invalid_artin_generator_try_new,
};
use crate::arbitrary::invalid::band::{InvalidBandTryNewData, arbitrary_invalid_band_try_new};
use braided::LetterValidationError;
use proptest::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum InvalidLetterTryNewData {
    InvalidArtinGenerator(InvalidArtinGeneratorTryNewData),
    InvalidBand(InvalidBandTryNewData),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum InvalidLetterMacroData {
    InvalidArtinGenerator(InvalidArtinGeneratorTryNewData),
    InvalidBand(InvalidBandTryNewData),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct InvalidLetterTryNew {
    pub data: InvalidLetterTryNewData,
    pub error: LetterValidationError,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct InvalidLetterMacro {
    pub data: InvalidLetterMacroData,
    pub error: LetterValidationError,
}

pub fn arbitrary_invalid_letter_try_new() -> impl Strategy<Value = InvalidLetterTryNew> {
    prop_oneof![
        arbitrary_invalid_artin_generator_try_new().prop_map(|invalid_artin_generator| {
            InvalidLetterTryNew {
                data: InvalidLetterTryNewData::InvalidArtinGenerator(invalid_artin_generator.data),
                error: LetterValidationError::ArtinValidation(invalid_artin_generator.error),
            }
        }),
        arbitrary_invalid_band_try_new().prop_map(|invalid_band| InvalidLetterTryNew {
            data: InvalidLetterTryNewData::InvalidBand(invalid_band.data),
            error: LetterValidationError::BandValidation(invalid_band.error),
        }),
    ]
}

pub fn arbitrary_invalid_letter_macro() -> impl Strategy<Value = InvalidLetterMacro> {
    prop_oneof![
        arbitrary_invalid_artin_generator_try_new().prop_map(|invalid_artin_generator| {
            InvalidLetterMacro {
                data: InvalidLetterMacroData::InvalidArtinGenerator(invalid_artin_generator.data),
                error: LetterValidationError::ArtinValidation(invalid_artin_generator.error),
            }
        }),
        arbitrary_invalid_band_try_new().prop_map(|invalid_band| InvalidLetterMacro {
            data: InvalidLetterMacroData::InvalidBand(invalid_band.data),
            error: LetterValidationError::BandValidation(invalid_band.error),
        }),
    ]
}
