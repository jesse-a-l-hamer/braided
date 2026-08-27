//! Integration tests to ensure that the `Letter` interface upholds generator interoperability
//! guarantees.

use braided_utils::telemetry::start_tracing;
use googletest::matchers::{eq, is_true};
use googletest::{expect_that, gtest};

use braided::{Letter, letter};

fn equal_letter_pairs() -> [(Letter, Letter); 2] {
    [
        (letter![1; +].unwrap(), letter![1 => 2; +].unwrap()),
        (letter![2; -].unwrap(), letter![2 => 3; -].unwrap()),
    ]
}

#[gtest]
fn equality_of_letters_does_not_depend_on_generator_type() {
    start_tracing();
    let equal_pairs = equal_letter_pairs();

    for (left, right) in equal_pairs {
        expect_that!(left == right, is_true());
    }
}

#[gtest]
fn decompose_does_not_depend_on_generator_type() {
    start_tracing();
    let equal_pairs = equal_letter_pairs();

    for (left, right) in equal_pairs {
        expect_that!(left.decompose(), eq(&right.decompose()));
    }
}

#[gtest]
fn computation_of_sign_does_not_depend_on_generator_type() {
    start_tracing();
    let equal_pairs = equal_letter_pairs();

    for (left, right) in equal_pairs {
        expect_that!(left.sign(), eq(right.sign()));
    }
}

#[gtest]
fn computation_of_foot_does_not_depend_on_generator_type() {
    start_tracing();
    let equal_pairs = equal_letter_pairs();

    for (left, right) in equal_pairs {
        expect_that!(left.foot(), eq(right.foot()));
    }
}

#[gtest]
fn computation_of_head_does_not_depend_on_generator_type() {
    start_tracing();
    let equal_pairs = equal_letter_pairs();

    for (left, right) in equal_pairs {
        expect_that!(left.head(), eq(right.head()));
    }
}

#[gtest]
fn computation_of_inverse_does_not_depend_on_generator_type() {
    start_tracing();
    let equal_pairs = equal_letter_pairs();

    for (left, right) in equal_pairs {
        expect_that!(left.inverse(), eq(right.inverse()));
    }
}

#[gtest]
fn computation_of_is_artin_does_not_depend_on_generator_type() {
    start_tracing();
    let equal_pairs = equal_letter_pairs();

    for (left, right) in equal_pairs {
        expect_that!(left.is_artin(), eq(right.is_artin()));
    }
}

#[gtest]
fn computation_of_height_does_not_depend_on_generator_type() {
    start_tracing();
    let equal_pairs = equal_letter_pairs();

    for (left, right) in equal_pairs {
        expect_that!(left.height(), eq(right.height()));
    }
}

#[gtest]
fn computation_of_artin_length_does_not_depend_on_generator_type() {
    start_tracing();
    let equal_pairs = equal_letter_pairs();

    for (left, right) in equal_pairs {
        expect_that!(left.artin_length(), eq(right.artin_length()));
    }
}

#[gtest]
fn computation_of_minimal_required_braid_index_does_not_depend_on_generator_type() {
    start_tracing();
    let equal_pairs = equal_letter_pairs();

    for (left, right) in equal_pairs {
        expect_that!(
            left.minimal_required_braid_index(),
            eq(right.minimal_required_braid_index())
        );
    }
}
