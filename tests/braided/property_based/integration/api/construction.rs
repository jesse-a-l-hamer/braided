//! Integration tests to check macro-based construction interface.

use braided::{BraidResult, LetterResult, Sign, WordResult, braid, letter, word};
use braided_utils::arbitrary::invalid;
use braided_utils::arbitrary::valid;
use braided_utils::telemetry::start_tracing;
use googletest::assert_that;
use googletest::matchers::{eq, err};
use proptest::prelude::*;

#[test]
fn valid_inputs_to_letter_macro_succeed_as_expected() {
    start_tracing();
    let mut test_runner = prop::test_runner::TestRunner::default();
    test_runner
        .run(
            &valid::letter::test_cases::letter_macro(None, None, None),
            |test_case| {
                let data = test_case.data;
                let expected = test_case.expected;

                let letter = if let Some(head) = data.head {
                    match data.sign {
                        Sign::Positive => letter![data.foot => head; +],
                        Sign::Negative => letter![data.foot => head; -],
                    }
                } else {
                    match data.sign {
                        Sign::Positive => letter![data.foot; +],
                        Sign::Negative => letter![data.foot; -],
                    }
                };

                assert_that!(letter, eq(expected));
                Ok(())
            },
        )
        .unwrap();
}

#[test]
fn invalid_inputs_to_letter_macro_fail_as_expected() {
    start_tracing();
    let mut test_runner = prop::test_runner::TestRunner::default();
    test_runner
        .run(&invalid::letter::test_cases::letter_macro(), |test_case| {
            let data = test_case.data;
            let error = test_case.error;
            let letter: LetterResult = match data {
                invalid::letter::test_cases::MacroData::InvalidArtinGenerator(
                    invalid_artin_data,
                ) => match invalid_artin_data {
                    invalid::artin::test_cases::TryNewData::InvalidHead(foot, sign) => match sign {
                        Sign::Positive => letter![foot; +],
                        Sign::Negative => letter![foot; -],
                    },
                    invalid::artin::test_cases::TryNewData::InvalidStrand(
                        invalid_strand_data,
                        sign,
                    ) => match invalid_strand_data {
                        invalid::strand::test_cases::TryNewData::Zero(foot) => match sign {
                            Sign::Positive => letter![foot; +],
                            Sign::Negative => letter![foot; -],
                        },
                        invalid::strand::test_cases::TryNewData::InvalidU16(foot) => match sign {
                            Sign::Positive => letter![foot; +],
                            Sign::Negative => letter![foot; -],
                        },
                    },
                },
                invalid::letter::test_cases::MacroData::InvalidBandGenerator(invalid_band_data) => {
                    match invalid_band_data {
                        invalid::band::test_cases::TryNewData::FootOnHead(foot, sign) => match sign
                        {
                            Sign::Positive => letter![foot => foot; +],
                            Sign::Negative => letter![foot => foot; -],
                        },
                        invalid::band::test_cases::TryNewData::FootOverHead {
                            foot,
                            head,
                            sign,
                        } => match sign {
                            Sign::Positive => letter![foot => head; +],
                            Sign::Negative => letter![foot => head; -],
                        },
                        invalid::band::test_cases::TryNewData::TooTall { foot, head, sign } => {
                            match sign {
                                Sign::Positive => letter![foot => head; +],
                                Sign::Negative => letter![foot => head; -],
                            }
                        }
                        invalid::band::test_cases::TryNewData::InvalidFoot { foot, head, sign } => {
                            match foot {
                                invalid::strand::test_cases::TryNewData::Zero(foot) => match sign {
                                    Sign::Positive => letter![foot => head; +],
                                    Sign::Negative => letter![foot => head; -],
                                },
                                invalid::strand::test_cases::TryNewData::InvalidU16(foot) => {
                                    match sign {
                                        Sign::Positive => letter![foot => head; +],
                                        Sign::Negative => letter![foot => head; -],
                                    }
                                }
                            }
                        }
                        invalid::band::test_cases::TryNewData::InvalidHead { foot, head, sign } => {
                            match head {
                                invalid::strand::test_cases::TryNewData::Zero(head) => match sign {
                                    Sign::Positive => letter![foot => head; +],
                                    Sign::Negative => letter![foot => head; -],
                                },
                                invalid::strand::test_cases::TryNewData::InvalidU16(head) => {
                                    match sign {
                                        Sign::Positive => letter![foot => head; +],
                                        Sign::Negative => letter![foot => head; -],
                                    }
                                }
                            }
                        }
                    }
                }
            };

            assert_that!(*letter, err(eq(error)));
            Ok(())
        })
        .unwrap();
}

