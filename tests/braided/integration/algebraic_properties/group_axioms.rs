//! Integration tests to check that group axioms hold for multiplication of braids.

use braided::braid;
use braided_utils::arbitrary::valid;
use braided_utils::telemetry::start_tracing;
use googletest::matchers::{anything, eq, err, is_true, ok};
use googletest::{assert_that, expect_that, gtest};
use proptest::prelude::*;

#[test]
fn closure_holds_as_expected() {
    start_tracing();
    let mut test_runner = prop::test_runner::TestRunner::default();

    test_runner
        .run(
            &valid::multiplication::test_cases::closure(Some(10), Some(10)),
            |test_case| {
                let data = test_case.data;
                let expected = test_case.expected;

                assert_that!((data.left * data.right).braid_index(), eq(expected));

                Ok(())
            },
        )
        .unwrap();
}

#[test]
fn associativity_holds_as_expected() {
    start_tracing();
    let mut test_runner = prop::test_runner::TestRunner::default();

    test_runner
        .run(
            &valid::multiplication::test_cases::associativity(Some(10), Some(10)),
            |test_case| {
                let data = test_case.data;
                let a = data.left;
                let b = data.middle;
                let c = data.right;
                let expected = test_case.expected;

                let lhs =
                    valid::multiplication::MulOperand::MulResult(a.clone() * b.clone()) * c.clone();
                let rhs = a * valid::multiplication::MulOperand::MulResult(b * c);

                assert_that!(lhs == rhs, eq(expected));

                Ok(())
            },
        )
        .unwrap();
}

#[test]
fn unitality_holds_as_expected() {
    start_tracing();
    let mut test_runner = prop::test_runner::TestRunner::default();

    test_runner
        .run(
            &valid::multiplication::test_cases::unitality(None, None),
            |test_case| {
                let operand = test_case.data.operand;
                let trivial = test_case.data.trivial;
                let expected = test_case.expected;

                assert_that!(operand.clone() * trivial.clone(), eq(&expected));
                assert_that!(trivial.clone() * operand.clone(), eq(&expected));

                Ok(())
            },
        )
        .unwrap();
}

#[test]
fn invertability_holds_as_expected() {
    start_tracing();
    let mut test_runner = prop::test_runner::TestRunner::default();

    test_runner
        .run(
            &valid::multiplication::test_cases::invertability(None, None),
            |test_case| {
                let operand = test_case.data.operand;
                let inverse = test_case.data.inverse;
                let expected = test_case.expected;

                assert_that!(operand.clone() * inverse.clone(), eq(&expected));
                assert_that!(inverse * operand, eq(&expected));

                Ok(())
            },
        )
        .unwrap();
}

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
