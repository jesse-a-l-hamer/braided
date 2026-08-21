use crate::arbitrary::valid;
use braided::{Braid, BraidIndex, BraidResult, Letter, LetterResult, Word, WordResult};
use proptest::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum MulResult {
    WordResult(WordResult),
    BraidResult(BraidResult),
}

impl MulResult {
    fn as_braid(&self, braid_index: Option<u16>) -> Self {
        match self {
            Self::WordResult(result) => match &**result {
                Ok(result) => {
                    if let Some(braid_index) = braid_index {
                        if braid_index < result.minimal_required_braid_index().into() {
                            panic!("Given braid index is below the minimum required for this word.")
                        } else {
                            Self::BraidResult(Braid::try_new(braid_index, result.clone()))
                        }
                    } else {
                        Self::BraidResult(Braid::try_new(
                            result.minimal_required_braid_index(),
                            result.clone(),
                        ))
                    }
                }
                Err(_) => panic!("Unexpected error result."),
            },
            Self::BraidResult(_) => self.clone(),
        }
    }
}

impl From<WordResult> for MulResult {
    fn from(value: WordResult) -> Self {
        Self::WordResult(value)
    }
}
impl From<BraidResult> for MulResult {
    fn from(value: BraidResult) -> Self {
        Self::BraidResult(value)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum MulOperand {
    Letter(Letter),
    LetterResult(LetterResult),
    Word(Word),
    WordResult(WordResult),
    Braid(Braid),
    BraidResult(BraidResult),
    MulResult(MulResult),
}

impl MulOperand {
    fn as_braid(&self, braid_index: Option<u16>) -> Self {
        match self.clone() {
            Self::Letter(operand) => {
                if braid_index.is_none_or(|braid_index| {
                    braid_index >= operand.minimal_required_braid_index().into()
                }) {
                    Self::Braid(Braid::try_from_letters(braid_index, &[operand]).clone_unwrap())
                } else {
                    panic!("Given braid index is less than required for letter.")
                }
            }
            Self::LetterResult(operand) => match *operand {
                Ok(operand) => {
                    if braid_index.is_none_or(|braid_index| {
                        braid_index >= operand.minimal_required_braid_index().into()
                    }) {
                        Self::Braid(Braid::try_from_letters(braid_index, &[operand]).clone_unwrap())
                    } else {
                        panic!("Given braid index is less than required for letter.")
                    }
                }
                Err(_) => panic!("Unexpected error operand."),
            },
            Self::Word(operand) => {
                if braid_index.is_none_or(|braid_index| {
                    braid_index >= operand.minimal_required_braid_index().into()
                }) {
                    Self::Braid(
                        Braid::try_new(
                            braid_index.unwrap_or(operand.minimal_required_braid_index().into()),
                            operand,
                        )
                        .clone_unwrap(),
                    )
                } else {
                    panic!("Given braid index is less than required for letter.")
                }
            }
            Self::WordResult(operand) => match &*operand {
                Ok(operand) => {
                    if braid_index.is_none_or(|braid_index| {
                        braid_index >= operand.minimal_required_braid_index().into()
                    }) {
                        Self::Braid(
                            Braid::try_new(
                                braid_index
                                    .unwrap_or(operand.minimal_required_braid_index().into()),
                                operand.clone(),
                            )
                            .clone_unwrap(),
                        )
                    } else {
                        panic!("Given braid index is less than required for letter.")
                    }
                }
                Err(_) => panic!("Unexpected error operand."),
            },
            Self::Braid(_) | Self::BraidResult(_) => self.clone(),
            Self::MulResult(operand) => Self::MulResult(operand.as_braid(braid_index)),
        }
    }
}

impl std::ops::Mul for MulOperand {
    type Output = MulResult;
    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Letter(lhs), Self::Letter(rhs)) => Self::Output::from(lhs * rhs),
            (Self::Letter(lhs), Self::LetterResult(rhs)) => Self::Output::from(lhs * rhs),
            (Self::Letter(lhs), Self::Word(rhs)) => Self::Output::from(lhs * rhs),
            (Self::Letter(lhs), Self::WordResult(rhs)) => Self::Output::from(lhs * rhs),
            (Self::Letter(lhs), Self::Braid(rhs)) => Self::Output::from(lhs * rhs),
            (Self::Letter(lhs), Self::BraidResult(rhs)) => Self::Output::from(lhs * rhs),
            (Self::Letter(lhs), Self::MulResult(MulResult::WordResult(rhs))) => {
                Self::Output::from(lhs * rhs)
            }
            (Self::Letter(lhs), Self::MulResult(MulResult::BraidResult(rhs))) => {
                Self::Output::from(lhs * rhs)
            }
            (Self::LetterResult(lhs), Self::Letter(rhs)) => Self::Output::from(lhs * rhs),
            (Self::LetterResult(lhs), Self::LetterResult(rhs)) => Self::Output::from(lhs * rhs),
            (Self::LetterResult(lhs), Self::Word(rhs)) => Self::Output::from(lhs * rhs),
            (Self::LetterResult(lhs), Self::WordResult(rhs)) => Self::Output::from(lhs * rhs),
            (Self::LetterResult(lhs), Self::Braid(rhs)) => Self::Output::from(lhs * rhs),
            (Self::LetterResult(lhs), Self::BraidResult(rhs)) => Self::Output::from(lhs * rhs),
            (Self::LetterResult(lhs), Self::MulResult(MulResult::WordResult(rhs))) => {
                Self::Output::from(lhs * rhs)
            }
            (Self::LetterResult(lhs), Self::MulResult(MulResult::BraidResult(rhs))) => {
                Self::Output::from(lhs * rhs)
            }
            (Self::Word(lhs), Self::Letter(rhs)) => Self::Output::from(lhs * rhs),
            (Self::Word(lhs), Self::LetterResult(rhs)) => Self::Output::from(lhs * rhs),
            (Self::Word(lhs), Self::Word(rhs)) => Self::Output::from(lhs * rhs),
            (Self::Word(lhs), Self::WordResult(rhs)) => Self::Output::from(lhs * rhs),
            (Self::Word(lhs), Self::Braid(rhs)) => Self::Output::from(lhs * rhs),
            (Self::Word(lhs), Self::BraidResult(rhs)) => Self::Output::from(lhs * rhs),
            (Self::Word(lhs), Self::MulResult(MulResult::WordResult(rhs))) => {
                Self::Output::from(lhs * rhs)
            }
            (Self::Word(lhs), Self::MulResult(MulResult::BraidResult(rhs))) => {
                Self::Output::from(lhs * rhs)
            }
            (Self::WordResult(lhs), Self::Letter(rhs)) => Self::Output::from(lhs * rhs),
            (Self::WordResult(lhs), Self::LetterResult(rhs)) => Self::Output::from(lhs * rhs),
            (Self::WordResult(lhs), Self::Word(rhs)) => Self::Output::from(lhs * rhs),
            (Self::WordResult(lhs), Self::WordResult(rhs)) => Self::Output::from(lhs * rhs),
            (Self::WordResult(lhs), Self::Braid(rhs)) => Self::Output::from(lhs * rhs),
            (Self::WordResult(lhs), Self::BraidResult(rhs)) => Self::Output::from(lhs * rhs),
            (Self::WordResult(lhs), Self::MulResult(MulResult::WordResult(rhs))) => {
                Self::Output::from(lhs * rhs)
            }
            (Self::WordResult(lhs), Self::MulResult(MulResult::BraidResult(rhs))) => {
                Self::Output::from(lhs * rhs)
            }
            (Self::Braid(lhs), Self::Letter(rhs)) => Self::Output::from(lhs * rhs),
            (Self::Braid(lhs), Self::LetterResult(rhs)) => Self::Output::from(lhs * rhs),
            (Self::Braid(lhs), Self::Word(rhs)) => Self::Output::from(lhs * rhs),
            (Self::Braid(lhs), Self::WordResult(rhs)) => Self::Output::from(lhs * rhs),
            (Self::Braid(lhs), Self::Braid(rhs)) => Self::Output::from(lhs * rhs),
            (Self::Braid(lhs), Self::BraidResult(rhs)) => Self::Output::from(lhs * rhs),
            (Self::Braid(lhs), Self::MulResult(MulResult::WordResult(rhs))) => {
                Self::Output::from(lhs * rhs)
            }
            (Self::Braid(lhs), Self::MulResult(MulResult::BraidResult(rhs))) => {
                Self::Output::from(lhs * rhs)
            }
            (Self::BraidResult(lhs), Self::Letter(rhs)) => Self::Output::from(lhs * rhs),
            (Self::BraidResult(lhs), Self::LetterResult(rhs)) => Self::Output::from(lhs * rhs),
            (Self::BraidResult(lhs), Self::Word(rhs)) => Self::Output::from(lhs * rhs),
            (Self::BraidResult(lhs), Self::WordResult(rhs)) => Self::Output::from(lhs * rhs),
            (Self::BraidResult(lhs), Self::Braid(rhs)) => Self::Output::from(lhs * rhs),
            (Self::BraidResult(lhs), Self::BraidResult(rhs)) => Self::Output::from(lhs * rhs),
            (Self::BraidResult(lhs), Self::MulResult(MulResult::WordResult(rhs))) => {
                Self::Output::from(lhs * rhs)
            }
            (Self::BraidResult(lhs), Self::MulResult(MulResult::BraidResult(rhs))) => {
                Self::Output::from(lhs * rhs)
            }
            (Self::MulResult(MulResult::WordResult(lhs)), Self::Letter(rhs)) => {
                Self::Output::from(lhs * rhs)
            }
            (Self::MulResult(MulResult::WordResult(lhs)), Self::LetterResult(rhs)) => {
                Self::Output::from(lhs * rhs)
            }
            (Self::MulResult(MulResult::WordResult(lhs)), Self::Word(rhs)) => {
                Self::Output::from(lhs * rhs)
            }
            (Self::MulResult(MulResult::WordResult(lhs)), Self::WordResult(rhs)) => {
                Self::Output::from(lhs * rhs)
            }
            (Self::MulResult(MulResult::WordResult(lhs)), Self::Braid(rhs)) => {
                Self::Output::from(lhs * rhs)
            }
            (Self::MulResult(MulResult::WordResult(lhs)), Self::BraidResult(rhs)) => {
                Self::Output::from(lhs * rhs)
            }
            (
                Self::MulResult(MulResult::WordResult(lhs)),
                Self::MulResult(MulResult::WordResult(rhs)),
            ) => Self::Output::from(lhs * rhs),
            (
                Self::MulResult(MulResult::WordResult(lhs)),
                Self::MulResult(MulResult::BraidResult(rhs)),
            ) => Self::Output::from(lhs * rhs),
            (Self::MulResult(MulResult::BraidResult(lhs)), Self::Letter(rhs)) => {
                Self::Output::from(lhs * rhs)
            }
            (Self::MulResult(MulResult::BraidResult(lhs)), Self::LetterResult(rhs)) => {
                Self::Output::from(lhs * rhs)
            }
            (Self::MulResult(MulResult::BraidResult(lhs)), Self::Word(rhs)) => {
                Self::Output::from(lhs * rhs)
            }
            (Self::MulResult(MulResult::BraidResult(lhs)), Self::WordResult(rhs)) => {
                Self::Output::from(lhs * rhs)
            }
            (Self::MulResult(MulResult::BraidResult(lhs)), Self::Braid(rhs)) => {
                Self::Output::from(lhs * rhs)
            }
            (Self::MulResult(MulResult::BraidResult(lhs)), Self::BraidResult(rhs)) => {
                Self::Output::from(lhs * rhs)
            }
            (
                Self::MulResult(MulResult::BraidResult(lhs)),
                Self::MulResult(MulResult::WordResult(rhs)),
            ) => Self::Output::from(lhs * rhs),
            (
                Self::MulResult(MulResult::BraidResult(lhs)),
                Self::MulResult(MulResult::BraidResult(rhs)),
            ) => Self::Output::from(lhs * rhs),
        }
    }
}

