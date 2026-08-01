#[macro_export]
macro_rules! artin {
    ($foot:expr; $power:expr) => {{
        let letter = if $power < 0 {
            $crate::ArtinGenerator::new($foot, $crate::Sign::Negative)
        } else {
            $crate::ArtinGenerator::new($foot, $crate::Sign::Positive)
        };
        let repetitions: usize = ($power as i16).abs().try_into().unwrap();
        let result: Result<Vec<$crate::ArtinGenerator>, $crate::ArtinValidationError> = match letter
        {
            Ok(generator) => Ok(vec![generator; repetitions]),
            Err(e) => Err(e),
        };
        result
    }};
}

#[macro_export]
macro_rules! band {
    ($foot:expr => $head:expr; $power:expr) => {{
        let letter = if $power < 0 {
            $crate::BandGenerator::new($foot, $head, $crate::Sign::Negative)
        } else {
            $crate::BandGenerator::new($foot, $head, $crate::Sign::Positive)
        };
        let repetitions: usize = ($power as i16).abs().try_into().unwrap();
        let result: Result<Vec<$crate::BandGenerator>, $crate::BandValidationError> = match letter {
            Ok(generator) => Ok(vec![generator; repetitions]),
            Err(e) => Err(e),
        };
        result
    }};
}

#[cfg(test)]
mod tests {
    use crate::{ArtinGenerator, BandGenerator, Sign};
    use googletest::assert_that;
    use googletest::matchers::{each, eq, is_empty, len, ok};

    // artin!

    #[test]
    fn macro_artin_with_zero_returns_empty_vector() {
        let trivial = artin![9; 0];
        assert_that!(trivial, ok(is_empty()));
    }

    #[test]
    fn macro_artin_with_power_returns_vector_of_positive_artin_generators() {
        let power_generator = artin![3; 4];
        assert_that!(power_generator, ok(len(eq(4))));
        assert_that!(
            power_generator,
            ok(each(eq(&ArtinGenerator::new(3, Sign::Positive).unwrap())))
        );
    }

    #[test]
    fn macro_artin_with_negative_power_returns_vector_of_negative_artin_generators() {
        let power_generator = artin![7; -5];
        assert_that!(power_generator, ok(len(eq(5))));
        assert_that!(
            power_generator,
            ok(each(eq(&ArtinGenerator::new(7, Sign::Negative).unwrap())))
        );
    }

    // band!

    #[test]
    fn macro_band_with_zero_creates_trivial_word() {
        let band = band![1 => 9; 0];

        assert_that!(band, ok(is_empty()));
    }

    #[test]
    fn macro_band_with_positive_power_creates_repeated_positive_band_word() {
        let band_power = band![6 => 11; 5];

        assert_that!(band_power, ok(len(eq(5))));
        assert_that!(
            band_power,
            ok(each(
                eq(&BandGenerator::new(6, 11, Sign::Positive).unwrap())
            ))
        )
    }

    #[test]
    fn macro_band_with_negative_power_creates_repeated_negative_band_word() {
        let band_power = band![6 => 11; -8];

        assert_that!(band_power, ok(len(eq(8))));
        assert_that!(
            band_power,
            ok(each(
                eq(&BandGenerator::new(6, 11, Sign::Negative).unwrap())
            ))
        )
    }
}
