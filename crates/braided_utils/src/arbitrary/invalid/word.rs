use crate::arbitrary::invalid;
use crate::arbitrary::valid;
use braided::{Letter, Sign, WordValidationError};
use proptest::prelude::*;

pub mod test_cases {
    use super::*;

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub enum TryNewData {
        InvalidLetter(
            (
                invalid::letter::test_cases::TryNewData,
                Vec<(valid::u16::Data, Option<valid::u16::Data>, Sign)>,
            ),
        ),
        TooLong(Vec<(valid::u16::Data, Option<valid::u16::Data>, Sign)>),
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub enum TryFromLettersData {
        TooLong(Vec<Letter>),
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub enum MacroData {
        ExponentFailsISizeCoercion((valid::u16::Data, Option<valid::u16::Data>, usize)),
        ExponentFailsU16Coercion(
            (
                valid::u16::Data,
                Option<valid::u16::Data>,
                invalid::u16::FailedU16ConversionData,
            ),
        ),
        InvalidLetter(
            (
                invalid::letter::test_cases::TryNewData,
                Vec<(valid::u16::Data, Option<valid::u16::Data>, Sign)>,
            ),
        ),
        TooLong {
            first: (valid::u16::Data, Option<valid::u16::Data>, isize),
            second: (valid::u16::Data, Option<valid::u16::Data>, isize),
        },
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct TryNew {
        pub data: TryNewData,
        pub error: WordValidationError,
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct TryFromLetters {
        pub data: TryFromLettersData,
        pub error: WordValidationError,
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct Macro {
        pub data: MacroData,
        pub error: WordValidationError,
    }

    fn try_new_invalid_letter() -> impl Strategy<Value = TryNew> {
        (
            invalid::letter::test_cases::try_new(),
            prop::collection::vec(
                valid::letter::data(None, None, None),
                0..(u16::MAX as usize),
            ),
        )
            .prop_map(|(invalid_letter, valid_letter_data)| TryNew {
                data: TryNewData::InvalidLetter((invalid_letter.data, valid_letter_data)),
                error: WordValidationError::LetterValidation(invalid_letter.error),
            })
    }
    fn try_new_too_long() -> impl Strategy<Value = TryNew> {
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
                    valid::letter::vector_of_data_with_given_artin_length(artin_length_1, None),
                    valid::letter::vector_of_data_with_given_artin_length(artin_length_2, None),
                )
            })
            .prop_map(|(artin_length_1, artin_length_2, letters_1, letters_2)| {
                let total_length = artin_length_1 as usize + artin_length_2 as usize;
                let all_letters = [letters_1, letters_2].concat();
                TryNew {
                    data: TryNewData::TooLong(all_letters),
                    error: WordValidationError::TooLong(total_length),
                }
            })
    }
    pub fn try_new() -> impl Strategy<Value = TryNew> {
        prop_oneof![try_new_invalid_letter(), try_new_too_long(),]
    }

    fn try_from_letters_too_long() -> impl Strategy<Value = TryFromLetters> {
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
                    valid::letter::vector_of_letters_with_given_artin_length(artin_length_1, None),
                    valid::letter::vector_of_letters_with_given_artin_length(artin_length_2, None),
                )
            })
            .prop_map(|(artin_length_1, artin_length_2, letters_1, letters_2)| {
                let total_length = artin_length_1 as usize + artin_length_2 as usize;
                let all_letters = [letters_1, letters_2].concat();
                TryFromLetters {
                    data: TryFromLettersData::TooLong(all_letters),
                    error: WordValidationError::TooLong(total_length),
                }
            })
    }
    pub fn try_from_letters() -> impl Strategy<Value = TryFromLetters> {
        prop_oneof![try_from_letters_too_long()]
    }

    fn macro_exponent_fails_isize_coercion() -> impl Strategy<Value = Macro> {
        (
            (isize::MAX as usize + 1)..usize::MAX,
            valid::letter::data(None, None, None),
        )
            .prop_map(|(exponent, letter_data)| Macro {
                data: MacroData::ExponentFailsISizeCoercion((
                    letter_data.0,
                    letter_data.1,
                    exponent,
                )),
                error: WordValidationError::FromInt(
                    <usize as TryInto<isize>>::try_into(exponent).unwrap_err(),
                ),
            })
    }
    fn macro_exponent_fails_u16_coercion() -> impl Strategy<Value = Macro> {
        invalid::u16::failed_u16_conversion()
            .prop_filter(
                "Exponent must be coercible to isize.",
                |invalid_u16| match invalid_u16.data {
                    invalid::u16::FailedU16ConversionData::PosOverflowI64(val) => {
                        val as u128 <= isize::MAX as u128
                    }
                    invalid::u16::FailedU16ConversionData::PosOverflowI128(val) => {
                        val as u128 <= isize::MAX as u128
                    }
                    invalid::u16::FailedU16ConversionData::PosOverflowU32(val) => {
                        val as u128 <= isize::MAX as u128
                    }
                    invalid::u16::FailedU16ConversionData::PosOverflowU64(val) => {
                        val as u128 <= isize::MAX as u128
                    }
                    invalid::u16::FailedU16ConversionData::PosOverflowU128(val) => {
                        val <= isize::MAX as u128
                    }
                    invalid::u16::FailedU16ConversionData::PosOverflowUSize(val) => {
                        val as u128 <= isize::MAX as u128
                    }
                    _ => true,
                },
            )
            .prop_flat_map(|invalid_u16| (Just(invalid_u16), valid::letter::data(None, None, None)))
            .prop_map(|(invalid_u16, (foot, head, _))| Macro {
                data: MacroData::ExponentFailsU16Coercion((foot, head, invalid_u16.data)),
                error: WordValidationError::FromInt(invalid_u16.error),
            })
    }
    fn macro_invalid_letter() -> impl Strategy<Value = Macro> {
        (
            invalid::letter::test_cases::try_new(),
            prop::collection::vec(
                valid::letter::data(None, None, None),
                0..(u16::MAX as usize),
            ),
        )
            .prop_map(|(invalid_letter, valid_letter_data)| Macro {
                data: MacroData::InvalidLetter((invalid_letter.data, valid_letter_data)),
                error: WordValidationError::LetterValidation(invalid_letter.error),
            })
    }
    fn macro_too_long() -> impl Strategy<Value = Macro> {
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
                    valid::letter::data(
                        None,
                        Some(u16::MAX.div_euclid(first_exponent).div_ceil(2)),
                        Some(u16::MAX.div_euclid(first_exponent)),
                    ),
                    valid::letter::data(
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
                    let first_letter_foot: usize =
                        <valid::u16::Data as TryInto<u16>>::try_into(first_letter_data.0).unwrap()
                            as usize;
                    let first_letter_head: usize = match first_letter_data.1 {
                        Some(first_letter_head) => {
                            <valid::u16::Data as TryInto<u16>>::try_into(first_letter_head).unwrap()
                                as usize
                        }
                        None => first_letter_foot + 1,
                    };
                    let first_letter_height: usize =
                        2 * (first_letter_head - first_letter_foot) - 1;

                    let second_letter_foot: usize =
                        <valid::u16::Data as TryInto<u16>>::try_into(second_letter_data.0).unwrap()
                            as usize;
                    let second_letter_head: usize = match second_letter_data.1 {
                        Some(second_letter_head) => {
                            <valid::u16::Data as TryInto<u16>>::try_into(second_letter_head)
                                .unwrap() as usize
                        }
                        None => second_letter_foot + 1,
                    };
                    let second_letter_height: usize =
                        2 * (second_letter_head - second_letter_foot) - 1;
                    let total_length: usize = ((first_exponent as usize) * first_letter_height)
                        + ((second_exponent as usize) * second_letter_height);
                    Macro {
                        data: MacroData::TooLong { first, second },
                        error: WordValidationError::TooLong(total_length),
                    }
                },
            )
    }
    pub fn r#macro() -> impl Strategy<Value = Macro> {
        prop_oneof![
            macro_exponent_fails_isize_coercion(),
            macro_exponent_fails_u16_coercion(),
            macro_invalid_letter(),
            macro_too_long(),
        ]
    }
}
