use crate::{
    Braid, BraidResult, BraidValidationError, Letter, LetterResult, Word, WordResult,
    WordValidationError,
};

impl std::ops::Mul<Letter> for Braid {
    type Output = BraidResult;
    fn mul(self, rhs: Letter) -> Self::Output {
        if let required_index = rhs.minimal_required_braid_index()
            && self.braid_index() < required_index
        {
            BraidResult::from(BraidValidationError::IndexTooSmall {
                index: self.braid_index(),
                minimal_required_index: required_index,
            })
        } else {
            match &*(self.word() * rhs) {
                Ok(word) => Self::try_new(self.braid_index(), word.clone()),
                Err(e) => BraidResult::from(BraidValidationError::from(*e)),
            }
        }
    }
}
impl std::ops::Mul<Braid> for Letter {
    type Output = BraidResult;

    fn mul(self, rhs: Braid) -> Self::Output {
        if let required_index = self.minimal_required_braid_index()
            && rhs.braid_index() < required_index
        {
            BraidResult::from(BraidValidationError::IndexTooSmall {
                index: rhs.braid_index(),
                minimal_required_index: required_index,
            })
        } else {
            match &*(self * rhs.word()) {
                Ok(word) => Braid::try_new(rhs.braid_index(), word.clone()),
                Err(e) => BraidResult::from(BraidValidationError::from(*e)),
            }
        }
    }
}
impl std::ops::Mul<Word> for Braid {
    type Output = BraidResult;
    fn mul(self, rhs: Word) -> Self::Output {
        if let required_index = rhs.minimal_required_braid_index()
            && self.braid_index() < required_index
        {
            BraidResult::from(BraidValidationError::IndexTooSmall {
                index: self.braid_index(),
                minimal_required_index: required_index,
            })
        } else {
            match &*(self.word() * rhs) {
                Ok(word) => Self::try_new(self.braid_index(), word.clone()),
                Err(e) => BraidResult::from(BraidValidationError::from(*e)),
            }
        }
    }
}
impl std::ops::Mul<Braid> for Word {
    type Output = BraidResult;

    fn mul(self, rhs: Braid) -> Self::Output {
        if let required_index = self.minimal_required_braid_index()
            && rhs.braid_index() < required_index
        {
            BraidResult::from(BraidValidationError::IndexTooSmall {
                index: rhs.braid_index(),
                minimal_required_index: required_index,
            })
        } else {
            match &*(self * rhs.word()) {
                Ok(word) => Braid::try_new(rhs.braid_index(), word.clone()),
                Err(e) => BraidResult::from(BraidValidationError::from(*e)),
            }
        }
    }
}
impl std::ops::Mul for Braid {
    type Output = BraidResult;
    fn mul(self, rhs: Self) -> Self::Output {
        if self.braid_index() != rhs.braid_index() {
            BraidResult::from(BraidValidationError::UnequalIndices {
                left: self.braid_index(),
                right: rhs.braid_index(),
            })
        } else {
            match &*(self.word() * rhs.word()) {
                Ok(word) => Self::try_new(self.braid_index(), word.clone()),
                Err(e) => BraidResult::from(BraidValidationError::from(*e)),
            }
        }
    }
}