pub fn operand_with_fixed_braid_index(
    braid_index: u16,
    max_artin_length: Option<u16>,
) -> impl Strategy<Value = MulOperand> {
    if braid_index == 0 {
        panic!("braid_index must be positive.");
    }
    if braid_index == 1 {
        return prop_oneof![
            Just(MulOperand::Word(Word::trivial())),
            Just(MulOperand::WordResult(WordResult::from(Word::trivial()))),
            Just(MulOperand::Braid(Braid::default())),
            Just(MulOperand::BraidResult(Braid::try_trivial(braid_index))),
        ]
        .boxed();
    }
    prop_oneof![
        valid::letter::new(Some(braid_index), None, max_artin_length).prop_map(MulOperand::Letter),
        valid::letter::new(Some(braid_index), None, max_artin_length)
            .prop_map(|letter| MulOperand::LetterResult(LetterResult::from(letter))),
        valid::word::new(Some(braid_index), max_artin_length).prop_map(MulOperand::Word),
        valid::word::new(Some(braid_index), max_artin_length)
            .prop_map(|word| MulOperand::WordResult(WordResult::from(word))),
        valid::braid::with_given_index(braid_index, max_artin_length).prop_map(MulOperand::Braid),
        valid::braid::with_given_index(braid_index, max_artin_length)
            .prop_map(|braid| MulOperand::BraidResult(BraidResult::from(braid)))
    ]
    .boxed()
}

