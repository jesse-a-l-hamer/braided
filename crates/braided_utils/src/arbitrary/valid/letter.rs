use crate::arbitrary::valid::u16::ValidPositiveU16Data;
use crate::arbitrary::valid::{
    arbitrary_artin_data, arbitrary_band_data, artin::arbitrary_artin_data_with_given_head,
    band::arbitrary_band_data_with_given_head,
};
use braided::{Letter, Sign};
use proptest::prelude::*;

pub fn arbitrary_letter_data(
    max_head: Option<u16>,
    max_height: Option<u16>,
    max_artin_length: Option<u16>,
) -> impl Strategy<Value = (ValidPositiveU16Data, Option<ValidPositiveU16Data>, Sign)> {
    prop_oneof![
        arbitrary_artin_data(None, max_head.map(|h| h - 1))
            .prop_map(|(foot_idx, sign)| (foot_idx, None, sign)),
        arbitrary_band_data(max_head, max_height, max_artin_length)
            .prop_map(|(foot_idx, head_idx, sign)| (foot_idx, Some(head_idx), sign)),
    ]
}

pub fn arbitrary_letter(
    max_head: Option<u16>,
    max_height: Option<u16>,
    max_artin_length: Option<u16>,
) -> impl Strategy<Value = Letter> {
    arbitrary_letter_data(max_head, max_height, max_artin_length)
        .prop_map(|(foot, head, sign)| Letter::try_new(foot, head, sign).unwrap())
}

pub fn arbitrary_letter_data_with_given_head(
    head: u16,
    max_height: Option<u16>,
    max_artin_length: Option<u16>,
) -> impl Strategy<Value = (ValidPositiveU16Data, Option<ValidPositiveU16Data>, Sign)> {
    prop_oneof![
        arbitrary_artin_data_with_given_head(head).prop_map(|(foot, sign)| (foot, None, sign)),
        arbitrary_band_data_with_given_head(head, max_height, max_artin_length)
            .prop_map(|(foot, head, sign)| (foot, Some(head), sign))
    ]
}

pub fn arbitrary_letter_with_given_head(
    head: u16,
    max_height: Option<u16>,
    max_artin_length: Option<u16>,
) -> impl Strategy<Value = Letter> {
    arbitrary_letter_data_with_given_head(head, max_height, max_artin_length)
        .prop_map(|(foot, head, sign)| Letter::try_new(foot, head, sign).unwrap())
}
