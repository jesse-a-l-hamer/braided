use crate::{
    ArtinGenerator, BandGenerator, BraidIndex, Letter, LetterValidationError, Sign,
    StrandValidationError,
};

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum WordValidationError {
    #[error("Attempting to create word of length {0} > {max}", max = u16::MAX)]
    TooLong(usize),
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
        LetterValidationError: From<<F as TryInto<u16>>::Error>
            + From<<H as TryInto<u16>>::Error>
            + From<std::convert::Infallible>,
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
    pub fn letters(&self) -> Vec<Letter> {
        self.0.clone()
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
                    .map(|l| l.clone().try_into().unwrap())
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

    pub fn is_trivial(&self) -> bool {
        self.0.is_empty()
    }
}

impl<L> TryFrom<Vec<L>> for Word
where
    L: Into<Letter>,
{
    type Error = WordValidationError;
    fn try_from(value: Vec<L>) -> Result<Self, Self::Error> {
        let mut letters: Vec<Letter> = Vec::new();
        for l in value.into_iter() {
            letters.push(l.into())
        }
        if let total_len = letters.iter().map(|l| l.artin_length() as usize).sum()
            && total_len > u16::MAX as usize
        {
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
impl std::ops::DerefMut for Word {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0[..]
    }
}

impl std::ops::Mul<Letter> for Word {
    type Output = Result<Word, WordValidationError>;

    fn mul(self, rhs: Letter) -> Self::Output {
        if let Some((last, rem)) = self.split_last() {
            Self::try_from([rem, &(last.clone() * rhs)?].concat())
        } else {
            Self::try_from(vec![rhs])
        }
    }
}
impl std::ops::Mul for Word {
    type Output = Result<Word, WordValidationError>;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.is_trivial() {
            Ok(rhs)
        } else if rhs.is_trivial() {
            Ok(self)
        } else {
            let (first, rem) = rhs.split_first().unwrap();
            (self * first.clone())? * Word::try_from(rem)?
        }
    }
}
