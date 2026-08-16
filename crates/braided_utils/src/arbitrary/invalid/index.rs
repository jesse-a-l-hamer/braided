use crate::arbitrary::invalid::u16::{InvalidU16Data, arbitrary_invalid_u16};
use braided::IndexValidationError;
use proptest::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum InvalidBraidIndexTryNewData {
    InvalidU16(InvalidU16Data),
    Zero(u16),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct InvalidBraidIndexTryNew {
    pub data: InvalidBraidIndexTryNewData,
    pub error: IndexValidationError,
}

pub fn arbitrary_invalid_braid_index() -> impl Strategy<Value = InvalidBraidIndexTryNew> {
    prop_oneof![
        Just(0u16).prop_map(|zero| InvalidBraidIndexTryNew {
            data: InvalidBraidIndexTryNewData::Zero(zero),
            error: IndexValidationError::Zero
        }),
        arbitrary_invalid_u16().prop_map(|invalid_u16| InvalidBraidIndexTryNew {
            data: InvalidBraidIndexTryNewData::InvalidU16(invalid_u16.data),
            error: IndexValidationError::from(invalid_u16.error)
        })
    ]
}
