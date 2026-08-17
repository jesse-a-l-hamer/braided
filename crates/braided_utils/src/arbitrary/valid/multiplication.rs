use crate::arbitrary::valid;
use braided::{Braid, BraidResult, Letter, LetterResult, Word, WordResult};
use proptest::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum MulResult {
    WordResult(WordResult),
    BraidResult(BraidResult),
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
    prop_oneof![
        valid::letter::new(Some(braid_index), None, max_artin_length).prop_map(MulOperand::Letter),
        valid::letter::new(Some(braid_index), None, max_artin_length)
            .prop_map(|letter| MulOperand::LetterResult(LetterResult::from(letter))),
        valid::word::new(Some(braid_index), max_artin_length).prop_map(MulOperand::Word),
        valid::word::new(Some(braid_index), max_artin_length)
            .prop_map(|word| MulOperand::WordResult(WordResult::from(word))),
        valid::braid::with_given_index(braid_index).prop_map(MulOperand::Braid),
        valid::braid::with_given_index(braid_index)
            .prop_map(|braid| MulOperand::BraidResult(BraidResult::from(braid)))
    ]
}

pub fn operands(
    max_braid_index: Option<u16>,
    max_artin_length: Option<u16>,
) -> impl Strategy<Value = (MulOperand, MulOperand)> {
    (
        1..=max_braid_index.unwrap_or(u16::MAX),
        0..=max_artin_length.unwrap_or(u16::MAX),
    )
        .prop_flat_map(|(braid_index, artin_length)| {
            (Just(braid_index), Just(artin_length), 0..=artin_length)
        })
        .prop_flat_map(|(braid_index, artin_length, lhs_length)| {
            (
                operand_with_fixed_braid_index(braid_index, Some(lhs_length)),
                operand_with_fixed_braid_index(braid_index, Some(artin_length - lhs_length)),
            )
        })
}

fn operand_from_letters(
    braid_index: u16,
    letters: Vec<Letter>,
) -> impl Strategy<Value = MulOperand> {
    let letter_weight: u32 = if letters.len() == 1 { 6 } else { 0 };
    prop_oneof![
        letter_weight => Just(MulOperand::Letter(*letters.last().unwrap())),
        letter_weight => Just(
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
                    ],
                )
            } else {
                (
                    Just(braid_index),
                    Just(artin_length),
                    prop_oneof![
                    1 => 0u16..=0,
                    1 => 1u16..=1,
                    0 => 2..=(artin_length-2),
                    0 => (artin_length-1)..=(artin_length-1),
                    0 => artin_length..=artin_length,
                    ],
                )
            }
        })
        .prop_flat_map(|(braid_index, artin_length, lhs_length)| {
            (
                Just(braid_index),
                valid::letter::vector_of_letters_with_given_artin_length(
                    lhs_length,
                    Some(braid_index),
                ),
                valid::letter::vector_of_letters_with_given_artin_length(
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
