//! Integration tests for the multiplication interface.

use braided::{
    Braid, BraidIndex, BraidResult, BraidValidationError, Letter, Sign, Word, WordValidationError,
    braid, letter, word,
};
use braided_utils::telemetry::start_tracing;
use googletest::matchers::{eq, err, ok};
use googletest::{assert_that, expect_that, gtest};

// LETTERS
#[gtest]
fn valid_multiplication_of_two_letters_succeeds() {
    start_tracing();
    let l1 = Letter::try_new(1, None::<u16>, Sign::Positive).unwrap();
    let l2 = Letter::try_new(2, Some(4), Sign::Negative).unwrap();

    let product_data = [vec![l1, l1], vec![l1, l2], vec![l2, l1], vec![l2, l2]];

    for pair in product_data {
        expect_that!(pair[0] * pair[1], eq(&Word::try_from_letters(&pair)));
    }
}

#[gtest]
fn invalid_multiplication_of_two_letters_fails() {
    start_tracing();
    let l1 = Letter::try_new(1, None::<u16>, Sign::Positive).unwrap();
    let l2 = Letter::try_new(1, Some(2u16.pow(15) + 1), Sign::Negative).unwrap();
    let error = WordValidationError::TooLong(u16::MAX as usize + 1);

    expect_that!(*(l1 * l2), err(eq(&error)));
    expect_that!(*(l2 * l1), err(eq(&error)));
}

#[gtest]
fn can_multiply_letter_and_letter_result() {
    start_tracing();
    let letter = letter![1; +].unwrap();
    let letter_result = letter![2; -];

    expect_that!(letter * letter_result, eq(&word![[1; 1], [2; -1]]));
    expect_that!(letter_result * letter, eq(&word![[2; -1], [1; 1]]));
}

#[test]
fn can_multiply_letter_result_and_letter_result() {
    start_tracing();
    let letter_result1 = letter![1; +];
    let letter_result2 = letter![2; -];

    assert_that!(letter_result1 * letter_result2, eq(&word![[1; 1], [2; -1]]));
}

#[gtest]
fn multiplication_with_invalid_letter_result_propagates_error() {
    start_tracing();
    let invalid_letter_result = letter![2 => 1; +];
    let invalid_word_result = word![[0; -1], [1 => 3; 3]];
    let invalid_braid_result = braid![(1); [1 => 3; 2]];

    let letter_error = WordValidationError::from(invalid_letter_result.unwrap_err());
    let braid_error = BraidValidationError::from(letter_error);

    let letter = letter![1; +].unwrap();
    let valid_letter_result = letter![2; -];
    let word = word![[1 => 3; 3]].clone_unwrap();
    let valid_word_result = word![[1; 2], [2; -3]];
    let braid = braid![(3); [1; -2], [2; -3]].clone_unwrap();
    let valid_braid_result = braid![(3); [1 => 3; -2], [2; 4]];

    expect_that!(*(letter * invalid_letter_result), err(eq(&letter_error)));
    expect_that!(*(invalid_letter_result * letter), err(eq(&letter_error)));
    expect_that!(
        *(valid_letter_result * invalid_letter_result),
        err(eq(&letter_error))
    );
    expect_that!(
        *(invalid_letter_result * valid_letter_result),
        err(eq(&letter_error))
    );
    expect_that!(
        *(invalid_letter_result * letter![2 => 1; -]),
        err(eq(&letter_error))
    );
    expect_that!(*(invalid_letter_result * &word), err(eq(&letter_error)));
    expect_that!(*(&word * invalid_letter_result), err(eq(&letter_error)));
    expect_that!(
        *(invalid_letter_result * word.clone()),
        err(eq(&letter_error))
    );
    expect_that!(
        *(word.clone() * invalid_letter_result),
        err(eq(&letter_error))
    );
    expect_that!(
        *(invalid_letter_result * valid_word_result.clone()),
        err(eq(&letter_error))
    );
    expect_that!(
        *(valid_word_result.clone() * invalid_letter_result),
        err(eq(&letter_error))
    );
    expect_that!(
        *(invalid_letter_result * invalid_word_result.clone()),
        err(eq(&letter_error))
    );
    expect_that!(
        *(invalid_letter_result * braid.clone()),
        err(eq(&braid_error))
    );
    expect_that!(
        *(braid.clone() * invalid_letter_result),
        err(eq(&braid_error))
    );
    expect_that!(*(invalid_letter_result * &braid), err(eq(&braid_error)));
    expect_that!(*(&braid * invalid_letter_result), err(eq(&braid_error)));
    expect_that!(
        *(invalid_letter_result * valid_braid_result.clone()),
        err(eq(&braid_error))
    );
    expect_that!(
        *(valid_braid_result.clone() * invalid_letter_result),
        err(eq(&braid_error))
    );
    expect_that!(
        *(invalid_letter_result * invalid_braid_result.clone()),
        err(eq(&braid_error))
    );
}

// WORDS
#[gtest]
fn valid_multiplicaation_of_word_and_letter_succeeds() {
    start_tracing();
    let letters = vec![
        Letter::try_new(1, Some(3), Sign::Positive).unwrap(),
        Letter::try_new(2, None::<u16>, Sign::Negative).unwrap(),
        Letter::try_new(1, Some(2), Sign::Positive).unwrap(),
    ];
    let word = Word::try_from_letters(&letters).clone_unwrap();
    let other_letter = Letter::try_new(3, Some(7), Sign::Negative).unwrap();

    expect_that!(
        word.clone() * other_letter,
        eq(&Word::try_from_letters(
            &[letters.clone(), vec![other_letter]].concat()
        ))
    );
    expect_that!(
        other_letter * word,
        eq(&Word::try_from_letters(
            &[vec![other_letter], letters].concat()
        ))
    );
}

