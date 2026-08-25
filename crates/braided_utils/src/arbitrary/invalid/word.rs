use crate::arbitrary::invalid;
use crate::arbitrary::valid;
use braided::LetterValidationError;
use braided::{Letter, Sign, WordValidationError};
use proptest::prelude::*;

pub fn data_with_invalid_letter(
    max_artin_length: Option<u16>,
) -> impl Strategy<
    Value = (
        Vec<(valid::u16::Data, Option<valid::u16::Data>, Sign)>,
        LetterValidationError,
    ),
> {
    (0..max_artin_length.unwrap_or(u16::MAX))
        .prop_flat_map(|artin_length| {
            (
                invalid::letter::test_cases::try_new(),
                valid::letter::vector_of_data_with_given_artin_length(artin_length, None),
            )
        })
        .prop_flat_map(|(invalid_letter, mut valid_letter_data)| {
            let invalid_letter_data: (valid::u16::Data, Option<valid::u16::Data>, Sign) =
                match invalid_letter.data {
                    invalid::letter::test_cases::TryNewData::InvalidArtinGenerator(
                        invalid_artin_generator,
                    ) => match invalid_artin_generator {
                        invalid::artin::test_cases::TryNewData::InvalidHead(foot, sign) => {
                            (valid::u16::Data::U16(foot), None, sign)
                        }
                        invalid::artin::test_cases::TryNewData::InvalidStrand(foot, sign) => {
                            match foot {
                                invalid::strand::test_cases::TryNewData::Zero(foot) => {
                                    (valid::u16::Data::U16(foot), None, sign)
                                }
                                invalid::strand::test_cases::TryNewData::InvalidU16(foot) => {
                                    (foot.into(), None, sign)
                                }
                            }
                        }
                    },
                    invalid::letter::test_cases::TryNewData::InvalidBand(
                        invalid_band_generator,
                    ) => match invalid_band_generator {
                        invalid::band::test_cases::TryNewData::FootOnHead(foot, sign) => (
                            valid::u16::Data::U16(foot),
                            Some(valid::u16::Data::U16(foot)),
                            sign,
                        ),
                        invalid::band::test_cases::TryNewData::FootOverHead {
                            foot,
                            head,
                            sign,
                        } => (
                            valid::u16::Data::U16(foot),
                            Some(valid::u16::Data::U16(head)),
                            sign,
                        ),
                        invalid::band::test_cases::TryNewData::TooTall { foot, head, sign } => (
                            valid::u16::Data::U16(foot),
                            Some(valid::u16::Data::U16(head)),
                            sign,
                        ),
                        invalid::band::test_cases::TryNewData::InvalidFoot { foot, head, sign } => {
                            match foot {
                                invalid::strand::test_cases::TryNewData::Zero(foot) => (
                                    valid::u16::Data::U16(foot),
                                    Some(valid::u16::Data::U16(head)),
                                    sign,
                                ),
                                invalid::strand::test_cases::TryNewData::InvalidU16(foot) => {
                                    (foot.into(), Some(valid::u16::Data::U16(head)), sign)
                                }
                            }
                        }
                        invalid::band::test_cases::TryNewData::InvalidHead { foot, head, sign } => {
                            match head {
                                invalid::strand::test_cases::TryNewData::Zero(head) => (
                                    valid::u16::Data::U16(foot),
                                    Some(valid::u16::Data::U16(head)),
                                    sign,
                                ),
                                invalid::strand::test_cases::TryNewData::InvalidU16(head) => {
                                    (valid::u16::Data::U16(foot), Some(head.into()), sign)
                                }
                            }
                        }
                    },
                };

            valid_letter_data.push(invalid_letter_data);

            (
                Just(valid_letter_data).prop_shuffle(),
                Just(invalid_letter.error),
            )
        })
}

pub fn macro_data_exponent_fails_isize_coercion_with_error()
-> impl Strategy<Value = (valid::word::MacroFactor<usize>, WordValidationError)> {
    (
        (isize::MAX as usize + 1)..=usize::MAX,
        valid::letter::data(None, None, None),
    )
        .prop_map(|(exponent, letter_data)| {
            (
                valid::word::MacroFactor(letter_data.0, letter_data.1, exponent),
                WordValidationError::FromInt(
                    <usize as TryInto<isize>>::try_into(exponent).unwrap_err(),
                ),
            )
        })
}

