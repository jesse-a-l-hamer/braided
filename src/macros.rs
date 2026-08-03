#[macro_export]
macro_rules! letter {
    ($foot:expr; +) => {
        $crate::Letter::new::<isize, isize>($foot, None, $crate::Sign::Positive)
    };
    ($foot:expr; -) => {
        $crate::Letter::new::<isize, isize>($foot, None, $crate::Sign::Negative)
    };
    ($foot:expr => $head:expr; +) => {
        $crate::Letter::new::<isize, isize>($foot, Some($head), $crate::Sign::Positive)
    };
    ($foot:expr => $head:expr; -) => {
        $crate::Letter::new::<isize, isize>($foot, Some($head), $crate::Sign::Negative)
    };
}

#[macro_export]
macro_rules! word {
    () => {
        $crate::Word::trivial()
    };
    ([$foot:expr; $exponent:expr]) => {{
        let exponent:isize = $exponent;
        let letter = if exponent < 0 {
            $crate::letter![$foot; -]
        } else {
            $crate::letter![$foot; +]
        };
        match letter {
            Ok(letter) => $crate::Word::try_from(vec![letter; exponent.abs().try_into().unwrap()]),
            Err(e) => Err($crate::WordValidationError::from(e)),
        }
    }};
    ([$foot:expr => $head:expr; $exponent:expr]) => {{
        let exponent:isize = $exponent;
        let letter = if exponent < 0 {
            $crate::letter![$foot => $head; -]
        } else {
            $crate::letter![$foot => $head; +]
        };
        match letter {
            Ok(letter) => $crate::Word::try_from(vec![letter; exponent.abs().try_into().unwrap()]),
            Err(e) => Err($crate::WordValidationError::from(e)),
        }
    }};
    ([$foot:expr; $exponent:expr], $($tail:tt)+) => {{
        match (word![[$foot; $exponent]], word![$($tail)+]) {
            (Ok(w1), Ok(w2)) => w1 * w2,
            (Err(e), _) => Err(e),
            (_, Err(e)) => Err(e),
        }
    }};
    ([$foot:expr => $head:expr; $exponent:expr], $($tail:tt)+) => {{
        match (word![[$foot => $head; $exponent]], word![$($tail)+]) {
            (Ok(w1), Ok(w2)) => w1 * w2,
            (Err(e), _) => Err(e),
            (_, Err(e)) => Err(e),
        }
    }};
}

