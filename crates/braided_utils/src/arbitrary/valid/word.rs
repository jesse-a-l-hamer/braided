use crate::arbitrary::valid;
use braided::{Sign, Word};
use proptest::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MacroFactor<E: TryInto<isize> + std::fmt::Debug>(
    pub valid::u16::Data,
    pub Option<valid::u16::Data>,
    pub E,
);

pub fn with_given_artin_length(
    artin_length: u16,
    max_head: Option<u16>,
) -> impl Strategy<Value = Word> {
    if let Some(max_head) = max_head
        && max_head < 2
        && 0 < artin_length
    {
        panic!("max_head must be at least 2.");
    }
    valid::letter::vector_with_given_artin_length(artin_length, max_head)
        .prop_map(|letters| Word::try_from_letters(&letters[..]).clone_unwrap())
}

pub fn macro_data_factor_with_given_head(
    head: u16,
    max_artin_length: Option<u16>,
) -> impl Strategy<Value = MacroFactor<isize>> {
    if head < 2 {
        panic!("head must be at least 2.");
    }
    (
        Just(head),
        1..=*[head - 1, max_artin_length.unwrap_or(u16::MAX).div_ceil(2)]
            .iter()
            .min()
            .unwrap(),
    )
        .prop_flat_map(move |(head, height)| {
            (
                Just(head),
                Just(height),
                prop_oneof![Just(Sign::Negative), Just(Sign::Positive)],
                1isize
                    ..=(max_artin_length
                        .unwrap_or(u16::MAX)
                        .div_euclid(2 * height - 1) as isize),
            )
                .prop_map(|(head, height, sign, exponent)| {
                    let foot = head - height;
                    match sign {
                        Sign::Negative => (height, foot, head, -exponent),
                        Sign::Positive => (height, foot, head, exponent),
                    }
                })
        })
        .prop_flat_map(|(height, foot, head, exponent)| {
            (
                Just(height),
                valid::u16::data(Some(foot), Some(foot)),
                valid::u16::data(Some(head), Some(head)),
                Just(exponent),
            )
        })
        .prop_perturb(|(height, foot, head, exponent), mut rng| {
            if height > 1 || rng.random_bool(0.5) {
                MacroFactor(foot, Some(head), exponent)
            } else {
                MacroFactor(foot, None, exponent)
            }
        })
}

pub fn macro_data_factor(
    max_head: Option<u16>,
    max_artin_length: Option<u16>,
) -> impl Strategy<Value = MacroFactor<isize>> {
    if let Some(max_head) = max_head
        && max_head < 2
    {
        panic!("Unable to generate nontrivial macro data when max_head < 2.");
    }
    (2..=max_head.unwrap_or(u16::MAX))
        .prop_flat_map(move |head| macro_data_factor_with_given_head(head, max_artin_length))
}

pub fn macro_data(
    max_head: Option<u16>,
    max_artin_length: Option<u16>,
) -> impl Strategy<Value = [MacroFactor<isize>; 3]> {
    if let Some(max_head) = max_head
        && max_head < 2
    {
        panic!("Unable to generate nontrivial macro data when max_head < 2.");
    }
    if let Some(max_artin_length) = max_artin_length
        && max_artin_length < 3
    {
        panic!("Max Artin length must be at least 3 to generate this data.")
    }
    [
        macro_data_factor(
            max_head,
            Some(max_artin_length.unwrap_or(u16::MAX).div_euclid(3)),
        ),
        macro_data_factor(
            max_head,
            Some(max_artin_length.unwrap_or(u16::MAX).div_euclid(3)),
        ),
        macro_data_factor(
            max_head,
            Some(max_artin_length.unwrap_or(u16::MAX).div_euclid(3)),
        ),
    ]
}

pub fn data(
    max_head: Option<u16>,
    max_artin_length: Option<u16>,
) -> impl Strategy<Value = Vec<(valid::u16::Data, Option<valid::u16::Data>, Sign)>> {
    if let Some(max_head) = max_head
        && max_head < 2
        && max_artin_length.is_none_or(|artin_length| 0 < artin_length)
    {
        panic!("max_head must be at least 2.");
    }
    (0..=max_artin_length.unwrap_or(u16::MAX)).prop_flat_map(move |artin_length| {
        valid::letter::vector_of_data_with_given_artin_length(artin_length, max_head)
    })
}

pub fn new(max_head: Option<u16>, max_artin_length: Option<u16>) -> impl Strategy<Value = Word> {
    if let Some(max_head) = max_head
        && max_head < 2
        && max_artin_length.is_none_or(|artin_length| 0 < artin_length)
    {
        panic!("max_head must be at least 2.");
    }
    (0..=max_artin_length.unwrap_or(u16::MAX))
        .prop_flat_map(move |artin_length| with_given_artin_length(artin_length, max_head))
}

