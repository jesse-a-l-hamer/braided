use crate::arbitrary::invalid::strand::{
    InvalidStrandTryNewData, arbitrary_invalid_strand_try_new,
};
use crate::arbitrary::valid::band::arbitrary_band_with_given_height;
use braided::{ArtinValidationError, BandGenerator, Letter, Sign};
use proptest::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum InvalidArtinGeneratorTryNewData {
    InvalidHead(u16, Sign),
    InvalidStrand(InvalidStrandTryNewData, Sign),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum InvalidArtinGeneratorTryFromBandData {
    InvalidBand(BandGenerator),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum InvalidArtinGeneratorTryFromLetterData {
    InvalidBandLetter(Letter),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct InvalidArtinGeneratorTryNew {
    pub data: InvalidArtinGeneratorTryNewData,
    pub error: ArtinValidationError,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct InvalidArtinGeneratorTryFromBand {
    pub data: InvalidArtinGeneratorTryFromBandData,
    pub error: ArtinValidationError,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct InvalidArtinGeneratorTryFromLetter {
    pub data: InvalidArtinGeneratorTryFromLetterData,
    pub error: ArtinValidationError,
}

pub fn arbitrary_invalid_artin_generator_try_new()
-> impl Strategy<Value = InvalidArtinGeneratorTryNew> {
    prop_oneof![
        (
            Just(u16::MAX),
            Just(Sign::Negative).prop_union(Just(Sign::Positive))
        )
            .prop_map(|(max, sign)| InvalidArtinGeneratorTryNew {
                data: InvalidArtinGeneratorTryNewData::InvalidHead(max, sign),
                error: ArtinValidationError::InvalidHead
            }),
        (
            arbitrary_invalid_strand_try_new(),
            Just(Sign::Negative).prop_union(Just(Sign::Positive))
        )
            .prop_map(|(invalid_strand, sign)| {
                InvalidArtinGeneratorTryNew {
                    data: InvalidArtinGeneratorTryNewData::InvalidStrand(invalid_strand.data, sign),
                    error: ArtinValidationError::StrandValidation(invalid_strand.error),
                }
            })
    ]
}

pub fn arbitrary_invalid_artin_generator_try_from_band()
-> impl Strategy<Value = InvalidArtinGeneratorTryFromBand> {
    (2..(u16::MAX - 1)).prop_flat_map(|height| {
        arbitrary_band_with_given_height(height, None).prop_map(|band| {
            InvalidArtinGeneratorTryFromBand {
                data: InvalidArtinGeneratorTryFromBandData::InvalidBand(band),
                error: ArtinValidationError::FromBand(band),
            }
        })
    })
}

pub fn arbitrary_invalid_artin_generator_try_from_letter()
-> impl Strategy<Value = InvalidArtinGeneratorTryFromLetter> {
    (2..(u16::MAX - 1)).prop_flat_map(|height| {
        arbitrary_band_with_given_height(height, None).prop_map(|band| {
            InvalidArtinGeneratorTryFromLetter {
                data: InvalidArtinGeneratorTryFromLetterData::InvalidBandLetter(Letter::Band(band)),
                error: ArtinValidationError::FromBand(band),
            }
        })
    })
}
