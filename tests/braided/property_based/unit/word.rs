use braided::Word;
use braided_utils::arbitrary::valid;
use braided_utils::telemetry::start_tracing;
use googletest::assert_that;
use googletest::matchers::{anything, eq, ok};
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

#[test]
fn valid_inputs_to_try_from_letters_succeed_as_expected() {
    start_tracing();
    let mut test_runner = prop::test_runner::TestRunner::default();

    test_runner
        .run(
            &valid::word::test_cases::try_from_letters(None, Some(1000)),
            |test_case| {
                let data = test_case.data.0;
                let expected = test_case.expected;

                assert_that!(Word::try_from_letters(&data[..]), eq(&expected));

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
            &valid::word::test_cases::inverse(None, Some(1000)),
            |test_case| {
                let data = test_case.data.0;
                let expected = test_case.expected;

                assert_that!(data.inverse(), eq(&expected));

                Ok(())
            },
        )
        .unwrap();
}

// #[test]
// fn invalid_inputs_to_try_new_fail_as_expected() {
//     start_tracing();
//     let mut test_runner = prop::test_runner::TestRunner::default();
//
//     test_runner
//         .run(
//             &invalid::word::test_cases::try_new(Some(100)),
//             |test_case| {
//                 let data = test_case.data;
//                 let error = test_case.error;
//
//                 let invalid_word = match data {
//                     invalid::word::test_cases::TryNewData::TooLong(word_data) => {
//                         Word::try_new(word_data)
//                     }
//                     invalid::word::test_cases::TryNewData::InvalidLetter(word_data) => {
//                         Word::try_new(word_data)
//                     }
//                 };
//
//                 assert_that!(*invalid_word, err(eq(&error)));
//
//                 Ok(())
//             },
//         )
//         .unwrap();
// }

// #[test]
// fn invalid_inputs_to_try_from_letters_fail_as_expected() {
//     start_tracing();
//     let mut test_runner = prop::test_runner::TestRunner::default();
//
//     test_runner
//         .run(
//             &invalid::word::test_cases::try_from_letters(),
//             |test_case| {
//                 let data = test_case.data.0;
//                 let error = test_case.error;
//
//                 let invalid_word = Word::try_from_letters(&data[..]);
//
//                 assert_that!(*invalid_word, err(eq(&error)));
//
//                 Ok(())
//             },
//         )
//         .unwrap();
// }
