/// Macro for conveniently constructing a [`Letter`](crate::Letter) of a braid word.
///
/// The syntax allows specifying either a single positive integer and sign (for the
/// [`Letter::Artin`](crate::Letter::Artin) variant) or two positive integers and a sign (for the
/// [`Letter::Band`](crate::Letter::Band) variant). See the examples below for details.
///
/// # Examples
///
/// ```
/// use braided::{Letter, Sign, letter};
/// use std::assert_matches;
/// # #[macro_use] extern crate braided;
/// # fn main() {
/// // A single integer and a sign is interpreted as an Artin generator:
/// let artin = letter![1; +];
/// assert_matches!(artin, Ok(Letter::Artin(_)));
/// assert_eq!(artin, Letter::new(1, None::<u16>, Sign::Positive));
///
/// // Two integers and a sign is interpreted as a Band generator:
/// let band = letter![2 => 4; -];
/// assert_matches!(band, Ok(Letter::Band(_)));
/// assert_eq!(band, Letter::new(2, Some(4), Sign::Negative));
/// # }
/// ```
///
/// # Errors
///
/// Under the hood, a call is made to the [`Letter::new`](crate::Letter::new) constructor, and
/// will return errors under the same circumstances.
///
/// ```
/// use braided::{
///     ArtinValidationError, BandValidationError, Letter, LetterValidationError,
///     StrandValidationError, letter,
/// };
/// use std::assert_matches;
/// # #[macro_use] extern crate braided;
/// # fn main() {
/// // Strand indices must be positive:
/// let bad_letter_1 = letter![-1; +];
/// assert_matches!(
///     bad_letter_1,
///     Err(LetterValidationError::ArtinValidation(ArtinValidationError::StrandValidation(_))),
/// );
/// let bad_letter_2 = letter![0 => 4; -];
/// assert_matches!(
///     bad_letter_2,
///     Err(LetterValidationError::BandValidation(BandValidationError::StrandValidation(_))),
/// );
///
/// // Strand indices must also be within the [`u16`] range:
/// let bad_letter_3 = letter![u16::MAX as u32 + 1; +];
/// assert_matches!(
///     bad_letter_3,
///     Err(
///         LetterValidationError::ArtinValidation(
///             ArtinValidationError::StrandValidation(
///                 StrandValidationError::FromInt(_)
///             )
///         )
///     ),
/// );
///
/// // Bands must be well-formed:
/// let bad_letter_4 = letter![4 => 1; +];
/// assert_matches!(
///     bad_letter_4,
///     Err(LetterValidationError::BandValidation(BandValidationError::FootOverHead {..})),
/// )
/// # }
/// ```
#[macro_export]
macro_rules! letter {
    ($foot:expr; +) => {
        $crate::Letter::new($foot, None::<u16>, $crate::Sign::Positive)
    };
    ($foot:expr; -) => {
        $crate::Letter::new($foot, None::<u16>, $crate::Sign::Negative)
    };
    ($foot:expr => $head:expr; +) => {
        $crate::Letter::new($foot, Some($head), $crate::Sign::Positive)
    };
    ($foot:expr => $head:expr; -) => {
        $crate::Letter::new($foot, Some($head), $crate::Sign::Negative)
    };
}

