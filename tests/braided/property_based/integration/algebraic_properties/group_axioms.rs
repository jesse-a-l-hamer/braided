use braided_utils::arbitrary::valid;
use braided_utils::telemetry::start_tracing;
use googletest::assert_that;
use googletest::matchers::eq;
use proptest::prelude::*;

#[test]
fn closure_holds_as_expected() {
    start_tracing();
    let mut test_runner = prop::test_runner::TestRunner::default();

    test_runner
        .run(
            &valid::multiplication::test_cases::closure(Some(10), Some(10)),
            |test_case| {
                let data = test_case.data;
                let expected = test_case.expected;

                assert_that!((data.left * data.right).braid_index(), eq(expected));

                Ok(())
            },
        )
        .unwrap();
}

#[test]
fn associativity_holds_as_expected() {
    start_tracing();
    let mut test_runner = prop::test_runner::TestRunner::default();

    test_runner
        .run(
            &valid::multiplication::test_cases::associativity(Some(10), Some(10)),
            |test_case| {
                let data = test_case.data;
                let a = data.left;
                let b = data.middle;
                let c = data.right;
                let expected = test_case.expected;

                let lhs =
                    valid::multiplication::MulOperand::MulResult(a.clone() * b.clone()) * c.clone();
                let rhs = a * valid::multiplication::MulOperand::MulResult(b * c);

                assert_that!(lhs == rhs, eq(expected));

                Ok(())
            },
        )
        .unwrap();
}

#[test]
fn unitality_holds_as_expected() {
    start_tracing();
    let mut test_runner = prop::test_runner::TestRunner::default();

    test_runner
        .run(
            &valid::multiplication::test_cases::unitality(None, None),
            |test_case| {
                let operand = test_case.data.operand;
                let trivial = test_case.data.trivial;
                let expected = test_case.expected;

                assert_that!(operand.clone() * trivial.clone(), eq(&expected));
                assert_that!(trivial.clone() * operand.clone(), eq(&expected));

                Ok(())
            },
        )
        .unwrap();
}

// #[test]
// fn invertability_holds_as_expected() {
//     start_tracing();
//     let mut test_runner = prop::test_runner::TestRunner::default();
//
//     test_runner
//         .run(
//             &valid::multiplication::test_cases::invertability(None, None),
//             |test_case| {
//                 let operand = test_case.data.operand;
//                 let inverse = test_case.data.inverse;
//                 let expected = test_case.expected;
//
//                 assert_that!(operand.clone() * inverse.clone(), eq(&expected));
//                 assert_that!(inverse * operand, eq(&expected));
//
//                 Ok(())
//             },
//         )
//         .unwrap();
// }
