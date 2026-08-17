use crate::arbitrary::invalid;
use braided::{Strand, StrandValidationError};
use proptest::prelude::*;

pub mod test_cases {
    use super::*;

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum TryNewData {
        InvalidU16(invalid::u16::FailedU16ConversionData),
        Zero(u16),
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum ArithmeticData {
        SubtractionOperands { left: Strand, right: Strand },
        AdditionOperands { left: Strand, right: Strand },
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct TryNew {
        pub data: TryNewData,
        pub error: StrandValidationError,
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct Arithmetic {
        pub data: ArithmeticData,
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

    fn subtraction() -> impl Strategy<Value = Arithmetic> {
        (1..u16::MAX)
            .prop_flat_map(|left| (Just(left), left..=u16::MAX))
            .prop_map(|(left, right)| Arithmetic {
                data: ArithmeticData::SubtractionOperands {
                    left: Strand::try_new(left).unwrap(),
                    right: Strand::try_new(right).unwrap(),
                },
                error: StrandValidationError::Subtraction { left, right },
            })
    }

    fn addition() -> impl Strategy<Value = Arithmetic> {
        (1..u16::MAX)
            .prop_flat_map(|left| (Just(left), (u16::MAX - left + 1)..=u16::MAX))
            .prop_map(|(left, right)| Arithmetic {
                data: ArithmeticData::AdditionOperands {
                    left: Strand::try_new(left).unwrap(),
                    right: Strand::try_new(right).unwrap(),
                },
                error: StrandValidationError::Addition { left, right },
            })
    }

    pub fn arithmetic() -> impl Strategy<Value = Arithmetic> {
        prop_oneof![subtraction(), addition()]
    }
}
