use crate::{Letter, LetterResult, Word, WordResult, WordValidationError};

impl std::ops::Mul<Letter> for Word {
    type Output = WordResult;
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: Letter) -> Self::Output {
        tracing::debug!("Attempting to multiply {:?} x {:?}", self, rhs);
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
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: Word) -> Self::Output {
        tracing::debug!("Attempting to multiply {:?} x {:?}", self, rhs);
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
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: Self) -> Self::Output {
        tracing::debug!("Attempting to multiply {:?} x {:?}", self, rhs);
        Self::try_from_letters(&[&self[..], &rhs[..]].concat())
    }
}

impl std::ops::Mul<Letter> for &Word {
    type Output = WordResult;
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: Letter) -> Self::Output {
        self.clone() * rhs
    }
}
impl std::ops::Mul<&Word> for Letter {
    type Output = WordResult;
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: &Word) -> Self::Output {
        self * rhs.clone()
    }
}
impl std::ops::Mul<Word> for &Word {
    type Output = WordResult;
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: Word) -> Self::Output {
        self.clone() * rhs
    }
}
impl std::ops::Mul<&Word> for Word {
    type Output = WordResult;
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: &Word) -> Self::Output {
        self * rhs.clone()
    }
}
impl std::ops::Mul for &Word {
    type Output = WordResult;
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: Self) -> Self::Output {
        self.clone() * rhs.clone()
    }
}

impl std::ops::Mul<LetterResult> for Word {
    type Output = WordResult;
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: LetterResult) -> Self::Output {
        match *rhs {
            Ok(rhs) => self * rhs,
            Err(rhs) => WordResult::from(Err(WordValidationError::from(rhs))),
        }
    }
}
impl std::ops::Mul<LetterResult> for &Word {
    type Output = WordResult;
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: LetterResult) -> Self::Output {
        match *rhs {
            Ok(rhs) => self * rhs,
            Err(rhs) => WordResult::from(Err(WordValidationError::from(rhs))),
        }
    }
}
impl std::ops::Mul<Word> for LetterResult {
    type Output = WordResult;
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: Word) -> Self::Output {
        match *self {
            Ok(lhs) => lhs * rhs,
            Err(lhs) => WordResult::from(Err(WordValidationError::from(lhs))),
        }
    }
}
impl std::ops::Mul<&Word> for LetterResult {
    type Output = WordResult;
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: &Word) -> Self::Output {
        match *self {
            Ok(lhs) => lhs * rhs,
            Err(lhs) => WordResult::from(Err(WordValidationError::from(lhs))),
        }
    }
}
impl std::ops::Mul<Letter> for WordResult {
    type Output = WordResult;
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: Letter) -> Self::Output {
        match &*self {
            Ok(lhs) => lhs * rhs,
            Err(lhs) => WordResult::from(Err(*lhs)),
        }
    }
}
impl std::ops::Mul<WordResult> for Letter {
    type Output = WordResult;
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: WordResult) -> Self::Output {
        match &*rhs {
            Ok(rhs) => self * rhs,
            Err(rhs) => WordResult::from(Err(*rhs)),
        }
    }
}
impl std::ops::Mul<WordResult> for Word {
    type Output = WordResult;
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: WordResult) -> Self::Output {
        match &*rhs {
            Ok(rhs) => self * rhs,
            Err(rhs) => WordResult::from(Err(*rhs)),
        }
    }
}
impl std::ops::Mul<WordResult> for &Word {
    type Output = WordResult;
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: WordResult) -> Self::Output {
        match &*rhs {
            Ok(rhs) => self * rhs,
            Err(rhs) => WordResult::from(Err(*rhs)),
        }
    }
}
impl std::ops::Mul<Word> for WordResult {
    type Output = WordResult;
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: Word) -> Self::Output {
        match &*self {
            Ok(lhs) => lhs * rhs,
            Err(lhs) => WordResult::from(Err(*lhs)),
        }
    }
}
impl std::ops::Mul<&Word> for WordResult {
    type Output = WordResult;
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: &Word) -> Self::Output {
        match &*self {
            Ok(lhs) => lhs * rhs,
            Err(lhs) => WordResult::from(Err(*lhs)),
        }
    }
}
impl std::ops::Mul<LetterResult> for WordResult {
    type Output = WordResult;
    #[tracing::instrument(level = "info")]
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
    #[tracing::instrument(level = "info")]
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
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: Self) -> Self::Output {
        match (&*self, &*rhs) {
            (Ok(lhs), Ok(rhs)) => lhs * rhs,
            (Err(lhs), _) => WordResult::from(Err(*lhs)),
            (_, Err(rhs)) => WordResult::from(Err(*rhs)),
        }
    }
}

