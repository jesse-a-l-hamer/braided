use braided::BandGenerator;
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
            &valid::band::test_cases::try_new(None, None, None),
            |test_case| {
                let data = test_case.data;

                let band_generator = BandGenerator::try_new(data.foot, data.head, data.sign);
                assert_that!(*band_generator, ok(anything()));

                let band_generator = band_generator.unwrap();

                assert_that!(band_generator.foot(), eq(test_case.expected_foot));
                assert_that!(band_generator.head(), eq(test_case.expected_head));
                assert_that!(band_generator.sign(), eq(test_case.expected_sign));
                assert_that!(band_generator.height(), eq(test_case.expected_height));
                assert_that!(band_generator.is_artin(), eq(test_case.expected_is_artin));
                assert_that!(
                    band_generator.artin_length(),
                    eq(test_case.expected_artin_length)
                );
                assert_that!(
                    band_generator.minimal_required_braid_index(),
                    eq(test_case.expected_minimal_required_braid_index),
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
        .run(&invalid::band::test_cases::try_new(), |test_case| {
            let data = test_case.data;
            let error = test_case.error;

            let invalid_band_generator = match data {
                invalid::band::test_cases::TryNewData::FootOnHead(foot, sign) => {
                    BandGenerator::try_new(foot, foot, sign)
                }
                invalid::band::test_cases::TryNewData::FootOverHead { foot, head, sign } => {
                    BandGenerator::try_new(foot, head, sign)
                }
                invalid::band::test_cases::TryNewData::TooTall { foot, head, sign } => {
                    BandGenerator::try_new(foot, head, sign)
                }
                invalid::band::test_cases::TryNewData::InvalidFoot { foot, head, sign } => {
                    match foot {
                        invalid::strand::test_cases::TryNewData::Zero(foot) => {
                            BandGenerator::try_new(foot, head, sign)
                        }
                        invalid::strand::test_cases::TryNewData::InvalidU16(foot) => {
                            BandGenerator::try_new(foot, head, sign)
                        }
                    }
                }
                invalid::band::test_cases::TryNewData::InvalidHead { foot, head, sign } => {
                    match head {
                        invalid::strand::test_cases::TryNewData::Zero(head) => {
                            BandGenerator::try_new(foot, head, sign)
                        }
                        invalid::strand::test_cases::TryNewData::InvalidU16(head) => {
                            BandGenerator::try_new(foot, head, sign)
                        }
                    }
                }
            };

            assert_that!(*invalid_band_generator, err(eq(error)));

            Ok(())
        })
        .unwrap();
}

#[test]
fn from_artin_computes_as_expected() {
    start_tracing();
    let mut test_runner = prop::test_runner::TestRunner::default();

    test_runner
        .run(&valid::band::test_cases::from_artin(None), |test_case| {
            let data = test_case.data.0;

            let band_generator = BandGenerator::from(data);
            assert_that!(band_generator, eq(test_case.expected));

            Ok(())
        })
        .unwrap();
}

#[test]
fn from_letter_computes_as_expected() {
    start_tracing();
    let mut test_runner = prop::test_runner::TestRunner::default();

    test_runner
        .run(
            &valid::band::test_cases::from_letter(None, None, None),
            |test_case| {
                let data = test_case.data.0;

                let band_generator = BandGenerator::from(data);
                assert_that!(band_generator, eq(test_case.expected));

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
            &valid::band::test_cases::inverse(None, None, None),
            |test_case| {
                let data = test_case.data.0;

                assert_that!(data.inverse(), eq(test_case.expected));

                Ok(())
            },
        )
        .unwrap();
}

// #[test]
// fn invalid_coalescence_fails_as_expected() {
//     start_tracing();
//     let mut test_runner = prop::test_runner::TestRunner::default();
//
//     test_runner
//         .run(&invalid::band::test_cases::coalesce(), |test_case| {
//             let data = test_case.data;
//             let error = test_case.error;
//
//             let invalid_band_generator = match data {
//                 invalid::band::test_cases::CoalesceData::NoGenerators(artin_generators) => {
//                     BandGenerator::try_coalesce(&artin_generators)
//                 }
//                 invalid::band::test_cases::CoalesceData::EvenGenerators(artin_generators) => {
//                     BandGenerator::try_coalesce(&artin_generators)
//                 }
//                 invalid::band::test_cases::CoalesceData::TooManyGenerators(artin_generators) => {
//                     BandGenerator::try_coalesce(&artin_generators)
//                 }
//                 invalid::band::test_cases::CoalesceData::IncontiguousSteps(artin_generators) => {
//                     BandGenerator::try_coalesce(&artin_generators)
//                 }
//                 invalid::band::test_cases::CoalesceData::ImbalancedStaircases(artin_generators) => {
//                     BandGenerator::try_coalesce(&artin_generators)
//                 }
//             };
//
//             assert_that!(*invalid_band_generator, err(eq(error)));
//
//             Ok(())
//         })
//         .unwrap();
// }
