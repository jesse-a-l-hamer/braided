use braided::{ArtinGenerator, Sign};
use proptest::prelude::*;

pub fn arbitrary_artin_data(max_foot: Option<u16>) -> impl Strategy<Value = (u16, Sign)> {
    (
        (1u16..=max_foot.unwrap_or(u16::MAX - 1)),
        prop_oneof![Just(Sign::Positive), Just(Sign::Negative)],
    )
}

pub fn arbitrary_artin_generator(max_foot: Option<u16>) -> impl Strategy<Value = ArtinGenerator> {
    arbitrary_artin_data(max_foot)
        .prop_map(|(foot, sign)| ArtinGenerator::try_new(foot, sign).unwrap())
}

pub fn arbitrary_artin_data_with_given_head(head: u16) -> impl Strategy<Value = (u16, Sign)> {
    if head < 2 {
        panic!("Head index must be at least 2.");
    }

    (
        Just(head),
        prop_oneof![Just(Sign::Positive), Just(Sign::Negative)],
    )
}
