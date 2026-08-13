//! Integration tests verifying that band decomposition works as expected.

use crate::telemetry::start_tracing;
use googletest::matchers::eq;
use googletest::{expect_that, gtest};

use braided::{braid, word};

#[gtest]
fn decomposition_works_as_expected_on_words() {
    start_tracing();
    let test_cases = [
        (
            word![[1 => 3; 1], [2; -1], [1; 1]].clone_unwrap(),
            word![[1; -1], [2; 1], [1; 1], [2; -1], [1; 1]].clone_unwrap(),
        ),
        (
            word![[1 => 5; -1]].clone_unwrap(),
            word![[1; -1], [2; -1], [3; -1], [4; -1], [3; 1], [2; 1], [1; 1]].clone_unwrap(),
        ),
    ];

    for (word, expected) in test_cases {
        expect_that!(word.decompose(), eq(&expected));
    }
}

#[gtest]
fn decomposition_works_as_expected_on_braids() {
    start_tracing();
    let test_cases = [
        (
            braid![(); [1 => 3; 1], [2; -1], [1; 1]].clone_unwrap(),
            braid![(); [1; -1], [2; 1], [1; 1], [2; -1], [1; 1]].clone_unwrap(),
        ),
        (
            braid![(); [1 => 5; -1]].clone_unwrap(),
            braid![(); [1; -1], [2; -1], [3; -1], [4; -1], [3; 1], [2; 1], [1; 1]].clone_unwrap(),
        ),
    ];

    for (braid, expected) in test_cases {
        expect_that!(braid.decompose(), eq(&expected));
    }
}
