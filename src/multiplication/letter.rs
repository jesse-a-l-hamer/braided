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
    use crate::{Letter, Sign, Word, WordValidationError, letter, word};
    use googletest::matchers::{eq, err, ok};
    use googletest::{assert_that, expect_that, gtest};

    #[gtest]
    fn valid_multiplication_succeeds_and_computes_as_expected() {
        let l1 = Letter::try_new(1, None::<u16>, Sign::Positive).unwrap();
        let l2 = Letter::try_new(2, Some(4), Sign::Negative).unwrap();

        let product_data = [vec![l1, l1], vec![l1, l2], vec![l2, l1], vec![l2, l2]];

        for pair in product_data {
            expect_that!(pair[0] * pair[1], eq(&Word::try_from_letters(&pair)));
        }
    }

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

    #[gtest]
    fn invalid_multiplication_fails() {
        let l1 = Letter::try_new(1, None::<u16>, Sign::Positive).unwrap();
        let l2 = Letter::try_new(1, Some(2u16.pow(15) + 1), Sign::Negative).unwrap();
        let error = WordValidationError::TooLong(u16::MAX as usize + 1);

        expect_that!(*(l1 * l2), err(eq(&error)));
        expect_that!(*(l2 * l1), err(eq(&error)));
    }

    #[gtest]
    fn can_multiply_letter_and_letter_result() {
        let letter = letter![1; +].unwrap();
        let letter_result = letter![2; -];

        expect_that!(letter * letter_result, eq(&word![[1; 1], [2; -1]]));
        expect_that!(letter_result * letter, eq(&word![[2; -1], [1; 1]]));
    }

    #[test]
    fn can_multiply_letter_result_and_letter_result() {
        let letter_result1 = letter![1; +];
        let letter_result2 = letter![2; -];

        assert_that!(letter_result1 * letter_result2, eq(&word![[1; 1], [2; -1]]));
    }

    #[gtest]
    fn multiplication_with_error_operand_propagates_error() {
        let letter = letter![1; +].unwrap();
        let valid_letter_result = letter![2; -];
        let invalid_letter_result = letter![0; +];
        let error = WordValidationError::from(invalid_letter_result.unwrap_err());

        expect_that!(*(letter * invalid_letter_result), err(eq(&error)));
        expect_that!(*(invalid_letter_result * letter), err(eq(&error)));

        expect_that!(
            *(valid_letter_result * invalid_letter_result),
            err(eq(&error))
        );
        expect_that!(
            *(invalid_letter_result * valid_letter_result),
            err(eq(&error))
        );
        expect_that!(
            *(invalid_letter_result * letter![2 => 1; -]),
            err(eq(&error))
        );
    }
}