pub fn operand(
    max_braid_index: Option<u16>,
    max_artin_length: Option<u16>,
) -> impl Strategy<Value = MulOperand> {
    (1..=max_braid_index.unwrap_or(u16::MAX)).prop_flat_map(move |braid_index| {
        operand_with_fixed_braid_index(braid_index, max_artin_length)
    })
}

fn operand_from_letters(
    braid_index: u16,
    letters: Vec<Letter>,
) -> impl Strategy<Value = MulOperand> {
    if braid_index == 1 && !letters.is_empty() {
        panic!("braid_index 1 only allows for trivial operands, but letters were given.");
    }
    if letters.len() == 1 {
        prop_oneof![
            6 => Just(MulOperand::Letter(*letters.last().unwrap())),
            6 => Just(
                MulOperand::LetterResult(LetterResult::from(Ok(*letters.last().unwrap())))
            ),
            1 => Just(MulOperand::Word(Word::try_from_letters(&letters[..]).clone_unwrap())),
            1 => Just(MulOperand::WordResult(Word::try_from_letters(&letters[..]))),
            1 => Just(
                MulOperand::Braid(
                    Braid::try_from_letters(Some(braid_index), &letters[..]).clone_unwrap()
                )
            ),
            1 => Just(
                MulOperand::BraidResult(Braid::try_from_letters(Some(braid_index), &letters[..]))
            ),
        ]
        .boxed()
    } else {
        prop_oneof![
            Just(MulOperand::Word(
                Word::try_from_letters(&letters[..]).clone_unwrap()
            )),
            Just(MulOperand::WordResult(Word::try_from_letters(&letters[..]))),
            Just(MulOperand::Braid(
                Braid::try_from_letters(Some(braid_index), &letters[..]).clone_unwrap()
            )),
            Just(MulOperand::BraidResult(Braid::try_from_letters(
                Some(braid_index),
                &letters[..]
            ))),
        ]
        .boxed()
    }
}