#[gtest]
fn valid_multiplication_of_word_and_word_succeeds() {
    start_tracing();
    let letters1 = vec![
        Letter::try_new(1, Some(3), Sign::Positive).unwrap(),
        Letter::try_new(2, None::<u16>, Sign::Negative).unwrap(),
        Letter::try_new(1, Some(2), Sign::Positive).unwrap(),
    ];
    let letters2 = vec![
        Letter::try_new(3, None::<u16>, Sign::Negative).unwrap(),
        Letter::try_new(2, Some(7), Sign::Negative).unwrap(),
    ];

    let word1 = Word::try_from_letters(&letters1).clone_unwrap();
    let word2 = Word::try_from_letters(&letters2).clone_unwrap();

    expect_that!(
        word1.clone() * word2.clone(),
        eq(&Word::try_from_letters(
            &[letters1.clone(), letters2.clone()].concat()
        ))
    );
    expect_that!(
        word2 * word1,
        eq(&Word::try_from_letters(&[letters2, letters1].concat()))
    );
}

#[gtest]
fn invalid_multiplication_of_word_with_letter_fails() {
    start_tracing();
    let short_word = Word::try_new([
        (1, Some(3), Sign::Positive),
        (2, None, Sign::Negative),
        (1, Some(2), Sign::Positive),
    ])
    .clone_unwrap();
    let long_word = Word::try_from_letters(&vec![
        Letter::try_new(1, None::<u16>, Sign::Positive)
            .unwrap();
        u16::MAX as usize
    ])
    .clone_unwrap();
    let short_letter = Letter::try_new(2, None::<u16>, Sign::Negative).unwrap();
    let tall_letter = Letter::try_new(1, Some(2usize.pow(15) + 1), Sign::Positive).unwrap();

    let invalid_products = [
        (
            short_word.clone() * tall_letter,
            u16::MAX as usize + 5,
            "short_word * tall_letter",
        ),
        (
            tall_letter * short_word,
            u16::MAX as usize + 5,
            "tall_letter * short_word",
        ),
        (
            long_word.clone() * short_letter,
            u16::MAX as usize + 1,
            "long_word * short_letter",
        ),
        (
            short_letter * long_word.clone(),
            u16::MAX as usize + 1,
            "short_letter * long_word",
        ),
        (
            long_word.clone() * tall_letter,
            2 * (u16::MAX as usize),
            "long_word * tall_letter",
        ),
        (
            tall_letter * long_word,
            2 * (u16::MAX as usize),
            "tall_letter * long_word",
        ),
    ];

    for (invalid_product, length, label) in invalid_products {
        expect_that!(
            *invalid_product,
            err(eq(&WordValidationError::TooLong(length))),
            "{label}",
        );
    }
}

#[gtest]
fn invalid_multiplication_of_word_with_word_fails() {
    start_tracing();
    let short_word = Word::try_new([
        (1, Some(3), Sign::Positive),
        (2, None, Sign::Negative),
        (1, Some(2), Sign::Positive),
    ])
    .clone_unwrap();
    let long_word = Word::try_from_letters(&vec![
        Letter::try_new(1, None::<u16>, Sign::Positive)
            .unwrap();
        u16::MAX as usize
    ])
    .clone_unwrap();

    let invalid_products = [
        (
            short_word.clone() * long_word.clone(),
            u16::MAX as usize + 5,
            "short_word * long_word",
        ),
        (
            long_word.clone() * short_word.clone(),
            u16::MAX as usize + 5,
            "long_word * short_word",
        ),
        (
            long_word.clone() * long_word.clone(),
            2 * (u16::MAX as usize),
            "long_word * long_word",
        ),
    ];
    for (invalid_product, length, label) in invalid_products {
        expect_that!(
            *invalid_product,
            err(eq(&WordValidationError::TooLong(length))),
            "{label}",
        );
    }
}

#[gtest]
fn can_multiply_letter_with_borrowed_word() {
    start_tracing();
    let letter = letter![1; +].unwrap();
    let word = word![[2; -1], [1 => 3; 2]].clone_unwrap();

    expect_that!(letter * &word, eq(&word![[1; 1], [2; -1], [1 => 3; 2]]));
    expect_that!(&word * letter, eq(&word![[2; -1], [1 => 3; 2], [1; 1]]));
}

#[gtest]
fn can_multiply_borrowed_word_and_word() {
    start_tracing();
    let word1 = word![[2; -1], [1 => 3; 2]].clone_unwrap();
    let word2 = word![[1; 7], [2; -3]].clone_unwrap();
    let word3 = word![[1 => 3; 4]].clone_unwrap();

    expect_that!(
        &word1 * word2,
        eq(&word![[2; -1], [1 => 3; 2], [1; 7], [2; -3]])
    );
    expect_that!(
        word3 * &word1,
        eq(&word![[1 => 3; 4], [2; -1], [1 => 3; 2]])
    );
}

#[test]
fn can_multiply_borrowed_word_and_borrowed_word() {
    start_tracing();
    let word1 = word![[2; -1], [1 => 3; 2]].clone_unwrap();
    let word2 = word![[1; 7], [2; -3]].clone_unwrap();

    assert_that!(
        &word1 * &word2,
        eq(&word![[2; -1], [1 => 3; 2], [1; 7], [2; -3]])
    );
}

#[gtest]
fn can_multiply_word_and_letter_result() {
    start_tracing();
    let letter_result = letter![1; +];
    let word = word![[2; -1], [1 => 3; 2]].clone_unwrap();

    expect_that!(
        word.clone() * letter_result,
        eq(&word![[2; -1], [1 => 3; 2], [1; 1]])
    );
    expect_that!(
        &word * letter_result,
        eq(&word![[2; -1], [1 => 3; 2], [1; 1]])
    );
    expect_that!(
        letter_result * word.clone(),
        eq(&word![[1; 1], [2; -1], [1 => 3; 2]])
    );
    expect_that!(
        letter_result * &word,
        eq(&word![[1; 1], [2; -1], [1 => 3; 2]])
    );
}

#[gtest]
fn can_multiply_letter_and_word_result() {
    start_tracing();
    let letter = letter![1; +].unwrap();
    let word_result = word![[2; -1], [1 => 3; 2]];

    expect_that!(
        word_result.clone() * letter,
        eq(&word![[2; -1], [1 => 3; 2], [1; 1]])
    );
    expect_that!(
        letter * word_result.clone(),
        eq(&word![[1; 1], [2; -1], [1 => 3; 2]])
    );
}

