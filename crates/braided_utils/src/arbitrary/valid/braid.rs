use crate::arbitrary::valid::word::arbitrary_word;
use braided::Braid ;
use proptest::{bits::u16, prelude::*};

pub fn arbitrary_braid_with_given_index(braid_index: u16) -> impl Strategy<Value = Braid> {
    arbitrary_word(Some(braid_index), None)
        .prop_map(move |word| Braid::try_new(braid_index, word).clone_unwrap())
}

pub fn arbitrary_braid(max_braid_index: Option<u16>) -> impl Strategy<Value = Braid> {
    (1..=max_braid_index.unwrap_or(u16::MAX)).prop_flat_map(arbitrary_braid_with_given_index)
}
