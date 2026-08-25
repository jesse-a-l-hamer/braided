use braided::BandGenerator;
use braided_utils::arbitrary::valid;
use braided_utils::telemetry::start_tracing;
use googletest::assert_that;
use googletest::matchers::eq;
use proptest::prelude::*;

#[test]
fn valid_band_generator_coalescence_succeeds_as_expected() {
    start_tracing();
    let mut test_runner = prop::test_runner::TestRunner::default();

    test_runner
        .run(
            &valid::coalescence::test_cases::coalesce_band(None, None, Some(100)),
            |test_case| {
                let data = test_case.data.0;
                let expected = test_case.expected;

                assert_that!(BandGenerator::try_coalesce(&data[..]), eq(expected));

                Ok(())
            },
        )
        .unwrap();
}

#[test]
fn word_coalescence_computes_as_expected() {
    start_tracing();
    let mut test_runner = prop::test_runner::TestRunner::default();

    test_runner
        .run(
            &valid::coalescence::test_cases::coalesce_word(None, None, Some(100), true),
            |test_case| {
                let data = test_case.data.0;
                let expected = test_case.expected;

                assert_that!(data.coalesce(), eq(&expected));

                Ok(())
            },
        )
        .unwrap();
}

#[test]
fn braid_coalescence_computes_as_expected() {
    start_tracing();
    let mut test_runner = prop::test_runner::TestRunner::default();

    test_runner
        .run(
            &valid::coalescence::test_cases::coalesce_braid(None, None, Some(100), true),
            |test_case| {
                let data = test_case.data.0;
                let expected = test_case.expected;

                assert_that!(data.coalesce(), eq(&expected));

                Ok(())
            },
        )
        .unwrap();
}
