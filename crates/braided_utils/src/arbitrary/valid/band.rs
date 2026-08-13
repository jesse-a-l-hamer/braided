use braided::{BandGenerator, Sign};
use proptest::prelude::*;

pub fn arbitrary_band_data_with_given_height(
    height: u16,
    max_head: Option<u16>,
) -> impl Strategy<Value = (u16, u16, Sign)> {
    (
        (1 + height.div_ceil(2))
            ..=*[height + 1, max_head.unwrap_or(u16::MAX)]
                .iter()
                .min()
                .unwrap(),
        prop_oneof![Just(Sign::Positive), Just(Sign::Negative)],
    )
        .prop_map(move |(head_idx, sign)| (head_idx - height, head_idx, sign))
}

pub fn arbitrary_band_data(
    max_head: Option<u16>,
    max_height: Option<u16>,
    max_artin_length: Option<u16>,
) -> impl Strategy<Value = (u16, u16, Sign)> {
    (2u16..max_head.unwrap_or(u16::MAX))
        .prop_flat_map(move |head_idx| {
            (
                Just(head_idx),
                1u16..=*[
                    head_idx - 1,
                    max_height.unwrap_or(u16::MAX.div_ceil(2)),
                    max_artin_length.unwrap_or(u16::MAX).div_ceil(2),
                ]
                .iter()
                .min()
                .unwrap(),
            )
        })
        .prop_flat_map(|(head_idx, height)| {
            (
                Just(head_idx - height),
                Just(head_idx),
                prop_oneof![Just(Sign::Positive), Just(Sign::Negative)],
            )
        })
}

pub fn arbitrary_band_with_given_height(
    height: u16,
    max_head: Option<u16>,
) -> impl Strategy<Value = BandGenerator> {
    arbitrary_band_data_with_given_height(height, max_head)
        .prop_map(|(foot, head, sign)| BandGenerator::try_new(foot, head, sign).unwrap())
}

pub fn arbitrary_band(
    max_head: Option<u16>,
    max_height: Option<u16>,
    max_artin_length: Option<u16>,
) -> impl Strategy<Value = BandGenerator> {
    arbitrary_band_data(max_head, max_height, max_artin_length)
        .prop_map(|(foot, head, sign)| BandGenerator::try_new(foot, head, sign).unwrap())
}