/// Constructs a [`Word`](crate::Word) given a sequence of letters with exponents.
///
/// The syntax for specifying each letter in the word is similar to that used in [`letter!`],
/// except that the single sign (+/-) is replaced by an integer exponent, which is used to infer
/// both the [`Sign`](crate::Sign) of the corresponding [`Letter`](crate::Letter) as well as the
/// number of consecutive occurrences of the given letter in the associated factor of the word.
/// See the examples below for illustration.
///
/// Note that, as with the [`Word`](crate::Word) struct itself, it is an error if the length of
/// the word _in [Artin letters](crate::Letter::Artin)_ cannot be coerced into a [`u16`]. In
/// particular, because each [band letter](crate::Letter::Band) is composed of `1 + 2*(height - 1)`,
/// where `height = band.head - band.foot`, it is possible to have an word whose
/// [Artin length](crate::Word::artin_length) exceeds [`u16::MAX`], even when its
/// [letter length](crate::Word::length) does not. More information on the fallibility of this
/// macro can be found in the _**Errors**_ section below.
///
/// # Examples
///
/// ```
/// use braided::{Sign, Word, word};
/// # #[macro_use] extern crate braided;
/// # fn main() {
/// // Passing nothing constructs the trivial word:
/// let trivial = word![];
/// assert_eq!(trivial, Word::trivial());
///
/// // Create a word from a single letter repeated several times:
/// let positive_artin_cubed = word![[2; 3]];
/// assert_eq!(positive_artin_cubed, Word::new(vec![(2, None::<u16>, Sign::Positive); 3]));
/// let negative_band_squared = word![[1 => 4; -2]];
/// assert_eq!(negative_band_squared, Word::new(vec![(1, Some(4), Sign::Negative); 2]));
///
/// // Create a word from an arbitrary sequence of factors, using either generator variant:
/// let wacky_word = word![[2 => 5; 7], [3; -2], [1 => 2; -3], [2; 9]];
/// assert_eq!(
///     wacky_word,
///     Word::new([
///         vec![(2, Some(5), Sign::Positive); 7],
///         vec![(3, None, Sign::Negative); 2],
///         vec![(1, Some(2), Sign::Negative); 3],
///         vec![(2, None, Sign::Positive); 9],
///     ].concat()),
/// );
/// # }
/// ```
///
/// # Errors
///
/// [`word!`] will return a [`WordValidationError`](crate::WordValidationError) in any context
/// where the associated [`Word::new`](crate::Word::new) function does. In particular, all of the
/// following are errors:
///
/// 1. Having an Artin length which exceeds [`u16::MAX`]
///    ([`WordValidationError::TooLong`](crate::WordValidationError::TooLong)).
/// ```
/// use braided::{WordValidationError, word};
/// use std::assert_matches;
/// # #[macro_use] extern crate braided;
/// # fn main() {
/// let long_word_artin = word![[1; -(u16::MAX as i64 + 1)]];
/// let long_word_bands = word![[1 => 3; (u16::MAX as u32).div_euclid(3) + 1]];
/// let long_product = word![[1; u16::MAX as u32 -1], [3; 2]];
///
/// assert_matches!(long_word_artin, Err(WordValidationError::TooLong(_)));
/// assert_matches!(long_word_bands, Err(WordValidationError::TooLong(_)));
/// assert_matches!(long_product, Err(WordValidationError::TooLong(_)));
/// # }
/// ```
///
/// 2. Having a malformed letter/factor
///    ([`WordValidationError::LetterValidation`](crate::WordValidationError::LetterValidation)).
/// ```
/// use braided::{WordValidationError, word};
/// use std::assert_matches;
/// # #[macro_use] extern crate braided;
/// # fn main() {
/// let malformed_at_start = word![[-1; 2], [1 => 4; -3], [2; -1]];
/// let malformed_in_middle = word![[1; 2], [4 => 1; -3], [0; -1]];
/// let malformed_at_end = word![[1; 2], [1 => 4; -3], [0; -1]];
///
/// assert_matches!(malformed_at_start, Err(WordValidationError::LetterValidation(_)));
/// assert_matches!(malformed_in_middle, Err(WordValidationError::LetterValidation(_)));
/// assert_matches!(malformed_at_end, Err(WordValidationError::LetterValidation(_)));
/// # }
/// ```
///
/// 3. Passing an exponent that fails to coerce into an [`i64`]
///    ([`WordValidationError::FromInt`](crate::WordValidationError::FromInt)).
/// ```
/// use braided::{WordValidationError, word};
/// use std::assert_matches;
/// # #[macro_use] extern crate braided;
/// # fn main() {
/// let too_big_exponent = word![[1; u64::MAX]];
/// assert_matches!(too_big_exponent, Err(WordValidationError::FromInt(_)));
/// # }
/// ```
#[macro_export]
macro_rules! word {
    () => {
        $crate::Word::trivial()
    };
    ([$foot:expr; $exponent:expr]) => {{
        match TryInto::<i64>::try_into($exponent) {
            Ok(exponent) => {
                let letter = if exponent < 0 {
                    $crate::letter![$foot; -]
                } else {
                    $crate::letter![$foot; +]
                };
                match letter {
                    Ok(letter) => $crate::Word::try_from(
                        vec![letter; exponent.unsigned_abs().try_into().unwrap()]
                    ),
                    Err(e) => Err($crate::WordValidationError::from(e)),
                }
            },
            Err(e) => Err($crate::WordValidationError::from(e))
        }
    }};
    ([$foot:expr => $head:expr; $exponent:expr]) => {{
        match TryInto::<i64>::try_into($exponent) {
            Ok(exponent) => {
                let letter = if exponent < 0 {
                    $crate::letter![$foot => $head; -]
                } else {
                    $crate::letter![$foot => $head; +]
                };
                match letter {
                    Ok(letter) => $crate::Word::try_from(
                        vec![letter; exponent.unsigned_abs().try_into().unwrap()]
                    ),
                    Err(e) => Err($crate::WordValidationError::from(e)),
                }
            },
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

/// Constructs a [`Braid`](crate::Braid) given an optional [index](crate::BraidIndex) and a
/// [word](crate::Word).
///
/// The macro input must always begin with a (possibly empty) expression surrounded by parentheses,
/// which is parsed as the braid index if given. After the parentheses is a semicolon (";"),
/// followed by an arbitrary sequence of bracketed expressions, each of which denotes a power of
/// a single letter in the resulting word. The word syntax is identical to that of the [`word!`]
/// macro.
///
/// If the braid index is not explicitly given, then it will be inferred as the minial required
/// index for the given word.
///
/// If an explicit index is provided but the word is empty, then the corresponding trivial braid
/// will be returned.
///
/// The macro will panic if neither the index nor the word are specified.
///
/// # Examples
///
/// ```
/// use braided::{Braid, Sign, braid};
/// # #[macro_use] extern crate braided;
/// # fn main() {
/// let trivial_3_braid = braid![(3)];
/// assert_eq!(trivial_3_braid, Braid::trivial(3));
///
/// let braid_with_inferred_index = braid![(); [1 => 3; -2], [3; 3], [1; 4]];
/// assert_eq!(braid_with_inferred_index, Braid::from_data(
///     None::<u16>,
///     [
///         vec![(1, Some(3), Sign::Negative); 2],
///         vec![(3, None, Sign::Positive); 3],
///         vec![(1, None, Sign::Positive); 4],
///     ]
///     .concat()
/// ));
/// assert_eq!(*braid_with_inferred_index.unwrap().index(), 4);
///
/// let braid_with_explicit_index = braid![(10); [1 => 3; -2], [3; 3], [1; 4]];
/// assert_eq!(braid_with_explicit_index, Braid::from_data(
///     Some(10),
///     [
///         vec![(1, Some(3), Sign::Negative); 2],
///         vec![(3, None, Sign::Positive); 3],
///         vec![(1, None, Sign::Positive); 4],
///     ]
///     .concat()
/// ));
/// assert_eq!(*braid_with_explicit_index.unwrap().index(), 10);
/// # }
/// ```
///
/// # Errors
///
/// The macro will return a [`BraidValidationError`](crate::BraidValidationError) in any of the
/// following circumstances:
///
/// 1. An explicitly provided index is smaller than is required by the given word
///    ([`BraidValidationError::IndexTooSmall`](crate::BraidValidationError::IndexTooSmall)).
///
/// ```
/// use braided::{Braid, BraidValidationError, Sign, braid};
/// use std::assert_matches;
/// # #[macro_use] extern crate braided;
/// # fn main() {
/// let index_too_small = braid![(1); [1; 1]];
///
/// assert_matches!(index_too_small, Err(BraidValidationError::IndexTooSmall { .. }));
/// # }
/// ```
///
/// 2. An explicitly given index fails validation
///    ([`BraidValidationError::IndexValidation`](crate::BraidValidationError::IndexValidation)).
///
/// ```
/// use braided::{Braid, BraidValidationError, Sign, braid};
/// use std::assert_matches;
/// # #[macro_use] extern crate braided;
/// # fn main() {
/// let negative_index = braid![(-1); [1; 1]];
/// let zero_index = braid![(0); [1; 1]];
/// let big_index = braid![(u16::MAX as u32 + 1); [1; 1]];
///
/// assert_matches!(negative_index, Err(BraidValidationError::IndexValidation(_)));
/// assert_matches!(zero_index, Err(BraidValidationError::IndexValidation(_)));
/// assert_matches!(big_index, Err(BraidValidationError::IndexValidation(_)));
/// # }
/// ```
///
/// 3. An explicitly given word fails validation
///    ([`BraidValidationError::WordValidation`](crate::BraidValidationError::WordValidation)).
///
/// ```
/// use braided::{Braid, BraidValidationError, Sign, braid};
/// use std::assert_matches;
/// # #[macro_use] extern crate braided;
/// # fn main() {
/// let too_long = braid![(); [1; u16::MAX], [2; -1]];
/// let invalid_letter = braid![(); [4 => 1; 2]];
/// let invalid_exponent = braid![(); [1; u64::MAX]];
///
/// assert_matches!(too_long, Err(BraidValidationError::WordValidation(_)));
/// assert_matches!(invalid_letter, Err(BraidValidationError::WordValidation(_)));
/// assert_matches!(invalid_exponent, Err(BraidValidationError::WordValidation(_)));
/// # }
/// ```
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
        let artins: [(Result<Letter, LetterValidationError>, u16, Sign); 2] = [
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
        let artins: [(Result<Letter, LetterValidationError>, u16, u16, Sign); 2] = [
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
                letter![(u16::MAX as usize) + 1; -],
                LetterValidationError::from(
                    ArtinGenerator::new(u16::MAX as u32 + 1, Sign::Negative)
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
        let words: [(Result<Word, WordValidationError>, u16, i32); 2] =
            [(word![[1; 3]], 1, 3), (word![[2; -4]], 2, -4)];
        for (word, foot, exp) in words {
            let letter = if exp < 0 {
                letter![foot; -].unwrap()
            } else {
                letter![foot; +].unwrap()
            };
            expect_that!(
                word,
                ok(eq(&Word::try_from(vec![
                    letter;
                    exp.unsigned_abs() as usize
                ])
                .unwrap()))
            )
        }
    }
    #[gtest]
    fn macro_word_constructs_exponent_of_single_band() {
        let words: [(Result<Word, WordValidationError>, u16, u16, i32); 2] = [
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
                ok(eq(&Word::try_from(vec![
                    letter;
                    exp.unsigned_abs() as usize
                ])
                .unwrap()))
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
                word![[1; 2], [2 => 5; -3], [u16::MAX as u32 + 1; 2]],
                Word::new(
                    [
                        vec![(1, None, Sign::Positive); 2],
                        vec![(2, Some(5), Sign::Negative); 3],
                        vec![(u16::MAX as u32 + 1, None, Sign::Positive); 2],
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
                word![[1; u16::MAX as u32 + 1]],
                Word::try_from(vec![letter![1; +].unwrap(); u16::MAX as usize + 1])
                    .err()
                    .unwrap(),
            ),
            (
                word![[1 => 3; (u16::MAX as u32).div_euclid(3)], [3; -1]],
                Word::try_from(
                    [
                        vec![letter![1 => 3; +].unwrap(); (u16::MAX as usize).div_euclid(3)],
                        vec![letter![3; -].unwrap(); 1],
                    ]
                    .concat(),
                )
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
                braid![(u16::MAX as u32 + 1);[1 => 3; 2], [2; -4], [3 => 4; 3]],
                Braid::from_data(
                    Some(u16::MAX as u32 + 1),
                    word![[1 => 3; 2], [2; -4], [3 => 4; 3]].unwrap(),
                )
                .err()
                .unwrap(),
            ),
            (
                braid![();[-1; 1], [1; 2], [2 => 5; -3]],
                Braid::from_data(
                    None::<u16>,
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
                    None::<u16>,
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
                braid![();[1; 2], [2 => 5; -3], [u16::MAX as u32 + 1; 2]],
                Braid::from_data(
                    None::<u16>,
                    [
                        vec![(1, None, Sign::Positive); 2],
                        vec![(2, Some(5), Sign::Negative); 3],
                        vec![(u16::MAX as u32 + 1, None, Sign::Positive); 2],
                    ]
                    .concat(),
                )
                .err()
                .unwrap(),
            ),
            (
                braid![();[4 => 1; 3], [1; 2], [2 => 5; -3]],
                Braid::from_data(
                    None::<u16>,
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
                braid![();[1; u16::MAX as u32 + 1]],
                Braid::from_data(
                    None::<u16>,
                    [vec![
                        (1, None::<u16>, Sign::Positive);
                        u16::MAX as usize + 1
                    ]]
                    .concat(),
                )
                .err()
                .unwrap(),
            ),
            (
                braid![();[1 => 3; u16::MAX as u32 - 1], [3; -2]],
                Braid::from_data(
                    None::<u16>,
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
