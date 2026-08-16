use crate::arbitrary::invalid::u16::{InvalidU16Data, arbitrary_invalid_u16};
use braided::{Strand, StrandValidationError};
use proptest::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum InvalidStrandTryNewData {
    InvalidU16(InvalidU16Data),
    Zero(u16),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum InvalidStrandArithmeticData {
    SubtractionOperands { left: Strand, right: Strand },
    AdditionOperands { left: Strand, right: Strand },
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct InvalidStrandTryNew {
    pub data: InvalidStrandTryNewData,
    pub error: StrandValidationError,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct InvalidStrandArithmetic {
    pub data: InvalidStrandArithmeticData,
    pub error: StrandValidationError,
}

pub fn arbitrary_invalid_strand_try_new() -> impl Strategy<Value = InvalidStrandTryNew> {
    prop_oneof![
        Just(0u16).prop_map(|zero| InvalidStrandTryNew {
            data: InvalidStrandTryNewData::Zero(zero),
            error: StrandValidationError::Zero
        }),
        arbitrary_invalid_u16().prop_map(|invalid_u16| InvalidStrandTryNew {
            data: InvalidStrandTryNewData::InvalidU16(invalid_u16.data),
            error: StrandValidationError::from(invalid_u16.error)
        })
    ]
}

fn arbitrary_invalid_strand_subtraction() -> impl Strategy<Value = InvalidStrandArithmetic> {
    (1..u16::MAX)
        .prop_flat_map(|left| (Just(left), left..=u16::MAX))
        .prop_map(|(left, right)| InvalidStrandArithmetic {
            data: InvalidStrandArithmeticData::SubtractionOperands {
                left: Strand::try_new(left).unwrap(),
                right: Strand::try_new(right).unwrap(),
            },
            error: StrandValidationError::Subtraction { left, right },
        })
}

fn arbitrary_invalid_strand_addition() -> impl Strategy<Value = InvalidStrandArithmetic> {
    (1..u16::MAX)
        .prop_flat_map(|left| (Just(left), (u16::MAX - left + 1)..=u16::MAX))
        .prop_map(|(left, right)| InvalidStrandArithmetic {
            data: InvalidStrandArithmeticData::AdditionOperands {
                left: Strand::try_new(left).unwrap(),
                right: Strand::try_new(right).unwrap(),
            },
            error: StrandValidationError::Addition { left, right },
        })
}

pub fn arbitrary_invalid_strand_arithmetic() -> impl Strategy<Value = InvalidStrandArithmetic> {
    prop_oneof![
        arbitrary_invalid_strand_subtraction(),
        arbitrary_invalid_strand_addition()
    ]
}
