//! Integration tests to check macro-based construction interface.

use braided::{
    Braid, BraidResult, BraidValidationError, LetterResult, Sign, Word, WordResult,
    WordValidationError, braid, letter, word,
};
use braided_utils::arbitrary::invalid;
use braided_utils::arbitrary::valid;
use braided_utils::telemetry::start_tracing;
use googletest::matchers::{eq, err};
use googletest::{assert_that, expect_that, gtest};
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

// #[test]
// fn invalid_inputs_to_word_macro_fail_as_expected() {
//     start_tracing();
//     let mut test_runner = prop::test_runner::TestRunner::default();
//     test_runner
//         .run(&invalid::word::test_cases::word_macro(), |test_case| {
//             let data = test_case.data;
//             let error = test_case.error;
//
//             let word: WordResult = match data {
//                 invalid::word::test_cases::MacroData::ExponentFailsISizeCoercion(factor) => {
//                     match factor.1 {
//                         Some(head) => word![[factor.0 => head; factor.2]],
//                         None => word![[factor.0; factor.2]],
//                     }
//                 }
//                 invalid::word::test_cases::MacroData::ExponentFailsU16Coercion(factor) => {
//                     match factor.1 {
//                         Some(head) => word![[factor.0 => head; factor.2]],
//                         None => word![[factor.0; factor.2]],
//                     }
//                 }
//                 invalid::word::test_cases::MacroData::InvalidLetter(factors)
//                 | invalid::word::test_cases::MacroData::TooLong(factors) => {
//                     let feet = (factors[0].0, factors[1].0, factors[2].0);
//                     let heads = (factors[0].1, factors[1].1, factors[2].1);
//                     let signs = (factors[0].2, factors[1].2, factors[2].2);
//
//                     match heads {
//                         (Some(h1), Some(h2), Some(h3)) => word![
//                             [feet.0 => h1; signs.0],
//                             [feet.1 => h2; signs.1],
//                             [feet.2 => h3; signs.2]
//                         ],
//                         (Some(h1), Some(h2), None) => word![
//                             [feet.0 => h1; signs.0], [feet.1 => h2; signs.1], [feet.2; signs.2]
//                         ],
//                         (Some(h1), None, Some(h3)) => word![
//                             [feet.0 => h1; signs.0], [feet.1; signs.1], [feet.2 => h3; signs.2]
//                         ],
//                         (Some(h1), None, None) => word![
//                             [feet.0 => h1; signs.0], [feet.1; signs.1], [feet.2; signs.2]
//                         ],
//                         (None, Some(h2), Some(h3)) => word![
//                             [feet.0; signs.0], [feet.1 => h2; signs.1], [feet.2 => h3; signs.2]
//                         ],
//                         (None, Some(h2), None) => word![
//                             [feet.0; signs.0], [feet.1 => h2; signs.1], [feet.2; signs.2]
//                         ],
//                         (None, None, Some(h3)) => word![
//                             [feet.0; signs.0], [feet.1; signs.1], [feet.2 => h3; signs.2]
//                         ],
//                         (None, None, None) => {
//                             word![[feet.0; signs.0], [feet.1; signs.1], [feet.2; signs.2]]
//                         }
//                     }
//                 }
//             };
//
//             assert_that!(*word, err(eq(&error)));
//             Ok(())
//         })
//         .unwrap();
// }

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