#[test]
fn valid_inputs_to_word_macro_succeed_as_expected() {
    start_tracing();
    let mut test_runner = prop::test_runner::TestRunner::default();
    test_runner
        .run(
            &valid::word::test_cases::word_macro(None, None),
            |test_case| {
                let data = test_case.data;
                let expected = test_case.expected;

                let word: WordResult = match data {
                    valid::word::test_cases::MacroData::Trivial => word![],
                    valid::word::test_cases::MacroData::NonTrivial(factors) => {
                        let factors = *factors;
                        let feet = (factors[0].0, factors[1].0, factors[2].0);
                        let heads = (factors[0].1, factors[1].1, factors[2].1);
                        let signs = (factors[0].2, factors[1].2, factors[2].2);

                        match heads {
                            (Some(h1), Some(h2), Some(h3)) => word![
                                [feet.0 => h1; signs.0],
                                [feet.1 => h2; signs.1],
                                [feet.2 => h3; signs.2]
                            ],
                            (Some(h1), Some(h2), None) => word![
                                [feet.0 => h1; signs.0], [feet.1 => h2; signs.1], [feet.2; signs.2]
                            ],
                            (Some(h1), None, Some(h3)) => word![
                                [feet.0 => h1; signs.0], [feet.1; signs.1], [feet.2 => h3; signs.2]
                            ],
                            (Some(h1), None, None) => word![
                                [feet.0 => h1; signs.0], [feet.1; signs.1], [feet.2; signs.2]
                            ],
                            (None, Some(h2), Some(h3)) => word![
                                [feet.0; signs.0], [feet.1 => h2; signs.1], [feet.2 => h3; signs.2]
                            ],
                            (None, Some(h2), None) => word![
                                [feet.0; signs.0], [feet.1 => h2; signs.1], [feet.2; signs.2]
                            ],
                            (None, None, Some(h3)) => word![
                                [feet.0; signs.0], [feet.1; signs.1], [feet.2 => h3; signs.2]
                            ],
                            (None, None, None) => {
                                word![[feet.0; signs.0], [feet.1; signs.1], [feet.2; signs.2]]
                            }
                        }
                    }
                };

                assert_that!(word, eq(&expected));
                Ok(())
            },
        )
        .unwrap();
}

#[test]
fn valid_inputs_to_braid_macro_succeed_as_expected() {
    start_tracing();
    let mut test_runner = prop::test_runner::TestRunner::default();
    test_runner
        .run(
            &valid::braid::test_cases::braid_macro(None, None),
            |test_case| {
                let data = test_case.data;
                let expected = test_case.expected;

                let braid: BraidResult = match data {
                    valid::braid::test_cases::MacroData::Trivial { braid_index } => {
                        braid![(braid_index)]
                    }
                    valid::braid::test_cases::MacroData::NonTrivial {
                        braid_index,
                        factors,
                    } => {
                        let feet = (factors[0].0, factors[1].0, factors[2].0);
                        let heads = (factors[0].1, factors[1].1, factors[2].1);
                        let signs = (factors[0].2, factors[1].2, factors[2].2);

                        match braid_index {
                            Some(braid_index) => match heads {
                                (Some(h1), Some(h2), Some(h3)) => braid![(braid_index);
                                    [feet.0 => h1; signs.0],
                                    [feet.1 => h2; signs.1],
                                    [feet.2 => h3; signs.2]
                                ],
                                (Some(h1), Some(h2), None) => braid![(braid_index);
                                    [feet.0 => h1; signs.0],
                                    [feet.1 => h2; signs.1],
                                    [feet.2; signs.2]
                                ],
                                (Some(h1), None, Some(h3)) => braid![(braid_index);
                                    [feet.0 => h1; signs.0],
                                    [feet.1; signs.1],
                                    [feet.2 => h3; signs.2]
                                ],
                                (Some(h1), None, None) => braid![(braid_index);
                                    [feet.0 => h1; signs.0],
                                    [feet.1; signs.1],
                                    [feet.2; signs.2]
                                ],
                                (None, Some(h2), Some(h3)) => braid![(braid_index);
                                    [feet.0; signs.0],
                                    [feet.1 => h2; signs.1],
                                    [feet.2 => h3; signs.2]
                                ],
                                (None, Some(h2), None) => braid![(braid_index);
                                    [feet.0; signs.0],
                                    [feet.1 => h2; signs.1],
                                    [feet.2; signs.2]
                                ],
                                (None, None, Some(h3)) => braid![(braid_index);
                                    [feet.0; signs.0],
                                    [feet.1; signs.1],
                                    [feet.2 => h3; signs.2]
                                ],
                                (None, None, None) => braid![(braid_index);
                                    [feet.0; signs.0],
                                    [feet.1; signs.1],
                                    [feet.2; signs.2]
                                ],
                            },
                            None => match heads {
                                (Some(h1), Some(h2), Some(h3)) => braid![ ();
                                    [feet.0 => h1; signs.0],
                                    [feet.1 => h2; signs.1],
                                    [feet.2 => h3; signs.2]
                                ],
                                (Some(h1), Some(h2), None) => braid![();
                                    [feet.0 => h1; signs.0],
                                    [feet.1 => h2; signs.1],
                                    [feet.2; signs.2]
                                ],
                                (Some(h1), None, Some(h3)) => braid![();
                                    [feet.0 => h1; signs.0],
                                    [feet.1; signs.1],
                                    [feet.2 => h3; signs.2]
                                ],
                                (Some(h1), None, None) => braid![();
                                    [feet.0 => h1; signs.0],
                                    [feet.1; signs.1],
                                    [feet.2; signs.2]
                                ],
                                (None, Some(h2), Some(h3)) => braid![();
                                    [feet.0; signs.0],
                                    [feet.1 => h2; signs.1],
                                    [feet.2 => h3; signs.2]
                                ],
                                (None, Some(h2), None) => braid![();
                                    [feet.0; signs.0],
                                    [feet.1 => h2; signs.1],
                                    [feet.2; signs.2]
                                ],
                                (None, None, Some(h3)) => braid![();
                                    [feet.0; signs.0],
                                    [feet.1; signs.1],
                                    [feet.2 => h3; signs.2]
                                ],
                                (None, None, None) => braid![();
                                    [feet.0; signs.0],
                                    [feet.1; signs.1],
                                    [feet.2; signs.2]
                                ],
                            },
                        }
                    }
                };

                assert_that!(braid, eq(&expected));
                Ok(())
            },
        )
        .unwrap();
}
