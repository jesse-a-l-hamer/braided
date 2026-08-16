use crate::arbitrary::valid::u16::{ValidPositiveU16Data, arbitrary_valid_positive_u16_data};
use braided::BraidIndex;
use proptest::prelude::*;

pub fn arbitrary_braid_index_data(
    min: Option<u16>,
    max: Option<u16>,
) -> impl Strategy<Value = ValidPositiveU16Data> {
    arbitrary_valid_positive_u16_data(min, max)
}

pub fn arbitrary_braid_index(
    min: Option<u16>,
    max: Option<u16>,
) -> impl Strategy<Value = BraidIndex> {
    arbitrary_braid_index_data(min, max).prop_map(|data| BraidIndex::try_new(data).unwrap())
}
