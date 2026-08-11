/// Represents the sign of a crossing in a braid.
///
/// # Examples
///
/// [`Sign`] can be negated:
///
/// ```
/// use braided::Sign;
///
/// assert_eq!(-Sign::Positive, Sign::Negative);
/// assert_eq!(-Sign::Negative, Sign::Positive);
/// ```
///
/// There is no "zero" variant. Something like the following will not compile:
///
/// ```compile_fail
/// let s = Sign::Zero;
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sign {
    /// The sign of a _positive_ crossing.
    ///
    /// If strands are stacked vertically and oriented left-to-right, then a positive crossing
    /// corresponds to the foot strand passing _under_ the head strand. This can be verified via
    /// the right-hand rule.
    Positive,
    /// The sign of a _negative_ crossing.
    ///
    /// If strands are stacked vertically and oriented left-to-right, then a negative crossing
    /// corresponds to the foot strand passing _over_ the head strand. This can be verified via
    /// the right-hand rule.
    Negative,
}

impl std::ops::Neg for Sign {
    type Output = Self;

    #[tracing::instrument(level="trace")]
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
