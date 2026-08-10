use crate::{Letter, LetterResult, Word, WordResult, WordValidationError};

impl std::ops::Mul for Letter {
    type Output = WordResult;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Artin(lhs), Self::Artin(rhs)) => {
                if lhs == rhs.inverse() {
                    WordResult::from(Word::trivial())
                } else {
                    Word::try_from_letters(&[lhs, rhs])
                }
            }
            (Self::Artin(lhs), Self::Band(rhs)) => {
                if rhs.inverse() == lhs.into() {
                    WordResult::from(Word::trivial())
                } else {
                    Word::try_from_letters(&[Self::Artin(lhs), Self::Band(rhs)])
                }
            }
            (Self::Band(lhs), Self::Artin(rhs)) => {
                if lhs.inverse() == rhs.into() {
                    WordResult::from(Word::trivial())
                } else {
                    Word::try_from_letters(&[Self::Band(lhs), Self::Artin(rhs)])
                }
            }
            (Self::Band(lhs), Self::Band(rhs)) => {
                if lhs == rhs.inverse() {
                    WordResult::from(Word::trivial())
                } else {
                    Word::try_from_letters(&[lhs, rhs])
                }
            }
        }
    }
}

impl std::ops::Mul<Letter> for LetterResult {
    type Output = WordResult;
    fn mul(self, rhs: Letter) -> Self::Output {
        match *self {
            Ok(lhs) => lhs * rhs,
            Err(lhs) => WordResult::from(Err(WordValidationError::from(lhs))),
        }
    }
}
impl std::ops::Mul<LetterResult> for Letter {
    type Output = WordResult;
    fn mul(self, rhs: LetterResult) -> Self::Output {
        match *rhs {
            Ok(rhs) => self * rhs,
            Err(rhs) => WordResult::from(Err(WordValidationError::from(rhs))),
        }
    }
}
impl std::ops::Mul for LetterResult {
    type Output = WordResult;
    fn mul(self, rhs: LetterResult) -> Self::Output {
        match (*self, *rhs) {
            (Ok(lhs), Ok(rhs)) => lhs * rhs,
            (Err(lhs), _) => WordResult::from(Err(WordValidationError::from(lhs))),
            (_, Err(rhs)) => WordResult::from(Err(WordValidationError::from(rhs))),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Letter, Sign, Word};
    use googletest::matchers::{eq, ok};
    use googletest::{expect_that, gtest};

    #[gtest]
    fn inverse_is_multiplicative_inverse() {
        let l1 = Letter::try_new(1, None::<u16>, Sign::Positive).unwrap();
        let l2 = Letter::try_new(2, Some(4), Sign::Negative).unwrap();
        let l3 = Letter::try_new(1, Some(2), Sign::Negative).unwrap();

        let product_data = [
            [l1, l1.inverse()],
            [l1.inverse(), l1],
            [l2, l2.inverse()],
            [l2.inverse(), l2],
            [l1, l3],
            [l3, l1],
        ];

        for pair in product_data {
            expect_that!(*(pair[0] * pair[1]), ok(eq(&Word::trivial())));
        }
    }
}
