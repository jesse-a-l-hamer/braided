use crate::{Letter, LetterResult, Word, WordResult, WordValidationError};

impl std::ops::Mul<Letter> for Word {
    type Output = WordResult;
    fn mul(self, rhs: Letter) -> Self::Output {
        if let Some((lhs_last, lhs_initial)) = self.split_last() {
            match &*(*lhs_last * rhs) {
                Ok(tail) => Self::try_from_letters(&[lhs_initial, tail].concat()),
                Err(WordValidationError::TooLong(tail_length)) => {
                    WordResult::from(WordValidationError::TooLong(
                        lhs_initial
                            .iter()
                            .map(|l| l.artin_length() as usize)
                            .sum::<usize>()
                            + tail_length,
                    ))
                }
                Err(_) => panic!("Unexpected error while computing {self:?} * {rhs:?}"),
            }
        } else {
            Self::try_from_letters(&[rhs])
        }
    }
}
impl std::ops::Mul<Word> for Letter {
    type Output = WordResult;
    fn mul(self, rhs: Word) -> Self::Output {
        if let Some((rhs_first, rhs_tail)) = rhs.split_first() {
            match &*(self * *rhs_first) {
                Ok(initial) => Word::try_from_letters(&[initial, rhs_tail].concat()),
                Err(WordValidationError::TooLong(initial_length)) => {
                    WordResult::from(WordValidationError::TooLong(
                        initial_length
                            + rhs_tail
                                .iter()
                                .map(|l| l.artin_length() as usize)
                                .sum::<usize>(),
                    ))
                }
                Err(_) => panic!("Unexpected error while computing {self:?} * {rhs:?}"),
            }
        } else {
            Word::try_from_letters(&[self])
        }
    }
}
#[allow(clippy::suspicious_arithmetic_impl)]
impl std::ops::Mul for Word {
    type Output = WordResult;
    fn mul(self, rhs: Self) -> Self::Output {
        let radius =
            match self
                .iter()
                .rev()
                .zip(rhs.iter())
                .try_fold(0usize, |radius, (left, &right)| {
                    if left.inverse() == right {
                        Ok(radius + 1)
                    } else {
                        Err(radius)
                    }
                }) {
                Ok(radius) => radius,
                Err(radius) => radius,
            };
        Self::try_from_letters(&[&self[..self.len() - radius], &rhs[radius..]].concat())
    }
}

impl std::ops::Mul<Letter> for &Word {
    type Output = WordResult;
    fn mul(self, rhs: Letter) -> Self::Output {
        self.clone() * rhs
    }
}
impl std::ops::Mul<&Word> for Letter {
    type Output = WordResult;
    fn mul(self, rhs: &Word) -> Self::Output {
        self * rhs.clone()
    }
}
impl std::ops::Mul<Word> for &Word {
    type Output = WordResult;
    fn mul(self, rhs: Word) -> Self::Output {
        self.clone() * rhs
    }
}
impl std::ops::Mul<&Word> for Word {
    type Output = WordResult;
    fn mul(self, rhs: &Word) -> Self::Output {
        self * rhs.clone()
    }
}
impl std::ops::Mul for &Word {
    type Output = WordResult;
    fn mul(self, rhs: Self) -> Self::Output {
        self.clone() * rhs.clone()
    }
}

