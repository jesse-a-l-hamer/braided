use braided::Word;
use braided_utils::arbitrary::valid;
use braided_utils::telemetry::start_tracing;
use googletest::assert_that;
use googletest::matchers::{anything, each, eq, ge, le, ok, predicate};
use proptest::prelude::*;

#[test]
fn partition_into_odd_numbers_works() {
    let mut test_runner = prop::test_runner::TestRunner::default();

    test_runner
        .run(
            &(0..=u16::MAX)
                .prop_flat_map(|partition_value| (Just(partition_value), 1..=partition_value))
                .prop_flat_map(|(partition_value, max_elem)| {
                    (
                        braided_utils::arbitrary::utils::partition_into_odd_numbers(
                            partition_value,
                            max_elem as usize,
                        ),
                        Just(partition_value),
                        Just(max_elem),
                    )
                }),
            |(partition, partition_value, max_elem)| {
                assert_that!(&partition, each(predicate(|&x: &u16| x % 2 == 1)));
                assert_that!(partition.iter().sum::<u16>(), eq(partition_value));
                assert_that!(&partition, each(le(&max_elem)));
                assert_that!(&partition, each(ge(&1)));
                Ok(())
            },
        )
        .unwrap();
}

#[test]
fn valid_inputs_to_try_new_succeed_as_expected() {
    start_tracing();
    let mut test_runner = prop::test_runner::TestRunner::default();

    test_runner
        .run(
            &valid::word::test_cases::try_new(Some(3), Some(3)),
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