pub fn macro_data_exponent_fails_u16_coercion_with_error() -> impl Strategy<
    Value = (
        valid::word::MacroFactor<invalid::u16::FailedU16ConversionData>,
        WordValidationError,
    ),
> {
    invalid::u16::bad_macro_exponent()
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
                invalid::u16::FailedU16ConversionData::AbsOverflowI64(val) => {
                    val.unsigned_abs() as u128 <= isize::MAX as u128
                }
                invalid::u16::FailedU16ConversionData::AbsOverflowI128(val) => {
                    val.unsigned_abs() <= isize::MAX as u128
                }
                _ => true,
            },
        )
        .prop_flat_map(|invalid_u16| (Just(invalid_u16), valid::letter::data(None, None, None)))
        .prop_map(|(invalid_u16, (foot, head, _))| {
            (
                valid::word::MacroFactor(foot, head, invalid_u16.data),
                WordValidationError::FromInt(invalid_u16.error),
            )
        })
}

fn macro_factor_data_invalid_letter_with_error()
-> impl Strategy<Value = (valid::word::MacroFactor<isize>, WordValidationError)> {
    (
        invalid::letter::test_cases::try_new(),
        -(u16::MAX as isize)..=(u16::MAX as isize),
    )
        .prop_map(|(invalid_letter, exponent)| {
            let error = WordValidationError::LetterValidation(invalid_letter.error);

            let factor: valid::word::MacroFactor<isize> = match invalid_letter.data {
                invalid::letter::test_cases::TryNewData::InvalidArtinGenerator(
                    invalid_artin_data,
                ) => match invalid_artin_data {
                    invalid::artin::test_cases::TryNewData::InvalidHead(foot, _) => {
                        valid::word::MacroFactor(valid::u16::Data::U16(foot), None, exponent)
                    }
                    invalid::artin::test_cases::TryNewData::InvalidStrand(
                        invalid_strand_data,
                        _,
                    ) => match invalid_strand_data {
                        invalid::strand::test_cases::TryNewData::Zero(foot) => {
                            valid::word::MacroFactor(valid::u16::Data::U16(foot), None, exponent)
                        }
                        invalid::strand::test_cases::TryNewData::InvalidU16(invalid_u16) => {
                            valid::word::MacroFactor(invalid_u16.into(), None, exponent)
                        }
                    },
                },
                invalid::letter::test_cases::TryNewData::InvalidBand(invalid_band_data) => {
                    match invalid_band_data {
                        invalid::band::test_cases::TryNewData::FootOnHead(foot, _) => {
                            valid::word::MacroFactor(
                                valid::u16::Data::U16(foot),
                                Some(valid::u16::Data::U16(foot)),
                                exponent,
                            )
                        }
                        invalid::band::test_cases::TryNewData::FootOverHead {
                            foot,
                            head,
                            sign: _,
                        } => valid::word::MacroFactor(
                            valid::u16::Data::U16(foot),
                            Some(valid::u16::Data::U16(head)),
                            exponent,
                        ),
                        invalid::band::test_cases::TryNewData::TooTall {
                            foot,
                            head,
                            sign: _,
                        } => valid::word::MacroFactor(
                            valid::u16::Data::U16(foot),
                            Some(valid::u16::Data::U16(head)),
                            exponent,
                        ),
                        invalid::band::test_cases::TryNewData::InvalidFoot {
                            foot: invalid_foot,
                            head,
                            sign: _,
                        } => match invalid_foot {
                            invalid::strand::test_cases::TryNewData::Zero(foot) => {
                                valid::word::MacroFactor(
                                    valid::u16::Data::U16(foot),
                                    Some(valid::u16::Data::U16(head)),
                                    exponent,
                                )
                            }
                            invalid::strand::test_cases::TryNewData::InvalidU16(invalid_foot) => {
                                valid::word::MacroFactor(
                                    invalid_foot.into(),
                                    Some(valid::u16::Data::U16(head)),
                                    exponent,
                                )
                            }
                        },
                        invalid::band::test_cases::TryNewData::InvalidHead {
                            foot,
                            head: invalid_head,
                            sign: _,
                        } => match invalid_head {
                            invalid::strand::test_cases::TryNewData::Zero(head) => {
                                valid::word::MacroFactor(
                                    valid::u16::Data::U16(foot),
                                    Some(valid::u16::Data::U16(head)),
                                    exponent,
                                )
                            }
                            invalid::strand::test_cases::TryNewData::InvalidU16(invalid_head) => {
                                valid::word::MacroFactor(
                                    valid::u16::Data::U16(foot),
                                    Some(invalid_head.into()),
                                    exponent,
                                )
                            }
                        },
                    }
                }
            };

            (factor, error)
        })
}

