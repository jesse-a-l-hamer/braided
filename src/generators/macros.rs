/// Constructs an [`ArtinGenerator`](crate::ArtinGenerator) given a foot strand and exponent.
///
/// This macro is primarily a convenience wrapper around the constructor
/// [`ArtinGenerator::new`](crate::ArtinGenerator::new). However, unlike that constructor,
/// [`artin!`] returns a `Vec<ArtinGenerator>` on success, even if the given exponent is 1 or -1.
///
/// # Examples
///
/// ```
/// use braided::{ArtinGenerator, Sign, artin};
///
/// # #[macro_use] extern crate braided;
/// # fn main() {
/// let my_artin = artin![2; 1].unwrap();
/// assert_eq!(*my_artin.first().unwrap(), ArtinGenerator::new(2, Sign::Positive).unwrap());
///
/// let my_many_artins = artin![1; -7].unwrap();
/// assert_eq!(my_many_artins, vec![ArtinGenerator::new(1, Sign::Negative).unwrap(); 7]);
///
/// // Using an exponent of 0 returns an empty vector:
/// let trivial = artin![9; 0].unwrap();
/// assert!(trivial.is_empty());
/// # }
/// ```
///
/// # Errors
///
/// This macro calls [`ArtinGenerator::new`](crate::ArtinGenerator::new), and will fail with the
/// same errors if an invalid foot strand index is given, or if the exponent cannot be parsed as
/// an `i16`. Please consult the documentation for that function for more information.
#[macro_export]
macro_rules! artin {
    ($foot:expr; $exp:expr) => {{
        let letter = if $exp < 0 {
            $crate::ArtinGenerator::new($foot, $crate::Sign::Negative)
        } else {
            $crate::ArtinGenerator::new($foot, $crate::Sign::Positive)
        };
        let repetitions: usize = ($exp as i16).abs().try_into().unwrap();
        let result: Result<Vec<$crate::ArtinGenerator>, $crate::ArtinValidationError> = match letter
        {
            Ok(generator) => Ok(vec![generator; repetitions]),
            Err(e) => Err(e),
        };
        result
    }};
}

/// Constructs a [`BandGenerator`](crate::BandGenerator) given foot and head strands, and exponent.
///
/// This macro is primarily a convenience wrapper around the constructor
/// [`BandGenerator::new`](crate::BandGenerator::new). However, unlike that constructor,
/// [`band!`] returns a `Vec<BandGenerator>` on success, even if the given exponent is 1 or -1.
///
/// # Examples
///
/// ```
/// use braided::{BandGenerator, Sign, band};
///
/// # #[macro_use] extern crate braided;
/// # fn main() {
/// let my_band = band![2 => 5; 1].unwrap();
/// assert_eq!(*my_band.first().unwrap(), BandGenerator::new(2, 5, Sign::Positive).unwrap());
///
/// let my_many_bands = band![1 => 3; -7].unwrap();
/// assert_eq!(my_many_bands, vec![BandGenerator::new(1, 3, Sign::Negative).unwrap(); 7]);
///
/// // Using an exponent of 0 returns an empty vector:
/// let trivial = band![9 => 19; 0].unwrap();
/// assert!(trivial.is_empty());
/// # }
/// ```
///
/// # Errors
///
/// This macro calls [`BandGenerator::new`](crate::BandGenerator::new), and will fail with the
/// same errors if an invalid foot or head strand index is given, or if the exponent cannot be
/// parsed as an `i16`. Please consult the documentation for that function for more information.
#[macro_export]
macro_rules! band {
    ($foot:expr => $head:expr; $exp:expr) => {{
        let letter = if $exp < 0 {
            $crate::BandGenerator::new($foot, $head, $crate::Sign::Negative)
        } else {
            $crate::BandGenerator::new($foot, $head, $crate::Sign::Positive)
        };
        let repetitions: usize = ($exp as i16).abs().try_into().unwrap();
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
