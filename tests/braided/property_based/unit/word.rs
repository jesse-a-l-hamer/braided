use braided::Word;
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
            &valid::word::test_cases::try_new(None, Some(1000)),
            |test_case| {
                let data = test_case.data.0;

                let word = Word::try_new(data);
                assert_that!(*word, ok(anything()));

                let word = word.clone_unwrap();

                assert_that!(word.letters(), eq(&test_case.expected_letters));
                assert_that!(word.is_trivial(), eq(test_case.expected_is_trivial));
                assert_that!(word.letter_length(), eq(test_case.expected_letter_length));
                assert_that!(word.artin_length(), eq(test_case.expected_artin_length));
                assert_that!(
                    word.minimal_required_braid_index(),
                    eq(test_case.expected_minimal_required_braid_index),
                );
                Ok(())
            },
        )
        .unwrap();
}