pub fn macro_data_invalid_letter_with_error()
-> impl Strategy<Value = ([valid::word::MacroFactor<isize>; 3], WordValidationError)> {
    (
        macro_factor_data_invalid_letter_with_error(),
        valid::word::macro_data_factor(None, None),
        valid::word::macro_data_factor(None, None),
    )
        .prop_flat_map(|((factor1, error), factor2, factor3)| {
            (
                Just([factor1, factor2, factor3]).prop_shuffle(),
                Just(error),
            )
        })
}

pub fn macro_data_too_long_with_error()
-> impl Strategy<Value = ([valid::word::MacroFactor<isize>; 3], WordValidationError)> {
    (1..=u16::MAX.div_euclid(4), 1..=u16::MAX.div_euclid(4))
        .prop_flat_map(|(h1, h2)| {
            (
                1..=u16::MAX.div_euclid(2 * (2 * h1 - 1)),
                1..=u16::MAX.div_euclid(2 * (2 * h2 - 1)),
                Just(h1),
                Just(h2),
            )
        })
        .prop_flat_map(|(e1, e2, h1, h2)| {
            (
                Just(h1),
                Just(e1),
                Just(h2),
                Just(e2),
                (u16::MAX - h1 * e1 - h2 * e2 + 1)..=u16::MAX,
            )
        })
        .prop_flat_map(|(h1, e1, h2, e2, e3)| {
            (
                Just((h1 * e1) as usize + (h2 * e2) as usize + e3 as usize),
                Just((e1, e2, e3)),
                valid::letter::data_with_given_height(None, h1),
                valid::letter::data_with_given_height(None, h2),
                valid::letter::data_with_given_height(None, 1),
            )
        })
        .prop_flat_map(|(total_length, exponents, l1, l2, l3)| {
            let error = WordValidationError::TooLong(total_length);

            let e1 = match l1.2 {
                Sign::Negative => -(exponents.0 as isize),
                Sign::Positive => exponents.0 as isize,
            };
            let e2 = match l2.2 {
                Sign::Negative => -(exponents.1 as isize),
                Sign::Positive => exponents.1 as isize,
            };
            let e3 = match l3.2 {
                Sign::Negative => -(exponents.2 as isize),
                Sign::Positive => exponents.2 as isize,
            };

            let factors = [
                valid::word::MacroFactor(l1.0, l1.1, e1),
                valid::word::MacroFactor(l2.0, l2.1, e2),
                valid::word::MacroFactor(l3.0, l3.1, e3),
            ];

            (Just(factors).prop_shuffle(), Just(error))
        })
}

