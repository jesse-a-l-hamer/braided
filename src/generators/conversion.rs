use crate::{ArtinGenerator, BandGenerator, Sign};

pub fn artin_to_band(generators: &[ArtinGenerator]) -> Vec<BandGenerator> {
    let mut bands = Vec::new();
    let num_generators = generators.len();
    if num_generators == 0 {
        return bands;
    }

    let mut radius = (num_generators - 1).div_euclid(2);
    let mut pivot = radius;

    loop {
        while pivot + radius < num_generators {
            let remaining_left = &generators[0..pivot - radius];
            let window = &generators[pivot - radius..pivot + radius + 1];
            let remaining_right = &generators[pivot + radius + 1..num_generators];
            if let Ok(band) = BandGenerator::from_artin(window) {
                bands.extend(artin_to_band(remaining_left));
                bands.push(band);
                bands.extend(artin_to_band(remaining_right));
                return bands;
            } else {
                pivot += 1;
            }
        }
        radius -= 1;
    }
}

pub fn band_to_artin(bands: &[BandGenerator]) -> Vec<ArtinGenerator> {
    bands
        .iter()
        .fold(Vec::new(), |w, b| [w, decompose_band(b)].concat())
}

fn decompose_band(band: &BandGenerator) -> Vec<ArtinGenerator> {
    // Band decomposition is infallible, so it's safe to unwrap any intermediate results
    let crossing = ArtinGenerator::new(band.head().index() - 1, band.sign()).unwrap();
    let mut left = Vec::new();
    for foot_idx in band.foot().index()..(band.head().index() - 1) {
        left.push(ArtinGenerator::new(foot_idx, Sign::Negative).unwrap());
    }
    let right = left.iter().rev().map(|a| -*a).collect();
    [left, vec![crossing], right].concat()
}

#[cfg(test)]
mod tests {
    use super::{artin_to_band, band_to_artin};
    use crate::{ArtinGenerator, BandGenerator, artin, band};
    use googletest::matchers::eq;
    use googletest::{assert_that, expect_that, gtest};

    // We'll be using these fixtures a lot
    fn get_band_word() -> Vec<BandGenerator> {
        vec![
            band![1, 4; +].unwrap(),
            band![2, 4; -].unwrap(),
            band![2, 5; -].unwrap(),
            band![1, 2; -].unwrap(),
            band![1, 3; +].unwrap(),
            band![3, 5; +].unwrap(),
        ]
    }
    fn get_artin_word_with_band_crossings_at_top_of_band() -> Vec<ArtinGenerator> {
        vec![
            // band 1
            artin![1; -].unwrap(),
            artin![2; -].unwrap(),
            artin![3; +].unwrap(),
            artin![2; +].unwrap(),
            artin![1; +].unwrap(),
            // band 2
            artin![2; -].unwrap(),
            artin![3; -].unwrap(),
            artin![2; +].unwrap(),
            // band 3
            artin![2; -].unwrap(),
            artin![3; -].unwrap(),
            artin![4; -].unwrap(),
            artin![3; +].unwrap(),
            artin![2; +].unwrap(),
            // band 4
            artin![1; -].unwrap(),
            // band 5
            artin![1; -].unwrap(),
            artin![2; +].unwrap(),
            artin![1; +].unwrap(),
            // band 6
            artin![3; -].unwrap(),
            artin![4; +].unwrap(),
            artin![3; +].unwrap(),
        ]
    }
    fn get_artin_word_with_band_crossings_at_arbitrary_positions() -> Vec<ArtinGenerator> {
        vec![
            // band 1
            artin![3; +].unwrap(),
            artin![2; +].unwrap(),
            artin![1; +].unwrap(),
            artin![2; -].unwrap(),
            artin![3; -].unwrap(),
            // band 2
            artin![3; +].unwrap(),
            artin![2; -].unwrap(),
            artin![3; -].unwrap(),
            // band 3
            artin![4; +].unwrap(),
            artin![2; -].unwrap(),
            artin![3; -].unwrap(),
            artin![4; -].unwrap(),
            artin![2; +].unwrap(),
            // band 4
            artin![1; -].unwrap(),
            // band 5
            artin![2; +].unwrap(),
            artin![1; +].unwrap(),
            artin![2; -].unwrap(),
            // band 6
            artin![4; +].unwrap(),
            artin![3; +].unwrap(),
            artin![4; -].unwrap(),
        ]
    }

    #[gtest]
    fn successful_conversion_from_artin_to_band() {
        let artin_word_with_band_crossings_at_top_of_band =
            get_artin_word_with_band_crossings_at_top_of_band();
        let artin_word_with_band_crossings_at_arbitrary_positions =
            get_artin_word_with_band_crossings_at_arbitrary_positions();
        expect_that!(
            artin_to_band(&artin_word_with_band_crossings_at_top_of_band),
            eq(&get_band_word())
        );
        expect_that!(
            artin_to_band(&artin_word_with_band_crossings_at_arbitrary_positions),
            eq(&get_band_word())
        );
    }

    #[test]
    fn successful_conversion_from_band_to_artin() {
        let band_word = get_band_word();
        assert_that!(
            band_to_artin(&band_word),
            eq(&get_artin_word_with_band_crossings_at_top_of_band())
        )
    }

    #[gtest]
    fn band_to_artin_is_left_inverse_of_artin_to_band() {
        let band_word = get_band_word();
        let converted_band_word = band_to_artin(&band_word);
        expect_that!(artin_to_band(&band_to_artin(&band_word)), eq(&band_word));
        expect_that!(
            band_to_artin(&artin_to_band(&converted_band_word)),
            eq(&converted_band_word)
        );
    }

    #[gtest]
    fn artin_to_band_is_left_inverse_of_band_to_artin_for_specific_band_representation() {
        let artin_word = get_artin_word_with_band_crossings_at_top_of_band();
        let converted_artin_word = artin_to_band(&artin_word);
        expect_that!(band_to_artin(&artin_to_band(&artin_word)), eq(&artin_word));
        expect_that!(
            artin_to_band(&band_to_artin(&converted_artin_word)),
            eq(&converted_artin_word)
        );
    }
}