#[macro_export]
macro_rules! braid {
    (($index:expr) $(;)?) => {$crate::Braid::trivial($index)};
    (($index:expr); $($tail:tt)+) => {
        match $crate::word![$($tail)+] {
            Ok(word) => $crate::Braid::new(Some($index), word),
            Err(e) => Err($crate::BraidValidationError::from(e)),
        }
    };
    ((); $($tail:tt)+) => {
        match $crate::word![$($tail)+] {
            Ok(w) => Ok($crate::Braid::from(w)),
            Err(e) => Err($crate::BraidValidationError::from(e)),
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::{
        ArtinGenerator, BandGenerator, Braid, BraidValidationError, Letter, LetterValidationError,
        Sign, Word, WordValidationError,
    };
    use googletest::matchers::{eq, err, ok};
    use googletest::{assert_that, expect_that, gtest};

    // letter!
    #[gtest]
    fn macro_letter_constructs_artin_letters() {
        let artins: [(Result<Letter, LetterValidationError>, isize, Sign); 2] = [
            (letter![1; +], 1, Sign::Positive),
            (letter![2; -], 2, Sign::Negative),
        ];
        for (artin, foot, sign) in artins {
            expect_that!(
                artin,
                ok(eq(&Letter::Artin(ArtinGenerator::new(foot, sign).unwrap()))),
            )
        }
    }
    #[gtest]
    fn macro_letter_constructs_band_letters() {
        let artins: [(Result<Letter, LetterValidationError>, isize, isize, Sign); 2] = [
            (letter![1 => 3; +], 1, 3, Sign::Positive),
            (letter![2 => 5; -], 2, 5, Sign::Negative),
        ];
        for (artin, foot, head, sign) in artins {
            expect_that!(
                artin,
                ok(eq(&Letter::Band(
                    BandGenerator::new(foot, head, sign).unwrap()
                ))),
            )
        }
    }
    #[gtest]
    fn macro_letter_fails_to_construct_invalid_letters() {
        let invalid_letters: [(Result<Letter, LetterValidationError>, LetterValidationError); 4] = [
            (
                letter![-1; +],
                LetterValidationError::from(ArtinGenerator::new(-1, Sign::Positive).err().unwrap()),
            ),
            (
                letter![0 => 4; -],
                LetterValidationError::from(
                    BandGenerator::new(0, 4, Sign::Negative).err().unwrap(),
                ),
            ),
            (
                letter![(u16::MAX as isize) + 1; -],
                LetterValidationError::from(
                    ArtinGenerator::new((u16::MAX as isize) + 1, Sign::Negative)
                        .err()
                        .unwrap(),
                ),
            ),
            (
                letter![4 => 1; +],
                LetterValidationError::from(
                    BandGenerator::new(4, 1, Sign::Positive).err().unwrap(),
                ),
            ),
        ];

        for (invalid_letter, error) in invalid_letters {
            expect_that!(invalid_letter, err(eq(&error)))
        }
    }

    // word!
    #[test]
    fn macro_word_empty_produces_trivial_word() {
        let trivial = word![];
        assert_that!(trivial, eq(&Word::trivial()))
    }
    #[gtest]
    fn macro_word_constructs_exponent_of_single_artin() {
        let words: [(Result<Word, WordValidationError>, isize, isize); 2] =
            [(word![[1; 3]], 1, 3), (word![[2; -4]], 2, -4)];
        for (word, foot, exp) in words {
            let letter = if exp < 0 {
                letter![foot; -].unwrap()
            } else {
                letter![foot; +].unwrap()
            };
            expect_that!(
                word,
                ok(eq(
                    &Word::try_from(vec![letter; exp.unsigned_abs()]).unwrap()
                ))
            )
        }
    }
    #[gtest]
    fn macro_word_constructs_exponent_of_single_band() {
        let words: [(Result<Word, WordValidationError>, isize, isize, isize); 2] = [
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
                word,
                ok(eq(
                    &Word::try_from(vec![letter; exp.unsigned_abs()]).unwrap()
                ))
            )
        }
    }
    #[gtest]
    fn macro_word_constructs_word_when_leading_letter_is_artin() {
        let word_with_positive_leading_artin = word![[1; 2], [2 => 4; -1], [3 => 4; -3], [2; 3]];
        expect_that!(
            word_with_positive_leading_artin,
            ok(eq(&Word::try_from(
                [
                    vec![letter![1; +].unwrap(); 2],
                    vec![letter![2 => 4; -].unwrap(); 1],
                    vec![letter![3 => 4; -].unwrap(); 3],
                    vec![letter![2; +].unwrap(); 3],
                ]
                .concat()
            )
            .unwrap()))
        );
        let word_with_negative_leading_artin = word![[1; -2], [2 => 4; -1], [3 => 4; -3], [2; 3]];
        expect_that!(
            word_with_negative_leading_artin,
            ok(eq(&Word::try_from(
                [
                    vec![letter![1; -].unwrap(); 2],
                    vec![letter![2 => 4; -].unwrap(); 1],
                    vec![letter![3 => 4; -].unwrap(); 3],
                    vec![letter![2; +].unwrap(); 3],
                ]
                .concat()
            )
            .unwrap()))
        );
    }
    #[gtest]
    fn macro_word_constructs_word_when_leading_letter_is_band() {
        let word_with_positive_leading_band = word![[2 => 4; 1], [1; 2], [3 => 4; -3], [2; 3]];
        expect_that!(
            word_with_positive_leading_band,
            ok(eq(&Word::try_from(
                [
                    vec![letter![2 => 4; +].unwrap(); 1],
                    vec![letter![1; +].unwrap(); 2],
                    vec![letter![3 => 4; -].unwrap(); 3],
                    vec![letter![2; +].unwrap(); 3],
                ]
                .concat()
            )
            .unwrap()))
        );
        let word_with_negative_leading_band = word![[2 => 4; -1], [1; 2], [3 => 4; -3], [2; 3]];
        expect_that!(
            word_with_negative_leading_band,
            ok(eq(&Word::try_from(
                [
                    vec![letter![2 => 4; -].unwrap(); 1],
                    vec![letter![1; +].unwrap(); 2],
                    vec![letter![3 => 4; -].unwrap(); 3],
                    vec![letter![2; +].unwrap(); 3],
                ]
                .concat()
            )
            .unwrap()))
        )
    }
    #[gtest]
    fn macro_word_fails_to_construct_invalid_words() {
        let invalid_words: [(Result<Word, WordValidationError>, WordValidationError); 6] = [
            (
                word![[-1; 1], [1; 2], [2 => 5; -3]],
                Word::new(
                    [
                        vec![(-1, None, Sign::Positive); 1],
                        vec![(1, None, Sign::Positive); 2],
                        vec![(2, Some(5), Sign::Negative); 3],
                    ]
                    .concat(),
                )
                .err()
                .unwrap(),
            ),
            (
                word![[1; 2], [0 => 4; -2], [2 => 5; -3]],
                Word::new(
                    [
                        vec![(1, None, Sign::Positive); 2],
                        vec![(0, Some(4), Sign::Negative); 2],
                        vec![(2, Some(5), Sign::Negative); 3],
                    ]
                    .concat(),
                )
                .err()
                .unwrap(),
            ),
            (
                word![[1; 2], [2 => 5; -3], [u16::MAX as isize + 1; 2]],
                Word::new(
                    [
                        vec![(1, None, Sign::Positive); 2],
                        vec![(2, Some(5), Sign::Negative); 3],
                        vec![(u16::MAX as isize + 1, None, Sign::Positive); 2],
                    ]
                    .concat(),
                )
                .err()
                .unwrap(),
            ),
            (
                word![[4 => 1; 3], [1; 2], [2 => 5; -3]],
                Word::new(
                    [
                        vec![(4, Some(1), Sign::Positive); 3],
                        vec![(1, None, Sign::Positive); 2],
                        vec![(2, Some(5), Sign::Negative); 3],
                    ]
                    .concat(),
                )
                .err()
                .unwrap(),
            ),
            (
                word![[1; u16::MAX as isize + 1]],
                Word::try_from(vec![letter![1; +].unwrap(); u16::MAX as usize + 1])
                    .err()
                    .unwrap(),
            ),
            (
                word![[1 => 3; u16::MAX as isize - 1], [3; -2]],
                Word::try_from([vec![letter![1 => 3; +].unwrap(); u16::MAX as usize - 1]].concat())
                    .err()
                    .unwrap(),
            ),
        ];
        for (invalid_word, error) in invalid_words {
            expect_that!(invalid_word, err(eq(&error)))
        }
    }

    // braid!
    #[test]
    fn macro_braid_constructs_trivial_braid_of_given_index() {
        let braid = braid![(10)];
        assert_that!(braid, ok(eq(&Braid::trivial(10).unwrap())))
    }
    #[test]
    fn macro_braid_constructs_nontrivial_braid_of_given_index() {
        let braid = braid![(10); [2 => 4; -1], [1; 2], [3 => 4; -3], [2; 3]];
        assert_that!(
            braid,
            ok(eq(&Braid::from_data(
                Some(10),
                word![[2 => 4; -1], [1; 2], [3 => 4; -3], [2; 3]].unwrap()
            )
            .unwrap()))
        )
    }
    #[test]
    fn macro_braid_constructs_nontrivial_braid_of_inferred_index() {
        let braid = braid![(); [2 => 4; -1], [1; 2], [3 => 4; -3], [2; 3]];
        assert_that!(
            braid,
            ok(eq(&Braid::from(
                word![[2 => 4; -1], [1; 2], [3 => 4; -3], [2; 3]].unwrap()
            )))
        )
    }
    #[gtest]
    fn macro_braid_fails_to_construct_invalid_braids() {
        let invalid_braids: [(Result<Braid, BraidValidationError>, BraidValidationError); 10] = [
            (
                braid![(1); [1; 1]],
                Braid::from_data(Some(1), word![[1; 1]].unwrap())
                    .err()
                    .unwrap(),
            ),
            (
                braid![(-1); [1 => 3; 2], [2; -4], [3 => 4; 3]],
                Braid::from_data(Some(-1), word![[1 => 3; 2], [2; -4], [3 => 4; 3]].unwrap())
                    .err()
                    .unwrap(),
            ),
            (
                braid![(0);[1 => 3; 2], [2; -4], [3 => 4; 3]],
                Braid::from_data(Some(0), word![[1 => 3; 2], [2; -4], [3 => 4; 3]].unwrap())
                    .err()
                    .unwrap(),
            ),
            (
                braid![(u16::MAX as isize + 1);[1 => 3; 2], [2; -4], [3 => 4; 3]],
                Braid::from_data(
                    Some(u16::MAX as isize + 1),
                    word![[1 => 3; 2], [2; -4], [3 => 4; 3]].unwrap(),
                )
                .err()
                .unwrap(),
            ),
            (
                braid![();[-1; 1], [1; 2], [2 => 5; -3]],
                Braid::from_data(
                    None::<isize>,
                    [
                        vec![(-1, None, Sign::Positive); 1],
                        vec![(1, None, Sign::Positive); 2],
                        vec![(2, Some(5), Sign::Negative); 3],
                    ]
                    .concat(),
                )
                .err()
                .unwrap(),
            ),
            (
                braid![();[1; 2], [0 => 4; -2], [2 => 5; -3]],
                Braid::from_data(
                    None::<isize>,
                    [
                        vec![(1, None, Sign::Positive); 2],
                        vec![(0, Some(4), Sign::Negative); 2],
                        vec![(2, Some(5), Sign::Negative); 3],
                    ]
                    .concat(),
                )
                .err()
                .unwrap(),
            ),
            (
                braid![();[1; 2], [2 => 5; -3], [u16::MAX as isize + 1; 2]],
                Braid::from_data(
                    None::<isize>,
                    [
                        vec![(1, None, Sign::Positive); 2],
                        vec![(2, Some(5), Sign::Negative); 3],
                        vec![(u16::MAX as isize + 1, None, Sign::Positive); 2],
                    ]
                    .concat(),
                )
                .err()
                .unwrap(),
            ),
            (
                braid![();[4 => 1; 3], [1; 2], [2 => 5; -3]],
                Braid::from_data(
                    None::<isize>,
                    [
                        vec![(4, Some(1), Sign::Positive); 3],
                        vec![(1, None, Sign::Positive); 2],
                        vec![(2, Some(5), Sign::Negative); 3],
                    ]
                    .concat(),
                )
                .err()
                .unwrap(),
            ),
            (
                braid![();[1; u16::MAX as isize + 1]],
                Braid::from_data(
                    None::<isize>,
                    [vec![
                        (1, None::<isize>, Sign::Positive);
                        u16::MAX as usize + 1
                    ]]
                    .concat(),
                )
                .err()
                .unwrap(),
            ),
            (
                braid![();[1 => 3; u16::MAX as isize - 1], [3; -2]],
                Braid::from_data(
                    None::<isize>,
                    [vec![(1, Some(3), Sign::Positive); u16::MAX as usize - 1]].concat(),
                )
                .err()
                .unwrap(),
            ),
        ];
        for (invalid_braid, error) in invalid_braids {
            expect_that!(invalid_braid, err(eq(&error)));
        }
    }
}
