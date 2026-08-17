use crate::arbitrary::valid;
use braided::Strand;
use proptest::prelude::*;

pub fn data(min: Option<u16>, max: Option<u16>) -> impl Strategy<Value = valid::u16::Data> {
    valid::u16::data(min, max)
}

pub fn new(min: Option<u16>, max: Option<u16>) -> impl Strategy<Value = Strand> {
    data(min, max).prop_map(|data| Strand::try_new(data).unwrap())
}

pub mod test_cases {
    use super::*;

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct TryNewData(pub valid::u16::Data);

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct TryNew {
        pub data: TryNewData,
        pub expected_index: u16,
    }

    pub fn try_new(min: Option<u16>, max: Option<u16>) -> impl Strategy<Value = TryNew> {
        data(min, max).prop_map(|valid_u16_data| TryNew {
            data: TryNewData(valid_u16_data),
            expected_index: valid_u16_data.try_into().unwrap(),
        })
    }
}
