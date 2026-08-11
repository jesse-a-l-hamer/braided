//! Integration tests to check macro-based construction interface.

use braided::{
    ArtinGenerator, BandGenerator, Braid, BraidResult, BraidValidationError, Letter, LetterResult,
    LetterValidationError, Sign, Word, WordResult, WordValidationError, braid, letter, word,
};
use googletest::matchers::{eq, err, ok};
use googletest::{assert_that, expect_that, gtest};

// letter!
#[gtest]
fn macro_letter_constructs_valid_letters() {
    let letters = [
        (letter![1 => 3; +], 1, Some(3), Sign::Positive),
        (letter![2 => 5; -], 2, Some(5), Sign::Negative),
        (letter![1; +], 1, None, Sign::Positive),
        (letter![2; -], 2, None, Sign::Negative),
    ];
    for (letter, foot, head, sign) in letters {
        expect_that!(letter, eq(Letter::try_new(foot, head, sign)))
    }
}
#[gtest]
fn macro_letter_fails_to_construct_invalid_letters() {
    let invalid_letters: [(LetterResult, LetterValidationError); 4] = [
        (
            letter![-1; +],
            LetterValidationError::from(ArtinGenerator::try_new(-1, Sign::Positive).err().unwrap()),
        ),
        (
            letter![0 => 4; -],
            LetterValidationError::from(
                BandGenerator::try_new(0, 4, Sign::Negative).err().unwrap(),
            ),
        ),
        (
            letter![(u16::MAX as usize) + 1; -],
            LetterValidationError::from(
                ArtinGenerator::try_new(u16::MAX as u32 + 1, Sign::Negative)
                    .err()
                    .unwrap(),
            ),
        ),
        (
            letter![4 => 1; +],
            LetterValidationError::from(
                BandGenerator::try_new(4, 1, Sign::Positive).err().unwrap(),
            ),
        ),
    ];

    for (invalid_letter, error) in invalid_letters {
        expect_that!(*invalid_letter, err(eq(error)))
    }
}

