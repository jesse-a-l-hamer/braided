use std::ops::Neg;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sign {
    Positive,
    Negative,
}

impl Neg for Sign {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Self::Positive => Self::Negative,
            Self::Negative => Self::Positive,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Sign;
    use googletest::assert_that;
    use googletest::matchers::eq;

    #[test]
    fn sign_can_be_negated() {
        assert_that!(-Sign::Positive, eq(Sign::Negative));
        assert_that!(-Sign::Negative, eq(Sign::Positive));
    }
}
