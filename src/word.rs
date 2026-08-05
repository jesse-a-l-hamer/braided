use crate::{
    ArtinGenerator, BandGenerator, BraidIndex, Letter, LetterValidationError, Sign,
    StrandValidationError,
};

#[derive(Debug, thiserror::Error, PartialEq, Eq, Clone, Copy)]
pub enum WordValidationError {
    #[error("Attempting to create word of length {0} > {max}", max = u16::MAX)]
    TooLong(u32),
    #[error(transparent)]
    LetterValidation(#[from] LetterValidationError),
    #[error(transparent)]
    FromInt(#[from] std::num::TryFromIntError),
    #[error(transparent)]
    Infallible(#[from] std::convert::Infallible),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Word(Vec<Letter>);

impl Word {
    pub fn new<D, F, H>(letter_data: D) -> Result<Self, WordValidationError>
    where
        D: IntoIterator<Item = (F, Option<H>, Sign)>,
        F: TryInto<u16>,
        H: TryInto<u16>,
        StrandValidationError: From<<F as TryInto<u16>>::Error>
            + From<<H as TryInto<u16>>::Error>
            + From<std::convert::Infallible>,
    {
        let mut letters = Vec::new();
        for (foot, head, sign) in letter_data {
            letters.push(Letter::new(foot, head, sign)?)
        }
        Word::try_from(letters)
    }
    pub fn trivial() -> Self {
        Self(Vec::new())
    }

    pub fn decompose(&self) -> Self {
        let mut artin_generators: Vec<ArtinGenerator> = Vec::new();
        for letter in self.iter() {
            match letter {
                Letter::Artin(artin_generator) => artin_generators.push(*artin_generator),
                Letter::Band(band_generator) => artin_generators.extend(band_generator.decompose()),
            }
        }
        Self::try_from(artin_generators).unwrap()
    }
    pub fn coalesce(&self) -> Self {
        // The coalescing algorithm requires that we start from a word which has been completely
        // decomposed as Artin generators.
        self.decompose().coalesce_decomposed()
    }
    fn coalesce_decomposed(&self) -> Self {
        if self.is_trivial() {
            return self.clone();
        }
        let num_letters = self.len(); // guaranteed to be > 0 since word is not trivial

        let mut radius = (num_letters - 1).div_euclid(2);
        let mut pivot = radius; // pivot is the index of the candidate band crossing

        // The following loop returns eventually, since if radius == 0, then window consists
        // of a single Artin generator, which trivially transforms into a band generator.
        loop {
            while pivot + radius < num_letters {
                let remaining_left = Word(self[0..pivot - radius].to_vec());
                let window: Vec<ArtinGenerator> = self[pivot - radius..pivot + radius + 1]
                    .iter()
                    .map(|l| (*l).try_into().unwrap())
                    .collect();
                let remaining_right = Word(self[pivot + radius + 1..num_letters].to_vec());
                if let Ok(band) = BandGenerator::coalesce(&window) {
                    // We can safely unwrap the following products, since we're operating on parts
                    // of a word which has already been length-checked (at its construction)
                    return ((remaining_left.coalesce_decomposed() * Letter::Band(band)).unwrap()
                        * remaining_right.coalesce_decomposed())
                    .unwrap();
                } else {
                    pivot += 1;
                }
            }
            radius -= 1;
        }
    }

    pub fn letters(&self) -> Vec<Letter> {
        self.0.clone()
    }
    pub fn is_trivial(&self) -> bool {
        self.0.is_empty()
    }
    pub fn length(&self) -> u16 {
        // length checks taken care of at construction, so unwrapping here is safe
        self.len().try_into().unwrap()
    }
    pub fn artin_length(&self) -> u16 {
        self.iter().map(|l| l.artin_length()).sum()
    }
    pub fn minimal_required_braid_index(&self) -> BraidIndex {
        self.iter()
            .map(|l| l.minimal_required_braid_index())
            .max()
            .unwrap_or(BraidIndex::new(1).unwrap())
    }
    pub fn inverse(&self) -> Self {
        Self(self.iter().rev().map(|l| l.inverse()).collect())
    }
}

impl Default for Word {
    fn default() -> Self {
        Word::trivial()
    }
}

impl<L> TryFrom<Vec<L>> for Word
where
    L: Into<Letter>,
{
    type Error = WordValidationError;
    fn try_from(value: Vec<L>) -> Result<Self, Self::Error> {
        let (total_len, letters) = value
            .into_iter()
            .map(|l| l.into())
            .map(|l| (l.artin_length() as u32, l))
            .fold((0u32, Vec::<Letter>::new()), |mut acc, (al, l)| {
                acc.0 += al;
                acc.1.push(l);
                acc
            });
        if total_len > u16::MAX as u32 {
            Err(WordValidationError::TooLong(total_len))
        } else {
            Ok(Self(letters))
        }
    }
}
impl<L> TryFrom<&[L]> for Word
where
    L: Into<Letter> + std::clone::Clone,
{
    type Error = WordValidationError;
    fn try_from(value: &[L]) -> Result<Self, Self::Error> {
        Self::try_from(value.to_vec())
    }
}

impl IntoIterator for Word {
    type Item = (u16, Option<u16>, Sign);
    type IntoIter = <Vec<(u16, Option<u16>, Sign)> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0
            .iter()
            .map(|l| match l {
                Letter::Artin(artin) => (artin.foot().into(), None, artin.sign()),
                Letter::Band(band) => (band.foot().into(), Some(band.head().into()), band.sign()),
            })
            .collect::<Vec<_>>()
            .into_iter()
    }
}
impl std::ops::Deref for Word {
    type Target = [Letter];

    fn deref(&self) -> &Self::Target {
        &self.0[..]
    }
}
impl AsRef<[Letter]> for Word {
    fn as_ref(&self) -> &[Letter] {
        self
    }
}

impl std::ops::Mul<Letter> for Word {
    type Output = Result<Word, WordValidationError>;
    fn mul(self, rhs: Letter) -> Self::Output {
        if let Some((lhs_last, lhs_initial)) = self.split_last() {
            match *lhs_last * rhs {
                Ok(tail) => Self::try_from([lhs_initial, &tail].concat()),
                Err(WordValidationError::TooLong(tail_length)) => {
                    Err(WordValidationError::TooLong(
                        lhs_initial
                            .iter()
                            .map(|l| l.artin_length() as u32)
                            .sum::<u32>()
                            + tail_length,
                    ))
                }
                Err(e) => Err(e),
            }
        } else {
            Self::try_from(vec![rhs])
        }
    }
}
impl std::ops::Mul<Word> for Letter {
    type Output = Result<Word, WordValidationError>;
    fn mul(self, rhs: Word) -> Self::Output {
        if let Some((rhs_first, rhs_tail)) = rhs.split_first() {
            match self * *rhs_first {
                Ok(initial) => Word::try_from([&initial, rhs_tail].concat()),
                Err(WordValidationError::TooLong(initial_length)) => {
                    Err(WordValidationError::TooLong(
                        initial_length
                            + rhs_tail
                                .iter()
                                .map(|l| l.artin_length() as u32)
                                .sum::<u32>(),
                    ))
                }
                Err(e) => Err(e),
            }
        } else {
            Word::try_from(vec![self])
        }
    }
}
impl std::ops::Mul for Word {
    type Output = Result<Word, WordValidationError>;
    fn mul(self, rhs: Self) -> Self::Output {
        let product_length = self.artin_length() as u32 + rhs.artin_length() as u32;
        if product_length > u16::MAX as u32 {
            return Err(WordValidationError::TooLong(product_length));
        }

        // At this point we can be sure that the product exists.
        // We will try to cancel as much as possible.
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
        Ok(Self(
            [&self[..self.len() - radius], &rhs[radius..]].concat(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::{Letter, Sign, Word, WordValidationError};
    use googletest::matchers::{anything, each, eq, err, is_false, is_true, ok, result_of_ref};
    use googletest::{assert_that, expect_that, gtest};

    #[gtest]
    fn construction_with_try_from_succeeds() {
        let letters = vec![
            Letter::new(1, None::<u16>, Sign::Positive).unwrap(),
            Letter::new(2, Some(5), Sign::Negative).unwrap(),
        ];
        let valid_words = [
            Word::try_from(letters.clone()),
            Word::try_from(&letters[..]),
        ];

        expect_that!(valid_words, each(ok(anything())));
        expect_that!(valid_words[0], eq(&valid_words[1]));
    }

    #[test]
    fn valid_construction_with_new_succeeds_and_is_as_expected() {
        let letter_data = [
            (1, None::<u16>, Sign::Positive),
            (2, Some(5), Sign::Negative),
        ];
        let valid_word = Word::new(letter_data);
        assert_that!(valid_word, ok(anything()));

        let letters: Vec<Letter> = letter_data
            .iter()
            .map(|(foot, head, sign)| Letter::new(*foot, *head, *sign).unwrap())
            .collect();
        assert_that!(valid_word, eq(&Word::try_from(letters)));
    }

    #[test]
    fn trivial_works_as_expected() {
        let trivial = Word::trivial();

        assert_that!(
            trivial,
            eq(&Word::new(Vec::<(u16, Option<u16>, _)>::new()).unwrap())
        );
    }

    #[test]
    fn default_word_is_trivial() {
        assert_that!(Word::default(), eq(&Word::trivial()));
    }

    #[test]
    fn decompose_computes_as_expected() {
        let word = Word::new([
            (1, Some(3), Sign::Positive),
            (2, None, Sign::Negative),
            (1, Some(2), Sign::Positive),
        ])
        .unwrap();
        let expected_decomposition = Word::new([
            (1, None::<u16>, Sign::Negative),
            (2, None, Sign::Positive),
            (1, None, Sign::Positive),
            (2, None, Sign::Negative),
            (1, None, Sign::Positive),
        ])
        .unwrap();

        assert_that!(word.decompose(), eq(&expected_decomposition));
    }

    #[test]
    fn coalesce_computes_as_expected() {
        let word = Word::new([
            (2, None::<u16>, Sign::Positive),
            (1, None, Sign::Positive),
            (2, None, Sign::Negative),
            (2, None, Sign::Negative),
            (1, None, Sign::Positive),
        ])
        .unwrap();
        let expected_coalescence = Word::new([
            (1, Some(3), Sign::Positive),
            (2, None, Sign::Negative),
            (1, Some(2), Sign::Positive),
        ])
        .unwrap();

        assert_that!(word.coalesce(), eq(&expected_coalescence));
    }

    #[gtest]
    fn properties_compute_as_expected() {
        let letters = vec![
            Letter::new(1, Some(3), Sign::Positive).unwrap(),
            Letter::new(2, None::<u16>, Sign::Negative).unwrap(),
            Letter::new(1, Some(2), Sign::Positive).unwrap(),
        ];
        let word = Word::try_from(&letters[..]).unwrap();

        expect_that!(word.letters(), eq(&letters));
        expect_that!(
            word.inverse(),
            eq(&Word::try_from(
                letters
                    .iter()
                    .rev()
                    .map(|&l| l.inverse())
                    .collect::<Vec<Letter>>()
            )
            .unwrap())
        );
        expect_that!(word.is_trivial(), is_false());
        expect_that!(Word::trivial().is_trivial(), is_true());
        expect_that!(word.length(), eq(letters.len().try_into().unwrap()));
        expect_that!(
            word.artin_length(),
            eq(letters.iter().map(|&l| l.artin_length()).sum())
        );
        expect_that!(
            word.minimal_required_braid_index(),
            eq(letters
                .iter()
                .map(|&l| l.minimal_required_braid_index())
                .max()
                .unwrap()),
        );
    }

    #[gtest]
    fn into_iterator_works_as_expected() {
        let letter_data = [
            (1, Some(3), Sign::Positive),
            (2, None, Sign::Negative),
            (1, Some(2), Sign::Positive),
        ];
        let word = Word::new(letter_data).unwrap();

        for (actual, expected) in word.into_iter().zip(letter_data) {
            expect_that!(actual, eq(expected));
        }
    }

    #[test]
    fn deref_yields_slice_of_letters() {
        let letters = [
            Letter::new(1, Some(3), Sign::Positive).unwrap(),
            Letter::new(2, None::<u16>, Sign::Negative).unwrap(),
            Letter::new(1, Some(2), Sign::Positive).unwrap(),
        ];
        let word = Word::try_from(&letters[..]).unwrap();

        assert_that!(*word, eq(&letters));
    }

    #[test]
    fn word_can_be_passed_as_ref_to_slice_of_letters() {
        fn as_ref_tester<W: AsRef<[Letter]>>(w: W, v: &[Letter]) -> bool {
            w.as_ref() == v
        }
    }
}
