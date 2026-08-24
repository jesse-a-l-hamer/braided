use braided::Braid;
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
            &valid::braid::test_cases::try_new(None, Some(1000)),
            |test_case| {
                let data = test_case.data;

                let braid = Braid::try_new(data.braid_index, data.word);
                assert_that!(*braid, ok(anything()));

                let braid = braid.clone_unwrap();

                assert_that!(braid.braid_index(), eq(test_case.expected_braid_index));
                assert_that!(braid.word(), eq(&test_case.expected_word));
                assert_that!(braid.letters(), eq(&test_case.expected_letters));
                assert_that!(braid.is_trivial(), eq(test_case.expected_is_trivial));
                assert_that!(braid.letter_length(), eq(test_case.expected_letter_length));
                assert_that!(braid.artin_length(), eq(test_case.expected_artin_length));
                assert_that!(braid.writhe(), eq(test_case.expected_writhe));
                assert_that!(
                    braid.minimal_required_braid_index(),
                    eq(test_case.expected_minimal_required_braid_index),
                );
                Ok(())
            },
        )
        .unwrap();
}