pub mod test_cases {
    use super::*;

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub enum TryNewData {
        InvalidLetter(Vec<(valid::u16::Data, Option<valid::u16::Data>, Sign)>),
        TooLong(Vec<(valid::u16::Data, Option<valid::u16::Data>, Sign)>),
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct TryFromLettersData(pub Vec<Letter>);

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct MacroExponentFailsISizeCoercionData(pub valid::word::MacroFactor<usize>);

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct MacroExponentFailsU16CoercionData(
        pub valid::word::MacroFactor<invalid::u16::FailedU16ConversionData>,
    );

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct MacroInvalidLetterData(pub [valid::word::MacroFactor<isize>; 3]);

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct MacroTooLongData(pub [valid::word::MacroFactor<isize>; 3]);

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
    pub struct MacroExponentFailsISizeCoercion {
        pub data: MacroExponentFailsISizeCoercionData,
        pub error: WordValidationError,
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct MacroExponentFailsU16Coercion {
        pub data: MacroExponentFailsU16CoercionData,
        pub error: WordValidationError,
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct MacroInvalidLetter {
        pub data: MacroInvalidLetterData,
        pub error: WordValidationError,
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct MacroTooLong {
        pub data: MacroTooLongData,
        pub error: WordValidationError,
    }

    fn try_new_invalid_letter(max_artin_length: Option<u16>) -> impl Strategy<Value = TryNew> {
        data_with_invalid_letter(max_artin_length).prop_map(|(word_data, error)| TryNew {
            data: TryNewData::InvalidLetter(word_data),
            error: WordValidationError::LetterValidation(error),
        })
    }
    fn try_new_too_long() -> impl Strategy<Value = TryNew> {
        let too_long_length: usize = u16::MAX as usize + 1;
        (1..=u16::MAX)
            .prop_flat_map(move |first_artin_length| {
                (
                    valid::letter::vector_of_data_with_given_artin_length(first_artin_length, None),
                    valid::letter::vector_of_data_with_given_artin_length(
                        too_long_length as u16 - first_artin_length,
                        None,
                    ),
                )
            })
            .prop_map(move |(letters_1, letters_2)| {
                let all_letters = [letters_1, letters_2].concat();
                TryNew {
                    data: TryNewData::TooLong(all_letters),
                    error: WordValidationError::TooLong(too_long_length),
                }
            })
    }
    pub fn try_new(max_artin_length: Option<u16>) -> impl Strategy<Value = TryNew> {
        prop_oneof![try_new_invalid_letter(max_artin_length), try_new_too_long(),]
    }

    fn try_from_letters_too_long() -> impl Strategy<Value = TryFromLetters> {
        let too_long_length: usize = u16::MAX as usize + 1;
        (1..=u16::MAX)
            .prop_flat_map(move |first_artin_length| {
                (
                    valid::letter::vector_with_given_artin_length(first_artin_length, None),
                    valid::letter::vector_with_given_artin_length(
                        too_long_length as u16 - first_artin_length,
                        None,
                    ),
                )
            })
            .prop_map(move |(letters_1, letters_2)| {
                let all_letters = [letters_1, letters_2].concat();
                TryFromLetters {
                    data: TryFromLettersData(all_letters),
                    error: WordValidationError::TooLong(too_long_length),
                }
            })
    }
    pub fn try_from_letters() -> impl Strategy<Value = TryFromLetters> {
        prop_oneof![try_from_letters_too_long()]
    }

    pub fn macro_exponent_fails_isize_coercion()
    -> impl Strategy<Value = MacroExponentFailsISizeCoercion> {
        macro_data_exponent_fails_isize_coercion_with_error().prop_map(|(factor, error)| {
            MacroExponentFailsISizeCoercion {
                data: MacroExponentFailsISizeCoercionData(factor),
                error,
            }
        })
    }
    pub fn macro_exponent_fails_u16_coercion()
    -> impl Strategy<Value = MacroExponentFailsU16Coercion> {
        macro_data_exponent_fails_u16_coercion_with_error().prop_map(|(factor, error)| {
            MacroExponentFailsU16Coercion {
                data: MacroExponentFailsU16CoercionData(factor),
                error,
            }
        })
    }
    pub fn macro_invalid_letter() -> impl Strategy<Value = MacroInvalidLetter> {
        macro_data_invalid_letter_with_error().prop_map(|(factors, error)| MacroInvalidLetter {
            data: MacroInvalidLetterData(factors),
            error,
        })
    }
    pub fn macro_too_long() -> impl Strategy<Value = MacroTooLong> {
        macro_data_too_long_with_error().prop_map(|(factors, error)| MacroTooLong {
            data: MacroTooLongData(factors),
            error,
        })
    }
}