#[gtest]
fn can_multiply_word_and_word_result() {
    start_tracing();
    let word = word![[1; 1]].clone_unwrap();
    let word_result = word![[2; -1], [1 => 3; 2]];

    expect_that!(
        word_result.clone() * word.clone(),
        eq(&word![[2; -1], [1 => 3; 2], [1; 1]])
    );
    expect_that!(
        word_result.clone() * &word,
        eq(&word![[2; -1], [1 => 3; 2], [1; 1]])
    );
    expect_that!(
        word.clone() * word_result.clone(),
        eq(&word![[1; 1], [2; -1], [1 => 3; 2]])
    );
    expect_that!(
        &word * word_result,
        eq(&word![[1; 1], [2; -1], [1 => 3; 2]])
    );
}

#[gtest]
fn can_multiply_letter_result_and_word_result() {
    start_tracing();
    let letter_result = letter![1; +];
    let word_result = word![[2; -1], [1 => 3; 2]];

    expect_that!(
        word_result.clone() * letter_result,
        eq(&word![[2; -1], [1 => 3; 2], [1; 1]])
    );
    expect_that!(
        letter_result * word_result,
        eq(&word![[1; 1], [2; -1], [1 => 3; 2]])
    );
}

#[test]
fn can_multiply_word_result_and_word_result() {
    start_tracing();
    let word_result1 = word![[1; 1]];
    let word_result2 = word![[2; -1], [1 => 3; 2]];

    assert_that!(
        word_result1 * word_result2,
        eq(&word![[1; 1], [2; -1], [1 => 3; 2]])
    );
}

#[gtest]
fn can_multiply_letter_and_word_with_borrowed_word_result() {
    start_tracing();
    let letter = letter![1; +].unwrap();
    let letter_result = letter![1; +];
    let word = word![[1; 1]].clone_unwrap();
    let word_result = word![[1; 1]];
    let borrowed_word_result = &word_result;

    let product = word![[1; 2]].clone_unwrap();

    expect_that!(*(borrowed_word_result * letter), ok(eq(&product)));
    expect_that!(*(letter * borrowed_word_result), ok(eq(&product)));
    expect_that!(*(borrowed_word_result * letter_result), ok(eq(&product)));
    expect_that!(*(letter_result * borrowed_word_result), ok(eq(&product)));
    expect_that!(*(borrowed_word_result * word.clone()), ok(eq(&product)));
    expect_that!(*(word.clone() * borrowed_word_result), ok(eq(&product)));
    expect_that!(*(borrowed_word_result * &word), ok(eq(&product)));
    expect_that!(*(&word * borrowed_word_result), ok(eq(&product)));
    expect_that!(
        *(borrowed_word_result * word_result.clone()),
        ok(eq(&product))
    );
    expect_that!(
        *(word_result.clone() * borrowed_word_result),
        ok(eq(&product))
    );
    expect_that!(
        *(borrowed_word_result * borrowed_word_result),
        ok(eq(&product))
    );
}

#[gtest]
fn multiplication_with_invalid_word_result_propagates_error() {
    start_tracing();
    let invalid_letter_result = letter![2 => 1; +];
    let invalid_word_result = word![[0; -1], [1 => 3; 3]];
    let invalid_braid_result = braid![(1); [1 => 3; 2]];

    let word_error = invalid_word_result.clone_unwrap_err();
    let braid_error = BraidValidationError::from(word_error);

    let letter = letter![1; +].unwrap();
    let valid_letter_result = letter![2; -];
    let word = word![[1 => 3; 3]].clone_unwrap();
    let valid_word_result = word![[1; 2], [2; -3]];
    let braid = braid![(3); [1; -2], [2; -3]].clone_unwrap();
    let valid_braid_result = braid![(3); [1 => 3; -2], [2; 4]];

    // WORD_RESULT
    expect_that!(
        *(invalid_word_result.clone() * letter),
        err(eq(&word_error))
    );
    expect_that!(
        *(letter * invalid_word_result.clone()),
        err(eq(&word_error))
    );
    expect_that!(
        *(invalid_word_result.clone() * valid_letter_result),
        err(eq(&word_error))
    );
    expect_that!(
        *(valid_letter_result * invalid_word_result.clone()),
        err(eq(&word_error))
    );
    expect_that!(*(invalid_word_result.clone() * &word), err(eq(&word_error)));
    expect_that!(*(&word * invalid_word_result.clone()), err(eq(&word_error)));
    expect_that!(
        *(invalid_word_result.clone() * word.clone()),
        err(eq(&word_error))
    );
    expect_that!(
        *(word.clone() * invalid_word_result.clone()),
        err(eq(&word_error))
    );
    expect_that!(
        *(invalid_word_result.clone() * valid_word_result.clone()),
        err(eq(&word_error))
    );
    expect_that!(
        *(valid_word_result.clone() * invalid_word_result.clone()),
        err(eq(&word_error))
    );
    expect_that!(
        *(invalid_word_result.clone() * invalid_letter_result),
        err(eq(&word_error))
    );
    expect_that!(
        *(invalid_word_result.clone() * word![[1; u16::MAX as u32 + 1]]),
        err(eq(&word_error))
    );
    expect_that!(
        *(invalid_word_result.clone() * braid.clone()),
        err(eq(&braid_error))
    );
    expect_that!(
        *(braid.clone() * invalid_word_result.clone()),
        err(eq(&braid_error))
    );
    expect_that!(
        *(invalid_word_result.clone() * &braid),
        err(eq(&braid_error))
    );
    expect_that!(
        *(&braid * invalid_word_result.clone()),
        err(eq(&braid_error))
    );
    expect_that!(
        *(invalid_word_result.clone() * valid_braid_result.clone()),
        err(eq(&braid_error))
    );
    expect_that!(
        *(valid_braid_result.clone() * invalid_word_result.clone()),
        err(eq(&braid_error))
    );
    expect_that!(
        *(invalid_word_result.clone() * invalid_braid_result.clone()),
        err(eq(&braid_error))
    );

    // BORROWED WORD_RESULT
    expect_that!(*(&invalid_word_result * letter), err(eq(&word_error)));
    expect_that!(*(letter * &invalid_word_result), err(eq(&word_error)));
    expect_that!(
        *(&invalid_word_result * valid_letter_result),
        err(eq(&word_error))
    );
    expect_that!(
        *(valid_letter_result * &invalid_word_result),
        err(eq(&word_error))
    );
    expect_that!(*(&invalid_word_result * &word), err(eq(&word_error)));
    expect_that!(*(&word * &invalid_word_result), err(eq(&word_error)));
    expect_that!(*(&invalid_word_result * word.clone()), err(eq(&word_error)));
    expect_that!(*(word.clone() * &invalid_word_result), err(eq(&word_error)));
    expect_that!(
        *(&invalid_word_result * valid_word_result.clone()),
        err(eq(&word_error))
    );
    expect_that!(
        *(valid_word_result.clone() * &invalid_word_result),
        err(eq(&word_error))
    );
    expect_that!(
        *(&invalid_word_result * invalid_letter_result),
        err(eq(&word_error))
    );
    expect_that!(
        *(&invalid_word_result * word![[1; u16::MAX as u32 + 1]]),
        err(eq(&word_error))
    );
    expect_that!(
        *(&invalid_word_result * &word![[1; u16::MAX as u32 + 1]]),
        err(eq(&word_error))
    );
    expect_that!(
        *(&invalid_word_result * braid.clone()),
        err(eq(&braid_error))
    );
    expect_that!(
        *(braid.clone() * &invalid_word_result),
        err(eq(&braid_error))
    );
    expect_that!(*(&invalid_word_result * &braid), err(eq(&braid_error)));
    expect_that!(*(&braid * &invalid_word_result), err(eq(&braid_error)));
    expect_that!(
        *(&invalid_word_result * valid_braid_result.clone()),
        err(eq(&braid_error))
    );
    expect_that!(
        *(valid_braid_result.clone() * &invalid_word_result),
        err(eq(&braid_error))
    );
    expect_that!(
        *(&invalid_word_result * invalid_braid_result.clone()),
        err(eq(&braid_error))
    );
}

