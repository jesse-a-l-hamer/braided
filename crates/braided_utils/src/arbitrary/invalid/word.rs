use crate::arbitrary::invalid::letter::{
    InvalidLetterTryNewData, arbitrary_invalid_letter_try_new,
};
use crate::arbitrary::invalid::u16::{InvalidU16Data, arbitrary_invalid_u16};
use crate::arbitrary::valid::arbitrary_letter_data;
use crate::arbitrary::valid::word::{
    arbitrary_vector_of_letter_data_with_given_artin_length,
    arbitrary_vector_of_letters_with_given_artin_length,
};
use braided::{Letter, Sign, WordValidationError};
use proptest::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum InvalidWordTryNewData {
    InvalidLetter((InvalidLetterTryNewData, Vec<(u16, Option<u16>, Sign)>)),
    TooLong(Vec<(u16, Option<u16>, Sign)>),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct InvalidWordTryNew {
    pub data: InvalidWordTryNewData,
    pub error: WordValidationError,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum InvalidWordTryFromLettersData {
    TooLong(Vec<Letter>),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct InvalidWordTryFromLetters {
    pub data: InvalidWordTryFromLettersData,
    pub error: WordValidationError,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum InvalidWordMacroData {
    ExponentFailsISizeCoercion((u16, Option<u16>, usize)),
    ExponentFailsU16Coercion((u16, Option<u16>, InvalidU16Data)),
    InvalidLetter((InvalidLetterTryNewData, Vec<(u16, Option<u16>, Sign)>)),
    TooLong {
        first: (u16, Option<u16>, isize),
        second: (u16, Option<u16>, isize),
    },
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct InvalidWordMacro {
    pub data: InvalidWordMacroData,
    pub error: WordValidationError,
}

fn arbitrary_invalid_word_invalid_letter_try_new() -> impl Strategy<Value = InvalidWordTryNew> {
    (
        arbitrary_invalid_letter_try_new(),
        prop::collection::vec(
            arbitrary_letter_data(None, None, None),
            0..(u16::MAX as usize),
        ),
    )
        .prop_map(|(invalid_letter, valid_letter_data)| InvalidWordTryNew {
            data: InvalidWordTryNewData::InvalidLetter((invalid_letter.data, valid_letter_data)),
            error: WordValidationError::LetterValidation(invalid_letter.error),
        })
}
fn arbitrary_invalid_word_too_long_try_new() -> impl Strategy<Value = InvalidWordTryNew> {
    (1..u16::MAX)
        .prop_flat_map(|artin_length_1| {
            (
                Just(artin_length_1),
                (u16::MAX - artin_length_1 + 1)..u16::MAX,
            )
        })
        .prop_flat_map(|(artin_length_1, artin_length_2)| {
            (
                Just(artin_length_1),
                Just(artin_length_2),
                arbitrary_vector_of_letter_data_with_given_artin_length(artin_length_1, None),
                arbitrary_vector_of_letter_data_with_given_artin_length(artin_length_2, None),
            )
        })
        .prop_map(|(artin_length_1, artin_length_2, letters_1, letters_2)| {
            let total_length = artin_length_1 as usize + artin_length_2 as usize;
            let all_letters = [letters_1, letters_2].concat();
            InvalidWordTryNew {
                data: InvalidWordTryNewData::TooLong(all_letters),
                error: WordValidationError::TooLong(total_length),
            }
        })
}
pub fn arbitrary_invalid_word_try_new() -> impl Strategy<Value = InvalidWordTryNew> {
    prop_oneof![
        arbitrary_invalid_word_invalid_letter_try_new(),
        arbitrary_invalid_word_too_long_try_new(),
    ]
}

fn arbitrary_invalid_word_too_long_try_from_letters()
-> impl Strategy<Value = InvalidWordTryFromLetters> {
    (1..u16::MAX)
        .prop_flat_map(|artin_length_1| {
            (
                Just(artin_length_1),
                (u16::MAX - artin_length_1 + 1)..u16::MAX,
            )
        })
        .prop_flat_map(|(artin_length_1, artin_length_2)| {
            (
                Just(artin_length_1),
                Just(artin_length_2),
                arbitrary_vector_of_letters_with_given_artin_length(artin_length_1, None),
                arbitrary_vector_of_letters_with_given_artin_length(artin_length_2, None),
            )
        })
        .prop_map(|(artin_length_1, artin_length_2, letters_1, letters_2)| {
            let total_length = artin_length_1 as usize + artin_length_2 as usize;
            let all_letters = [letters_1, letters_2].concat();
            InvalidWordTryFromLetters {
                data: InvalidWordTryFromLettersData::TooLong(all_letters),
                error: WordValidationError::TooLong(total_length),
            }
        })
}
pub fn arbitrary_invalid_word_try_from_letters() -> impl Strategy<Value = InvalidWordTryFromLetters>
{
    prop_oneof![arbitrary_invalid_word_too_long_try_from_letters()]
}

fn arbitrary_invalid_word_exponent_fails_isize_coercion() -> impl Strategy<Value = InvalidWordMacro>
{
    (
        (isize::MAX as usize + 1)..usize::MAX,
        arbitrary_letter_data(None, None, None),
    )
        .prop_map(|(exponent, letter_data)| InvalidWordMacro {
            data: InvalidWordMacroData::ExponentFailsISizeCoercion((
                letter_data.0,
                letter_data.1,
                exponent,
            )),
            error: WordValidationError::FromInt(
                <usize as TryInto<isize>>::try_into(exponent).unwrap_err(),
            ),
        })
}
fn arbitrary_invalid_word_exponent_fails_u16_coercion() -> impl Strategy<Value = InvalidWordMacro> {
    arbitrary_invalid_u16()
        .prop_filter(
            "Exponent must be coercible to isize.",
            |invalid_u16| match invalid_u16.data {
                InvalidU16Data::PosOverflowI64(val) => val as u128 <= isize::MAX as u128,
                InvalidU16Data::PosOverflowI128(val) => val as u128 <= isize::MAX as u128,
                InvalidU16Data::PosOverflowU32(val) => val as u128 <= isize::MAX as u128,
                InvalidU16Data::PosOverflowU64(val) => val as u128 <= isize::MAX as u128,
                InvalidU16Data::PosOverflowU128(val) => val <= isize::MAX as u128,
                InvalidU16Data::PosOverflowUSize(val) => val as u128 <= isize::MAX as u128,
                _ => true,
            },
        )
        .prop_flat_map(|invalid_u16| (Just(invalid_u16), arbitrary_letter_data(None, None, None)))
        .prop_map(|(invalid_u16, (foot, head, _))| InvalidWordMacro {
            data: InvalidWordMacroData::ExponentFailsU16Coercion((foot, head, invalid_u16.data)),
            error: WordValidationError::FromInt(invalid_u16.error),
        })
}
fn arbitrary_invalid_word_invalid_letter_macro() -> impl Strategy<Value = InvalidWordMacro> {
    (
        arbitrary_invalid_letter_try_new(),
        prop::collection::vec(
            arbitrary_letter_data(None, None, None),
            0..(u16::MAX as usize),
        ),
    )
        .prop_map(|(invalid_letter, valid_letter_data)| InvalidWordMacro {
            data: InvalidWordMacroData::InvalidLetter((invalid_letter.data, valid_letter_data)),
            error: WordValidationError::LetterValidation(invalid_letter.error),
        })
}
fn arbitrary_invalid_word_too_long_macro() -> impl Strategy<Value = InvalidWordMacro> {
    (1..u16::MAX)
        .prop_flat_map(|first_exponent| {
            (
                Just(first_exponent),
                (u16::MAX - first_exponent + 1)..u16::MAX,
            )
        })
        .prop_flat_map(|(first_exponent, second_exponent)| {
            (
                Just(first_exponent),
                Just(second_exponent),
                arbitrary_letter_data(
                    None,
                    Some(u16::MAX.div_euclid(first_exponent).div_ceil(2)),
                    Some(u16::MAX.div_euclid(first_exponent)),
                ),
                arbitrary_letter_data(
                    None,
                    Some(u16::MAX.div_euclid(second_exponent).div_ceil(2)),
                    Some(u16::MAX.div_euclid(second_exponent)),
                ),
            )
        })
        .prop_map(
            |(first_exponent, second_exponent, first_letter_data, second_letter_data)| {
                let first = if first_letter_data.2 == Sign::Positive {
                    (
                        first_letter_data.0,
                        first_letter_data.1,
                        first_exponent as isize,
                    )
                } else {
                    (
                        first_letter_data.0,
                        first_letter_data.1,
                        -(first_exponent as isize),
                    )
                };
                let second = if second_letter_data.2 == Sign::Positive {
                    (
                        second_letter_data.0,
                        second_letter_data.1,
                        second_exponent as isize,
                    )
                } else {
                    (
                        second_letter_data.0,
                        second_letter_data.1,
                        -(second_exponent as isize),
                    )
                };
                let total_length: usize = ((first_exponent as usize)
                    * (2 * (first_letter_data.1.unwrap_or(first_letter_data.0 + 1) as usize
                        - first_letter_data.0 as usize)
                        - 1))
                    + ((second_exponent as usize)
                        * (2 * (second_letter_data.1.unwrap_or(second_letter_data.0 + 1)
                            as usize
                            - second_letter_data.0 as usize)
                            - 1));
                InvalidWordMacro {
                    data: InvalidWordMacroData::TooLong { first, second },
                    error: WordValidationError::TooLong(total_length),
                }
            },
        )
}
pub fn arbitrary_invalid_word_macro() -> impl Strategy<Value = InvalidWordMacro> {
    prop_oneof![
        arbitrary_invalid_word_exponent_fails_isize_coercion(),
        arbitrary_invalid_word_exponent_fails_u16_coercion(),
        arbitrary_invalid_word_invalid_letter_macro(),
        arbitrary_invalid_word_too_long_macro(),
    ]
}
