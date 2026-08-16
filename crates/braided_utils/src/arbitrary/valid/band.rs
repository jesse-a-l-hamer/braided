use crate::arbitrary::valid::u16::{ValidPositiveU16Data, arbitrary_valid_positive_u16_data};
use braided::{BandGenerator, Sign};
use proptest::prelude::*;

pub fn arbitrary_band_data_with_given_height(
    height: u16,
    max_head: Option<u16>,
) -> impl Strategy<Value = (ValidPositiveU16Data, ValidPositiveU16Data, Sign)> {
    let min_head = 1 + height.div_ceil(2);
    let max_head = [height + 1, max_head.unwrap_or(u16::MAX)]
        .iter()
        .min()
        .cloned();
    (
        min_head..=max_head.unwrap(),
        prop_oneof![Just(Sign::Positive), Just(Sign::Negative)],
    )
        .prop_flat_map(move |(head_idx, sign)| {
            (
                arbitrary_valid_positive_u16_data(Some(head_idx - height), Some(head_idx - height)),
                arbitrary_valid_positive_u16_data(Some(head_idx), Some(head_idx)),
                Just(sign),
            )
        })
}

pub fn arbitrary_band_data(
    max_head: Option<u16>,
    max_height: Option<u16>,
    max_artin_length: Option<u16>,
) -> impl Strategy<Value = (ValidPositiveU16Data, ValidPositiveU16Data, Sign)> {
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
                arbitrary_valid_positive_u16_data(Some(head_idx - height), Some(head_idx - height)),
                arbitrary_valid_positive_u16_data(Some(head_idx), Some(head_idx)),
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

pub fn arbitrary_band_data_with_given_head(
    head: u16,
    max_height: Option<u16>,
    max_artin_length: Option<u16>,
) -> impl Strategy<Value = (ValidPositiveU16Data, ValidPositiveU16Data, Sign)> {
    if head < 2 {
        panic!("Head index must be at least 2.");
    }
    let max_height = *[
        head - 1,
        max_height.unwrap_or(u16::MAX.div_ceil(2)),
        max_artin_length.unwrap_or(u16::MAX).div_ceil(2),
    ]
    .iter()
    .min()
    .unwrap();

    (
        Just(head),
        1..=max_height,
        prop_oneof![Just(Sign::Negative), Just(Sign::Positive)],
    )
        .prop_flat_map(|(head, height, sign)| {
            (
                arbitrary_valid_positive_u16_data(Some(head - height), Some(head - height)),
                arbitrary_valid_positive_u16_data(Some(head), Some(head)),
                Just(sign),
            )
        })
}

pub fn arbitrary_band(
    max_head: Option<u16>,
    max_height: Option<u16>,
    max_artin_length: Option<u16>,
) -> impl Strategy<Value = BandGenerator> {
    arbitrary_band_data(max_head, max_height, max_artin_length)
        .prop_map(|(foot, head, sign)| BandGenerator::try_new(foot, head, sign).unwrap())
}
