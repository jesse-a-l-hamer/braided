use braided::Letter;
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
            &valid::letter::test_cases::try_new(None, None, None),
            |test_case| {
                let data = test_case.data;

                let letter = Letter::try_new(data.foot, data.head, data.sign);
                assert_that!(*letter, ok(anything()));

                let letter = letter.unwrap();

                assert_that!(letter.foot(), eq(test_case.expected_foot));
                assert_that!(letter.head(), eq(test_case.expected_head));
                assert_that!(letter.sign(), eq(test_case.expected_sign));
                assert_that!(letter.height(), eq(test_case.expected_height));
                assert_that!(letter.is_artin(), eq(test_case.expected_is_artin));
                assert_that!(letter.artin_length(), eq(test_case.expected_artin_length));
                assert_that!(
                    letter.minimal_required_braid_index(),
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
        .run(&invalid::letter::test_cases::try_new(), |test_case| {
            let data = test_case.data;
            let error = test_case.error;

            let invalid_letter = match data {
                invalid::letter::test_cases::TryNewData::InvalidArtinGenerator(artin_generator) => {
                    match artin_generator {
                        invalid::artin::test_cases::TryNewData::InvalidHead(foot, sign) => {
                            Letter::try_new(foot, None::<u16>, sign)
                        }
                        invalid::artin::test_cases::TryNewData::InvalidStrand(foot, sign) => {
                            match foot {
                                invalid::strand::test_cases::TryNewData::Zero(foot) => {
                                    Letter::try_new(foot, None::<u16>, sign)
                                }
                                invalid::strand::test_cases::TryNewData::InvalidU16(foot) => {
                                    Letter::try_new(foot, None::<u16>, sign)
                                }
                            }
                        }
                    }
                }
                invalid::letter::test_cases::TryNewData::InvalidBand(band_generator) => {
                    match band_generator {
                        invalid::band::test_cases::TryNewData::FootOnHead(foot, sign) => {
                            Letter::try_new(foot, Some(foot), sign)
                        }
                        invalid::band::test_cases::TryNewData::FootOverHead {
                            foot,
                            head,
                            sign,
                        } => Letter::try_new(foot, Some(head), sign),
                        invalid::band::test_cases::TryNewData::TooTall { foot, head, sign } => {
                            Letter::try_new(foot, Some(head), sign)
                        }
                        invalid::band::test_cases::TryNewData::InvalidFoot { foot, head, sign } => {
                            match foot {
                                invalid::strand::test_cases::TryNewData::Zero(foot) => {
                                    Letter::try_new(foot, Some(head), sign)
                                }
                                invalid::strand::test_cases::TryNewData::InvalidU16(foot) => {
                                    Letter::try_new(foot, Some(head), sign)
                                }
                            }
                        }
                        invalid::band::test_cases::TryNewData::InvalidHead { foot, head, sign } => {
                            match head {
                                invalid::strand::test_cases::TryNewData::Zero(head) => {
                                    Letter::try_new(foot, Some(head), sign)
                                }
                                invalid::strand::test_cases::TryNewData::InvalidU16(head) => {
                                    Letter::try_new(foot, Some(head), sign)
                                }
                            }
                        }
                    }
                }
            };

            assert_that!(*invalid_letter, err(eq(error)));

            Ok(())
        })
        .unwrap();
}