// #[test]
// fn invalid_inputs_to_braid_macro_fail_as_expected() {
//     start_tracing();
//     let mut test_runner = prop::test_runner::TestRunner::default();
//     test_runner
//         .run(
//             &invalid::braid::test_cases::braid_macro(None, None),
//             |test_case| {
//                 let data = test_case.data;
//                 let error = test_case.error;
//
//                 let braid: BraidResult = match data {
//                     invalid::braid::test_cases::MacroData::IndexTooSmall(braid_index, factors) => {
//                         let feet = (factors[0].0, factors[1].0, factors[2].0);
//                         let heads = (factors[0].1, factors[1].1, factors[2].1);
//                         let signs = (factors[0].2, factors[1].2, factors[2].2);
//
//                         match heads {
//                             (Some(h1), Some(h2), Some(h3)) => braid![(braid_index);
//                                 [feet.0 => h1; signs.0],
//                                 [feet.1 => h2; signs.1],
//                                 [feet.2 => h3; signs.2]
//                             ],
//                             (Some(h1), Some(h2), None) => braid![(braid_index);
//                                 [feet.0 => h1; signs.0],
//                                 [feet.1 => h2; signs.1],
//                                 [feet.2; signs.2]
//                             ],
//                             (Some(h1), None, Some(h3)) => braid![(braid_index);
//                                 [feet.0 => h1; signs.0],
//                                 [feet.1; signs.1],
//                                 [feet.2 => h3; signs.2]
//                             ],
//                             (Some(h1), None, None) => braid![(braid_index);
//                                 [feet.0 => h1; signs.0],
//                                 [feet.1; signs.1],
//                                 [feet.2; signs.2]
//                             ],
//                             (None, Some(h2), Some(h3)) => braid![(braid_index);
//                                 [feet.0; signs.0],
//                                 [feet.1 => h2; signs.1],
//                                 [feet.2 => h3; signs.2]
//                             ],
//                             (None, Some(h2), None) => braid![(braid_index);
//                                 [feet.0; signs.0],
//                                 [feet.1 => h2; signs.1],
//                                 [feet.2; signs.2]
//                             ],
//                             (None, None, Some(h3)) => braid![(braid_index);
//                                 [feet.0; signs.0],
//                                 [feet.1; signs.1],
//                                 [feet.2 => h3; signs.2]
//                             ],
//                             (None, None, None) => braid![(braid_index);
//                                 [feet.0; signs.0],
//                                 [feet.1; signs.1],
//                                 [feet.2; signs.2]
//                             ],
//                         }
//                     }
//                     invalid::braid::test_cases::MacroData::InvalidIndex(braid_index, factors) => {
//                         match braid_index {
//                             invalid::index::test_cases::TryNewData::Zero(braid_index) => {
//                                 let feet = (factors[0].0, factors[1].0, factors[2].0);
//                                 let heads = (factors[0].1, factors[1].1, factors[2].1);
//                                 let signs = (factors[0].2, factors[1].2, factors[2].2);
//
//                                 match heads {
//                                     (Some(h1), Some(h2), Some(h3)) => braid![(braid_index);
//                                         [feet.0 => h1; signs.0],
//                                         [feet.1 => h2; signs.1],
//                                         [feet.2 => h3; signs.2]
//                                     ],
//                                     (Some(h1), Some(h2), None) => braid![(braid_index);
//                                         [feet.0 => h1; signs.0],
//                                         [feet.1 => h2; signs.1],
//                                         [feet.2; signs.2]
//                                     ],
//                                     (Some(h1), None, Some(h3)) => braid![(braid_index);
//                                         [feet.0 => h1; signs.0],
//                                         [feet.1; signs.1],
//                                         [feet.2 => h3; signs.2]
//                                     ],
//                                     (Some(h1), None, None) => braid![(braid_index);
//                                         [feet.0 => h1; signs.0],
//                                         [feet.1; signs.1],
//                                         [feet.2; signs.2]
//                                     ],
//                                     (None, Some(h2), Some(h3)) => braid![(braid_index);
//                                         [feet.0; signs.0],
//                                         [feet.1 => h2; signs.1],
//                                         [feet.2 => h3; signs.2]
//                                     ],
//                                     (None, Some(h2), None) => braid![(braid_index);
//                                         [feet.0; signs.0],
//                                         [feet.1 => h2; signs.1],
//                                         [feet.2; signs.2]
//                                     ],
//                                     (None, None, Some(h3)) => braid![(braid_index);
//                                         [feet.0; signs.0],
//                                         [feet.1; signs.1],
//                                         [feet.2 => h3; signs.2]
//                                     ],
//                                     (None, None, None) => braid![(braid_index);
//                                         [feet.0; signs.0],
//                                         [feet.1; signs.1],
//                                         [feet.2; signs.2]
//                                     ],
//                                 }
//                             }
//                             invalid::index::test_cases::TryNewData::InvalidU16(braid_index) => {
//                                 let feet = (factors[0].0, factors[1].0, factors[2].0);
//                                 let heads = (factors[0].1, factors[1].1, factors[2].1);
//                                 let signs = (factors[0].2, factors[1].2, factors[2].2);
//
//                                 match heads {
//                                     (Some(h1), Some(h2), Some(h3)) => braid![(braid_index);
//                                         [feet.0 => h1; signs.0],
//                                         [feet.1 => h2; signs.1],
//                                         [feet.2 => h3; signs.2]
//                                     ],
//                                     (Some(h1), Some(h2), None) => braid![(braid_index);
//                                         [feet.0 => h1; signs.0],
//                                         [feet.1 => h2; signs.1],
//                                         [feet.2; signs.2]
//                                     ],
//                                     (Some(h1), None, Some(h3)) => braid![(braid_index);
//                                         [feet.0 => h1; signs.0],
//                                         [feet.1; signs.1],
//                                         [feet.2 => h3; signs.2]
//                                     ],
//                                     (Some(h1), None, None) => braid![(braid_index);
//                                         [feet.0 => h1; signs.0],
//                                         [feet.1; signs.1],
//                                         [feet.2; signs.2]
//                                     ],
//                                     (None, Some(h2), Some(h3)) => braid![(braid_index);
//                                         [feet.0; signs.0],
//                                         [feet.1 => h2; signs.1],
//                                         [feet.2 => h3; signs.2]
//                                     ],
//                                     (None, Some(h2), None) => braid![(braid_index);
//                                         [feet.0; signs.0],
//                                         [feet.1 => h2; signs.1],
//                                         [feet.2; signs.2]
//                                     ],
//                                     (None, None, Some(h3)) => braid![(braid_index);
//                                         [feet.0; signs.0],
//                                         [feet.1; signs.1],
//                                         [feet.2 => h3; signs.2]
//                                     ],
//                                     (None, None, None) => braid![(braid_index);
//                                         [feet.0; signs.0],
//                                         [feet.1; signs.1],
//                                         [feet.2; signs.2]
//                                     ],
//                                 }
//                             }
//                         }
//                     }
//                     invalid::braid::test_cases::MacroData::InvalidWord(braid_index, factors) => {
//                         match braid_index {
//                             None => match factors {
//                                 invalid::word::test_cases::MacroData::ExponentFailsISizeCoercion(
//                                     factor
//                                 ) => {
//                                     match factor.1 {
//                                         Some(head) => braid![(); [factor.0 => head; factor.2]],
//                                         None => braid![(); [factor.0; factor.2]],
//                                     }
//                                 }
//                                 invalid::word::test_cases::MacroData::ExponentFailsU16Coercion(
//                                     factor
//                                 ) => {
//                                     match factor.1 {
//                                         Some(head) => braid![(); [factor.0 => head; factor.2]],
//                                         None => braid![(); [factor.0; factor.2]],
//                                     }
//                                 }
//                                 invalid::word::test_cases::MacroData::InvalidLetter(factors)
//                                 | invalid::word::test_cases::MacroData::TooLong(factors) => {
//                                     let feet = (factors[0].0, factors[1].0, factors[2].0);
//                                     let heads = (factors[0].1, factors[1].1, factors[2].1);
//                                     let signs = (factors[0].2, factors[1].2, factors[2].2);
//
//                                     match heads {
//                                         (Some(h1), Some(h2), Some(h3)) => braid![ ();
//                                             [feet.0 => h1; signs.0],
//                                             [feet.1 => h2; signs.1],
//                                             [feet.2 => h3; signs.2]
//                                         ],
//                                         (Some(h1), Some(h2), None) => braid![();
//                                             [feet.0 => h1; signs.0],
//                                             [feet.1 => h2; signs.1],
//                                             [feet.2; signs.2]
//                                         ],
//                                         (Some(h1), None, Some(h3)) => braid![();
//                                             [feet.0 => h1; signs.0],
//                                             [feet.1; signs.1],
//                                             [feet.2 => h3; signs.2]
//                                         ],
//                                         (Some(h1), None, None) => braid![();
//                                             [feet.0 => h1; signs.0],
//                                             [feet.1; signs.1],
//                                             [feet.2; signs.2]
//                                         ],
//                                         (None, Some(h2), Some(h3)) => braid![();
//                                             [feet.0; signs.0],
//                                             [feet.1 => h2; signs.1],
//                                             [feet.2 => h3; signs.2]
//                                         ],
//                                         (None, Some(h2), None) => braid![();
//                                             [feet.0; signs.0],
//                                             [feet.1 => h2; signs.1],
//                                             [feet.2; signs.2]
//                                         ],
//                                         (None, None, Some(h3)) => braid![();
//                                             [feet.0; signs.0],
//                                             [feet.1; signs.1],
//                                             [feet.2 => h3; signs.2]
//                                         ],
//                                         (None, None, None) => braid![();
//                                             [feet.0; signs.0],
//                                             [feet.1; signs.1],
//                                             [feet.2; signs.2]
//                                         ],
//                                     }
//                                 }
//                             },
//                             Some(braid_index) => match factors {
//                                 invalid::word::test_cases::MacroData::ExponentFailsISizeCoercion(
//                                     factor
//                                 ) => {
//                                     match factor.1 {
//                                         Some(head) => braid![(braid_index);
//                                             [factor.0 => head; factor.2]
//                                         ],
//                                         None => braid![(braid_index); [factor.0; factor.2]],
//                                     }
//                                 }
//                                 invalid::word::test_cases::MacroData::ExponentFailsU16Coercion(
//                                     factor
//                                 ) => {
//                                     match factor.1 {
//                                         Some(head) => braid![(braid_index);
//                                             [factor.0 => head; factor.2]
//                                         ],
//                                         None => braid![(braid_index); [factor.0; factor.2]],
//                                     }
//                                 }
//                                 invalid::word::test_cases::MacroData::InvalidLetter(factors)
//                                 | invalid::word::test_cases::MacroData::TooLong(factors) => {
//                                     let feet = (factors[0].0, factors[1].0, factors[2].0);
//                                     let heads = (factors[0].1, factors[1].1, factors[2].1);
//                                     let signs = (factors[0].2, factors[1].2, factors[2].2);
//
//                                     match heads {
//                                         (Some(h1), Some(h2), Some(h3)) => braid![(braid_index);
//                                             [feet.0 => h1; signs.0],
//                                             [feet.1 => h2; signs.1],
//                                             [feet.2 => h3; signs.2]
//                                         ],
//                                         (Some(h1), Some(h2), None) => braid![(braid_index);
//                                             [feet.0 => h1; signs.0],
//                                             [feet.1 => h2; signs.1],
//                                             [feet.2; signs.2]
//                                         ],
//                                         (Some(h1), None, Some(h3)) => braid![(braid_index);
//                                             [feet.0 => h1; signs.0],
//                                             [feet.1; signs.1],
//                                             [feet.2 => h3; signs.2]
//                                         ],
//                                         (Some(h1), None, None) => braid![(braid_index);
//                                             [feet.0 => h1; signs.0],
//                                             [feet.1; signs.1],
//                                             [feet.2; signs.2]
//                                         ],
//                                         (None, Some(h2), Some(h3)) => braid![(braid_index);
//                                             [feet.0; signs.0],
//                                             [feet.1 => h2; signs.1],
//                                             [feet.2 => h3; signs.2]
//                                         ],
//                                         (None, Some(h2), None) => braid![(braid_index);
//                                             [feet.0; signs.0],
//                                             [feet.1 => h2; signs.1],
//                                             [feet.2; signs.2]
//                                         ],
//                                         (None, None, Some(h3)) => braid![(braid_index);
//                                             [feet.0; signs.0],
//                                             [feet.1; signs.1],
//                                             [feet.2 => h3; signs.2]
//                                         ],
//                                         (None, None, None) => braid![(braid_index);
//                                             [feet.0; signs.0],
//                                             [feet.1; signs.1],
//                                             [feet.2; signs.2]
//                                         ],
//                                     }
//                                 }
//                             },
//                         }
//                     }
//                 };
//
//                 assert_that!(*braid, err(eq(&error)));
//                 Ok(())
//             },
//         )
//         .unwrap();
// }

