use crate::arbitrary::invalid;
use crate::arbitrary::valid;
use braided::{BraidIndex, BraidValidationError, Letter, Sign, Word};
use proptest::bits::u16;
use proptest::prelude::*;

pub fn vector_of_word_data_where_one_letter_has_given_head_above_other_letters(
    head: u16,
    max_height: Option<u16>,
    max_artin_length: Option<u16>,
) -> impl Strategy<Value = Vec<(valid::u16::Data, Option<valid::u16::Data>, Sign)>> {
    if head < 3 {
        panic!("Head must be at least 3 to generate this data.");
    }

    valid::letter::data_with_given_head(head, max_height, max_artin_length)
        .prop_flat_map(move |(foot_idx, head_idx, sign)| {
            let height: u16 = head - <valid::u16::Data as Into<u16>>::into(foot_idx);
            let max_artin_length = max_artin_length.map(|max| max - (2 * height - 1));
            (
                Just((foot_idx, head_idx, sign)),
                valid::word::data(Some(head - 1), max_artin_length),
            )
        })
        .prop_map(|(fixed_head_letter, word_data)| [vec![fixed_head_letter], word_data].concat())
        .prop_shuffle()
}

fn macro_data_where_one_factor_has_given_head_above_other_letters(
    head: u16,
    max_artin_length: Option<u16>,
) -> impl Strategy<Value = [valid::word::MacroFactor<isize>; 3]> {
    let max_artin_length = Some(max_artin_length.unwrap_or(u16::MAX).div_euclid(3));
    (
        valid::word::macro_data_factor_with_given_head(head, max_artin_length),
        valid::word::macro_data_factor(Some(head - 1), max_artin_length),
        valid::word::macro_data_factor(Some(head - 1), max_artin_length),
    )
        .prop_flat_map(|(factor1, factor2, factor3)| {
            Just([factor1, factor2, factor3]).prop_shuffle()
        })
}

pub fn macro_data_index_too_small(
    max_braid_index: Option<u16>,
    max_artin_length: Option<u16>,
) -> impl Strategy<
    Value = (
        valid::u16::Data,
        [valid::word::MacroFactor<isize>; 3],
        BraidValidationError,
    ),
> {
    valid::u16::data(None, Some(max_braid_index.unwrap_or(u16::MAX - 1))).prop_flat_map(
        move |braid_index| {
            let braid_index_u16 = <valid::u16::Data as Into<u16>>::into(braid_index);
            if braid_index_u16 == 1 {
                (
                    Just(braid_index),
                    valid::word::macro_data(None, max_artin_length),
                )
                    .prop_map(move |(braid_index, factors)| {
                        let first_factor = factors.first().unwrap();
                        let first_head: u16 = if let Some(head) = first_factor.1 {
                            head.into()
                        } else {
                            (first_factor.0 + 1).into()
                        };
                        (
                            braid_index,
                            factors,
                            BraidValidationError::IndexTooSmall {
                                index: BraidIndex::try_new(braid_index_u16).unwrap(),
                                minimal_required_index: BraidIndex::try_new(first_head).unwrap(),
                            },
                        )
                    })
                    .boxed()
            } else {
                (Just(braid_index), (1u16 + braid_index_u16)..=u16::MAX)
                    .prop_flat_map(move |(braid_index, big_head)| {
                        (
                            Just(braid_index),
                            macro_data_where_one_factor_has_given_head_above_other_letters(
                                big_head,
                                max_artin_length,
                            ),
                            Just(BraidValidationError::IndexTooSmall {
                                index: BraidIndex::try_new(braid_index).unwrap(),
                                minimal_required_index: BraidIndex::try_new(big_head).unwrap(),
                            }),
                        )
                    })
                    .boxed()
            }
        },
    )
}