impl std::ops::Mul<Letter> for &Braid {
    type Output = BraidResult;
    fn mul(self, rhs: Letter) -> Self::Output {
        self.clone() * rhs
    }
}
impl std::ops::Mul<&Braid> for Letter {
    type Output = BraidResult;
    fn mul(self, rhs: &Braid) -> Self::Output {
        self * rhs.clone()
    }
}
impl std::ops::Mul<Word> for &Braid {
    type Output = BraidResult;
    fn mul(self, rhs: Word) -> Self::Output {
        self.clone() * rhs
    }
}
impl std::ops::Mul<&Braid> for Word {
    type Output = BraidResult;
    fn mul(self, rhs: &Braid) -> Self::Output {
        self * rhs.clone()
    }
}
impl std::ops::Mul<&Word> for Braid {
    type Output = BraidResult;
    fn mul(self, rhs: &Word) -> Self::Output {
        self * rhs.clone()
    }
}
impl std::ops::Mul<Braid> for &Word {
    type Output = BraidResult;
    fn mul(self, rhs: Braid) -> Self::Output {
        self.clone() * rhs
    }
}
impl std::ops::Mul<&Word> for &Braid {
    type Output = BraidResult;
    fn mul(self, rhs: &Word) -> Self::Output {
        self.clone() * rhs.clone()
    }
}
impl std::ops::Mul<&Braid> for &Word {
    type Output = BraidResult;
    fn mul(self, rhs: &Braid) -> Self::Output {
        self.clone() * rhs.clone()
    }
}
impl std::ops::Mul<Braid> for &Braid {
    type Output = BraidResult;
    fn mul(self, rhs: Braid) -> Self::Output {
        self.clone() * rhs
    }
}
impl std::ops::Mul<&Braid> for Braid {
    type Output = BraidResult;
    fn mul(self, rhs: &Braid) -> Self::Output {
        self * rhs.clone()
    }
}
impl std::ops::Mul<&Braid> for &Braid {
    type Output = BraidResult;
    fn mul(self, rhs: &Braid) -> Self::Output {
        self.clone() * rhs.clone()
    }
}

