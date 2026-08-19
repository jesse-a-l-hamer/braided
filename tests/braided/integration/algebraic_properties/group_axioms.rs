//! Integration tests to check that group axioms hold for multiplication of braids.

use braided_utils::telemetry::start_tracing;
use googletest::matchers::{anything, eq, err, is_true, ok};
use googletest::{expect_that, gtest};

use braided::braid;

#[gtest]
fn multiplication_of_braids_with_different_indices_fails() {
    start_tracing();
    let test_cases = [
        (braid![(1);], braid![(2);]),
        (braid![(3);], braid![(4);]),
        (braid![(5);], braid![(6);]),
        (braid![(10);], braid![(11);]),
    ];

    for (left, right) in test_cases {
        expect_that!(*(left * right), err(anything()));
    }
}

#[gtest]
fn multiplication_of_braids_preserves_index() {
    start_tracing();
    let test_cases = [
        (braid![(1);], braid![(1);], braid![(1);].clone_unwrap()),
        (
            braid![(2); [1; 1]],
            braid![(2); [1; 1]],
            braid![(2); [1; 2]].clone_unwrap(),
        ),
        (
            braid![(3); [1 => 2; 1]],
            braid![(3); [2; 1]],
            braid![(3); [1 => 2; 1], [2; 1]].clone_unwrap(),
        ),
        (
            braid![(4); [1; -1], [2; 1]],
            braid![(4); [2; -1]],
            braid![(4); [1; -1]].clone_unwrap(),
        ),
    ];

    for (left, right, expected) in test_cases {
        expect_that!(*(left * right), ok(eq(&expected)));
    }
}

#[gtest]
fn multiplication_of_braids_is_associative() {
    start_tracing();
    let test_cases = [
        (braid![(2);], braid![(2);], braid![(2);]),
        (
            braid![(3); [1; 1]],
            braid![(3); [1; 1]],
            braid![(3); [1; 2]],
        ),
        (
            braid![(4); [1 => 2; 1]],
            braid![(4); [2; 1]],
            braid![(4); [1 => 2; 1], [2; 1]],
        ),
        (
            braid![(5); [1; -1], [2; 1]],
            braid![(5); [2; -1], [3; 1]],
            braid![(5); [1; -1], [2; -1], [3; 1]],
        ),
    ];

    for (left, middle, right) in test_cases {
        expect_that!(
            (&left * &middle) * &right == &left * (&middle * &right),
            is_true()
        );
    }
}

#[gtest]
fn trivial_braid_is_multiplicative_identity() {
    start_tracing();
    let test_cases = [
        braid![(1);],
        braid![(2); [1; 1]],
        braid![(3); [1 => 2; -1]],
        braid![(4); [1; 1], [2; 1]],
        braid![(5); [1 => 2; 1], [2; 1]],
        braid![(6); [1; -1], [2; -1], [3; -1]],
    ];

    for braid in test_cases {
        let trivial = braid![(braid.clone_unwrap().braid_index())];
        expect_that!(&braid * &trivial == braid, is_true());
        expect_that!(&trivial * &braid == braid, is_true());
    }
}

#[gtest]
fn braid_inverse_is_multiplicative_inverse() {
    start_tracing();
    let test_cases = [
        braid![(1);],
        braid![(2); [1; 1]],
        braid![(3); [1 => 2; -1]],
        braid![(4); [1; 1], [2; 1]],
        braid![(5); [1 => 2; 1], [2; 1]],
        braid![(6); [1; -1], [2; -1], [3; -1]],
    ];

    for braid in test_cases {
        let trivial = braid![(braid.clone_unwrap().braid_index())];
        expect_that!(
            &braid * braid.clone_unwrap().inverse() == trivial,
            is_true()
        );
        expect_that!(
            braid.clone_unwrap().inverse() * &braid == trivial,
            is_true()
        );
    }
}
