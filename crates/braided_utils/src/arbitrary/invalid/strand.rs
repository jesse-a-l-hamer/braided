use crate::arbitrary::invalid;
use crate::arbitrary::valid;
use braided::{Strand, StrandValidationError};
use proptest::prelude::*;

pub fn addition_data() -> impl Strategy<
    Value = (
        valid::strand::AdditionOperand,
        valid::strand::AdditionOperand,
    ),
> {
    (1..=u16::MAX)
        .prop_flat_map(|left| (Just(left), (u16::MAX - left + 1)..=u16::MAX))
        .prop_perturb(|(left, right), mut rng| {
            let left = if rng.random_bool(0.5) {
                valid::strand::AdditionOperand::Strand(Strand::try_new(left).unwrap())
            } else {
                valid::strand::AdditionOperand::U16(left)
            };

            let right = match left {
                valid::strand::AdditionOperand::U16(_) => {
                    valid::strand::AdditionOperand::Strand(Strand::try_new(right).unwrap())
                }
                valid::strand::AdditionOperand::Strand(_) => {
                    if rng.random_bool(0.5) {
                        valid::strand::AdditionOperand::Strand(Strand::try_new(right).unwrap())
                    } else {
                        valid::strand::AdditionOperand::U16(right)
                    }
                }
            };

            (left, right)
        })
}

pub fn subtraction_data() -> impl Strategy<
    Value = (
        valid::strand::SubtractionOperand,
        valid::strand::SubtractionOperand,
    ),
> {
    (1..=u16::MAX)
        .prop_flat_map(|left| (Just(left), left..=u16::MAX))
        .prop_perturb(|(left, right), mut rng| {
            let left = if rng.random_bool(0.5) {
                valid::strand::SubtractionOperand::Strand(Strand::try_new(left).unwrap())
            } else {
                valid::strand::SubtractionOperand::U16(left)
            };

            let right = match left {
                valid::strand::SubtractionOperand::U16(_) => {
                    valid::strand::SubtractionOperand::Strand(Strand::try_new(right).unwrap())
                }
                valid::strand::SubtractionOperand::Strand(_) => {
                    if rng.random_bool(0.5) {
                        valid::strand::SubtractionOperand::Strand(Strand::try_new(right).unwrap())
                    } else {
                        valid::strand::SubtractionOperand::U16(right)
                    }
                }
            };

            (left, right)
        })
}

pub mod test_cases {
    use super::*;

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum TryNewData {
        InvalidU16(invalid::u16::FailedU16ConversionData),
        Zero(u16),
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct AdditionData {
        pub left: valid::strand::AdditionOperand,
        pub right: valid::strand::AdditionOperand,
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct SubtractionData {
        pub left: valid::strand::SubtractionOperand,
        pub right: valid::strand::SubtractionOperand,
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct TryNew {
        pub data: TryNewData,
        pub error: StrandValidationError,
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct Addition {
        pub data: AdditionData,
        pub error: StrandValidationError,
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct Subtraction {
        pub data: SubtractionData,
        pub error: StrandValidationError,
    }

    pub fn try_new() -> impl Strategy<Value = TryNew> {
        prop_oneof![
            Just(0u16).prop_map(|zero| TryNew {
                data: TryNewData::Zero(zero),
                error: StrandValidationError::Zero
            }),
            invalid::u16::failed_u16_conversion().prop_map(|invalid_u16| TryNew {
                data: TryNewData::InvalidU16(invalid_u16.data),
                error: StrandValidationError::from(invalid_u16.error)
            })
        ]
    }

    pub fn addition() -> impl Strategy<Value = Addition> {
        addition_data().prop_map(|(left, right)| Addition {
            data: AdditionData { left, right },
            error: StrandValidationError::Addition {
                left: left.into(),
                right: right.into(),
            },
        })
    }

    pub fn subtraction() -> impl Strategy<Value = Subtraction> {
        subtraction_data().prop_map(|(left, right)| Subtraction {
            data: SubtractionData { left, right },
            error: StrandValidationError::Subtraction {
                left: left.into(),
                right: right.into(),
            },
        })
    }
}
