//! Integration tests to ensure algebraic operations cancel as expected.

use braided_utils::telemetry::start_tracing;
use googletest::matchers::eq;
use googletest::{expect_that, gtest};

use braided::{braid, letter, word};

#[gtest]
fn multiplication_of_letter_and_letter_cancels_as_expected() {
    start_tracing();
    let test_cases = [
        (letter![1; +], letter![1 => 2; -], word![]),
        (letter![1; -], letter![1 => 2; +], word![]),
        (letter![1 => 2; +], letter![1; -], word![]),
        (letter![1 => 2; -], letter![1; +], word![]),
    ];

    for (left, right, expected) in test_cases {
        expect_that!(left * right, eq(&expected));
    }
}

#[gtest]
fn multiplication_of_letter_and_word_cancels_as_expected() {
    start_tracing();
    let test_cases = [
        (
            letter![1; +],
            word![[1; -2], [2; 3]],
            word![[1; -1], [2; 3]],
        ),
        (
            letter![1; -],
            word![[1 => 2; 2], [2; 3]],
            word![[1; 1], [2; 3]],
        ),
        (
            letter![1 => 2; +],
            word![[1; -2], [2; 3]],
            word![[1; -1], [2; 3]],
        ),
        (
            letter![1 => 2; -],
            word![[1 => 2; 1], [2; 3]],
            word![[2; 3]],
        ),
    ];

    for (left, right, expected) in test_cases {
        expect_that!(left * right, eq(&expected));
    }
}

#[gtest]
fn multiplication_of_word_and_letter_cancels_as_expected() {
    start_tracing();
    let test_cases = [
        (
            word![[2; 3], [1; -2]],
            letter![1; +],
            word![[2; 3], [1; -1]],
        ),
        (
            word![[2; 3], [1 => 2; 2]],
            letter![1; -],
            word![[2; 3], [1; 1]],
        ),
        (
            word![[2; 3], [1; -2]],
            letter![1 => 2; +],
            word![[2; 3], [1; -1]],
        ),
        (
            word![[2; 3], [1 => 2; 1]],
            letter![1 => 2; -],
            word![[2; 3]],
        ),
    ];

    for (left, right, expected) in test_cases {
        expect_that!(left * right, eq(&expected));
    }
}

#[gtest]
fn multiplication_of_letter_and_braid_cancels_as_expected() {
    start_tracing();
    let test_cases = [
        (
            letter![1; +],
            braid![(); [1; -2], [2; 3]],
            braid![(); [1; -1], [2; 3]],
        ),
        (
            letter![1; -],
            braid![(); [1 => 2; 2], [2; 3]],
            braid![(); [1; 1], [2; 3]],
        ),
        (
            letter![1 => 2; +],
            braid![(); [1; -2], [2; 3]],
            braid![(); [1; -1], [2; 3]],
        ),
        (
            letter![1 => 2; -],
            braid![(); [1 => 2; 1], [2; 3]],
            braid![(); [2; 3]],
        ),
    ];

    for (left, right, expected) in test_cases {
        expect_that!(left * right, eq(&expected));
    }
}

#[gtest]
fn multiplication_of_braid_and_letter_cancels_as_expected() {
    start_tracing();
    let test_cases = [
        (
            braid![(); [2; 3], [1; -2]],
            letter![1; +],
            braid![(); [2; 3], [1; -1]],
        ),
        (
            braid![(); [2; 3], [1 => 2; 2]],
            letter![1; -],
            braid![(); [2; 3], [1; 1]],
        ),
        (
            braid![(); [2; 3], [1; -2]],
            letter![1 => 2; +],
            braid![(); [2; 3], [1; -1]],
        ),
        (
            braid![(); [2; 3], [1 => 2; 1]],
            letter![1 => 2; -],
            braid![(); [2; 3]],
        ),
    ];

    for (left, right, expected) in test_cases {
        expect_that!(left * right, eq(&expected));
    }
}

#[gtest]
fn multiplication_of_word_and_word_cancels_as_expected() {
    start_tracing();
    let test_cases = [
        (
            word![[1; 3], [2; -4]],
            word![[2; 6], [1; -2]],
            word![[1; 3], [2; 2], [1; -2]],
        ),
        (
            word![[1; 3], [2; -4]],
            word![[2;4], [1 => 3; -2]],
            word![[1;3], [1 => 3; -2]],
        ),
        (
            word![[1; 3], [2; -4]],
            word![[2;4], [1 => 2;-2]],
            word![[1; 1]],
        ),
    ];

    for (left, right, expected) in test_cases {
        expect_that!(left * right, eq(&expected));
    }
}

#[gtest]
fn multiplication_of_word_and_braid_cancels_as_expected() {
    start_tracing();
    let test_cases = [
        (
            word![[1; 3], [2; -4]],
            braid![(); [2;6], [1;-2]],
            braid![(); [1;3], [2;2], [1;-2]],
        ),
        (
            word![[1; 3], [2; -4]],
            braid![(); [2;4], [1 => 3; -2]],
            braid![(); [1;3], [1 => 3; -2]],
        ),
        (
            word![[1; 3], [2; -4]],
            braid![(); [2;4], [1 => 2;-2]],
            braid![(3); [1;1]],
        ),
    ];

    for (left, right, expected) in test_cases {
        expect_that!(left * right, eq(&expected));
    }
}

#[gtest]
fn multiplication_of_braid_and_word_cancels_as_expected() {
    start_tracing();
    let test_cases = [
        (
            braid![(); [1; 3], [2; -4]],
            word![[2; 6], [1; -2]],
            braid![(); [1;3], [2;2], [1;-2]],
        ),
        (
            braid![(); [1; 3], [2; -4]],
            word![[2;4], [1 => 3; -2]],
            braid![(); [1;3], [1 => 3; -2]],
        ),
        (
            braid![(); [1; 3], [2; -4]],
            word![[2;4], [1 => 2;-2]],
            braid![(3); [1;1]],
        ),
    ];

    for (left, right, expected) in test_cases {
        expect_that!(left * right, eq(&expected));
    }
}

#[gtest]
fn multiplication_of_braid_and_braid_cancels_as_expected() {
    start_tracing();
    let test_cases = [
        (
            braid![(); [1; 3], [2; -4]],
            braid![(); [2;6], [1;-2]],
            braid![(); [1;3], [2;2], [1;-2]],
        ),
        (
            braid![(); [1; 3], [2; -4]],
            braid![(); [2;4], [1 => 3; -2]],
            braid![(); [1;3], [1 => 3; -2]],
        ),
        (
            braid![(); [1; 3], [2; -4]],
            braid![(); [2;4], [1 => 2;-2]],
            braid![(3); [1;1]],
        ),
    ];

    for (left, right, expected) in test_cases {
        expect_that!(left * right, eq(&expected));
    }
}
