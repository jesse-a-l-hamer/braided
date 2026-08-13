use crate::arbitrary::valid::{arbitrary_word, arbitrary_word_data};
use braided::{Braid, Sign};
use proptest::prelude::*;

pub fn arbitrary_braid_data_with_given_index(
    braid_index: u16,
) -> impl Strategy<Value = (u16, Vec<(u16, Option<u16>, Sign)>)> {
    arbitrary_word_data(Some(braid_index), None).prop_map(move |word_data| (braid_index, word_data))
}

pub fn arbitrary_braid_with_given_index(braid_index: u16) -> impl Strategy<Value = Braid> {
    arbitrary_word(Some(braid_index), None)
        .prop_map(move |word| Braid::try_new(braid_index, word).clone_unwrap())
}

pub fn arbitrary_braid_data(
    max_braid_index: Option<u16>,
) -> impl Strategy<Value = (u16, Vec<(u16, Option<u16>, Sign)>)> {
    (1..=max_braid_index.unwrap_or(u16::MAX)).prop_flat_map(arbitrary_braid_data_with_given_index)
}

pub fn arbitrary_braid(max_braid_index: Option<u16>) -> impl Strategy<Value = Braid> {
    (1..=max_braid_index.unwrap_or(u16::MAX)).prop_flat_map(arbitrary_braid_with_given_index)
}
