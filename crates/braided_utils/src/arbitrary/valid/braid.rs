use crate::arbitrary::valid;
use braided::{Braid, Sign};
use proptest::prelude::*;

pub fn data_with_given_index(
    braid_index: u16,
) -> impl Strategy<
    Value = (
        valid::u16::Data,
        Vec<(valid::u16::Data, Option<valid::u16::Data>, Sign)>,
    ),
> {
    valid::word::data(Some(braid_index), None).prop_flat_map(move |word_data| {
        (
            valid::u16::data(Some(braid_index), Some(braid_index)),
            Just(word_data),
        )
    })
}

pub fn with_given_index(braid_index: u16) -> impl Strategy<Value = Braid> {
    valid::word::new(Some(braid_index), None)
        .prop_map(move |word| Braid::try_new(braid_index, word).clone_unwrap())
}

pub fn data(
    max_braid_index: Option<u16>,
) -> impl Strategy<
    Value = (
        valid::u16::Data,
        Vec<(valid::u16::Data, Option<valid::u16::Data>, Sign)>,
    ),
> {
    (1..=max_braid_index.unwrap_or(u16::MAX)).prop_flat_map(data_with_given_index)
}

pub fn new(max_braid_index: Option<u16>) -> impl Strategy<Value = Braid> {
    (1..=max_braid_index.unwrap_or(u16::MAX)).prop_flat_map(with_given_index)
}