// word!
#[test]
fn macro_word_empty_produces_trivial_word() {
    let trivial = word![];
    assert_that!(*trivial, ok(eq(&Word::trivial())))
}
#[gtest]
fn macro_word_constructs_exponent_of_single_artin() {
    let words: [(WordResult, u16, i32); 2] = [(word![[1; 3]], 1, 3), (word![[2; -4]], 2, -4)];
    for (word, foot, exp) in words {
        let letter = if exp < 0 {
            letter![foot; -].unwrap()
        } else {
            letter![foot; +].unwrap()
        };
        expect_that!(
            *word,
            ok(eq(&Word::try_from_letters(&vec![
                letter;
                exp.unsigned_abs()
                    as usize
            ])
            .clone_unwrap()))
        )
    }
}
#[gtest]
fn macro_word_constructs_exponent_of_single_band() {
    let words: [(WordResult, u16, u16, i32); 2] = [
        (word![[1 => 4; 3]], 1, 4, 3),
        (word![[2 => 7; -4]], 2, 7, -4),
    ];
    for (word, foot, head, exp) in words {
        let letter = if exp < 0 {
            letter![foot => head; -].unwrap()
        } else {
            letter![foot => head; +].unwrap()
        };
        expect_that!(
            *word,
            ok(eq(&Word::try_from_letters(&vec![
                letter;
                exp.unsigned_abs()
                    as usize
            ])
            .clone_unwrap()))
        )
    }
}
#[gtest]
fn macro_word_constructs_word_when_leading_letter_is_artin() {
    let word_with_positive_leading_artin = word![[1; 2], [2 => 4; -1], [3 => 4; -3], [2; 3]];
    expect_that!(
        *word_with_positive_leading_artin,
        ok(eq(&Word::try_from_letters(
            &[
                vec![letter![1; +].unwrap(); 2],
                vec![letter![2 => 4; -].unwrap(); 1],
                vec![letter![3 => 4; -].unwrap(); 3],
                vec![letter![2; +].unwrap(); 3],
            ]
            .concat()
        )
        .clone_unwrap()))
    );
    let word_with_negative_leading_artin = word![[1; -2], [2 => 4; -1], [3 => 4; -3], [2; 3]];
    expect_that!(
        *word_with_negative_leading_artin,
        ok(eq(&Word::try_from_letters(
            &[
                vec![letter![1; -].unwrap(); 2],
                vec![letter![2 => 4; -].unwrap(); 1],
                vec![letter![3 => 4; -].unwrap(); 3],
                vec![letter![2; +].unwrap(); 3],
            ]
            .concat()
        )
        .clone_unwrap()))
    );
}
#[gtest]
fn macro_word_constructs_word_when_leading_letter_is_band() {
    let word_with_positive_leading_band = word![[2 => 4; 1], [1; 2], [3 => 4; -3], [2; 3]];
    expect_that!(
        *word_with_positive_leading_band,
        ok(eq(&Word::try_from_letters(
            &[
                vec![letter![2 => 4; +].unwrap(); 1],
                vec![letter![1; +].unwrap(); 2],
                vec![letter![3 => 4; -].unwrap(); 3],
                vec![letter![2; +].unwrap(); 3],
            ]
            .concat()
        )
        .clone_unwrap()))
    );
    let word_with_negative_leading_band = word![[2 => 4; -1], [1; 2], [3 => 4; -3], [2; 3]];
    expect_that!(
        *word_with_negative_leading_band,
        ok(eq(&Word::try_from_letters(
            &[
                vec![letter![2 => 4; -].unwrap(); 1],
                vec![letter![1; +].unwrap(); 2],
                vec![letter![3 => 4; -].unwrap(); 3],
                vec![letter![2; +].unwrap(); 3],
            ]
            .concat()
        )
        .clone_unwrap()))
    )
}
#[gtest]
fn macro_word_fails_to_construct_invalid_words() {
    let invalid_words: [(WordResult, WordValidationError); 6] = [
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
        ),
        (
            word![[1; u16::MAX as u32 + 1]],
            Word::try_from_letters(&vec![letter![1; +].unwrap(); u16::MAX as usize + 1])
                .clone_unwrap_err(),
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
        ),
    ];
    for (invalid_word, error) in invalid_words {
        expect_that!(*invalid_word, err(eq(&error)))
    }
}

// braid!
#[test]
fn macro_braid_constructs_trivial_braid_of_given_index() {
    let braid = braid![(10)];
    assert_that!(*braid, ok(eq(&Braid::try_trivial(10).clone_unwrap())))
}
#[test]
fn macro_braid_constructs_nontrivial_braid_of_given_index() {
    let braid = braid![(10); [2 => 4; -1], [1; 2], [3 => 4; -3], [2; 3]];
    assert_that!(
        *braid,
        ok(eq(&Braid::try_from_data(
            Some(10),
            word![[2 => 4; -1], [1; 2], [3 => 4; -3], [2; 3]].clone_unwrap()
        )
        .clone_unwrap()))
    )
}
#[test]
fn macro_braid_constructs_nontrivial_braid_of_inferred_index() {
    let braid = braid![(); [2 => 4; -1], [1; 2], [3 => 4; -3], [2; 3]];
    assert_that!(
        *braid,
        ok(eq(&Braid::from(
            word![[2 => 4; -1], [1; 2], [3 => 4; -3], [2; 3]].clone_unwrap()
        )))
    )
}
#[gtest]
fn macro_braid_fails_to_construct_invalid_braids() {
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
            Braid::try_from_data(
                None::<u16>,
                [vec![
                    (1, None::<u16>, Sign::Positive);
                    u16::MAX as usize + 1
                ]]
                .concat(),
            )
            .clone_unwrap_err(),
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
