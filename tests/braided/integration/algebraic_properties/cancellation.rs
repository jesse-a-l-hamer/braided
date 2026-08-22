//! Integration tests to ensure algebraic operations cancel as expected.

// use braided::{braid, letter, word};
// use braided_utils::arbitrary::valid;
// use braided_utils::telemetry::start_tracing;
// use googletest::matchers::eq;
// use googletest::{assert_that, expect_that, gtest};
// use proptest::prelude::*;
//
// #[test]
// fn cancelling_multiplication_of_valid_operands_succeeds() {
//     start_tracing();
//     let mut test_runner = prop::test_runner::TestRunner::default();
//
//     test_runner
//         .run(
//             &valid::multiplication::test_cases::cancelling_product(None, None),
//             |test_case| {
//                 let data = test_case.data;
//                 let expected = test_case.expected;
//
//                 assert_that!(data.left * data.right, eq(&expected));
//
//                 Ok(())
//             },
//         )
//         .unwrap();
// }
//
// #[gtest]
// fn multiplication_of_letter_and_letter_cancels_as_expected() {
//     start_tracing();
//     let test_cases = [
//         (letter![1; +], letter![1 => 2; -], word![]),
//         (letter![1; -], letter![1 => 2; +], word![]),
//         (letter![1 => 2; +], letter![1; -], word![]),
//         (letter![1 => 2; -], letter![1; +], word![]),
//     ];
//
//     for (left, right, expected) in test_cases {
//         expect_that!(left * right, eq(&expected));
//     }
// }
//
// #[gtest]
// fn multiplication_of_letter_and_word_cancels_as_expected() {
//     start_tracing();
//     let test_cases = [
//         (
//             letter![1; +],
//             word![[1; -2], [2; 3]],
//             word![[1; -1], [2; 3]],
//         ),
//         (
//             letter![1; -],
//             word![[1 => 2; 2], [2; 3]],
//             word![[1; 1], [2; 3]],
//         ),
//         (
//             letter![1 => 2; +],
//             word![[1; -2], [2; 3]],
//             word![[1; -1], [2; 3]],
//         ),
//         (
//             letter![1 => 2; -],
//             word![[1 => 2; 1], [2; 3]],
//             word![[2; 3]],
//         ),
//     ];
//
//     for (left, right, expected) in test_cases {
//         expect_that!(left * right, eq(&expected));
//     }
// }
//
// #[gtest]
// fn multiplication_of_word_and_letter_cancels_as_expected() {
//     start_tracing();
//     let test_cases = [
//         (
//             word![[2; 3], [1; -2]],
//             letter![1; +],
//             word![[2; 3], [1; -1]],
//         ),
//         (
//             word![[2; 3], [1 => 2; 2]],
//             letter![1; -],
//             word![[2; 3], [1; 1]],
//         ),
//         (
//             word![[2; 3], [1; -2]],
//             letter![1 => 2; +],
//             word![[2; 3], [1; -1]],
//         ),
//         (
//             word![[2; 3], [1 => 2; 1]],
//             letter![1 => 2; -],
//             word![[2; 3]],
//         ),
//     ];
//
//     for (left, right, expected) in test_cases {
//         expect_that!(left * right, eq(&expected));
//     }
// }
//
// #[gtest]
// fn multiplication_of_letter_and_braid_cancels_as_expected() {
//     start_tracing();
//     let test_cases = [
//         (
//             letter![1; +],
//             braid![(); [1; -2], [2; 3]],
//             braid![(); [1; -1], [2; 3]],
//         ),
//         (
//             letter![1; -],
//             braid![(); [1 => 2; 2], [2; 3]],
//             braid![(); [1; 1], [2; 3]],
//         ),
//         (
//             letter![1 => 2; +],
//             braid![(); [1; -2], [2; 3]],
//             braid![(); [1; -1], [2; 3]],
//         ),
//         (
//             letter![1 => 2; -],
//             braid![(); [1 => 2; 1], [2; 3]],
//             braid![(); [2; 3]],
//         ),
//     ];
//
//     for (left, right, expected) in test_cases {
//         expect_that!(left * right, eq(&expected));
//     }
// }
//
// #[gtest]
// fn multiplication_of_braid_and_letter_cancels_as_expected() {
//     start_tracing();
//     let test_cases = [
//         (
//             braid![(); [2; 3], [1; -2]],
//             letter![1; +],
//             braid![(); [2; 3], [1; -1]],
//         ),
//         (
//             braid![(); [2; 3], [1 => 2; 2]],
//             letter![1; -],
//             braid![(); [2; 3], [1; 1]],
//         ),
//         (
//             braid![(); [2; 3], [1; -2]],
//             letter![1 => 2; +],
//             braid![(); [2; 3], [1; -1]],
//         ),
//         (
//             braid![(); [2; 3], [1 => 2; 1]],
//             letter![1 => 2; -],
//             braid![(); [2; 3]],
//         ),
//     ];
//
//     for (left, right, expected) in test_cases {
//         expect_that!(left * right, eq(&expected));
//     }
// }
//
// #[gtest]
// fn multiplication_of_word_and_word_cancels_as_expected() {
//     start_tracing();
//     let test_cases = [
//         (
//             word![[1; 3], [2; -4]],
//             word![[2; 6], [1; -2]],
//             word![[1; 3], [2; 2], [1; -2]],
//         ),
//         (
//             word![[1; 3], [2; -4]],
//             word![[2;4], [1 => 3; -2]],
//             word![[1;3], [1 => 3; -2]],
//         ),
//         (
//             word![[1; 3], [2; -4]],
//             word![[2;4], [1 => 2;-2]],
//             word![[1; 1]],
//         ),
//     ];
//
//     for (left, right, expected) in test_cases {
//         expect_that!(left * right, eq(&expected));
//     }
// }
//
// #[gtest]
// fn multiplication_of_word_and_braid_cancels_as_expected() {
//     start_tracing();
//     let test_cases = [
//         (
//             word![[1; 3], [2; -4]],
//             braid![(); [2;6], [1;-2]],
//             braid![(); [1;3], [2;2], [1;-2]],
//         ),
//         (
//             word![[1; 3], [2; -4]],
//             braid![(); [2;4], [1 => 3; -2]],
//             braid![(); [1;3], [1 => 3; -2]],
//         ),
//         (
//             word![[1; 3], [2; -4]],
//             braid![(); [2;4], [1 => 2;-2]],
//             braid![(3); [1;1]],
//         ),
//     ];
//
//     for (left, right, expected) in test_cases {
//         expect_that!(left * right, eq(&expected));
//     }
// }
//
// #[gtest]
// fn multiplication_of_braid_and_word_cancels_as_expected() {
//     start_tracing();
//     let test_cases = [
//         (
//             braid![(); [1; 3], [2; -4]],
//             word![[2; 6], [1; -2]],
//             braid![(); [1;3], [2;2], [1;-2]],
//         ),
//         (
//             braid![(); [1; 3], [2; -4]],
//             word![[2;4], [1 => 3; -2]],
//             braid![(); [1;3], [1 => 3; -2]],
//         ),
//         (
//             braid![(); [1; 3], [2; -4]],
//             word![[2;4], [1 => 2;-2]],
//             braid![(3); [1;1]],
//         ),
//     ];
//
//     for (left, right, expected) in test_cases {
//         expect_that!(left * right, eq(&expected));
//     }
// }
//
// #[gtest]
// fn multiplication_of_braid_and_braid_cancels_as_expected() {
//     start_tracing();
//     let test_cases = [
//         (
//             braid![(); [1; 3], [2; -4]],
//             braid![(); [2;6], [1;-2]],
//             braid![(); [1;3], [2;2], [1;-2]],
//         ),
//         (
//             braid![(); [1; 3], [2; -4]],
//             braid![(); [2;4], [1 => 3; -2]],
//             braid![(); [1;3], [1 => 3; -2]],
//         ),
//         (
//             braid![(); [1; 3], [2; -4]],
//             braid![(); [2;4], [1 => 2;-2]],
//             braid![(3); [1;1]],
//         ),
//     ];
//
//     for (left, right, expected) in test_cases {
//         expect_that!(left * right, eq(&expected));
//     }
// }