pub fn operands_and_product_from_letters(
    braid_index: u16,
    lhs_letters: Vec<Letter>,
    rhs_letters: Vec<Letter>,
    product_letters: Vec<Letter>,
) -> impl Strategy<Value = (MulOperand, MulOperand, MulResult)> {
    (
        operand_from_letters(braid_index, lhs_letters),
        operand_from_letters(braid_index, rhs_letters),
    )
        .prop_map(move |(lhs, rhs)| match (lhs.clone(), rhs.clone()) {
            (MulOperand::Braid(_), _)
            | (MulOperand::BraidResult(_), _)
            | (_, MulOperand::Braid(_))
            | (_, MulOperand::BraidResult(_)) => (
                lhs.clone(),
                rhs.clone(),
                MulResult::BraidResult(Braid::try_from_letters(
                    Some(braid_index),
                    &product_letters[..],
                )),
            ),
            (_, _) => (
                lhs,
                rhs,
                MulResult::WordResult(Word::try_from_letters(&product_letters[..])),
            ),
        })
}

pub fn non_cancelling_operands_and_product_as_letters(
    max_braid_index: Option<u16>,
    max_artin_length: Option<u16>,
) -> impl Strategy<Value = (u16, Vec<Letter>, Vec<Letter>, Vec<Letter>)> {
    if let Some(braid_index) = max_braid_index
        && braid_index < 1
    {
        panic!("braid_index must be positive.");
    }
    (
        1..=max_braid_index.unwrap_or(u16::MAX),
        0..=max_artin_length.unwrap_or(u16::MAX),
    )
        .prop_flat_map(|(braid_index, artin_length)| {
            if artin_length > 1 {
                (
                    Just(braid_index),
                    Just(artin_length),
                    prop_oneof![
                        1 => 0u16..=0,
                        4 => 1u16..=1,
                        20 => 2..=(artin_length-2),
                        4 => (artin_length-1)..=(artin_length-1),
                        1 => artin_length..=artin_length,
                    ]
                    .boxed(),
                )
            } else {
                (
                    Just(braid_index),
                    Just(artin_length),
                    prop_oneof![0u16..=0, 1u16..=1,].boxed(),
                )
            }
        })
        .prop_flat_map(|(braid_index, artin_length, lhs_length)| {
            (
                Just(braid_index),
                valid::letter::vector_with_given_artin_length(lhs_length, Some(braid_index)),
                valid::letter::vector_with_given_artin_length(
                    artin_length - lhs_length,
                    Some(braid_index),
                ),
            )
        })
        .prop_map(|(braid_index, lhs_letters, rhs_letters)| {
            let mut lhs_letters = lhs_letters;

            // ensure no cancellation will occur at the word boundary
            if let Some(lhs_last) = lhs_letters.last_mut()
                && let Some(rhs_first) = rhs_letters.first()
                && *lhs_last == rhs_first.inverse()
            {
                *lhs_last = (*lhs_last).inverse();
            }

            (
                braid_index,
                lhs_letters.clone(),
                rhs_letters.clone(),
                [lhs_letters, rhs_letters].concat(),
            )
        })
}