// BRAIDS

#[gtest]
fn valid_multiplication_of_braid_with_letter_succeeds() {
    start_tracing();
    let letters = vec![
        Letter::try_new(1, Some(3), Sign::Positive).unwrap(),
        Letter::try_new(2, None::<u16>, Sign::Negative).unwrap(),
        Letter::try_new(2, Some(8), Sign::Positive).unwrap(),
    ];
    let braid = Braid::try_from_letters(None::<u16>, &letters).clone_unwrap();
    let other_letter = Letter::try_new(3, Some(7), Sign::Negative).unwrap();

    expect_that!(
        braid.clone() * other_letter,
        eq(&Braid::try_from_letters(
            None::<u16>,
            &[letters.clone(), vec![other_letter]].concat()
        ))
    );
    expect_that!(
        other_letter * braid,
        eq(&Braid::try_from_letters(
            None::<u16>,
            &[vec![other_letter], letters].concat()
        ))
    );
}

#[gtest]
fn valid_multiplication_of_braid_with_word_succeeds_and_computes_as_expected() {
    start_tracing();
    let braid = Braid::try_from_letters(
        None::<u16>,
        &[
            Letter::try_new(1, Some(3), Sign::Positive).unwrap(),
            Letter::try_new(2, None::<u16>, Sign::Negative).unwrap(),
            Letter::try_new(2, Some(8), Sign::Positive).unwrap(),
        ],
    )
    .clone_unwrap();
    let word = Word::try_from_letters(&[
        Letter::try_new(3, None::<u16>, Sign::Negative).unwrap(),
        Letter::try_new(2, Some(7), Sign::Negative).unwrap(),
    ])
    .clone_unwrap();

    expect_that!(
        braid.clone() * word.clone(),
        eq(&Braid::try_new(
            8,
            (braid.word() * word.clone()).clone_unwrap()
        )),
    );
    expect_that!(
        word.clone() * braid.clone(),
        eq(&Braid::try_new(8, (word * braid.word()).clone_unwrap()))
    );
}

#[gtest]
fn valid_multiplication_of_braid_with_braid_succeeds() {
    start_tracing();
    let braid1 = Braid::try_from_letters(
        None::<u16>,
        &[
            Letter::try_new(1, Some(3), Sign::Positive).unwrap(),
            Letter::try_new(2, None::<u16>, Sign::Negative).unwrap(),
            Letter::try_new(2, Some(8), Sign::Positive).unwrap(),
        ],
    )
    .clone_unwrap();
    let braid2 = Braid::try_new(
        8,
        Word::try_from_letters(&[
            Letter::try_new(3, None::<u16>, Sign::Negative).unwrap(),
            Letter::try_new(2, Some(7), Sign::Negative).unwrap(),
        ])
        .clone_unwrap(),
    )
    .clone_unwrap();

    expect_that!(
        braid1.clone() * braid2.clone(),
        eq(&Braid::try_new(
            8,
            (braid1.word() * braid2.word()).clone_unwrap()
        )),
    );
    expect_that!(
        braid2.clone() * braid1.clone(),
        eq(&Braid::try_new(
            8,
            (braid2.word() * braid1.word()).clone_unwrap()
        ))
    );
}

