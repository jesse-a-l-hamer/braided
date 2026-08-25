use braided_utils::arbitrary::valid;
use braided_utils::telemetry::start_tracing;
use googletest::assert_that;
use googletest::matchers::eq;
use proptest::prelude::*;

#[test]
fn band_generator_decomposes_as_expected() {
    start_tracing();
    let mut test_runner = prop::test_runner::TestRunner::default();

    test_runner
        .run(
            &valid::coalescence::test_cases::decompose_band(Some(10), Some(10), Some(10)),
            |test_case| {
                let data = test_case.data.0;
                let expected = test_case.expected;

                assert_that!(data.decompose(), eq(&expected));

                Ok(())
            },
        )
        .unwrap();
}

#[test]
fn letter_decomposes_as_expected() {
    start_tracing();
    let mut test_runner = prop::test_runner::TestRunner::default();

    test_runner
        .run(
            &valid::coalescence::test_cases::decompose_letter(Some(100), Some(100), Some(100)),
            |test_case| {
                let data = test_case.data.0;
                let expected = test_case.expected;

                assert_that!(data.decompose(), eq(&expected));

                Ok(())
            },
        )
        .unwrap();
}

#[test]
fn word_decomposes_as_expected() {
    start_tracing();
    let mut test_runner = prop::test_runner::TestRunner::default();

    test_runner
        .run(
            &valid::coalescence::test_cases::decompose_word(Some(100), Some(100), Some(100), true),
            |test_case| {
                let data = test_case.data.0;
                let expected = test_case.expected;

                assert_that!(data.decompose(), eq(&expected));

                Ok(())
            },
        )
        .unwrap();
}

#[test]
fn braid_decomposes_as_expected() {
    start_tracing();
    let mut test_runner = prop::test_runner::TestRunner::default();

    test_runner
        .run(
            &valid::coalescence::test_cases::decompose_braid(Some(100), Some(100), Some(100), true),
            |test_case| {
                let data = test_case.data.0;
                let expected = test_case.expected;

                assert_that!(data.decompose(), eq(&expected));

                Ok(())
            },
        )
        .unwrap();
}
