use crate::arbitrary::valid;
use braided::{Braid, BraidIndex, BraidResult, Letter, LetterResult, Word, WordResult};
use proptest::{bits::u16, prelude::*};

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
    pub fn braid_index(&self) -> BraidIndex {
        match self {
            Self::WordResult(_) => {
                panic!("Braid index only exists for braid multiplication results.")
            }
            Self::BraidResult(result) => result.clone_unwrap().braid_index(),
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

    fn as_braid_result(&self) -> MulResult {
        match self {
            Self::Braid(braid) => MulResult::BraidResult(BraidResult::from(braid.clone())),
            Self::BraidResult(braid_result) => MulResult::BraidResult(braid_result.clone()),
            _ => panic!("Only braid-type operands may be converted to braid results."),
        }
    }

    fn trivial(braid_index: u16) -> Self {
        Self::BraidResult(Braid::try_trivial(braid_index))
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

pub fn operands_and_product_as_letters(
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
                        20 => 2..=(artin_length-2).max(2),
                        4 => (artin_length-1)..=(artin_length-1),
                        1 => artin_length..=artin_length,
                    ]
                    .boxed(),
                )
            } else if artin_length == 1 {
                (
                    Just(braid_index),
                    Just(artin_length),
                    prop_oneof![0u16..=0, 1u16..=1].boxed(),
                )
            } else {
                (Just(braid_index), Just(artin_length), (0u16..=0).boxed())
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
            (
                braid_index,
                lhs_letters.clone(),
                rhs_letters.clone(),
                [lhs_letters, rhs_letters].concat(),
            )
        })
}

pub fn operands_and_product(
    max_braid_index: Option<u16>,
    max_artin_length: Option<u16>,
) -> impl Strategy<Value = (MulOperand, MulOperand, MulResult)> {
    operands_and_product_as_letters(max_braid_index, max_artin_length).prop_flat_map(
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

pub mod test_cases {
    use super::*;

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct MultiplicationData {
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
    pub struct UnitalityData {
        pub operand: MulOperand,
        pub trivial: MulOperand,
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct Multiplication {
        pub data: MultiplicationData,
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
        pub expected: MulResult,
    }

    pub fn multiplication(
        max_braid_index: Option<u16>,
        max_artin_length: Option<u16>,
    ) -> impl Strategy<Value = Multiplication> {
        operands_and_product(max_braid_index, max_artin_length).prop_map(
            |(left, right, expected)| Multiplication {
                data: MultiplicationData { left, right },
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
        (1..=max_braid_index.unwrap_or(u16::MAX))
            .prop_flat_map(move |braid_index| {
                (
                    Just(braid_index),
                    operand_with_fixed_braid_index(braid_index, max_artin_length),
                )
            })
            .prop_map(|(braid_index, operand)| {
                let operand = operand.as_braid(Some(braid_index));
                let trivial = MulOperand::trivial(braid_index);
                let operand_as_result = operand.as_braid_result();
                Unitality {
                    data: UnitalityData { operand, trivial },
                    expected: operand_as_result,
                }
            })
    }
}
