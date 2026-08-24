use crate::arbitrary::valid::{self, word::MacroFactor};
use braided::{Braid, Sign, Word};
use proptest::prelude::*;

pub fn data_with_given_index(
    braid_index: u16,
    max_artin_length: Option<u16>,
) -> impl Strategy<
    Value = (
        valid::u16::Data,
        Vec<(valid::u16::Data, Option<valid::u16::Data>, Sign)>,
    ),
> {
    if braid_index == 0 {
        panic!("braid_index must be positive.");
    }
    if braid_index == 1 {
        (
            valid::u16::data(Some(braid_index), Some(braid_index)),
            Just(Vec::new()),
        )
            .boxed()
    } else {
        valid::word::data(Some(braid_index), max_artin_length)
            .prop_flat_map(move |word_data| {
                (
                    valid::u16::data(Some(braid_index), Some(braid_index)),
                    Just(word_data),
                )
            })
            .boxed()
    }
}

pub fn with_given_index(
    braid_index: u16,
    max_artin_length: Option<u16>,
) -> impl Strategy<Value = Braid> {
    if braid_index == 0 {
        panic!("braid_index must be positive.");
    }
    if braid_index == 1 {
        Just(Braid::try_new(braid_index, Word::trivial()).clone_unwrap()).boxed()
    } else {
        valid::word::new(Some(braid_index), max_artin_length)
            .prop_map(move |word| Braid::try_new(braid_index, word).clone_unwrap())
            .boxed()
    }
}

pub fn data(
    max_braid_index: Option<u16>,
    max_artin_length: Option<u16>,
) -> impl Strategy<
    Value = (
        valid::u16::Data,
        Vec<(valid::u16::Data, Option<valid::u16::Data>, Sign)>,
    ),
> {
    if let Some(braid_index) = max_braid_index
        && braid_index == 0
    {
        panic!("braid_index must be positive.");
    }
    (1..=max_braid_index.unwrap_or(u16::MAX))
        .prop_flat_map(move |braid_index| data_with_given_index(braid_index, max_artin_length))
}

pub fn macro_data(
    max_braid_index: Option<u16>,
    max_artin_length: Option<u16>,
) -> impl Strategy<Value = (Option<valid::u16::Data>, [MacroFactor<isize>; 3])> {
    if let Some(braid_index) = max_braid_index
        && braid_index < 2
    {
        panic!("braid_index must be at least 2 to generate nontrivial macro data.");
    }
    valid::index::data(Some(2), max_braid_index).prop_flat_map(move |braid_index| {
        (
            Just(braid_index).prop_perturb(|braid_index, mut rng| {
                if rng.random_bool(0.5) {
                    None
                } else {
                    Some(braid_index)
                }
            }),
            valid::word::macro_data(Some(braid_index.try_into().unwrap()), max_artin_length),
        )
    })
}

pub fn new(
    max_braid_index: Option<u16>,
    max_artin_length: Option<u16>,
) -> impl Strategy<Value = Braid> {
    if let Some(braid_index) = max_braid_index
        && braid_index == 0
    {
        panic!("braid_index must be positive.");
    }
    (1..=max_braid_index.unwrap_or(u16::MAX))
        .prop_flat_map(move |braid_index| with_given_index(braid_index, max_artin_length))
}

