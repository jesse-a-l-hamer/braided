use crate::arbitrary::invalid::index::{
    InvalidBraidIndexTryNewData, arbitrary_invalid_braid_index,
};
use crate::arbitrary::invalid::word::{
    InvalidWordMacroData, InvalidWordTryFromLettersData, InvalidWordTryNewData,
    arbitrary_invalid_word_macro, arbitrary_invalid_word_try_from_letters,
    arbitrary_invalid_word_try_new,
};
use crate::arbitrary::valid::arbitrary_word_data;
use crate::arbitrary::valid::letter::arbitrary_letter_data_with_given_head;
use crate::arbitrary::valid::u16::{ValidPositiveU16Data, arbitrary_valid_positive_u16_data};
use crate::arbitrary::valid::word::{
    arbitrary_word_macro_data, arbitrary_word_macro_data_single_factor,
    arbitrary_word_macro_data_single_factor_with_given_head,
};
use braided::{BraidIndex, BraidValidationError, Letter, Sign, Word};
use proptest::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum InvalidBraidTryNewData {
    IndexTooSmall(ValidPositiveU16Data, Word),
    InvalidIndex(InvalidBraidIndexTryNewData, Word),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum InvalidBraidTryFromDataData {
    IndexTooSmall(
        Option<ValidPositiveU16Data>,
        Vec<(ValidPositiveU16Data, Option<ValidPositiveU16Data>, Sign)>,
    ),
    InvalidIndex(
        InvalidBraidIndexTryNewData,
        Vec<(ValidPositiveU16Data, Option<ValidPositiveU16Data>, Sign)>,
    ),
    InvalidWord(Option<ValidPositiveU16Data>, InvalidWordTryNewData),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum InvalidBraidTryFromLettersData {
    IndexTooSmall(Option<ValidPositiveU16Data>, Vec<Letter>),
    InvalidIndex(InvalidBraidIndexTryNewData, Vec<Letter>),
    InvalidWord(Option<ValidPositiveU16Data>, InvalidWordTryFromLettersData),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum InvalidBraidTryTrivialData {
    InvalidIndex(InvalidBraidIndexTryNewData),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum InvalidBraidMacroData {
    IndexTooSmall(
        ValidPositiveU16Data,
        [(ValidPositiveU16Data, Option<ValidPositiveU16Data>, isize); 5],
    ),
    InvalidIndex(
        InvalidBraidIndexTryNewData,
        [(ValidPositiveU16Data, Option<ValidPositiveU16Data>, isize); 5],
    ),
    InvalidWord(Option<ValidPositiveU16Data>, InvalidWordMacroData),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct InvalidBraidTryNew {
    pub data: InvalidBraidTryNewData,
    pub error: BraidValidationError,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct InvalidBraidTryFromData {
    pub data: InvalidBraidTryFromDataData,
    pub error: BraidValidationError,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct InvalidBraidTryFromLetters {
    pub data: InvalidBraidTryFromLettersData,
    pub error: BraidValidationError,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct InvalidBraidTryTrivial {
    pub data: InvalidBraidTryTrivialData,
    pub error: BraidValidationError,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct InvalidBraidMacro {
    pub data: InvalidBraidMacroData,
    pub error: BraidValidationError,
}

fn arbitrary_word_data_with_single_letter_with_given_head(
    head: u16,
    max_height: Option<u16>,
    max_artin_length: Option<u16>,
) -> impl Strategy<Value = Vec<(ValidPositiveU16Data, Option<ValidPositiveU16Data>, Sign)>> {
    if head < 3 {
        panic!("Head must be at least 3 to generate the appropriate data.");
    }

    arbitrary_letter_data_with_given_head(head, max_height, max_artin_length)
        .prop_flat_map(move |(foot_idx, head_idx, sign)| {
            let height: u16 =
                head - <ValidPositiveU16Data as TryInto<u16>>::try_into(foot_idx).unwrap();
            let max_artin_length = max_artin_length.map(|max| max - (2 * height - 1));
            (
                Just((foot_idx, head_idx, sign)),
                arbitrary_word_data(Some(head - 1), max_artin_length),
            )
        })
        .prop_map(|(fixed_head_letter, word_data)| [vec![fixed_head_letter], word_data].concat())
        .prop_shuffle()
}

fn arbitrary_invalid_braid_try_new_index_too_small(
    max_braid_index: Option<u16>,
    max_height: Option<u16>,
    max_artin_length: Option<u16>,
) -> impl Strategy<Value = InvalidBraidTryNew> {
    (2..=(max_braid_index.unwrap_or(u16::MAX) - 1))
        .prop_flat_map(move |braid_index| {
            (
                Just(braid_index),
                (braid_index + 1)..=max_braid_index.unwrap_or(u16::MAX),
            )
        })
        .prop_flat_map(move |(braid_index, head)| {
            (
                arbitrary_valid_positive_u16_data(Some(braid_index), Some(braid_index)),
                arbitrary_word_data_with_single_letter_with_given_head(
                    head,
                    max_height,
                    max_artin_length,
                ),
            )
                .prop_map(move |(braid_index, word_data)| InvalidBraidTryNew {
                    data: InvalidBraidTryNewData::IndexTooSmall(
                        braid_index,
                        Word::try_new(word_data).clone_unwrap(),
                    ),
                    error: BraidValidationError::IndexTooSmall {
                        index: BraidIndex::try_new(braid_index).unwrap(),
                        minimal_required_index: BraidIndex::try_new(head).unwrap(),
                    },
                })
        })
}
fn arbitrary_invalid_braid_try_new_invalid_index(
    max_head: Option<u16>,
    max_artin_length: Option<u16>,
) -> impl Strategy<Value = InvalidBraidTryNew> {
    (
        arbitrary_invalid_braid_index(),
        arbitrary_word_data(max_head, max_artin_length),
    )
        .prop_map(|(invalid_braid_index, word_data)| InvalidBraidTryNew {
            data: InvalidBraidTryNewData::InvalidIndex(
                invalid_braid_index.data,
                Word::try_new(word_data).clone_unwrap(),
            ),
            error: BraidValidationError::IndexValidation(invalid_braid_index.error),
        })
}
pub fn arbitrary_invalid_braid_try_new(
    max_head: Option<u16>,
    max_height: Option<u16>,
    max_artin_length: Option<u16>,
) -> impl Strategy<Value = InvalidBraidTryNew> {
    prop_oneof![
        arbitrary_invalid_braid_try_new_index_too_small(max_head, max_height, max_artin_length),
        arbitrary_invalid_braid_try_new_invalid_index(max_head, max_artin_length),
    ]
}

fn arbitrary_invalid_braid_try_from_data_index_too_small(
    max_braid_index: Option<u16>,
    max_height: Option<u16>,
    max_artin_length: Option<u16>,
) -> impl Strategy<Value = InvalidBraidTryFromData> {
    (2..=(max_braid_index.unwrap_or(u16::MAX) - 1))
        .prop_flat_map(move |braid_index| {
            (
                Just(braid_index),
                (braid_index + 1)..=max_braid_index.unwrap_or(u16::MAX),
            )
        })
        .prop_flat_map(move |(braid_index, head)| {
            (
                Just(braid_index),
                arbitrary_valid_positive_u16_data(Some(braid_index), Some(braid_index)),
                arbitrary_word_data_with_single_letter_with_given_head(
                    head,
                    max_height,
                    max_artin_length,
                ),
            )
                .prop_map(move |(braid_index, braid_index_data, word_data)| {
                    InvalidBraidTryFromData {
                        data: InvalidBraidTryFromDataData::IndexTooSmall(
                            Some(braid_index_data),
                            word_data,
                        ),
                        error: BraidValidationError::IndexTooSmall {
                            index: BraidIndex::try_new(braid_index).unwrap(),
                            minimal_required_index: BraidIndex::try_new(head).unwrap(),
                        },
                    }
                })
        })
}
fn arbitrary_invalid_braid_try_from_data_invalid_index(
    max_braid_index: Option<u16>,
    max_artin_length: Option<u16>,
) -> impl Strategy<Value = InvalidBraidTryFromData> {
    (
        arbitrary_invalid_braid_index(),
        arbitrary_word_data(max_braid_index, max_artin_length),
    )
        .prop_map(|(invalid_braid_index, word_data)| InvalidBraidTryFromData {
            data: InvalidBraidTryFromDataData::InvalidIndex(invalid_braid_index.data, word_data),
            error: BraidValidationError::IndexValidation(invalid_braid_index.error),
        })
}
fn arbitrary_invalid_braid_try_from_data_invalid_word(
    max_braid_index: Option<u16>,
) -> impl Strategy<Value = InvalidBraidTryFromData> {
    (
        arbitrary_valid_positive_u16_data(Some(2), max_braid_index),
        arbitrary_invalid_word_try_new(),
    )
        .prop_flat_map(|(braid_index, invalid_word)| {
            (
                Just(Some(braid_index)).prop_union(Just(None)),
                Just(invalid_word),
            )
        })
        .prop_map(|(braid_index, invalid_word)| InvalidBraidTryFromData {
            data: InvalidBraidTryFromDataData::InvalidWord(braid_index, invalid_word.data),
            error: BraidValidationError::WordValidation(invalid_word.error),
        })
}
pub fn arbitrary_invalid_braid_try_from_data(
    max_braid_index: Option<u16>,
    max_height: Option<u16>,
    max_artin_length: Option<u16>,
) -> impl Strategy<Value = InvalidBraidTryFromData> {
    prop_oneof![
        arbitrary_invalid_braid_try_from_data_index_too_small(
            max_braid_index,
            max_height,
            max_artin_length
        ),
        arbitrary_invalid_braid_try_from_data_invalid_index(max_braid_index, max_artin_length),
        arbitrary_invalid_braid_try_from_data_invalid_word(max_braid_index),
    ]
}

fn arbitrary_invalid_braid_try_from_letters_index_too_small(
    max_braid_index: Option<u16>,
    max_height: Option<u16>,
    max_artin_length: Option<u16>,
) -> impl Strategy<Value = InvalidBraidTryFromLetters> {
    (2..=(max_braid_index.unwrap_or(u16::MAX) - 1))
        .prop_flat_map(move |braid_index| {
            (
                Just(braid_index),
                (braid_index + 1)..=max_braid_index.unwrap_or(u16::MAX),
            )
        })
        .prop_flat_map(move |(braid_index, head)| {
            (
                Just(braid_index),
                arbitrary_valid_positive_u16_data(Some(braid_index), Some(braid_index)),
                arbitrary_word_data_with_single_letter_with_given_head(
                    head,
                    max_height,
                    max_artin_length,
                ),
            )
                .prop_map(move |(braid_index, braid_index_data, word_data)| {
                    InvalidBraidTryFromLetters {
                        data: InvalidBraidTryFromLettersData::IndexTooSmall(
                            Some(braid_index_data),
                            word_data
                                .iter()
                                .map(|(foot, head, sign)| {
                                    Letter::try_new(*foot, *head, *sign).unwrap()
                                })
                                .collect(),
                        ),
                        error: BraidValidationError::IndexTooSmall {
                            index: BraidIndex::try_new(braid_index).unwrap(),
                            minimal_required_index: BraidIndex::try_new(head).unwrap(),
                        },
                    }
                })
        })
}
fn arbitrary_invalid_braid_try_from_letters_invalid_index(
    max_braid_index: Option<u16>,
    max_artin_length: Option<u16>,
) -> impl Strategy<Value = InvalidBraidTryFromLetters> {
    (
        arbitrary_invalid_braid_index(),
        arbitrary_word_data(max_braid_index, max_artin_length).prop_map(|word_data| {
            word_data
                .iter()
                .map(|(foot, head, sign)| Letter::try_new(*foot, *head, *sign).unwrap())
                .collect()
        }),
    )
        .prop_map(
            |(invalid_braid_index, word_data)| InvalidBraidTryFromLetters {
                data: InvalidBraidTryFromLettersData::InvalidIndex(
                    invalid_braid_index.data,
                    word_data,
                ),
                error: BraidValidationError::IndexValidation(invalid_braid_index.error),
            },
        )
}
fn arbitrary_invalid_braid_try_from_letters_invalid_word(
    max_braid_index: Option<u16>,
) -> impl Strategy<Value = InvalidBraidTryFromLetters> {
    (
        arbitrary_valid_positive_u16_data(Some(2), max_braid_index),
        arbitrary_invalid_word_try_from_letters(),
    )
        .prop_flat_map(|(braid_index, invalid_word)| {
            (
                Just(Some(braid_index)).prop_union(Just(None)),
                Just(invalid_word),
            )
        })
        .prop_map(|(braid_index, invalid_word)| InvalidBraidTryFromLetters {
            data: InvalidBraidTryFromLettersData::InvalidWord(braid_index, invalid_word.data),
            error: BraidValidationError::WordValidation(invalid_word.error),
        })
}
pub fn arbitrary_invalid_braid_try_from_letters(
    max_braid_index: Option<u16>,
    max_height: Option<u16>,
    max_artin_length: Option<u16>,
) -> impl Strategy<Value = InvalidBraidTryFromLetters> {
    prop_oneof![
        arbitrary_invalid_braid_try_from_letters_index_too_small(
            max_braid_index,
            max_height,
            max_artin_length
        ),
        arbitrary_invalid_braid_try_from_letters_invalid_index(max_braid_index, max_artin_length),
        arbitrary_invalid_braid_try_from_letters_invalid_word(max_braid_index),
    ]
}

fn arbitrary_invalid_braid_try_trivial_invalid_index()
-> impl Strategy<Value = InvalidBraidTryTrivial> {
    arbitrary_invalid_braid_index().prop_map(|invalid_braid_index| InvalidBraidTryTrivial {
        data: InvalidBraidTryTrivialData::InvalidIndex(invalid_braid_index.data),
        error: BraidValidationError::IndexValidation(invalid_braid_index.error),
    })
}
pub fn arbitrary_invalid_braid_try_trivial() -> impl Strategy<Value = InvalidBraidTryTrivial> {
    prop_oneof![arbitrary_invalid_braid_try_trivial_invalid_index()]
}

fn arbitrary_invalid_braid_macro_index_too_small(
    max_braid_index: Option<u16>,
    max_artin_length: Option<u16>,
) -> impl Strategy<Value = InvalidBraidMacro> {
    if max_artin_length.unwrap_or(u16::MAX) < 5 {
        panic!("Max Artin length must be at least 5 to generate this data.")
    }
    if max_braid_index.unwrap_or(u16::MAX) < 2 {
        panic!("Max head must be at least 2 to generate this data.")
    }
    (2..=(max_braid_index.unwrap_or(u16::MAX) - 1))
        .prop_flat_map(move |braid_index| {
            (
                Just(braid_index),
                (braid_index + 1)..=max_braid_index.unwrap_or(u16::MAX),
            )
        })
        .prop_flat_map(move |(braid_index, head)| {
            (
                Just(braid_index),
                arbitrary_word_macro_data_single_factor_with_given_head(
                    head,
                    Some(max_artin_length.unwrap_or(u16::MAX).div_euclid(5)),
                ),
                [
                    arbitrary_word_macro_data_single_factor(
                        Some(head - 1),
                        Some(max_artin_length.unwrap_or(u16::MAX).div_euclid(5)),
                    ),
                    arbitrary_word_macro_data_single_factor(
                        Some(head - 1),
                        Some(max_artin_length.unwrap_or(u16::MAX).div_euclid(5)),
                    ),
                    arbitrary_word_macro_data_single_factor(
                        Some(head - 1),
                        Some(max_artin_length.unwrap_or(u16::MAX).div_euclid(5)),
                    ),
                    arbitrary_word_macro_data_single_factor(
                        Some(head - 1),
                        Some(max_artin_length.unwrap_or(u16::MAX).div_euclid(5)),
                    ),
                ],
            )
                .prop_flat_map(|(braid_index, fixed_head_factor, other_factors)| {
                    let factors: [(ValidPositiveU16Data, Option<ValidPositiveU16Data>, isize); 5] =
                        (
                            fixed_head_factor,
                            other_factors[0],
                            other_factors[1],
                            other_factors[2],
                            other_factors[3],
                        )
                            .into();
                    (
                        Just(braid_index),
                        arbitrary_valid_positive_u16_data(Some(braid_index), Some(braid_index)),
                        Just(factors).prop_shuffle(),
                    )
                })
                .prop_map(
                    move |(braid_index, braid_index_data, word_data)| InvalidBraidMacro {
                        data: InvalidBraidMacroData::IndexTooSmall(braid_index_data, word_data),
                        error: BraidValidationError::IndexTooSmall {
                            index: BraidIndex::try_new(braid_index).unwrap(),
                            minimal_required_index: BraidIndex::try_new(head).unwrap(),
                        },
                    },
                )
        })
}
fn arbitrary_invalid_braid_macro_invalid_index(
    max_braid_index: Option<u16>,
    max_artin_length: Option<u16>,
) -> impl Strategy<Value = InvalidBraidMacro> {
    (
        arbitrary_invalid_braid_index(),
        arbitrary_word_macro_data(max_braid_index, max_artin_length),
    )
        .prop_map(|(invalid_braid_index, word_data)| InvalidBraidMacro {
            data: InvalidBraidMacroData::InvalidIndex(invalid_braid_index.data, word_data),
            error: BraidValidationError::IndexValidation(invalid_braid_index.error),
        })
}
fn arbitrary_invalid_braid_macro_invalid_word(
    max_braid_index: Option<u16>,
) -> impl Strategy<Value = InvalidBraidMacro> {
    arbitrary_valid_positive_u16_data(Some(2), max_braid_index).prop_flat_map(|braid_index| {
        (
            Just(Some(braid_index)).prop_union(Just(None)),
            arbitrary_invalid_word_macro(),
        )
            .prop_map(|(braid_index, invalid_word)| InvalidBraidMacro {
                data: InvalidBraidMacroData::InvalidWord(braid_index, invalid_word.data),
                error: BraidValidationError::WordValidation(invalid_word.error),
            })
    })
}
pub fn arbitrary_invalid_braid_macro(
    max_braid_index: Option<u16>,
    max_artin_length: Option<u16>,
) -> impl Strategy<Value = InvalidBraidMacro> {
    prop_oneof![
        arbitrary_invalid_braid_macro_index_too_small(max_braid_index, max_artin_length),
        arbitrary_invalid_braid_macro_invalid_index(max_braid_index, max_artin_length),
        arbitrary_invalid_braid_macro_invalid_word(max_braid_index),
    ]
}
