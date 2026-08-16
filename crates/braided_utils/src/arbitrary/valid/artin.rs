use crate::arbitrary::valid::u16::{ValidPositiveU16Data, arbitrary_valid_positive_u16_data};
use braided::{ArtinGenerator, Sign};
use proptest::prelude::*;

pub fn arbitrary_artin_data(
    min_foot: Option<u16>,
    max_foot: Option<u16>,
) -> impl Strategy<Value = (ValidPositiveU16Data, Sign)> {
    (
        arbitrary_valid_positive_u16_data(min_foot, Some(max_foot.unwrap_or(u16::MAX - 1))),
        prop_oneof![Just(Sign::Positive), Just(Sign::Negative)],
    )
}

pub fn arbitrary_artin_generator(
    min_foot: Option<u16>,
    max_foot: Option<u16>,
) -> impl Strategy<Value = ArtinGenerator> {
    arbitrary_artin_data(min_foot, max_foot)
        .prop_map(|(foot, sign)| ArtinGenerator::try_new(foot, sign).unwrap())
}

pub fn arbitrary_artin_data_with_given_head(
    head: u16,
) -> impl Strategy<Value = (ValidPositiveU16Data, Sign)> {
    if head < 2 {
        panic!("Head index must be at least 2.");
    }

    (
        arbitrary_valid_positive_u16_data(Some(head - 1), Some(head - 1)),
        prop_oneof![Just(Sign::Positive), Just(Sign::Negative)],
    )
}
