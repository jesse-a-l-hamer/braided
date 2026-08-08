use crate::{Letter, Word, WordValidationError};

impl std::ops::Mul for Letter {
    type Output = Result<Word, WordValidationError>;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Artin(lhs), Self::Artin(rhs)) => {
                if lhs == rhs.inverse() {
                    Ok(Word::trivial())
                } else {
                    Word::try_from(vec![lhs, rhs])
                }
            }
            (Self::Artin(lhs), Self::Band(rhs)) => {
                if rhs.inverse() == lhs.into() {
                    Ok(Word::trivial())
                } else {
                    Word::try_from(vec![Self::Artin(lhs), Self::Band(rhs)])
                }
            }
            (Self::Band(lhs), Self::Artin(rhs)) => {
                if lhs.inverse() == rhs.into() {
                    Ok(Word::trivial())
                } else {
                    Word::try_from(vec![Self::Band(lhs), Self::Artin(rhs)])
                }
            }
            (Self::Band(lhs), Self::Band(rhs)) => {
                if lhs == rhs.inverse() {
                    Ok(Word::trivial())
                } else {
                    Word::try_from(vec![lhs, rhs])
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Letter, Sign, Word, WordValidationError};
    use googletest::matchers::{eq, err, ok};
    use googletest::{expect_that, gtest};

    #[gtest]
    fn valid_multiplication_succeeds_and_computes_as_expected() {
        let l1 = Letter::new(1, None::<u16>, Sign::Positive).unwrap();
        let l2 = Letter::new(2, Some(4), Sign::Negative).unwrap();

        let product_data = [vec![l1, l1], vec![l1, l2], vec![l2, l1], vec![l2, l2]];

        for pair in product_data {
            expect_that!(pair[0] * pair[1], eq(&Word::try_from(pair)));
        }
    }

    #[gtest]
    fn inverse_is_multiplicative_inverse() {
        let l1 = Letter::new(1, None::<u16>, Sign::Positive).unwrap();
        let l2 = Letter::new(2, Some(4), Sign::Negative).unwrap();
        let l3 = Letter::new(1, Some(2), Sign::Negative).unwrap();

        let product_data = [
            [l1, l1.inverse()],
            [l1.inverse(), l1],
            [l2, l2.inverse()],
            [l2.inverse(), l2],
            [l1, l3],
            [l3, l1],
        ];

        for pair in product_data {
            expect_that!(pair[0] * pair[1], ok(eq(&Word::trivial())));
        }
    }

    #[gtest]
    fn invalid_multiplication_fails() {
        let l1 = Letter::new(1, None::<u16>, Sign::Positive).unwrap();
        let l2 = Letter::new(1, Some(2u16.pow(15) + 1), Sign::Negative).unwrap();
        let error = WordValidationError::TooLong(u16::MAX as u32 + 1);

        expect_that!(l1 * l2, err(eq(&error)));
        expect_that!(l2 * l1, err(eq(&error)));
    }
}
