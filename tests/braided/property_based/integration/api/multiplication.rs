//! Integration tests for the multiplication interface.

use braided_utils::arbitrary::valid;
use braided_utils::telemetry::start_tracing;
use googletest::assert_that;
use googletest::matchers::eq;
use proptest::prelude::*;

#[test]
fn multiplication_of_valid_operands_succeeds() {
    start_tracing();
    let mut test_runner = prop::test_runner::TestRunner::default();

    test_runner
        .run(
            &valid::multiplication::test_cases::multiplication(None, None),
            |test_case| {
                let data = test_case.data;
                let expected = test_case.expected;

                assert_that!(data.left * data.right, eq(&expected));

                Ok(())
            },
        )
        .unwrap();
}
