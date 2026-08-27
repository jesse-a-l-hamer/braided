use braided::Strand;
use braided_utils::arbitrary::invalid;
use braided_utils::arbitrary::valid;
use braided_utils::telemetry::start_tracing;
use googletest::assert_that;
use googletest::matchers::{eq, err};
use proptest::prelude::*;

#[test]
fn valid_inputs_to_try_new_succeed_as_expected() {
    start_tracing();
    let mut test_runner = prop::test_runner::TestRunner::default();

    test_runner
        .run(
            &valid::strand::test_cases::try_new(None, None),
            |test_case| {
                let data = test_case.data.0;
                let expected_index = test_case.expected_index;

                let strand = Strand::try_new(data).unwrap();

                assert_that!(<Strand as Into<u16>>::into(strand), eq(expected_index));

                Ok(())
            },
        )
        .unwrap();
}

#[test]
fn invalid_inputs_to_try_new_fail_as_expected() {
    start_tracing();
    let mut test_runner = prop::test_runner::TestRunner::default();

    test_runner
        .run(&invalid::strand::test_cases::try_new(), |test_case| {
            let data = test_case.data;
            let error = test_case.error;

            let invalid_strand = match data {
                invalid::strand::test_cases::TryNewData::Zero(zero) => Strand::try_new(zero),
                invalid::strand::test_cases::TryNewData::InvalidU16(invalid_u16) => {
                    Strand::try_new(invalid_u16)
                }
            };

            assert_that!(*invalid_strand, err(eq(error)));

            Ok(())
        })
        .unwrap();
}

#[test]
fn valid_addition_succeeds_as_expected() {
    start_tracing();
    let mut test_runner = prop::test_runner::TestRunner::default();

    test_runner
        .run(&valid::strand::test_cases::addition(None), |test_case| {
            let data = test_case.data;
            let expected = test_case.expected;

            let left = data.left;
            let right = data.right;

            assert_that!(left + right, eq(expected));

            Ok(())
        })
        .unwrap();
}

#[test]
fn invalid_addition_fails_as_expected() {
    start_tracing();
    let mut test_runner = prop::test_runner::TestRunner::default();

    test_runner
        .run(&invalid::strand::test_cases::addition(), |test_case| {
            let data = test_case.data;
            let error = test_case.error;

            let left = data.left;
            let right = data.right;

            assert_that!(*(left + right), err(eq(error)));

            Ok(())
        })
        .unwrap();
}

#[test]
fn valid_subtraction_succeeds_as_expected() {
    start_tracing();
    let mut test_runner = prop::test_runner::TestRunner::default();

    test_runner
        .run(&valid::strand::test_cases::subtraction(None), |test_case| {
            let data = test_case.data;
            let expected = test_case.expected;

            let left = data.left;
            let right = data.right;

            assert_that!(left - right, eq(expected));

            Ok(())
        })
        .unwrap();
}

#[test]
fn invalid_subtraction_fails_as_expected() {
    start_tracing();
    let mut test_runner = prop::test_runner::TestRunner::default();

    test_runner
        .run(&invalid::strand::test_cases::subtraction(), |test_case| {
            let data = test_case.data;
            let error = test_case.error;

            let left = data.left;
            let right = data.right;

            assert_that!(*(left - right), err(eq(error)));

            Ok(())
        })
        .unwrap();
}