impl std::ops::Mul<Letter> for &WordResult {
    type Output = WordResult;
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: Letter) -> Self::Output {
        (*self).clone() * rhs
    }
}
impl std::ops::Mul<&WordResult> for Letter {
    type Output = WordResult;
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: &WordResult) -> Self::Output {
        self * (*rhs).clone()
    }
}
impl std::ops::Mul<LetterResult> for &WordResult {
    type Output = WordResult;
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: LetterResult) -> Self::Output {
        (*self).clone() * rhs
    }
}
impl std::ops::Mul<&WordResult> for LetterResult {
    type Output = WordResult;
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: &WordResult) -> Self::Output {
        self * (*rhs).clone()
    }
}
impl std::ops::Mul<Word> for &WordResult {
    type Output = WordResult;
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: Word) -> Self::Output {
        (*self).clone() * rhs
    }
}
impl std::ops::Mul<&WordResult> for Word {
    type Output = WordResult;
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: &WordResult) -> Self::Output {
        self * (*rhs).clone()
    }
}
impl std::ops::Mul<&Word> for &WordResult {
    type Output = WordResult;
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: &Word) -> Self::Output {
        (*self).clone() * rhs
    }
}
impl std::ops::Mul<&WordResult> for &Word {
    type Output = WordResult;
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: &WordResult) -> Self::Output {
        self * (*rhs).clone()
    }
}
impl std::ops::Mul<WordResult> for &WordResult {
    type Output = WordResult;
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: WordResult) -> Self::Output {
        (*self).clone() * rhs
    }
}
impl std::ops::Mul<&WordResult> for WordResult {
    type Output = WordResult;
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: &WordResult) -> Self::Output {
        self * (*rhs).clone()
    }
}
impl std::ops::Mul for &WordResult {
    type Output = WordResult;
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: Self) -> Self::Output {
        (*self).clone() * (*rhs).clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Letter, Sign, Word};
    use googletest::matchers::{eq, ok};
    use googletest::{expect_that, gtest};

    #[gtest]
    fn trivial_word_is_multiplicative_identity_for_letters_and_words() {
        let letter = Letter::try_new(1, None::<u16>, Sign::Positive).unwrap();

        expect_that!(
            *(letter * Word::trivial()),
            ok(eq(&Word::try_from_letters(&[letter]).clone_unwrap()))
        );
        expect_that!(
            *(Word::trivial() * letter),
            ok(eq(&Word::try_from_letters(&[letter]).clone_unwrap()))
        );

        let word = Word::try_from_letters(&[
            Letter::try_new(1, Some(3), Sign::Positive).unwrap(),
            Letter::try_new(2, None::<u16>, Sign::Negative).unwrap(),
            Letter::try_new(1, Some(2), Sign::Positive).unwrap(),
        ])
        .clone_unwrap();

        expect_that!(*(word.clone() * Word::trivial()), ok(eq(&word)));
        expect_that!(*(Word::trivial() * word.clone()), ok(eq(&word)))
    }

    // #[gtest]
    // fn multiplication_of_word_with_inverse_yields_trivial_word() {
    //     let word = Word::try_from_letters(&[
    //         Letter::try_new(1, Some(3), Sign::Positive).unwrap(),
    //         Letter::try_new(2, None::<u16>, Sign::Negative).unwrap(),
    //         Letter::try_new(1, Some(2), Sign::Positive).unwrap(),
    //     ])
    //     .clone_unwrap();
    //
    //     expect_that!(*(word.clone() * word.inverse()), ok(eq(&Word::trivial())));
    //     expect_that!(*(word.inverse() * word.clone()), ok(eq(&Word::trivial())));
    // }
}