#[gtest]
fn invalid_multiplication_with_braid_fails_as_expected() {
    start_tracing();
    let letter = Letter::try_new(7, None::<u16>, Sign::Positive).unwrap();
    let word = Word::try_new(vec![
        (2, Some(8), Sign::Negative),
        (1, None::<u16>, Sign::Positive),
    ])
    .clone_unwrap();
    let invalid_braids: Vec<(BraidResult, BraidValidationError, &'static str)> = vec![
        (
            Braid::try_from_data(
                None::<u16>,
                vec![
                    (1, None::<u16>, Sign::Positive),
                    (2, Some(5), Sign::Negative),
                    (3, None::<u16>, Sign::Negative),
                    (4, Some(5), Sign::Positive),
                ],
            )
            .clone_unwrap()
                * letter,
            BraidValidationError::IndexTooSmall {
                index: BraidIndex::try_new(5).unwrap(),
                minimal_required_index: BraidIndex::try_new(8).unwrap(),
            },
            "index too small, braid * letter",
        ),
        (
            letter
                * Braid::try_from_data(
                    None::<u16>,
                    vec![
                        (1, None::<u16>, Sign::Positive),
                        (2, Some(5), Sign::Negative),
                        (3, None::<u16>, Sign::Negative),
                        (4, Some(5), Sign::Positive),
                    ],
                )
                .clone_unwrap(),
            BraidValidationError::IndexTooSmall {
                index: BraidIndex::try_new(5).unwrap(),
                minimal_required_index: BraidIndex::try_new(8).unwrap(),
            },
            "index too small, letter * braid",
        ),
        (
            Braid::try_from_data(
                None::<u16>,
                vec![
                    (1, None::<u16>, Sign::Positive),
                    (2, Some(5), Sign::Negative),
                    (3, None::<u16>, Sign::Negative),
                    (4, Some(5), Sign::Positive),
                ],
            )
            .clone_unwrap()
                * word.clone(),
            BraidValidationError::IndexTooSmall {
                index: BraidIndex::try_new(5).unwrap(),
                minimal_required_index: BraidIndex::try_new(8).unwrap(),
            },
            "index too small, braid * word",
        ),
        (
            word.clone()
                * Braid::try_from_data(
                    None::<u16>,
                    vec![
                        (1, None::<u16>, Sign::Positive),
                        (2, Some(5), Sign::Negative),
                        (3, None::<u16>, Sign::Negative),
                        (4, Some(5), Sign::Positive),
                    ],
                )
                .clone_unwrap(),
            BraidValidationError::IndexTooSmall {
                index: BraidIndex::try_new(5).unwrap(),
                minimal_required_index: BraidIndex::try_new(8).unwrap(),
            },
            "index too small, word * braid",
        ),
        (
            Braid::try_from_data(Some(10), word.clone()).clone_unwrap()
                * Braid::try_from_data(Some(11), word.clone()).clone_unwrap(),
            BraidValidationError::UnequalIndices {
                left: BraidIndex::try_new(10).unwrap(),
                right: BraidIndex::try_new(11).unwrap(),
            },
            "unequal indices",
        ),
        (
            Braid::try_from_data(
                Some(10),
                vec![(1, None::<u16>, Sign::Positive); u16::MAX as usize],
            )
            .clone_unwrap()
                * letter,
            BraidValidationError::from(
                Word::try_new(vec![
                    (1, None::<u16>, Sign::Positive);
                    u16::MAX as usize + 1
                ])
                .clone_unwrap_err(),
            ),
            "word failed validation, braid * letter",
        ),
        (
            letter
                * Braid::try_from_data(
                    Some(10),
                    vec![(1, None::<u16>, Sign::Positive); u16::MAX as usize],
                )
                .clone_unwrap(),
            BraidValidationError::from(
                Word::try_new(vec![
                    (1, None::<u16>, Sign::Positive);
                    u16::MAX as usize + 1
                ])
                .clone_unwrap_err(),
            ),
            "word failed validation, letter * braid",
        ),
        (
            Braid::try_from_data(
                Some(10),
                vec![(1, None::<u16>, Sign::Positive); u16::MAX as usize],
            )
            .clone_unwrap()
                * word.clone(),
            BraidValidationError::from(
                Word::try_new(vec![
                    (1, None::<u16>, Sign::Positive);
                    u16::MAX as usize + 12
                ])
                .clone_unwrap_err(),
            ),
            "word failed validation, braid * word",
        ),
        (
            word * Braid::try_from_data(
                Some(10),
                vec![(1, None::<u16>, Sign::Positive); u16::MAX as usize],
            )
            .clone_unwrap(),
            BraidValidationError::from(
                Word::try_new(vec![
                    (1, None::<u16>, Sign::Positive);
                    u16::MAX as usize + 12
                ])
                .clone_unwrap_err(),
            ),
            "word failed validation, word * braid",
        ),
        (
            Braid::try_from_data(
                None::<u16>,
                vec![(1, None::<u16>, Sign::Positive); u16::MAX as usize],
            )
            .clone_unwrap()
                * Braid::try_from_data(None::<u16>, vec![(1, None::<u16>, Sign::Positive)])
                    .clone_unwrap(),
            BraidValidationError::from(
                Word::try_new(vec![
                    (1, None::<u16>, Sign::Positive);
                    u16::MAX as usize + 1
                ])
                .clone_unwrap_err(),
            ),
            "word failed validation, long_braid * short_braid",
        ),
        (
            Braid::try_from_data(None::<u16>, vec![(1, None::<u16>, Sign::Positive)])
                .clone_unwrap()
                * Braid::try_from_data(
                    None::<u16>,
                    vec![(1, None::<u16>, Sign::Positive); u16::MAX as usize],
                )
                .clone_unwrap(),
            BraidValidationError::from(
                Word::try_new(vec![
                    (1, None::<u16>, Sign::Positive);
                    u16::MAX as usize + 1
                ])
                .clone_unwrap_err(),
            ),
            "word failed validation, short_braid * long_braid",
        ),
    ];

    for (invalid_braid, error, label) in invalid_braids {
        expect_that!(*invalid_braid, err(eq(&error)), "{label}")
    }
}

#[gtest]
fn can_multiply_borrowed_braid_with_letter() {
    start_tracing();
    let letter = letter![1; +].unwrap();
    let braid = braid![(); [2; -1], [1 => 3; 3]].clone_unwrap();

    expect_that!(
        &braid * letter,
        eq(&braid![(); [2; -1], [1 => 3; 3], [1; 1]])
    );
    expect_that!(
        letter * &braid,
        eq(&braid![(); [1; 1], [2; -1], [1 => 3; 3]])
    );
}

