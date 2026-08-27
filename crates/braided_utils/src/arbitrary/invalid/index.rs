use crate::arbitrary::invalid;
use braided::IndexValidationError;
use proptest::prelude::*;

pub mod test_cases {
    use super::*;

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum TryNewData {
        InvalidU16(invalid::u16::FailedU16ConversionData),
        Zero(u16),
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct TryNew {
        pub data: TryNewData,
        pub error: IndexValidationError,
    }

    pub fn try_new() -> impl Strategy<Value = TryNew> {
        prop_oneof![
            Just(0u16).prop_map(|zero| TryNew {
                data: TryNewData::Zero(zero),
                error: IndexValidationError::Zero
            }),
            invalid::u16::failed_u16_conversion().prop_map(|invalid_u16| TryNew {
                data: TryNewData::InvalidU16(invalid_u16.data),
                error: IndexValidationError::from(invalid_u16.error)
            })
        ]
    }
}