pub mod test_cases {
    use super::*;
    use braided::{BraidIndex, Letter, Strand, WordResult};

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct TryNewData(pub Vec<(valid::u16::Data, Option<valid::u16::Data>, Sign)>);

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct TryFromLettersData(pub Vec<Letter>);

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub enum MacroData {
        Trivial,
        NonTrivial(Box<[MacroFactor<isize>; 3]>),
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct InverseData(pub Word);

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct TryNew {
        pub data: TryNewData,
        pub expected_letters: Vec<Letter>,
        pub expected_is_trivial: bool,
        pub expected_letter_length: u16,
        pub expected_artin_length: u16,
        pub expected_minimal_required_braid_index: BraidIndex,
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct TryFromLetters {
        pub data: TryFromLettersData,
        pub expected: WordResult,
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct Macro {
        pub data: MacroData,
        pub expected: WordResult,
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct Inverse {
        pub data: InverseData,
        pub expected: Word,
    }

    pub fn try_new(
        max_head: Option<u16>,
        max_artin_length: Option<u16>,
    ) -> impl Strategy<Value = TryNew> {
        data(max_head, max_artin_length).prop_map(|word_data| {
            let expected_letters = word_data
                .iter()
                .map(|(foot, head, sign)| Letter::try_new(*foot, *head, *sign).unwrap())
                .collect::<Vec<Letter>>();
            let expected_is_trivial = expected_letters.is_empty();
            let expected_letter_length: u16 = expected_letters.len().try_into().unwrap();
            let expected_artin_length: u16 = expected_letters.iter().fold(0u16, |acc, &letter| {
                if letter.artin_length() <= u16::MAX - acc {
                    acc + letter.artin_length()
                } else {
                    panic!(
                        "Gonna overflow: acc = {acc}; letter.artin_length() = {}",
                        letter.artin_length()
                    )
                }
            });
            let expected_minimal_required_braid_index = expected_letters
                .iter()
                .fold(BraidIndex::try_new(1).unwrap(), |acc, &letter| {
                    acc.max(letter.minimal_required_braid_index())
                });
            TryNew {
                data: TryNewData(word_data.clone()),
                expected_letters,
                expected_artin_length,
                expected_is_trivial,
                expected_letter_length,
                expected_minimal_required_braid_index,
            }
        })
    }

    pub fn try_from_letters(
        max_head: Option<u16>,
        max_artin_length: Option<u16>,
    ) -> impl Strategy<Value = TryFromLetters> {
        valid::letter::vector(max_head, max_artin_length).prop_map(|letters| TryFromLetters {
            data: TryFromLettersData(letters.clone()),
            expected: Word::try_new(
                letters
                    .iter()
                    .map(|&l| (l.foot(), Some(l.head()), l.sign()))
                    .collect::<Vec<(Strand, Option<Strand>, Sign)>>(),
            ),
        })
    }

    pub fn word_macro(
        max_head: Option<u16>,
        max_artin_length: Option<u16>,
    ) -> impl Strategy<Value = Macro> {
        prop_oneof![
            9 => macro_data(max_head, max_artin_length).prop_map(|factors| {
                let mut word_data: Vec<(valid::u16::Data, Option<valid::u16::Data>, Sign)> =
                    Vec::new();

                for factor in &factors[..] {
                    if factor.2 > 0 {
                        word_data.extend(vec![
                            (factor.0, factor.1, Sign::Positive);
                            factor.2.unsigned_abs()
                        ])
                    } else {
                        word_data.extend(vec![
                            (factor.0, factor.1, Sign::Negative);
                            factor.2.unsigned_abs()
                        ])
                    }
                }

                let expected = Word::try_new(word_data);

                Macro {
                    data: MacroData::NonTrivial(Box::new(factors)),
                    expected,
                }
            }),
            1 => Just(Macro {
                data: MacroData::Trivial,
                expected: WordResult::from(Word::trivial())
            })
        ]
    }

    pub fn inverse(
        max_head: Option<u16>,
        max_artin_length: Option<u16>,
    ) -> impl Strategy<Value = Inverse> {
        data(max_head, max_artin_length).prop_map(|word_data| Inverse {
            data: InverseData(Word::try_new(word_data.clone()).clone_unwrap()),
            expected: Word::try_new(
                word_data
                    .iter()
                    .rev()
                    .map(|(foot, head, sign)| (*foot, *head, -*sign)),
            )
            .clone_unwrap(),
        })
    }
}
