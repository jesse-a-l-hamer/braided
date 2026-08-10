//! Integration tests to ensure that the `Letter` interface upholds generator interoperability
//! guarantees.

use googletest::matchers::{eq, err, ok};
use googletest::{assert_that, expect_that, gtest};

use braided::{ArtinGenerator, BandGenerator, Letter, Sign, letter};

#[gtest]
fn can_construct_letter_from_any_generator() {}

#[gtest]
fn can_compare_any_two_letters_for_equality() {}

#[gtest]
fn can_convert_artin_letter_to_band_letter() {}

#[gtest]
fn can_convert_valid_band_letter_to_artin_letter() {}

#[gtest]
fn attempting_to_convert_invalid_band_letter_to_artin_fails() {}

#[gtest]
fn can_decompose_band_letter_into_artin_letters() {}
