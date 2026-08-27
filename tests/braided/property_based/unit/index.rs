use braided::BraidIndex;
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
            &valid::index::test_cases::try_new(None, None),
            |test_case| {
                let data = test_case.data.0;
                let expected_index = test_case.expected_index;

                let braid_index = BraidIndex::try_new(data).unwrap();

                assert_that!(
                    <BraidIndex as Into<u16>>::into(braid_index),
                    eq(expected_index)
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
        .run(&invalid::index::test_cases::try_new(), |test_case| {
            let data = test_case.data;
            let error = test_case.error;

            let invalid_braid_index = match data {
                invalid::index::test_cases::TryNewData::Zero(zero) => BraidIndex::try_new(zero),
                invalid::index::test_cases::TryNewData::InvalidU16(invalid_u16) => {
                    BraidIndex::try_new(invalid_u16)
                }
            };

            assert_that!(*invalid_braid_index, err(eq(error)));

            Ok(())
        })
        .unwrap();
}