#[gtest]
fn can_multiply_borrowed_braid_with_word() {
    start_tracing();
    let word1 = word![[1; 1], [2; -3]].clone_unwrap();
    let word2 = word![[1 => 3; 2]].clone_unwrap();
    let braid = braid![(); [2; -1], [1 => 3; 3]].clone_unwrap();

    expect_that!(
        &braid * word1,
        eq(&braid![(); [2; -1], [1 => 3; 3], [1; 1], [2; -3]])
    );
    expect_that!(
        word2 * &braid,
        eq(&braid![(); [1 => 3; 2], [2; -1], [1 => 3; 3]])
    );
}

#[gtest]
fn can_multiply_braid_with_borrowed_word() {
    start_tracing();
    let braid1 = braid![(); [1; 1], [2; -3]].clone_unwrap();
    let braid2 = braid![(); [1 => 3; 2]].clone_unwrap();
    let word = word![[2; -1], [1 => 3; 3]].clone_unwrap();

    expect_that!(
        &word * braid1,
        eq(&braid![(); [2; -1], [1 => 3; 3], [1; 1], [2; -3]])
    );
    expect_that!(
        braid2 * &word,
        eq(&braid![(); [1 => 3; 2], [2; -1], [1 => 3; 3]])
    );
}

#[gtest]
fn can_multiply_borrowed_braid_with_borrowed_word() {
    start_tracing();
    let braid = braid![(); [1; 1], [2; -3]].clone_unwrap();
    let word = word![[2; -1], [1 => 3; 3]].clone_unwrap();

    expect_that!(
        &word * &braid,
        eq(&braid![(); [2; -1], [1 => 3; 3], [1; 1], [2; -3]])
    );
    expect_that!(
        &braid * &word,
        eq(&braid![(); [1; 1], [2; -3], [2; -1], [1 => 3; 3]])
    );
}

#[gtest]
fn can_multiply_braid_with_borrowed_braid() {
    start_tracing();
    let braid1 = braid![(); [1; 1], [2; -3]].clone_unwrap();
    let braid2 = braid![(); [1 => 3; 2]].clone_unwrap();
    let braid = braid![(); [2; -1], [1 => 3; 3]].clone_unwrap();

    expect_that!(
        &braid * braid1,
        eq(&braid![(); [2; -1], [1 => 3; 3], [1; 1], [2; -3]])
    );
    expect_that!(
        braid2 * &braid,
        eq(&braid![(); [1 => 3; 2], [2; -1], [1 => 3; 3]])
    );
}

#[test]
fn can_multiply_borrowed_braid_with_borrowed_braid() {
    start_tracing();
    let braid1 = braid![(); [1; 1], [2; -3]].clone_unwrap();
    let braid2 = braid![(); [1 => 3; 2]].clone_unwrap();

    assert_that!(
        &braid1 * &braid2,
        eq(&braid![(); [1; 1], [2; -3], [1 => 3; 2]])
    );
}

#[gtest]
fn can_multiply_braid_with_letter_result() {
    start_tracing();
    let letter_result = letter![1; +];
    let braid = braid![(); [1 => 3; -3], [2; 7]].clone_unwrap();

    let letter_times_braid = braid![(); [1; 1], [1 => 3; -3], [2; 7]];
    let braid_times_letter = braid![(); [1 => 3; -3], [2; 7], [1; 1]];

    let tests = [
        (
            letter_result * braid.clone(),
            &letter_times_braid,
            "R<L> * B",
        ),
        (letter_result * &braid, &letter_times_braid, "R<L> * &B"),
        (
            braid.clone() * letter_result,
            &braid_times_letter,
            "B * R<L>",
        ),
        (&braid * letter_result, &braid_times_letter, "&B * R<L>"),
    ];

    for (actual, expected, label) in tests {
        expect_that!(actual, eq(expected), "{label}");
    }
}

#[gtest]
fn can_multiply_braid_with_word_result() {
    start_tracing();
    let word_result = word![[1; 3], [2; -7]];
    let braid = braid![(); [1 => 3; 2], [1; 1]].clone_unwrap();

    let word_times_braid = braid![(); [1; 3], [2; -7], [1 => 3; 2], [1; 1]];
    let braid_times_word = braid![(); [1 => 3; 2], [1; 1], [1; 3], [2; -7]];

    let tests = [
        (
            word_result.clone() * braid.clone(),
            &word_times_braid,
            "R<W> * B",
        ),
        (word_result.clone() * &braid, &word_times_braid, "R<W> * &B"),
        (
            braid.clone() * word_result.clone(),
            &braid_times_word,
            "B * R<W>",
        ),
        (&braid * word_result, &braid_times_word, "&B * R<W>"),
    ];

    for (actual, expected, label) in tests {
        expect_that!(actual, eq(expected), "{label}");
    }
}

#[gtest]
fn can_multiply_letter_with_braid_result() {
    start_tracing();
    let letter = letter![1; +].unwrap();
    let braid_result = braid![(); [1 => 3; -3], [2; 7]];

    let letter_times_braid = braid![(); [1; 1], [1 => 3; -3], [2; 7]];
    let braid_times_letter = braid![(); [1 => 3; -3], [2; 7], [1; 1]];

    let tests = [
        (
            letter * braid_result.clone(),
            &letter_times_braid,
            "L * R<B>",
        ),
        (
            braid_result.clone() * letter,
            &braid_times_letter,
            "R<B> * L",
        ),
    ];

    for (actual, expected, label) in tests {
        expect_that!(actual, eq(expected), "{label}");
    }
}

