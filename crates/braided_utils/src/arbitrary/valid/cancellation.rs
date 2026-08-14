use crate::arbitrary::valid::multiplication::{
    MulOperand, MulResult, arbitrary_mul_operands_with_product_from_letters,
    arbitrary_non_cancelling_mul_operands_with_product_as_letters,
};
use crate::arbitrary::valid::word::arbitrary_vector_of_letters_with_given_artin_length;
use proptest::prelude::*;

pub fn arbitrary_cancelling_mul_operands_with_product(
    max_braid_index: Option<u16>,
    max_artin_length: Option<u16>,
) -> impl Strategy<Value = (MulOperand, MulOperand, MulResult)> {
    arbitrary_non_cancelling_mul_operands_with_product_as_letters(max_braid_index, max_artin_length)
        .prop_filter(
            "Braid index must be greater than 2 to construct testable cancelling products.",
            |(braid_index, _, _, _)| *braid_index > 2u16,
        )
        .prop_flat_map(
            move |(braid_index, lhs_letters, rhs_letters, product_letters)| {
                let max_artin_length = max_artin_length.unwrap_or(u16::MAX);
                let lhs_length: u16 = lhs_letters.len().try_into().unwrap();
                let rhs_length: u16 = rhs_letters.len().try_into().unwrap();
                let max_cancelling_length =
                    *[max_artin_length - lhs_length, max_artin_length - rhs_length]
                        .iter()
                        .min()
                        .unwrap();
                (
                    Just(braid_index),
                    Just(lhs_letters),
                    Just(rhs_letters),
                    Just(product_letters),
                    (1..=max_cancelling_length).prop_flat_map(move |cancelling_length| {
                        arbitrary_vector_of_letters_with_given_artin_length(
                            cancelling_length,
                            Some(braid_index),
                        )
                    }),
                )
            },
        )
        .prop_flat_map(
            |(braid_index, lhs_letters, rhs_letters, product_letters, cancelling_letters)| {
                let inverse_cancelling_letters = cancelling_letters
                    .iter()
                    .rev()
                    .map(|l| l.inverse())
                    .collect();
                arbitrary_mul_operands_with_product_from_letters(
                    braid_index,
                    [lhs_letters, cancelling_letters.clone()].concat(),
                    [inverse_cancelling_letters, rhs_letters].concat(),
                    product_letters,
                )
            },
        )
}
