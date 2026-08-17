use crate::arbitrary::invalid;
use crate::arbitrary::valid;
use braided::{BraidIndex, BraidValidationError, Letter, Sign, Word};
use proptest::prelude::*;

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
        IndexTooSmall(
            valid::u16::Data,
            [(valid::u16::Data, Option<valid::u16::Data>, isize); 5],
        ),
        InvalidIndex(
            invalid::index::test_cases::TryNewData,
            [(valid::u16::Data, Option<valid::u16::Data>, isize); 5],
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
    pub struct MacroTest {
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
                    valid::word::data_where_single_letter_has_given_head(
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
                    valid::word::data_where_single_letter_has_given_head(
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
                    valid::word::data_where_single_letter_has_given_head(
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
    ) -> impl Strategy<Value = MacroTest> {
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
                    valid::word::macro_data_where_one_factor_has_given_head(
                        head,
                        Some(max_artin_length.unwrap_or(u16::MAX).div_euclid(5)),
                    ),
                    [
                        valid::word::macro_data_with_single_factor(
                            Some(head - 1),
                            Some(max_artin_length.unwrap_or(u16::MAX).div_euclid(5)),
                        ),
                        valid::word::macro_data_with_single_factor(
                            Some(head - 1),
                            Some(max_artin_length.unwrap_or(u16::MAX).div_euclid(5)),
                        ),
                        valid::word::macro_data_with_single_factor(
                            Some(head - 1),
                            Some(max_artin_length.unwrap_or(u16::MAX).div_euclid(5)),
                        ),
                        valid::word::macro_data_with_single_factor(
                            Some(head - 1),
                            Some(max_artin_length.unwrap_or(u16::MAX).div_euclid(5)),
                        ),
                    ],
                )
                    .prop_flat_map(|(braid_index, fixed_head_factor, other_factors)| {
                        let factors: [(valid::u16::Data, Option<valid::u16::Data>, isize); 5] = (
                            fixed_head_factor,
                            other_factors[0],
                            other_factors[1],
                            other_factors[2],
                            other_factors[3],
                        )
                            .into();
                        (
                            Just(braid_index),
                            valid::u16::data(Some(braid_index), Some(braid_index)),
                            Just(factors).prop_shuffle(),
                        )
                    })
                    .prop_map(
                        move |(braid_index, braid_index_data, word_data)| MacroTest {
                            data: MacroData::IndexTooSmall(braid_index_data, word_data),
                            error: BraidValidationError::IndexTooSmall {
                                index: BraidIndex::try_new(braid_index).unwrap(),
                                minimal_required_index: BraidIndex::try_new(head).unwrap(),
                            },
                        },
                    )
            })
    }
    fn macro_invalid_index(
        max_braid_index: Option<u16>,
        max_artin_length: Option<u16>,
    ) -> impl Strategy<Value = MacroTest> {
        (
            invalid::index::test_cases::try_new(),
            valid::word::macro_data(max_braid_index, max_artin_length),
        )
            .prop_map(|(invalid_braid_index, word_data)| MacroTest {
                data: MacroData::InvalidIndex(invalid_braid_index.data, word_data),
                error: BraidValidationError::IndexValidation(invalid_braid_index.error),
            })
    }
    fn macro_invalid_word(max_braid_index: Option<u16>) -> impl Strategy<Value = MacroTest> {
        valid::u16::data(Some(2), max_braid_index).prop_flat_map(|braid_index| {
            (
                Just(Some(braid_index)).prop_union(Just(None)),
                invalid::word::test_cases::r#macro(),
            )
                .prop_map(|(braid_index, invalid_word)| MacroTest {
                    data: MacroData::InvalidWord(braid_index, invalid_word.data),
                    error: BraidValidationError::WordValidation(invalid_word.error),
                })
        })
    }
    pub fn macro_test(
        max_braid_index: Option<u16>,
        max_artin_length: Option<u16>,
    ) -> impl Strategy<Value = MacroTest> {
        prop_oneof![
            macro_index_too_small(max_braid_index, max_artin_length),
            macro_invalid_index(max_braid_index, max_artin_length),
            macro_invalid_word(max_braid_index),
        ]
    }
}