#[gtest]
fn can_multiply_word_with_braid_result() {
    start_tracing();
    let word = word![[1; 3], [2; -7]].clone_unwrap();
    let braid_result = braid![(); [1 => 3; 2], [1; 1]];

    let word_times_braid = braid![(); [1; 3], [2; -7], [1 => 3; 2], [1; 1]];
    let braid_times_word = braid![(); [1 => 3; 2], [1; 1], [1; 3], [2; -7]];

    let tests = [
        (
            word.clone() * braid_result.clone(),
            &word_times_braid,
            "W * R<B>",
        ),
        (&word * braid_result.clone(), &word_times_braid, "&W * R<B>"),
        (
            braid_result.clone() * word.clone(),
            &braid_times_word,
            "R<B> * W",
        ),
        (braid_result * &word, &braid_times_word, "R<B> * &W"),
    ];

    for (actual, expected, label) in tests {
        expect_that!(actual, eq(expected), "{label}");
    }
}

#[gtest]
fn can_multiply_braid_with_braid_result() {
    start_tracing();
    let braid = braid![(); [1; 3], [2; -7]].clone_unwrap();
    let braid_result = braid![(); [1 => 3; 2], [1; 1]];

    let braid_times_braid_result = braid![(); [1; 3], [2; -7], [1 => 3; 2], [1; 1]];
    let braid_result_times_braid = braid![(); [1 => 3; 2], [1; 1], [1; 3], [2; -7]];
    let tests = [
        (
            braid.clone() * braid_result.clone(),
            &braid_times_braid_result,
            "B * R<B>",
        ),
        (
            &braid * braid_result.clone(),
            &braid_times_braid_result,
            "&B * R<B>",
        ),
        (
            braid_result.clone() * braid.clone(),
            &braid_result_times_braid,
            "R<B> * B",
        ),
        (
            braid_result * &braid,
            &braid_result_times_braid,
            "R<B> * &B",
        ),
    ];

    for (actual, expected, label) in tests {
        expect_that!(actual, eq(expected), "{label}");
    }
}

#[gtest]
fn can_multiply_letter_result_with_braid_result() {
    start_tracing();
    let letter_result = letter![1; +];
    let braid_result = braid![(); [1 => 3; -3], [2; 7]];

    let letter_times_braid = braid![(); [1; 1], [1 => 3; -3], [2; 7]];
    let braid_times_letter = braid![(); [1 => 3; -3], [2; 7], [1; 1]];

    let tests = [
        (
            letter_result * braid_result.clone(),
            &letter_times_braid,
            "R<L> * R<B>",
        ),
        (
            braid_result.clone() * letter_result,
            &braid_times_letter,
            "R<B> * R<L>",
        ),
    ];

    for (actual, expected, label) in tests {
        expect_that!(actual, eq(expected), "{label}");
    }
}

#[gtest]
fn can_multiply_word_result_with_braid_result() {
    start_tracing();
    let word_result = word![[1; 3], [2; -7]];
    let braid_result = braid![(); [1 => 3; 2], [1; 1]];

    let word_times_braid = braid![(); [1; 3], [2; -7], [1 => 3; 2], [1; 1]];
    let braid_times_word = braid![(); [1 => 3; 2], [1; 1], [1; 3], [2; -7]];

    let tests = [
        (
            word_result.clone() * braid_result.clone(),
            &word_times_braid,
            "R<W> * R<B>",
        ),
        (
            braid_result.clone() * word_result.clone(),
            &braid_times_word,
            "R<B> * R<W>",
        ),
    ];

    for (actual, expected, label) in tests {
        expect_that!(actual, eq(expected), "{label}");
    }
}

#[test]
fn can_multiply_braid_result_with_braid_result() {
    start_tracing();
    let braid_result1 = braid![(); [1; 3], [2; -7]];
    let braid_result2 = braid![(); [1 => 3; 2], [1; 1]];

    assert_that!(
        braid_result1 * braid_result2,
        eq(&braid![(); [1; 3], [2; -7], [1 => 3; 2], [1; 1]])
    );
}

#[gtest]
fn can_multiply_braid_with_borrowed_word_result() {
    start_tracing();
    let braid = braid![(); [1; 1]].clone_unwrap();
    let braid_result = braid![(); [1; 1]];

    let borrowed_word_result = &word![[1; 1]];

    let product = braid![(); [1; 2]].clone_unwrap();

    expect_that!(*(borrowed_word_result * braid.clone()), ok(eq(&product)));
    expect_that!(*(braid.clone() * borrowed_word_result), ok(eq(&product)));
    expect_that!(*(borrowed_word_result * &braid), ok(eq(&product)));
    expect_that!(*(&braid * borrowed_word_result), ok(eq(&product)));
    expect_that!(
        *(borrowed_word_result * braid_result.clone()),
        ok(eq(&product))
    );
    expect_that!(
        *(braid_result.clone() * borrowed_word_result),
        ok(eq(&product))
    );
}

#[gtest]
fn can_multiply_any_with_borrowed_braid_result() {
    start_tracing();
    let letter = letter![1; +].unwrap();
    let letter_result = letter![1; +];
    let word = word![[1; 1]].clone_unwrap();
    let word_result = word![[1; 1]];
    let braid = braid![(); [1; 1]].clone_unwrap();
    let braid_result = braid![(); [1; 1]];

    let borrowed_braid_result = &braid_result;

    let product = braid![(); [1; 2]].clone_unwrap();

    expect_that!(*(borrowed_braid_result * letter), ok(eq(&product)));
    expect_that!(*(letter * borrowed_braid_result), ok(eq(&product)));
    expect_that!(*(borrowed_braid_result * letter_result), ok(eq(&product)));
    expect_that!(*(letter_result * borrowed_braid_result), ok(eq(&product)));
    expect_that!(*(borrowed_braid_result * word.clone()), ok(eq(&product)));
    expect_that!(*(word.clone() * borrowed_braid_result), ok(eq(&product)));
    expect_that!(*(borrowed_braid_result * &word), ok(eq(&product)));
    expect_that!(*(&word * borrowed_braid_result), ok(eq(&product)));
    expect_that!(
        *(borrowed_braid_result * word_result.clone()),
        ok(eq(&product))
    );
    expect_that!(
        *(word_result.clone() * borrowed_braid_result),
        ok(eq(&product))
    );
    expect_that!(*(borrowed_braid_result * &word_result), ok(eq(&product)));
    expect_that!(*(&word_result * borrowed_braid_result), ok(eq(&product)));
    expect_that!(*(borrowed_braid_result * braid.clone()), ok(eq(&product)));
    expect_that!(*(braid.clone() * borrowed_braid_result), ok(eq(&product)));
    expect_that!(*(borrowed_braid_result * &braid), ok(eq(&product)));
    expect_that!(*(&braid * borrowed_braid_result), ok(eq(&product)));
    expect_that!(
        *(borrowed_braid_result * braid_result.clone()),
        ok(eq(&product))
    );
    expect_that!(
        *(braid_result.clone() * borrowed_braid_result),
        ok(eq(&product))
    );
    expect_that!(
        *(borrowed_braid_result * borrowed_braid_result),
        ok(eq(&product))
    );
}

