use crate::arbitrary::valid::u16::{ValidPositiveU16Data, arbitrary_valid_positive_u16_data};
use braided::Strand;
use proptest::prelude::*;

pub fn arbitrary_strand_data(
    min: Option<u16>,
    max: Option<u16>,
) -> impl Strategy<Value = ValidPositiveU16Data> {
    arbitrary_valid_positive_u16_data(min, max)
}

pub fn arbitrary_strand(min: Option<u16>, max: Option<u16>) -> impl Strategy<Value = Strand> {
    arbitrary_strand_data(min, max).prop_map(|data| Strand::try_new(data).unwrap())
}