impl std::ops::Mul<Braid> for LetterResult {
    type Output = BraidResult;
    fn mul(self, rhs: Braid) -> Self::Output {
        match *self {
            Ok(lhs) => lhs * rhs,
            Err(lhs) => BraidResult::from(Err(BraidValidationError::from(
                WordValidationError::from(lhs),
            ))),
        }
    }
}
impl std::ops::Mul<&Braid> for LetterResult {
    type Output = BraidResult;
    fn mul(self, rhs: &Braid) -> Self::Output {
        match *self {
            Ok(lhs) => lhs * rhs,
            Err(lhs) => BraidResult::from(Err(BraidValidationError::from(
                WordValidationError::from(lhs),
            ))),
        }
    }
}
impl std::ops::Mul<LetterResult> for Braid {
    type Output = BraidResult;
    fn mul(self, rhs: LetterResult) -> Self::Output {
        match *rhs {
            Ok(rhs) => self * rhs,
            Err(rhs) => BraidResult::from(Err(BraidValidationError::from(
                WordValidationError::from(rhs),
            ))),
        }
    }
}
impl std::ops::Mul<LetterResult> for &Braid {
    type Output = BraidResult;
    fn mul(self, rhs: LetterResult) -> Self::Output {
        match *rhs {
            Ok(rhs) => self * rhs,
            Err(rhs) => BraidResult::from(Err(BraidValidationError::from(
                WordValidationError::from(rhs),
            ))),
        }
    }
}
impl std::ops::Mul<Braid> for WordResult {
    type Output = BraidResult;
    fn mul(self, rhs: Braid) -> Self::Output {
        match &*self {
            Ok(lhs) => lhs * rhs,
            Err(lhs) => BraidResult::from(Err(BraidValidationError::from(*lhs))),
        }
    }
}
impl std::ops::Mul<&Braid> for WordResult {
    type Output = BraidResult;
    fn mul(self, rhs: &Braid) -> Self::Output {
        match &*self {
            Ok(lhs) => lhs * rhs,
            Err(lhs) => BraidResult::from(Err(BraidValidationError::from(*lhs))),
        }
    }
}
impl std::ops::Mul<WordResult> for Braid {
    type Output = BraidResult;
    fn mul(self, rhs: WordResult) -> Self::Output {
        match &*rhs {
            Ok(rhs) => self * rhs,
            Err(rhs) => BraidResult::from(Err(BraidValidationError::from(*rhs))),
        }
    }
}
impl std::ops::Mul<WordResult> for &Braid {
    type Output = BraidResult;
    fn mul(self, rhs: WordResult) -> Self::Output {
        match &*rhs {
            Ok(rhs) => self * rhs,
            Err(rhs) => BraidResult::from(Err(BraidValidationError::from(*rhs))),
        }
    }
}
impl std::ops::Mul<Braid> for BraidResult {
    type Output = BraidResult;
    fn mul(self, rhs: Braid) -> Self::Output {
        match &*self {
            Ok(lhs) => lhs * rhs,
            Err(lhs) => BraidResult::from(Err(*lhs)),
        }
    }
}
impl std::ops::Mul<&Braid> for BraidResult {
    type Output = BraidResult;
    fn mul(self, rhs: &Braid) -> Self::Output {
        match &*self {
            Ok(lhs) => lhs * rhs,
            Err(lhs) => BraidResult::from(Err(*lhs)),
        }
    }
}
impl std::ops::Mul<BraidResult> for Braid {
    type Output = BraidResult;
    fn mul(self, rhs: BraidResult) -> Self::Output {
        match &*rhs {
            Ok(rhs) => self * rhs,
            Err(rhs) => BraidResult::from(Err(*rhs)),
        }
    }
}
impl std::ops::Mul<BraidResult> for &Braid {
    type Output = BraidResult;
    fn mul(self, rhs: BraidResult) -> Self::Output {
        match &*rhs {
            Ok(rhs) => self * rhs,
            Err(rhs) => BraidResult::from(Err(*rhs)),
        }
    }
}
impl std::ops::Mul<Word> for BraidResult {
    type Output = BraidResult;
    fn mul(self, rhs: Word) -> Self::Output {
        match &*self {
            Ok(lhs) => lhs * rhs,
            Err(lhs) => BraidResult::from(Err(*lhs)),
        }
    }
}
impl std::ops::Mul<&Word> for BraidResult {
    type Output = BraidResult;
    fn mul(self, rhs: &Word) -> Self::Output {
        match &*self {
            Ok(lhs) => lhs * rhs,
            Err(lhs) => BraidResult::from(Err(*lhs)),
        }
    }
}
impl std::ops::Mul<BraidResult> for Word {
    type Output = BraidResult;
    fn mul(self, rhs: BraidResult) -> Self::Output {
        match &*rhs {
            Ok(rhs) => self * rhs,
            Err(rhs) => BraidResult::from(Err(*rhs)),
        }
    }
}
impl std::ops::Mul<BraidResult> for &Word {
    type Output = BraidResult;
    fn mul(self, rhs: BraidResult) -> Self::Output {
        match &*rhs {
            Ok(rhs) => self * rhs,
            Err(rhs) => BraidResult::from(Err(*rhs)),
        }
    }
}
impl std::ops::Mul<Letter> for BraidResult {
    type Output = BraidResult;
    fn mul(self, rhs: Letter) -> Self::Output {
        match &*self {
            Ok(lhs) => lhs * rhs,
            Err(lhs) => BraidResult::from(Err(*lhs)),
        }
    }
}
impl std::ops::Mul<BraidResult> for Letter {
    type Output = BraidResult;
    fn mul(self, rhs: BraidResult) -> Self::Output {
        match &*rhs {
            Ok(rhs) => self * rhs,
            Err(rhs) => BraidResult::from(Err(*rhs)),
        }
    }
}
impl std::ops::Mul<LetterResult> for BraidResult {
    type Output = BraidResult;
    fn mul(self, rhs: LetterResult) -> Self::Output {
        match (&*self, *rhs) {
            (Ok(lhs), Ok(rhs)) => lhs * rhs,
            (Err(lhs), _) => BraidResult::from(Err(*lhs)),
            (_, Err(rhs)) => BraidResult::from(Err(BraidValidationError::from(
                WordValidationError::from(rhs),
            ))),
        }
    }
}
impl std::ops::Mul<BraidResult> for LetterResult {
    type Output = BraidResult;
    fn mul(self, rhs: BraidResult) -> Self::Output {
        match (*self, &*rhs) {
            (Ok(lhs), Ok(rhs)) => lhs * rhs,
            (Err(lhs), _) => BraidResult::from(Err(BraidValidationError::from(
                WordValidationError::from(lhs),
            ))),
            (_, Err(rhs)) => BraidResult::from(Err(*rhs)),
        }
    }
}
impl std::ops::Mul<WordResult> for BraidResult {
    type Output = BraidResult;
    fn mul(self, rhs: WordResult) -> Self::Output {
        match (&*self, &*rhs) {
            (Ok(lhs), Ok(rhs)) => lhs * rhs,
            (Err(lhs), _) => BraidResult::from(Err(*lhs)),
            (_, Err(rhs)) => BraidResult::from(Err(BraidValidationError::from(*rhs))),
        }
    }
}
impl std::ops::Mul<BraidResult> for WordResult {
    type Output = BraidResult;
    fn mul(self, rhs: BraidResult) -> Self::Output {
        match (&*self, &*rhs) {
            (Ok(lhs), Ok(rhs)) => lhs * rhs,
            (Err(lhs), _) => BraidResult::from(Err(BraidValidationError::from(*lhs))),
            (_, Err(rhs)) => BraidResult::from(Err(*rhs)),
        }
    }
}
impl std::ops::Mul<BraidResult> for BraidResult {
    type Output = BraidResult;
    fn mul(self, rhs: BraidResult) -> Self::Output {
        match (&*self, &*rhs) {
            (Ok(lhs), Ok(rhs)) => lhs * rhs,
            (Err(lhs), _) => BraidResult::from(Err(*lhs)),
            (_, Err(rhs)) => BraidResult::from(Err(*rhs)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        Braid, BraidIndex, BraidResult, BraidValidationError, Letter, Sign, Word, braid, letter,
        word,
    };
    use googletest::matchers::{eq, err};
    use googletest::{assert_that, expect_that, gtest};

    #[gtest]
    fn valid_multiplication_with_letter_succeeds_and_computes_as_expected() {
        let letters = vec![
            Letter::try_new(1, Some(3), Sign::Positive).unwrap(),
            Letter::try_new(2, None::<u16>, Sign::Negative).unwrap(),
            Letter::try_new(2, Some(8), Sign::Positive).unwrap(),
        ];
        let braid = Braid::try_from_letters(None::<u16>, &letters).clone_unwrap();
        let other_letter = Letter::try_new(3, Some(7), Sign::Negative).unwrap();

        expect_that!(
            braid.clone() * other_letter,
            eq(&Braid::try_from_letters(
                None::<u16>,
                &[letters.clone(), vec![other_letter]].concat()
            ))
        );
        expect_that!(
            other_letter * braid,
            eq(&Braid::try_from_letters(
                None::<u16>,
                &[vec![other_letter], letters].concat()
            ))
        );
    }

    #[gtest]
    fn valid_multiplication_with_word_succeeds_and_computes_as_expected() {
        let braid = Braid::try_from_letters(
            None::<u16>,
            &[
                Letter::try_new(1, Some(3), Sign::Positive).unwrap(),
                Letter::try_new(2, None::<u16>, Sign::Negative).unwrap(),
                Letter::try_new(2, Some(8), Sign::Positive).unwrap(),
            ],
        )
        .clone_unwrap();
        let word = Word::try_from_letters(&[
            Letter::try_new(3, None::<u16>, Sign::Negative).unwrap(),
            Letter::try_new(2, Some(7), Sign::Negative).unwrap(),
        ])
        .clone_unwrap();

        expect_that!(
            braid.clone() * word.clone(),
            eq(&Braid::try_new(
                8,
                (braid.word() * word.clone()).clone_unwrap()
            )),
        );
        expect_that!(
            word.clone() * braid.clone(),
            eq(&Braid::try_new(8, (word * braid.word()).clone_unwrap()))
        );
    }

    #[gtest]
    fn valid_multiplication_with_braid_succeeds_and_computes_as_expected() {
        let braid1 = Braid::try_from_letters(
            None::<u16>,
            &[
                Letter::try_new(1, Some(3), Sign::Positive).unwrap(),
                Letter::try_new(2, None::<u16>, Sign::Negative).unwrap(),
                Letter::try_new(2, Some(8), Sign::Positive).unwrap(),
            ],
        )
        .clone_unwrap();
        let braid2 = Braid::try_new(
            8,
            Word::try_from_letters(&[
                Letter::try_new(3, None::<u16>, Sign::Negative).unwrap(),
                Letter::try_new(2, Some(7), Sign::Negative).unwrap(),
            ])
            .clone_unwrap(),
        )
        .clone_unwrap();

        expect_that!(
            braid1.clone() * braid2.clone(),
            eq(&Braid::try_new(
                8,
                (braid1.word() * braid2.word()).clone_unwrap()
            )),
        );
        expect_that!(
            braid2.clone() * braid1.clone(),
            eq(&Braid::try_new(
                8,
                (braid2.word() * braid1.word()).clone_unwrap()
            ))
        );
    }

    #[gtest]
    fn invalid_multiplication_fails_as_expected() {
        let letter = Letter::try_new(7, None::<u16>, Sign::Positive).unwrap();
        let word = Word::try_new(vec![
            (2, Some(8), Sign::Negative),
            (1, None::<u16>, Sign::Positive),
        ])
        .clone_unwrap();
        let invalid_braids: Vec<(BraidResult, BraidValidationError, &'static str)> = vec![
            (
                Braid::try_from_data(
                    None::<u16>,
                    vec![
                        (1, None::<u16>, Sign::Positive),
                        (2, Some(5), Sign::Negative),
                        (3, None::<u16>, Sign::Negative),
                        (4, Some(5), Sign::Positive),
                    ],
                )
                .clone_unwrap()
                    * letter,
                BraidValidationError::IndexTooSmall {
                    index: BraidIndex::try_new(5).unwrap(),
                    minimal_required_index: BraidIndex::try_new(8).unwrap(),
                },
                "index too small, braid * letter",
            ),
            (
                letter
                    * Braid::try_from_data(
                        None::<u16>,
                        vec![
                            (1, None::<u16>, Sign::Positive),
                            (2, Some(5), Sign::Negative),
                            (3, None::<u16>, Sign::Negative),
                            (4, Some(5), Sign::Positive),
                        ],
                    )
                    .clone_unwrap(),
                BraidValidationError::IndexTooSmall {
                    index: BraidIndex::try_new(5).unwrap(),
                    minimal_required_index: BraidIndex::try_new(8).unwrap(),
                },
                "index too small, letter * braid",
            ),
            (
                Braid::try_from_data(
                    None::<u16>,
                    vec![
                        (1, None::<u16>, Sign::Positive),
                        (2, Some(5), Sign::Negative),
                        (3, None::<u16>, Sign::Negative),
                        (4, Some(5), Sign::Positive),
                    ],
                )
                .clone_unwrap()
                    * word.clone(),
                BraidValidationError::IndexTooSmall {
                    index: BraidIndex::try_new(5).unwrap(),
                    minimal_required_index: BraidIndex::try_new(8).unwrap(),
                },
                "index too small, braid * word",
            ),
            (
                word.clone()
                    * Braid::try_from_data(
                        None::<u16>,
                        vec![
                            (1, None::<u16>, Sign::Positive),
                            (2, Some(5), Sign::Negative),
                            (3, None::<u16>, Sign::Negative),
                            (4, Some(5), Sign::Positive),
                        ],
                    )
                    .clone_unwrap(),
                BraidValidationError::IndexTooSmall {
                    index: BraidIndex::try_new(5).unwrap(),
                    minimal_required_index: BraidIndex::try_new(8).unwrap(),
                },
                "index too small, word * braid",
            ),
            (
                Braid::try_from_data(Some(10), word.clone()).clone_unwrap()
                    * Braid::try_from_data(Some(11), word.clone()).clone_unwrap(),
                BraidValidationError::UnequalIndices {
                    left: BraidIndex::try_new(10).unwrap(),
                    right: BraidIndex::try_new(11).unwrap(),
                },
                "unequal indices",
            ),
            (
                Braid::try_from_data(
                    Some(10),
                    vec![(1, None::<u16>, Sign::Positive); u16::MAX as usize],
                )
                .clone_unwrap()
                    * letter,
                BraidValidationError::from(
                    Word::try_new(vec![
                        (1, None::<u16>, Sign::Positive);
                        u16::MAX as usize + 1
                    ])
                    .clone_unwrap_err(),
                ),
                "word failed validation, braid * letter",
            ),
            (
                letter
                    * Braid::try_from_data(
                        Some(10),
                        vec![(1, None::<u16>, Sign::Positive); u16::MAX as usize],
                    )
                    .clone_unwrap(),
                BraidValidationError::from(
                    Word::try_new(vec![
                        (1, None::<u16>, Sign::Positive);
                        u16::MAX as usize + 1
                    ])
                    .clone_unwrap_err(),
                ),
                "word failed validation, letter * braid",
            ),
            (
                Braid::try_from_data(
                    Some(10),
                    vec![(1, None::<u16>, Sign::Positive); u16::MAX as usize],
                )
                .clone_unwrap()
                    * word.clone(),
                BraidValidationError::from(
                    Word::try_new(vec![
                        (1, None::<u16>, Sign::Positive);
                        u16::MAX as usize + 12
                    ])
                    .clone_unwrap_err(),
                ),
                "word failed validation, braid * word",
            ),
            (
                word * Braid::try_from_data(
                    Some(10),
                    vec![(1, None::<u16>, Sign::Positive); u16::MAX as usize],
                )
                .clone_unwrap(),
                BraidValidationError::from(
                    Word::try_new(vec![
                        (1, None::<u16>, Sign::Positive);
                        u16::MAX as usize + 12
                    ])
                    .clone_unwrap_err(),
                ),
                "word failed validation, word * braid",
            ),
            (
                Braid::try_from_data(
                    None::<u16>,
                    vec![(1, None::<u16>, Sign::Positive); u16::MAX as usize],
                )
                .clone_unwrap()
                    * Braid::try_from_data(None::<u16>, vec![(1, None::<u16>, Sign::Positive)])
                        .clone_unwrap(),
                BraidValidationError::from(
                    Word::try_new(vec![
                        (1, None::<u16>, Sign::Positive);
                        u16::MAX as usize + 1
                    ])
                    .clone_unwrap_err(),
                ),
                "word failed validation, long_braid * short_braid",
            ),
            (
                Braid::try_from_data(None::<u16>, vec![(1, None::<u16>, Sign::Positive)])
                    .clone_unwrap()
                    * Braid::try_from_data(
                        None::<u16>,
                        vec![(1, None::<u16>, Sign::Positive); u16::MAX as usize],
                    )
                    .clone_unwrap(),
                BraidValidationError::from(
                    Word::try_new(vec![
                        (1, None::<u16>, Sign::Positive);
                        u16::MAX as usize + 1
                    ])
                    .clone_unwrap_err(),
                ),
                "word failed validation, short_braid * long_braid",
            ),
        ];

        for (invalid_braid, error, label) in invalid_braids {
            expect_that!(*invalid_braid, err(eq(&error)), "{label}")
        }
    }

    #[gtest]
    fn can_multiply_borrowed_braid_with_letter() {
        let letter = letter![1; +].unwrap();
        let braid = braid![(); [2; -1], [1 => 3; 3]].clone_unwrap();

        expect_that!(
            &braid * letter,
            eq(&braid![(); [2; -1], [1 => 3; 3], [1; 1]])
        );
        expect_that!(
            letter * &braid,
            eq(&braid![(); [1; 1], [2; -1], [1 => 3; 3]])
        );
    }

    #[gtest]
    fn can_multiply_borrowed_braid_with_word() {
        let word1 = word![[1; 1], [2; -3]].clone_unwrap();
        let word2 = word![[1 => 3; 2]].clone_unwrap();
        let braid = braid![(); [2; -1], [1 => 3; 3]].clone_unwrap();

        expect_that!(
            &braid * word1,
            eq(&braid![(); [2; -1], [1 => 3; 3], [1; 1], [2; -3]])
        );
        expect_that!(
            word2 * &braid,
            eq(&braid![(); [1 => 3; 2], [2; -1], [1 => 3; 3]])
        );
    }

    #[gtest]
    fn can_multiply_braid_with_borrowed_word() {
        let braid1 = braid![(); [1; 1], [2; -3]].clone_unwrap();
        let braid2 = braid![(); [1 => 3; 2]].clone_unwrap();
        let word = word![[2; -1], [1 => 3; 3]].clone_unwrap();

        expect_that!(
            &word * braid1,
            eq(&braid![(); [2; -1], [1 => 3; 3], [1; 1], [2; -3]])
        );
        expect_that!(
            braid2 * &word,
            eq(&braid![(); [1 => 3; 2], [2; -1], [1 => 3; 3]])
        );
    }

    #[gtest]
    fn can_multiply_borrowed_braid_with_borrowed_word() {
        let braid = braid![(); [1; 1], [2; -3]].clone_unwrap();
        let word = word![[2; -1], [1 => 3; 3]].clone_unwrap();

        expect_that!(
            &word * &braid,
            eq(&braid![(); [2; -1], [1 => 3; 3], [1; 1], [2; -3]])
        );
        expect_that!(
            &braid * &word,
            eq(&braid![(); [1; 1], [2; -3], [2; -1], [1 => 3; 3]])
        );
    }

    #[gtest]
    fn can_multiply_braid_with_borrowed_braid() {
        let braid1 = braid![(); [1; 1], [2; -3]].clone_unwrap();
        let braid2 = braid![(); [1 => 3; 2]].clone_unwrap();
        let braid = braid![(); [2; -1], [1 => 3; 3]].clone_unwrap();

        expect_that!(
            &braid * braid1,
            eq(&braid![(); [2; -1], [1 => 3; 3], [1; 1], [2; -3]])
        );
        expect_that!(
            braid2 * &braid,
            eq(&braid![(); [1 => 3; 2], [2; -1], [1 => 3; 3]])
        );
    }

    #[test]
    fn can_multiply_borrowed_braid_with_borrowed_braid() {
        let braid1 = braid![(); [1; 1], [2; -3]].clone_unwrap();
        let braid2 = braid![(); [1 => 3; 2]].clone_unwrap();

        assert_that!(
            &braid1 * &braid2,
            eq(&braid![(); [1; 1], [2; -3], [1 => 3; 2]])
        );
    }

    #[gtest]
    fn can_multiply_braid_with_letter_result() {
        let letter_result = letter![1; +];
        let braid = braid![(); [1 => 3; -3], [2; 7]].clone_unwrap();

        let letter_times_braid = braid![(); [1; 1], [1 => 3; -3], [2; 7]];
        let braid_times_letter = braid![(); [1 => 3; -3], [2; 7], [1; 1]];

        let tests = [
            (
                letter_result * braid.clone(),
                &letter_times_braid,
                "R<L> * B",
            ),
            (letter_result * &braid, &letter_times_braid, "R<L> * &B"),
            (
                braid.clone() * letter_result,
                &braid_times_letter,
                "B * R<L>",
            ),
            (&braid * letter_result, &braid_times_letter, "&B * R<L>"),
        ];

        for (actual, expected, label) in tests {
            expect_that!(actual, eq(expected), "{label}");
        }
    }

    #[gtest]
    fn can_multiply_braid_with_word_result() {
        let word_result = word![[1; 3], [2; -7]];
        let braid = braid![(); [1 => 3; 2], [1; 1]].clone_unwrap();

        let word_times_braid = braid![(); [1; 3], [2; -7], [1 => 3; 2], [1; 1]];
        let braid_times_word = braid![(); [1 => 3; 2], [1; 1], [1; 3], [2; -7]];

        let tests = [
            (
                word_result.clone() * braid.clone(),
                &word_times_braid,
                "R<W> * B",
            ),
            (word_result.clone() * &braid, &word_times_braid, "R<W> * &B"),
            (
                braid.clone() * word_result.clone(),
                &braid_times_word,
                "B * R<W>",
            ),
            (&braid * word_result, &braid_times_word, "&B * R<W>"),
        ];

        for (actual, expected, label) in tests {
            expect_that!(actual, eq(expected), "{label}");
        }
    }

    #[gtest]
    fn can_multiply_letter_with_braid_result() {
        let letter = letter![1; +].unwrap();
        let braid_result = braid![(); [1 => 3; -3], [2; 7]];

        let letter_times_braid = braid![(); [1; 1], [1 => 3; -3], [2; 7]];
        let braid_times_letter = braid![(); [1 => 3; -3], [2; 7], [1; 1]];

        let tests = [
            (
                letter * braid_result.clone(),
                &letter_times_braid,
                "L * R<B>",
            ),
            (
                braid_result.clone() * letter,
                &braid_times_letter,
                "R<B> * L",
            ),
        ];

        for (actual, expected, label) in tests {
            expect_that!(actual, eq(expected), "{label}");
        }
    }

    #[gtest]
    fn can_multiply_word_with_braid_result() {
        let word = word![[1; 3], [2; -7]].clone_unwrap();
        let braid_result = braid![(); [1 => 3; 2], [1; 1]];

        let word_times_braid = braid![(); [1; 3], [2; -7], [1 => 3; 2], [1; 1]];
        let braid_times_word = braid![(); [1 => 3; 2], [1; 1], [1; 3], [2; -7]];

        let tests = [
            (
                word.clone() * braid_result.clone(),
                &word_times_braid,
                "W * R<B>",
            ),
            (&word * braid_result.clone(), &word_times_braid, "&W * R<B>"),
            (
                braid_result.clone() * word.clone(),
                &braid_times_word,
                "R<B> * W",
            ),
            (braid_result * &word, &braid_times_word, "R<B> * &W"),
        ];

        for (actual, expected, label) in tests {
            expect_that!(actual, eq(expected), "{label}");
        }
    }

    #[gtest]
    fn can_multiply_braid_with_braid_result() {
        let braid = braid![(); [1; 3], [2; -7]].clone_unwrap();
        let braid_result = braid![(); [1 => 3; 2], [1; 1]];

        let braid_times_braid_result = braid![(); [1; 3], [2; -7], [1 => 3; 2], [1; 1]];
        let braid_result_times_braid = braid![(); [1 => 3; 2], [1; 1], [1; 3], [2; -7]];
        let tests = [
            (
                braid.clone() * braid_result.clone(),
                &braid_times_braid_result,
                "B * R<B>",
            ),
            (
                &braid * braid_result.clone(),
                &braid_times_braid_result,
                "&B * R<B>",
            ),
            (
                braid_result.clone() * braid.clone(),
                &braid_result_times_braid,
                "R<B> * B",
            ),
            (
                braid_result * &braid,
                &braid_result_times_braid,
                "R<B> * &B",
            ),
        ];

        for (actual, expected, label) in tests {
            expect_that!(actual, eq(expected), "{label}");
        }
    }

    #[gtest]
    fn can_multiply_letter_result_with_braid_result() {
        let letter_result = letter![1; +];
        let braid_result = braid![(); [1 => 3; -3], [2; 7]];

        let letter_times_braid = braid![(); [1; 1], [1 => 3; -3], [2; 7]];
        let braid_times_letter = braid![(); [1 => 3; -3], [2; 7], [1; 1]];

        let tests = [
            (
                letter_result * braid_result.clone(),
                &letter_times_braid,
                "R<L> * R<B>",
            ),
            (
                braid_result.clone() * letter_result,
                &braid_times_letter,
                "R<B> * R<L>",
            ),
        ];

        for (actual, expected, label) in tests {
            expect_that!(actual, eq(expected), "{label}");
        }
    }

    #[gtest]
    fn can_multiply_word_result_with_braid_result() {
        let word_result = word![[1; 3], [2; -7]];
        let braid_result = braid![(); [1 => 3; 2], [1; 1]];

        let word_times_braid = braid![(); [1; 3], [2; -7], [1 => 3; 2], [1; 1]];
        let braid_times_word = braid![(); [1 => 3; 2], [1; 1], [1; 3], [2; -7]];

        let tests = [
            (
                word_result.clone() * braid_result.clone(),
                &word_times_braid,
                "R<W> * R<B>",
            ),
            (
                braid_result.clone() * word_result.clone(),
                &braid_times_word,
                "R<B> * R<W>",
            ),
        ];

        for (actual, expected, label) in tests {
            expect_that!(actual, eq(expected), "{label}");
        }
    }

    #[test]
    fn can_multiply_braid_result_with_braid_result() {
        let braid_result1 = braid![(); [1; 3], [2; -7]];
        let braid_result2 = braid![(); [1 => 3; 2], [1; 1]];

        assert_that!(
            braid_result1 * braid_result2,
            eq(&braid![(); [1; 3], [2; -7], [1 => 3; 2], [1; 1]])
        );
    }
}