#[gtest]
fn macro_word_fails_to_construct_invalid_words() {
    start_tracing();
    let invalid_words: [(WordResult, WordValidationError, &'static str); 6] = [
        (
            word![[-1; 1], [1; 2], [2 => 5; -3]],
            Word::try_new(
                [
                    vec![(-1, None, Sign::Positive); 1],
                    vec![(1, None, Sign::Positive); 2],
                    vec![(2, Some(5), Sign::Negative); 3],
                ]
                .concat(),
            )
            .clone_unwrap_err(),
            "test case 1",
        ),
        (
            word![[1; 2], [0 => 4; -2], [2 => 5; -3]],
            Word::try_new(
                [
                    vec![(1, None, Sign::Positive); 2],
                    vec![(0, Some(4), Sign::Negative); 2],
                    vec![(2, Some(5), Sign::Negative); 3],
                ]
                .concat(),
            )
            .clone_unwrap_err(),
            "test case 2",
        ),
        (
            word![[1; 2], [2 => 5; -3], [u16::MAX as u32 + 1; 2]],
            Word::try_new(
                [
                    vec![(1, None, Sign::Positive); 2],
                    vec![(2, Some(5), Sign::Negative); 3],
                    vec![(u16::MAX as u32 + 1, None, Sign::Positive); 2],
                ]
                .concat(),
            )
            .clone_unwrap_err(),
            "test case 3",
        ),
        (
            word![[4 => 1; 3], [1; 2], [2 => 5; -3]],
            Word::try_new(
                [
                    vec![(4, Some(1), Sign::Positive); 3],
                    vec![(1, None, Sign::Positive); 2],
                    vec![(2, Some(5), Sign::Negative); 3],
                ]
                .concat(),
            )
            .clone_unwrap_err(),
            "test case 4",
        ),
        (
            word![[1; u16::MAX as u32 + 1]],
            WordValidationError::from(
                <u32 as TryInto<u16>>::try_into(u16::MAX as u32 + 1).unwrap_err(),
            ),
            "test case 5",
        ),
        (
            word![[1 => 3; (u16::MAX as u32).div_euclid(3)], [3; -1]],
            Word::try_from_letters(
                &[
                    vec![letter![1 => 3; +].unwrap(); (u16::MAX as usize).div_euclid(3)],
                    vec![letter![3; -].unwrap(); 1],
                ]
                .concat(),
            )
            .clone_unwrap_err(),
            "test case 6",
        ),
    ];
    for (invalid_word, error, label) in invalid_words {
        expect_that!(*invalid_word, err(eq(&error)), "{label}")
    }
}

#[gtest]
fn macro_braid_fails_to_construct_invalid_braids() {
    start_tracing();
    let invalid_braids: [(BraidResult, BraidValidationError); 10] = [
        (
            braid![(1); [1; 1]],
            Braid::try_from_data(Some(1), word![[1; 1]].clone_unwrap()).clone_unwrap_err(),
        ),
        (
            braid![(-1); [1 => 3; 2], [2; -4], [3 => 4; 3]],
            Braid::try_from_data(
                Some(-1),
                word![[1 => 3; 2], [2; -4], [3 => 4; 3]].clone_unwrap(),
            )
            .clone_unwrap_err(),
        ),
        (
            braid![(0);[1 => 3; 2], [2; -4], [3 => 4; 3]],
            Braid::try_from_data(
                Some(0),
                word![[1 => 3; 2], [2; -4], [3 => 4; 3]].clone_unwrap(),
            )
            .clone_unwrap_err(),
        ),
        (
            braid![(u16::MAX as u32 + 1);[1 => 3; 2], [2; -4], [3 => 4; 3]],
            Braid::try_from_data(
                Some(u16::MAX as u32 + 1),
                word![[1 => 3; 2], [2; -4], [3 => 4; 3]].clone_unwrap(),
            )
            .clone_unwrap_err(),
        ),
        (
            braid![();[-1; 1], [1; 2], [2 => 5; -3]],
            Braid::try_from_data(
                None::<u16>,
                [
                    vec![(-1, None, Sign::Positive); 1],
                    vec![(1, None, Sign::Positive); 2],
                    vec![(2, Some(5), Sign::Negative); 3],
                ]
                .concat(),
            )
            .clone_unwrap_err(),
        ),
        (
            braid![();[1; 2], [0 => 4; -2], [2 => 5; -3]],
            Braid::try_from_data(
                None::<u16>,
                [
                    vec![(1, None, Sign::Positive); 2],
                    vec![(0, Some(4), Sign::Negative); 2],
                    vec![(2, Some(5), Sign::Negative); 3],
                ]
                .concat(),
            )
            .clone_unwrap_err(),
        ),
        (
            braid![();[1; 2], [2 => 5; -3], [u16::MAX as u32 + 1; 2]],
            Braid::try_from_data(
                None::<u16>,
                [
                    vec![(1, None, Sign::Positive); 2],
                    vec![(2, Some(5), Sign::Negative); 3],
                    vec![(u16::MAX as u32 + 1, None, Sign::Positive); 2],
                ]
                .concat(),
            )
            .clone_unwrap_err(),
        ),
        (
            braid![();[4 => 1; 3], [1; 2], [2 => 5; -3]],
            Braid::try_from_data(
                None::<u16>,
                [
                    vec![(4, Some(1), Sign::Positive); 3],
                    vec![(1, None, Sign::Positive); 2],
                    vec![(2, Some(5), Sign::Negative); 3],
                ]
                .concat(),
            )
            .clone_unwrap_err(),
        ),
        (
            braid![();[1; u16::MAX as u32 + 1]],
            BraidValidationError::WordValidation(WordValidationError::from(
                <u32 as TryInto<u16>>::try_into(u16::MAX as u32 + 1).unwrap_err(),
            )),
        ),
        (
            braid![();[1 => 3; u16::MAX as u32 - 1], [3; -2]],
            Braid::try_from_data(
                None::<u16>,
                [vec![(1, Some(3), Sign::Positive); u16::MAX as usize - 1]].concat(),
            )
            .clone_unwrap_err(),
        ),
    ];
    for (invalid_braid, error) in invalid_braids {
        expect_that!(*invalid_braid, err(eq(&error)));
    }
}
