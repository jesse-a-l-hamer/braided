use braided::ArtinGenerator;
use braided_utils::arbitrary::{invalid, valid};
use braided_utils::telemetry::start_tracing;
use googletest::assert_that;
use googletest::matchers::{anything, eq, err, ok};
use proptest::prelude::*;

#[test]
fn valid_inputs_to_try_new_succeed_as_expected() {
    start_tracing();
    let mut test_runner = prop::test_runner::TestRunner::default();

    test_runner
        .run(
            &valid::artin::test_cases::try_new(None, None),
            |test_case| {
                let data = test_case.data;

                let artin_generator = ArtinGenerator::try_new(data.foot, data.sign);
                assert_that!(*artin_generator, ok(anything()));

                let artin_generator = artin_generator.unwrap();

                assert_that!(artin_generator.foot(), eq(test_case.expected_foot));
                assert_that!(artin_generator.head(), eq(test_case.expected_head));
                assert_that!(artin_generator.sign(), eq(test_case.expected_sign));
                assert_that!(
                    artin_generator.minimal_required_braid_index(),
                    eq(test_case.expected_minimal_required_braid_index),
                );
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
        .run(&invalid::artin::test_cases::try_new(), |test_case| {
            let data = test_case.data;
            let error = test_case.error;

            let invalid_artin_generator = match data {
                invalid::artin::test_cases::TryNewData::InvalidHead(foot, sign) => {
                    ArtinGenerator::try_new(foot, sign)
                }
                invalid::artin::test_cases::TryNewData::InvalidStrand(invalid_strand, sign) => {
                    match invalid_strand {
                        invalid::strand::test_cases::TryNewData::Zero(foot) => {
                            ArtinGenerator::try_new(foot, sign)
                        }
                        invalid::strand::test_cases::TryNewData::InvalidU16(foot) => {
                            ArtinGenerator::try_new(foot, sign)
                        }
                    }
                }
            };

            assert_that!(*invalid_artin_generator, err(eq(error)));

            Ok(())
        })
        .unwrap();
}

#[test]
fn valid_inputs_to_try_from_band_succeed_as_expected() {
    start_tracing();
    let mut test_runner = prop::test_runner::TestRunner::default();

    test_runner
        .run(
            &valid::artin::test_cases::try_from_band(None),
            |test_case| {
                let data = test_case.data.0;
                let expected = test_case.expected;

                let artin_generator = ArtinGenerator::try_from_band(data);
                assert_that!(artin_generator, eq(expected));

                Ok(())
            },
        )
        .unwrap();
}

#[test]
fn invalid_inputs_to_try_from_band_fail_as_expected() {
    start_tracing();
    let mut test_runner = prop::test_runner::TestRunner::default();

    test_runner
        .run(&invalid::artin::test_cases::try_from_band(), |test_case| {
            let data = test_case.data.0;
            let error = test_case.error;

            let invalid_artin_generator = ArtinGenerator::try_from_band(data);

            assert_that!(*invalid_artin_generator, err(eq(error)));

            Ok(())
        })
        .unwrap();
}

#[test]
fn valid_inputs_to_try_from_letter_succeed_as_expected() {
    start_tracing();
    let mut test_runner = prop::test_runner::TestRunner::default();

    test_runner
        .run(
            &valid::artin::test_cases::try_from_letter(None),
            |test_case| {
                let data = test_case.data.0;
                let expected = test_case.expected;

                let artin_generator = ArtinGenerator::try_from_letter(data);
                assert_that!(artin_generator, eq(expected));

                Ok(())
            },
        )
        .unwrap();
}

#[test]
fn invalid_inputs_to_try_from_letter_fail_as_expected() {
    start_tracing();
    let mut test_runner = prop::test_runner::TestRunner::default();

    test_runner
        .run(
            &invalid::artin::test_cases::try_from_letter(),
            |test_case| {
                let data = test_case.data.0;
                let error = test_case.error;

                let invalid_artin_generator = ArtinGenerator::try_from_letter(data);

                assert_that!(*invalid_artin_generator, err(eq(error)));

                Ok(())
            },
        )
        .unwrap();
}

#[test]
fn inverse_computes_as_expected() {
    start_tracing();
    let mut test_runner = prop::test_runner::TestRunner::default();

    test_runner
        .run(
            &valid::artin::test_cases::inverse(None, None),
            |test_case| {
                let data = test_case.data.0;
                let expected = test_case.expected;

                assert_that!(data.inverse(), eq(expected));

                Ok(())
            },
        )
        .unwrap();
}