pub fn non_cancelling_operands_and_product(
    max_braid_index: Option<u16>,
    max_artin_length: Option<u16>,
) -> impl Strategy<Value = (MulOperand, MulOperand, MulResult)> {
    non_cancelling_operands_and_product_as_letters(max_braid_index, max_artin_length).prop_flat_map(
        |(braid_index, lhs_letters, rhs_letters, product_letters)| {
            operands_and_product_from_letters(
                braid_index,
                lhs_letters,
                rhs_letters,
                product_letters,
            )
        },
    )
}

pub fn cancelling_operands_with_product(
    max_braid_index: Option<u16>,
    max_artin_length: Option<u16>,
) -> impl Strategy<
    Value = (
        valid::multiplication::MulOperand,
        valid::multiplication::MulOperand,
        valid::multiplication::MulResult,
    ),
> {
    valid::multiplication::non_cancelling_operands_and_product_as_letters(
        max_braid_index,
        max_artin_length,
    )
    .prop_filter(
        "Braid index must be greater than 2 to construct testable cancelling products.",
        |(braid_index, _, _, _)| *braid_index > 2u16,
    )
    .prop_flat_map(
        move |(braid_index, lhs_letters, rhs_letters, product_letters)| {
            let max_artin_length = max_artin_length.unwrap_or(u16::MAX);
            let lhs_length: u16 = lhs_letters.len().try_into().unwrap();
            let rhs_length: u16 = rhs_letters.len().try_into().unwrap();
            let max_cancelling_length =
                *[max_artin_length - lhs_length, max_artin_length - rhs_length]
                    .iter()
                    .min()
                    .unwrap();
            (
                Just(braid_index),
                Just(lhs_letters),
                Just(rhs_letters),
                Just(product_letters),
                (1..=max_cancelling_length).prop_flat_map(move |cancelling_length| {
                    valid::letter::vector_with_given_artin_length(
                        cancelling_length,
                        Some(braid_index),
                    )
                }),
            )
        },
    )
    .prop_flat_map(
        |(braid_index, lhs_letters, rhs_letters, product_letters, cancelling_letters)| {
            let inverse_cancelling_letters = cancelling_letters
                .iter()
                .rev()
                .map(|l| l.inverse())
                .collect();
            valid::multiplication::operands_and_product_from_letters(
                braid_index,
                [lhs_letters, cancelling_letters.clone()].concat(),
                [inverse_cancelling_letters, rhs_letters].concat(),
                product_letters,
            )
        },
    )
}
pub mod test_cases {
    use super::*;

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct NonCancellingProductData {
        pub left: MulOperand,
        pub right: MulOperand,
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct CancellingProductData {
        pub left: MulOperand,
        pub right: MulOperand,
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct ClosureData {
        pub left: MulOperand,
        pub right: MulOperand,
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct AssociativityData {
        pub left: MulOperand,
        pub middle: MulOperand,
        pub right: MulOperand,
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct UnitalityData(pub MulOperand);

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct InvertabilityData(pub MulOperand);

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct NonCancellingProduct {
        pub data: NonCancellingProductData,
        pub expected: MulResult,
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct CancellingProduct {
        pub data: CancellingProductData,
        pub expected: MulResult,
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct Closure {
        pub data: ClosureData,
        pub expected: BraidIndex,
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct Associativity {
        pub data: AssociativityData,
        pub expected: bool,
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct Unitality {
        pub data: UnitalityData,
        pub expected: bool,
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct Invertability {
        pub data: InvertabilityData,
        pub expected: bool,
    }

    pub fn non_cancelling_product(
        max_braid_index: Option<u16>,
        max_artin_length: Option<u16>,
    ) -> impl Strategy<Value = NonCancellingProduct> {
        non_cancelling_operands_and_product(max_braid_index, max_artin_length).prop_map(
            |(left, right, expected)| NonCancellingProduct {
                data: NonCancellingProductData { left, right },
                expected,
            },
        )
    }

    pub fn cancelling_product(
        max_braid_index: Option<u16>,
        max_artin_length: Option<u16>,
    ) -> impl Strategy<Value = CancellingProduct> {
        cancelling_operands_with_product(max_braid_index, max_artin_length).prop_map(
            |(left, right, expected)| CancellingProduct {
                data: CancellingProductData { left, right },
                expected,
            },
        )
    }

    pub fn closure(
        max_braid_index: Option<u16>,
        max_artin_length: Option<u16>,
    ) -> impl Strategy<Value = Closure> {
        (1..=max_braid_index.unwrap_or(u16::MAX))
            .prop_flat_map(move |braid_index| {
                (
                    Just(braid_index),
                    operand_with_fixed_braid_index(braid_index, max_artin_length),
                    operand_with_fixed_braid_index(braid_index, max_artin_length),
                )
            })
            .prop_map(|(braid_index, left, right)| Closure {
                data: ClosureData {
                    left: left.as_braid(Some(braid_index)),
                    right: right.as_braid(Some(braid_index)),
                },
                expected: BraidIndex::try_new(braid_index).unwrap(),
            })
    }

    pub fn associativity(
        max_braid_index: Option<u16>,
        max_artin_length: Option<u16>,
    ) -> impl Strategy<Value = Associativity> {
        (1..=max_braid_index.unwrap_or(u16::MAX))
            .prop_flat_map(move |braid_index| {
                (
                    Just(braid_index),
                    operand_with_fixed_braid_index(braid_index, max_artin_length),
                    operand_with_fixed_braid_index(braid_index, max_artin_length),
                    operand_with_fixed_braid_index(braid_index, max_artin_length),
                )
            })
            .prop_map(|(braid_index, left, middle, right)| Associativity {
                data: AssociativityData {
                    left: left.as_braid(Some(braid_index)),
                    middle: middle.as_braid(Some(braid_index)),
                    right: right.as_braid(Some(braid_index)),
                },
                expected: true,
            })
    }

    pub fn unitality(
        max_braid_index: Option<u16>,
        max_artin_length: Option<u16>,
    ) -> impl Strategy<Value = Unitality> {
        operand(max_braid_index, max_artin_length).prop_map(|operand| Unitality {
            data: UnitalityData(operand),
            expected: true,
        })
    }

    pub fn invertability(
        max_braid_index: Option<u16>,
        max_artin_length: Option<u16>,
    ) -> impl Strategy<Value = Invertability> {
        operand(max_braid_index, max_artin_length).prop_map(|operand| Invertability {
            data: InvertabilityData(operand),
            expected: true,
        })
    }
}