pub mod test_cases {
    use super::*;
    use braided::{BraidIndex, BraidResult, Letter, Word};

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct TryNewData {
        pub braid_index: valid::u16::Data,
        pub word: Word,
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct TryFromDataData {
        pub braid_index: Option<valid::u16::Data>,
        pub word_data: Vec<(valid::u16::Data, Option<valid::u16::Data>, Sign)>,
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct TryFromLettersData {
        pub braid_index: Option<valid::u16::Data>,
        pub letters: Vec<Letter>,
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct TryTrivialData(pub valid::u16::Data);

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub enum MacroData {
        NonTrivial {
            braid_index: Option<valid::u16::Data>,
            factors: Box<[valid::word::MacroFactor<isize>; 3]>,
        },
        Trivial {
            braid_index: valid::u16::Data,
        },
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct InverseData(pub Braid);

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct TryNew {
        pub data: TryNewData,
        pub expected_braid_index: BraidIndex,
        pub expected_word: Word,
        pub expected_letters: Vec<Letter>,
        pub expected_minimal_required_braid_index: BraidIndex,
        pub expected_writhe: i32,
        pub expected_letter_length: u16,
        pub expected_artin_length: u16,
        pub expected_is_trivial: bool,
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct TryFromData {
        pub data: TryFromDataData,
        pub expected: BraidResult,
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct TryFromLetters {
        pub data: TryFromLettersData,
        pub expected: BraidResult,
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct TryTrivial {
        pub data: TryTrivialData,
        pub expected: BraidResult,
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct Macro {
        pub data: MacroData,
        pub expected: BraidResult,
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct Inverse {
        pub data: InverseData,
        pub expected: Braid,
    }

    pub fn try_new(
        max_braid_index: Option<u16>,
        max_artin_length: Option<u16>,
    ) -> impl Strategy<Value = TryNew> {
        valid::index::data(None, max_braid_index)
            .prop_flat_map(move |braid_index| {
                (
                    Just(braid_index),
                    valid::word::new(Some(braid_index.try_into().unwrap()), max_artin_length),
                )
            })
            .prop_map(|(braid_index, word)| {
                let expected_braid_index = BraidIndex::try_new(braid_index).unwrap();
                let expected_letters = word.letters();
                let expected_minimal_required_braid_index = word.minimal_required_braid_index();
                let expected_letter_length = word.letter_length();
                let expected_artin_length = word.artin_length();
                let expected_is_trivial = word.is_trivial();
                let expected_word = word.clone();
                let expected_writhe = expected_letters.iter().fold(0i32, |acc, &letter| {
                    if letter.sign() == Sign::Positive {
                        acc + 1
                    } else {
                        acc - 1
                    }
                });
                TryNew {
                    data: TryNewData { braid_index, word },
                    expected_braid_index,
                    expected_word,
                    expected_artin_length,
                    expected_letters,
                    expected_minimal_required_braid_index,
                    expected_letter_length,
                    expected_is_trivial,
                    expected_writhe,
                }
            })
    }

    pub fn try_from_data(
        max_braid_index: Option<u16>,
        max_artin_length: Option<u16>,
    ) -> impl Strategy<Value = TryFromData> {
        data(max_braid_index, max_artin_length).prop_perturb(|(braid_index, word_data), mut rng| {
            let word = Word::try_new(word_data.clone()).clone_unwrap();

            let (data_braid_index, real_braid_index) = if rng.random_bool(0.5) {
                (
                    None::<valid::u16::Data>,
                    valid::u16::Data::BraidIndex(word.minimal_required_braid_index()),
                )
            } else {
                (Some(braid_index), braid_index)
            };

            let expected = Braid::try_new(real_braid_index, word.clone());

            TryFromData {
                data: TryFromDataData {
                    braid_index: data_braid_index,
                    word_data,
                },
                expected,
            }
        })
    }

    pub fn try_from_letters(
        max_braid_index: Option<u16>,
        max_artin_length: Option<u16>,
    ) -> impl Strategy<Value = TryFromLetters> {
        data(max_braid_index, max_artin_length).prop_perturb(|(braid_index, word_data), mut rng| {
            let (letters, minimal_required_braid_index) = word_data.iter().fold(
                (Vec::new(), 1u16),
                |(mut letters, braid_index), (foot, head, sign)| {
                    letters.push(Letter::try_new(*foot, *head, *sign).unwrap());
                    let minimal_required_braid_index: u16 =
                        head.unwrap_or(*foot + 1).try_into().unwrap();
                    (letters, braid_index.max(minimal_required_braid_index))
                },
            );

            let (data_braid_index, real_braid_index) = if rng.random_bool(0.5) {
                (
                    None::<valid::u16::Data>,
                    valid::u16::Data::U16(minimal_required_braid_index),
                )
            } else {
                (Some(braid_index), braid_index)
            };

            let expected = Braid::try_new(
                real_braid_index,
                Word::try_from_letters(&letters[..]).clone_unwrap(),
            );

            TryFromLetters {
                data: TryFromLettersData {
                    braid_index: data_braid_index,
                    letters,
                },
                expected,
            }
        })
    }

    pub fn try_trivial(max_braid_index: Option<u16>) -> impl Strategy<Value = TryTrivial> {
        valid::index::data(None, max_braid_index).prop_map(|braid_index| TryTrivial {
            data: TryTrivialData(braid_index),
            expected: Braid::try_new(braid_index, Word::trivial()),
        })
    }

    pub fn braid_macro(
        max_braid_index: Option<u16>,
        max_artin_length: Option<u16>,
    ) -> impl Strategy<Value = Macro> {
        prop_oneof![
            macro_data(max_braid_index, max_artin_length).prop_map(|(braid_index, factors)| {
                let (word_data, minimal_required_braid_index) = factors.iter().fold(
                    (Vec::new(), 1u16),
                    |(mut word_data, braid_index),
                     valid::word::MacroFactor(foot, head, exponent)| {
                        if *exponent < 0 {
                            word_data.extend(vec![
                                (*foot, *head, Sign::Negative);
                                exponent.unsigned_abs()
                            ]);
                        } else {
                            word_data.extend(vec![
                                (*foot, *head, Sign::Positive);
                                exponent.unsigned_abs()
                            ]);
                        }
                        let minimal_required_braid_index: u16 =
                            head.unwrap_or(*foot + 1).try_into().unwrap();
                        (word_data, braid_index.max(minimal_required_braid_index))
                    },
                );

                let expected = if let Some(braid_index) = braid_index {
                    Braid::try_new(braid_index, Word::try_new(word_data.clone()).clone_unwrap())
                } else {
                    Braid::try_new(
                        minimal_required_braid_index,
                        Word::try_new(word_data.clone()).clone_unwrap(),
                    )
                };

                Macro {
                    data: MacroData::NonTrivial {
                        braid_index,
                        factors: Box::new(factors),
                    },
                    expected,
                }
            }),
            valid::index::data(None, max_braid_index).prop_map(|braid_index| Macro {
                data: MacroData::Trivial { braid_index },
                expected: Braid::try_trivial(braid_index)
            })
        ]
    }

    pub fn inverse(
        max_braid_index: Option<u16>,
        max_artin_length: Option<u16>,
    ) -> impl Strategy<Value = Inverse> {
        data(max_braid_index, max_artin_length).prop_map(|(braid_index, word_data)| {
            let word = Word::try_new(word_data).clone_unwrap();
            let inverse_word = word.inverse();
            Inverse {
                data: InverseData(Braid::try_new(braid_index, word).clone_unwrap()),
                expected: Braid::try_new(braid_index, inverse_word).clone_unwrap(),
            }
        })
    }
}