pub mod test_cases {
    use super::*;

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub enum TryNewData {
        IndexTooSmall(valid::u16::Data, Word),
        InvalidIndex(invalid::index::test_cases::TryNewData, Word),
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub enum TryFromDataData {
        IndexTooSmall(
            Option<valid::u16::Data>,
            Vec<(valid::u16::Data, Option<valid::u16::Data>, Sign)>,
        ),
        InvalidIndex(
            invalid::index::test_cases::TryNewData,
            Vec<(valid::u16::Data, Option<valid::u16::Data>, Sign)>,
        ),
        InvalidWord(
            Option<valid::u16::Data>,
            invalid::word::test_cases::TryNewData,
        ),
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub enum TryFromLettersData {
        IndexTooSmall(Option<valid::u16::Data>, Vec<Letter>),
        InvalidIndex(invalid::index::test_cases::TryNewData, Vec<Letter>),
        InvalidWord(
            Option<valid::u16::Data>,
            invalid::word::test_cases::TryFromLettersData,
        ),
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub enum TryTrivialData {
        InvalidIndex(invalid::index::test_cases::TryNewData),
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub enum MacroData {
        IndexTooSmall(valid::u16::Data, [valid::word::MacroFactor<isize>; 3]),
        InvalidIndex(
            invalid::index::test_cases::TryNewData,
            [valid::word::MacroFactor<isize>; 3],
        ),
        InvalidWord(
            Option<valid::u16::Data>,
            invalid::word::test_cases::MacroData,
        ),
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct TryNew {
        pub data: TryNewData,
        pub error: BraidValidationError,
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct TryFromData {
        pub data: TryFromDataData,
        pub error: BraidValidationError,
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct TryFromLetters {
        pub data: TryFromLettersData,
        pub error: BraidValidationError,
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct TryTrivial {
        pub data: TryTrivialData,
        pub error: BraidValidationError,
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct Macro {
        pub data: MacroData,
        pub error: BraidValidationError,
    }

    fn try_new_index_too_small(
        max_braid_index: Option<u16>,
        max_height: Option<u16>,
        max_artin_length: Option<u16>,
    ) -> impl Strategy<Value = TryNew> {
        (2..=(max_braid_index.unwrap_or(u16::MAX) - 1))
            .prop_flat_map(move |braid_index| {
                (
                    Just(braid_index),
                    (braid_index + 1)..=max_braid_index.unwrap_or(u16::MAX),
                )
            })
            .prop_flat_map(move |(braid_index, head)| {
                (
                    valid::u16::data(Some(braid_index), Some(braid_index)),
                    vector_of_word_data_where_one_letter_has_given_head_above_other_letters(
                        head,
                        max_height,
                        max_artin_length,
                    ),
                )
                    .prop_map(move |(braid_index, word_data)| TryNew {
                        data: TryNewData::IndexTooSmall(
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
    fn try_new_invalid_index(
        max_head: Option<u16>,
        max_artin_length: Option<u16>,
    ) -> impl Strategy<Value = TryNew> {
        (
            invalid::index::test_cases::try_new(),
            valid::word::data(max_head, max_artin_length),
        )
            .prop_map(|(invalid_braid_index, word_data)| TryNew {
                data: TryNewData::InvalidIndex(
                    invalid_braid_index.data,
                    Word::try_new(word_data).clone_unwrap(),
                ),
                error: BraidValidationError::IndexValidation(invalid_braid_index.error),
            })
    }
    pub fn try_new(
        max_head: Option<u16>,
        max_height: Option<u16>,
        max_artin_length: Option<u16>,
    ) -> impl Strategy<Value = TryNew> {
        prop_oneof![
            try_new_index_too_small(max_head, max_height, max_artin_length),
            try_new_invalid_index(max_head, max_artin_length),
        ]
    }

    fn try_from_data_index_too_small(
        max_braid_index: Option<u16>,
        max_height: Option<u16>,
        max_artin_length: Option<u16>,
    ) -> impl Strategy<Value = TryFromData> {
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
                    valid::u16::data(Some(braid_index), Some(braid_index)),
                    vector_of_word_data_where_one_letter_has_given_head_above_other_letters(
                        head,
                        max_height,
                        max_artin_length,
                    ),
                )
                    .prop_map(move |(braid_index, braid_index_data, word_data)| {
                        TryFromData {
                            data: TryFromDataData::IndexTooSmall(Some(braid_index_data), word_data),
                            error: BraidValidationError::IndexTooSmall {
                                index: BraidIndex::try_new(braid_index).unwrap(),
                                minimal_required_index: BraidIndex::try_new(head).unwrap(),
                            },
                        }
                    })
            })
    }
    fn try_from_data_invalid_index(
        max_braid_index: Option<u16>,
        max_artin_length: Option<u16>,
    ) -> impl Strategy<Value = TryFromData> {
        (
            invalid::index::test_cases::try_new(),
            valid::word::data(max_braid_index, max_artin_length),
        )
            .prop_map(|(invalid_braid_index, word_data)| TryFromData {
                data: TryFromDataData::InvalidIndex(invalid_braid_index.data, word_data),
                error: BraidValidationError::IndexValidation(invalid_braid_index.error),
            })
    }
    fn try_from_data_invalid_word(
        max_braid_index: Option<u16>,
    ) -> impl Strategy<Value = TryFromData> {
        (
            valid::u16::data(Some(2), max_braid_index),
            invalid::word::test_cases::try_new(),
        )
            .prop_flat_map(|(braid_index, invalid_word)| {
                (
                    Just(Some(braid_index)).prop_union(Just(None)),
                    Just(invalid_word),
                )
            })
            .prop_map(|(braid_index, invalid_word)| TryFromData {
                data: TryFromDataData::InvalidWord(braid_index, invalid_word.data),
                error: BraidValidationError::WordValidation(invalid_word.error),
            })
    }
    pub fn try_from_data(
        max_braid_index: Option<u16>,
        max_height: Option<u16>,
        max_artin_length: Option<u16>,
    ) -> impl Strategy<Value = TryFromData> {
        prop_oneof![
            try_from_data_index_too_small(max_braid_index, max_height, max_artin_length),
            try_from_data_invalid_index(max_braid_index, max_artin_length),
            try_from_data_invalid_word(max_braid_index),
        ]
    }

    fn try_from_letters_index_too_small(
        max_braid_index: Option<u16>,
        max_height: Option<u16>,
        max_artin_length: Option<u16>,
    ) -> impl Strategy<Value = TryFromLetters> {
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
                    valid::u16::data(Some(braid_index), Some(braid_index)),
                    vector_of_word_data_where_one_letter_has_given_head_above_other_letters(
                        head,
                        max_height,
                        max_artin_length,
                    ),
                )
                    .prop_map(move |(braid_index, braid_index_data, word_data)| {
                        TryFromLetters {
                            data: TryFromLettersData::IndexTooSmall(
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
    fn try_from_letters_invalid_index(
        max_braid_index: Option<u16>,
        max_artin_length: Option<u16>,
    ) -> impl Strategy<Value = TryFromLetters> {
        (
            invalid::index::test_cases::try_new(),
            valid::word::data(max_braid_index, max_artin_length).prop_map(|word_data| {
                word_data
                    .iter()
                    .map(|(foot, head, sign)| Letter::try_new(*foot, *head, *sign).unwrap())
                    .collect()
            }),
        )
            .prop_map(|(invalid_braid_index, word_data)| TryFromLetters {
                data: TryFromLettersData::InvalidIndex(invalid_braid_index.data, word_data),
                error: BraidValidationError::IndexValidation(invalid_braid_index.error),
            })
    }
    fn try_from_letters_invalid_word(
        max_braid_index: Option<u16>,
    ) -> impl Strategy<Value = TryFromLetters> {
        (
            valid::u16::data(Some(2), max_braid_index),
            invalid::word::test_cases::try_from_letters(),
        )
            .prop_flat_map(|(braid_index, invalid_word)| {
                (
                    Just(Some(braid_index)).prop_union(Just(None)),
                    Just(invalid_word),
                )
            })
            .prop_map(|(braid_index, invalid_word)| TryFromLetters {
                data: TryFromLettersData::InvalidWord(braid_index, invalid_word.data),
                error: BraidValidationError::WordValidation(invalid_word.error),
            })
    }
    pub fn try_from_letters(
        max_braid_index: Option<u16>,
        max_height: Option<u16>,
        max_artin_length: Option<u16>,
    ) -> impl Strategy<Value = TryFromLetters> {
        prop_oneof![
            try_from_letters_index_too_small(max_braid_index, max_height, max_artin_length),
            try_from_letters_invalid_index(max_braid_index, max_artin_length),
            try_from_letters_invalid_word(max_braid_index),
        ]
    }

    fn try_trivial_invalid_index() -> impl Strategy<Value = TryTrivial> {
        invalid::index::test_cases::try_new().prop_map(|invalid_braid_index| TryTrivial {
            data: TryTrivialData::InvalidIndex(invalid_braid_index.data),
            error: BraidValidationError::IndexValidation(invalid_braid_index.error),
        })
    }
    pub fn try_trivial() -> impl Strategy<Value = TryTrivial> {
        prop_oneof![try_trivial_invalid_index()]
    }

    fn macro_index_too_small(
        max_braid_index: Option<u16>,
        max_artin_length: Option<u16>,
    ) -> impl Strategy<Value = Macro> {
        macro_data_index_too_small(max_braid_index, max_artin_length).prop_map(
            |(braid_index, factors, error)| Macro {
                data: MacroData::IndexTooSmall(braid_index, factors),
                error,
            },
        )
    }
    fn macro_invalid_index(
        max_braid_index: Option<u16>,
        max_artin_length: Option<u16>,
    ) -> impl Strategy<Value = Macro> {
        (
            invalid::index::test_cases::try_new(),
            valid::word::macro_data(max_braid_index, max_artin_length),
        )
            .prop_map(|(invalid_braid_index, word_data)| Macro {
                data: MacroData::InvalidIndex(invalid_braid_index.data, word_data),
                error: BraidValidationError::IndexValidation(invalid_braid_index.error),
            })
    }
    fn macro_invalid_word(max_braid_index: Option<u16>) -> impl Strategy<Value = Macro> {
        valid::u16::data(Some(2), max_braid_index).prop_flat_map(|braid_index| {
            (
                Just(Some(braid_index)).prop_union(Just(None)),
                invalid::word::test_cases::word_macro(),
            )
                .prop_map(|(braid_index, invalid_word)| Macro {
                    data: MacroData::InvalidWord(braid_index, invalid_word.data),
                    error: BraidValidationError::WordValidation(invalid_word.error),
                })
        })
    }
    pub fn braid_macro(
        max_braid_index: Option<u16>,
        max_artin_length: Option<u16>,
    ) -> impl Strategy<Value = Macro> {
        prop_oneof![
            macro_index_too_small(max_braid_index, max_artin_length),
            macro_invalid_index(max_braid_index, max_artin_length),
            macro_invalid_word(max_braid_index),
        ]
    }
}