#[gtest]
fn multiplication_with_invalid_braid_result_propagates_error() {
    start_tracing();
    let invalid_letter_result = letter![2 => 1; +];
    let invalid_word_result = word![[0; -1], [1 => 3; 3]];
    let invalid_braid_result = braid![(1); [1 => 3; 2]];

    let braid_error = invalid_braid_result.clone_unwrap_err();

    let letter = letter![1; +].unwrap();
    let valid_letter_result = letter![2; -];
    let word = word![[1 => 3; 3]].clone_unwrap();
    let valid_word_result = word![[1; 2], [2; -3]];
    let braid = braid![(3); [1; -2], [2; -3]].clone_unwrap();
    let valid_braid_result = braid![(3); [1 => 3; -2], [2; 4]];

    // BRAID RESULT
    expect_that!(
        *(invalid_braid_result.clone() * letter),
        err(eq(&braid_error))
    );
    expect_that!(
        *(letter * invalid_braid_result.clone()),
        err(eq(&braid_error))
    );
    expect_that!(
        *(invalid_braid_result.clone() * valid_letter_result),
        err(eq(&braid_error))
    );
    expect_that!(
        *(valid_letter_result * invalid_braid_result.clone()),
        err(eq(&braid_error))
    );
    expect_that!(
        *(invalid_braid_result.clone() * word.clone()),
        err(eq(&braid_error))
    );
    expect_that!(
        *(word.clone() * invalid_braid_result.clone()),
        err(eq(&braid_error))
    );
    expect_that!(
        *(invalid_braid_result.clone() * &word),
        err(eq(&braid_error))
    );
    expect_that!(
        *(&word * invalid_braid_result.clone()),
        err(eq(&braid_error))
    );
    expect_that!(
        *(invalid_braid_result.clone() * valid_word_result.clone()),
        err(eq(&braid_error))
    );
    expect_that!(
        *(valid_word_result.clone() * invalid_braid_result.clone()),
        err(eq(&braid_error))
    );
    expect_that!(
        *(invalid_braid_result.clone() * braid.clone()),
        err(eq(&braid_error))
    );
    expect_that!(
        *(braid.clone() * invalid_braid_result.clone()),
        err(eq(&braid_error))
    );
    expect_that!(
        *(invalid_braid_result.clone() * &braid),
        err(eq(&braid_error))
    );
    expect_that!(
        *(&braid * invalid_braid_result.clone()),
        err(eq(&braid_error))
    );
    expect_that!(
        *(invalid_braid_result.clone() * valid_braid_result.clone()),
        err(eq(&braid_error))
    );
    expect_that!(
        *(valid_braid_result.clone() * invalid_braid_result.clone()),
        err(eq(&braid_error))
    );
    expect_that!(
        *(invalid_braid_result.clone() * invalid_letter_result),
        err(eq(&braid_error))
    );
    expect_that!(
        *(invalid_braid_result.clone() * invalid_word_result.clone()),
        err(eq(&braid_error))
    );
    expect_that!(
        *(invalid_braid_result.clone() * braid![(0); [1; 1]]),
        err(eq(&braid_error))
    );

    // BORROWED BRAID RESULT
    expect_that!(*(&invalid_braid_result * letter), err(eq(&braid_error)));
    expect_that!(*(letter * &invalid_braid_result), err(eq(&braid_error)));
    expect_that!(
        *(&invalid_braid_result * valid_letter_result),
        err(eq(&braid_error))
    );
    expect_that!(
        *(valid_letter_result * &invalid_braid_result),
        err(eq(&braid_error))
    );
    expect_that!(
        *(&invalid_braid_result * word.clone()),
        err(eq(&braid_error))
    );
    expect_that!(
        *(word.clone() * &invalid_braid_result),
        err(eq(&braid_error))
    );
    expect_that!(*(&invalid_braid_result * &word), err(eq(&braid_error)));
    expect_that!(*(&word * &invalid_braid_result), err(eq(&braid_error)));
    expect_that!(
        *(&invalid_braid_result * valid_word_result.clone()),
        err(eq(&braid_error))
    );
    expect_that!(
        *(valid_word_result.clone() * &invalid_braid_result),
        err(eq(&braid_error))
    );
    expect_that!(
        *(&invalid_braid_result * braid.clone()),
        err(eq(&braid_error))
    );
    expect_that!(
        *(braid.clone() * &invalid_braid_result),
        err(eq(&braid_error))
    );
    expect_that!(*(&invalid_braid_result * &braid), err(eq(&braid_error)));
    expect_that!(*(&braid * &invalid_braid_result), err(eq(&braid_error)));
    expect_that!(
        *(&invalid_braid_result * valid_braid_result.clone()),
        err(eq(&braid_error))
    );
    expect_that!(
        *(valid_braid_result.clone() * &invalid_braid_result),
        err(eq(&braid_error))
    );
    expect_that!(
        *(&invalid_braid_result * invalid_letter_result),
        err(eq(&braid_error))
    );
    expect_that!(
        *(&invalid_braid_result * invalid_word_result.clone()),
        err(eq(&braid_error))
    );
    expect_that!(
        *(&invalid_braid_result * braid![(0); [1; 1]]),
        err(eq(&braid_error))
    );
    expect_that!(
        *(&invalid_braid_result * &braid![(0); [1; 1]]),
        err(eq(&braid_error))
    );
}