impl std::ops::Mul<LetterResult> for Word {
    type Output = WordResult;
    fn mul(self, rhs: LetterResult) -> Self::Output {
        match *rhs {
            Ok(rhs) => self * rhs,
            Err(rhs) => WordResult::from(Err(WordValidationError::from(rhs))),
        }
    }
}
impl std::ops::Mul<LetterResult> for &Word {
    type Output = WordResult;
    fn mul(self, rhs: LetterResult) -> Self::Output {
        match *rhs {
            Ok(rhs) => self * rhs,
            Err(rhs) => WordResult::from(Err(WordValidationError::from(rhs))),
        }
    }
}
impl std::ops::Mul<Word> for LetterResult {
    type Output = WordResult;
    fn mul(self, rhs: Word) -> Self::Output {
        match *self {
            Ok(lhs) => lhs * rhs,
            Err(lhs) => WordResult::from(Err(WordValidationError::from(lhs))),
        }
    }
}
impl std::ops::Mul<&Word> for LetterResult {
    type Output = WordResult;
    fn mul(self, rhs: &Word) -> Self::Output {
        match *self {
            Ok(lhs) => lhs * rhs,
            Err(lhs) => WordResult::from(Err(WordValidationError::from(lhs))),
        }
    }
}
impl std::ops::Mul<Letter> for WordResult {
    type Output = WordResult;
    fn mul(self, rhs: Letter) -> Self::Output {
        match &*self {
            Ok(lhs) => lhs * rhs,
            Err(lhs) => WordResult::from(Err(*lhs)),
        }
    }
}
impl std::ops::Mul<WordResult> for Letter {
    type Output = WordResult;
    fn mul(self, rhs: WordResult) -> Self::Output {
        match &*rhs {
            Ok(rhs) => self * rhs,
            Err(rhs) => WordResult::from(Err(*rhs)),
        }
    }
}
impl std::ops::Mul<WordResult> for Word {
    type Output = WordResult;
    fn mul(self, rhs: WordResult) -> Self::Output {
        match &*rhs {
            Ok(rhs) => self * rhs,
            Err(rhs) => WordResult::from(Err(*rhs)),
        }
    }
}
impl std::ops::Mul<WordResult> for &Word {
    type Output = WordResult;
    fn mul(self, rhs: WordResult) -> Self::Output {
        match &*rhs {
            Ok(rhs) => self * rhs,
            Err(rhs) => WordResult::from(Err(*rhs)),
        }
    }
}
impl std::ops::Mul<Word> for WordResult {
    type Output = WordResult;
    fn mul(self, rhs: Word) -> Self::Output {
        match &*self {
            Ok(lhs) => lhs * rhs,
            Err(lhs) => WordResult::from(Err(*lhs)),
        }
    }
}
impl std::ops::Mul<&Word> for WordResult {
    type Output = WordResult;
    fn mul(self, rhs: &Word) -> Self::Output {
        match &*self {
            Ok(lhs) => lhs * rhs,
            Err(lhs) => WordResult::from(Err(*lhs)),
        }
    }
}
impl std::ops::Mul<LetterResult> for WordResult {
    type Output = WordResult;
    fn mul(self, rhs: LetterResult) -> Self::Output {
        match (&*self, *rhs) {
            (Ok(lhs), Ok(rhs)) => lhs * rhs,
            (Err(lhs), _) => WordResult::from(Err(*lhs)),
            (_, Err(rhs)) => WordResult::from(Err(WordValidationError::from(rhs))),
        }
    }
}
impl std::ops::Mul<WordResult> for LetterResult {
    type Output = WordResult;
    fn mul(self, rhs: WordResult) -> Self::Output {
        match (*self, &*rhs) {
            (Ok(lhs), Ok(rhs)) => lhs * rhs,
            (Err(lhs), _) => WordResult::from(Err(WordValidationError::from(lhs))),
            (_, Err(rhs)) => WordResult::from(Err(*rhs)),
        }
    }
}
impl std::ops::Mul for WordResult {
    type Output = WordResult;
    fn mul(self, rhs: Self) -> Self::Output {
        match (&*self, &*rhs) {
            (Ok(lhs), Ok(rhs)) => lhs * rhs,
            (Err(lhs), _) => WordResult::from(Err(*lhs)),
            (_, Err(rhs)) => WordResult::from(Err(*rhs)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Letter, Sign, Word, WordValidationError, letter, word};
    use googletest::matchers::{eq, err, ok};
    use googletest::{assert_that, expect_that, gtest};

    #[gtest]
    fn valid_multiplication_with_letter_succeeds_and_computes_as_expected() {
        let letters = vec![
            Letter::try_new(1, Some(3), Sign::Positive).unwrap(),
            Letter::try_new(2, None::<u16>, Sign::Negative).unwrap(),
            Letter::try_new(1, Some(2), Sign::Positive).unwrap(),
        ];
        let word = Word::try_from_letters(&letters).as_ref().unwrap().clone();
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
    fn valid_multiplication_with_word_succeeds_and_computes_as_expected() {
        let letters1 = vec![
            Letter::try_new(1, Some(3), Sign::Positive).unwrap(),
            Letter::try_new(2, None::<u16>, Sign::Negative).unwrap(),
            Letter::try_new(1, Some(2), Sign::Positive).unwrap(),
        ];
        let letters2 = vec![
            Letter::try_new(3, None::<u16>, Sign::Negative).unwrap(),
            Letter::try_new(2, Some(7), Sign::Negative).unwrap(),
        ];

        let word1 = Word::try_from_letters(&letters1).as_ref().unwrap().clone();
        let word2 = Word::try_from_letters(&letters2).as_ref().unwrap().clone();

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
    fn trivial_word_is_multiplicative_identity() {
        let letter = Letter::try_new(1, None::<u16>, Sign::Positive).unwrap();

        expect_that!(
            *(letter * Word::trivial()),
            ok(eq(&Word::try_from_letters(&vec![letter])
                .as_ref()
                .unwrap()
                .clone()))
        );
        expect_that!(
            *(Word::trivial() * letter),
            ok(eq(&Word::try_from_letters(&vec![letter])
                .as_ref()
                .unwrap()
                .clone()))
        );

        let word = Word::try_from_letters(&vec![
            Letter::try_new(1, Some(3), Sign::Positive).unwrap(),
            Letter::try_new(2, None::<u16>, Sign::Negative).unwrap(),
            Letter::try_new(1, Some(2), Sign::Positive).unwrap(),
        ])
        .as_ref()
        .unwrap()
        .clone();

        expect_that!(*(word.clone() * Word::trivial()), ok(eq(&word)));
        expect_that!(*(Word::trivial() * word.clone()), ok(eq(&word)))
    }

    #[gtest]
    fn multiplication_with_inverse_yields_trivial() {
        let word = Word::try_from_letters(&vec![
            Letter::try_new(1, Some(3), Sign::Positive).unwrap(),
            Letter::try_new(2, None::<u16>, Sign::Negative).unwrap(),
            Letter::try_new(1, Some(2), Sign::Positive).unwrap(),
        ])
        .as_ref()
        .unwrap()
        .clone();

        expect_that!(*(word.clone() * word.inverse()), ok(eq(&Word::trivial())));
        expect_that!(*(word.inverse() * word.clone()), ok(eq(&Word::trivial())));
    }

    #[gtest]
    fn invalid_mult_with_letter_fails() {
        let short_word = Word::try_new([
            (1, Some(3), Sign::Positive),
            (2, None, Sign::Negative),
            (1, Some(2), Sign::Positive),
        ])
        .as_ref()
        .unwrap()
        .clone();
        let long_word =
            Word::try_from_letters(&vec![
                Letter::try_new(1, None::<u16>, Sign::Positive)
                    .unwrap();
                u16::MAX as usize
            ])
            .as_ref()
            .unwrap()
            .clone();
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
    fn invalid_mult_with_word_fails() {
        let short_word = Word::try_new([
            (1, Some(3), Sign::Positive),
            (2, None, Sign::Negative),
            (1, Some(2), Sign::Positive),
        ])
        .as_ref()
        .unwrap()
        .clone();
        let long_word =
            Word::try_from_letters(&vec![
                Letter::try_new(1, None::<u16>, Sign::Positive)
                    .unwrap();
                u16::MAX as usize
            ])
            .as_ref()
            .unwrap()
            .clone();

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
        let letter = letter![1; +].unwrap();
        let word = word![[2; -1], [1 => 3; 2]].unwrap();

        expect_that!(letter * &word, eq(&word![[1; 1], [2; -1], [1 => 3; 2]]));
        expect_that!(&word * letter, eq(&word![[2; -1], [1 => 3; 2], [1; 1]]));
    }

    #[gtest]
    fn can_multiply_borrowed_word_and_word() {
        let word1 = word![[2; -1], [1 => 3; 2]].unwrap();
        let word2 = word![[1; 7], [2; -3]].unwrap();
        let word3 = word![[1 => 3; 4]].unwrap();

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
        let word1 = word![[2; -1], [1 => 3; 2]].unwrap();
        let word2 = word![[1; 7], [2; -3]].unwrap();

        assert_that!(
            &word1 * &word2,
            eq(&word![[2; -1], [1 => 3; 2], [1; 7], [2; -3]])
        );
    }

    #[gtest]
    fn can_multiply_word_and_letter_result() {
        let letter_result = letter![1; +];
        let word = word![[2; -1], [1 => 3; 2]].unwrap();

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
        let word = word![[1; 1]].unwrap();
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
        let word_result1 = word![[1; 1]];
        let word_result2 = word![[2; -1], [1 => 3; 2]];

        assert_that!(
            word_result1 * word_result2,
            eq(&word![[1; 1], [2; -1], [1 => 3; 2]])
        );
    }
}
