use crate::{Braid, BraidValidationError, Letter, Word};

impl std::ops::Mul<Letter> for Braid {
    type Output = Result<Self, BraidValidationError>;
    fn mul(self, rhs: Letter) -> Self::Output {
        if let required_index = rhs.minimal_required_braid_index()
            && self.braid_index() < required_index
        {
            Err(BraidValidationError::IndexTooSmall {
                index: self.braid_index(),
                minimal_required_index: required_index,
            })
        } else {
            Self::new(Some(self.braid_index()), (self.word() * rhs)?)
        }
    }
}
impl std::ops::Mul<Braid> for Letter {
    type Output = Result<Braid, BraidValidationError>;

    fn mul(self, rhs: Braid) -> Self::Output {
        if let required_index = self.minimal_required_braid_index()
            && rhs.braid_index() < required_index
        {
            Err(BraidValidationError::IndexTooSmall {
                index: rhs.braid_index(),
                minimal_required_index: required_index,
            })
        } else {
            Braid::new(Some(rhs.braid_index()), (self * rhs.word())?)
        }
    }
}
impl std::ops::Mul<Word> for Braid {
    type Output = Result<Self, BraidValidationError>;
    fn mul(self, rhs: Word) -> Self::Output {
        if let required_index = rhs.minimal_required_braid_index()
            && self.braid_index() < required_index
        {
            Err(BraidValidationError::IndexTooSmall {
                index: self.braid_index(),
                minimal_required_index: required_index,
            })
        } else {
            Self::new(Some(self.braid_index()), (self.word() * rhs)?)
        }
    }
}
impl std::ops::Mul<Braid> for Word {
    type Output = Result<Braid, BraidValidationError>;

    fn mul(self, rhs: Braid) -> Self::Output {
        if let required_index = self.minimal_required_braid_index()
            && rhs.braid_index() < required_index
        {
            Err(BraidValidationError::IndexTooSmall {
                index: rhs.braid_index(),
                minimal_required_index: required_index,
            })
        } else {
            Braid::new(Some(rhs.braid_index()), (self * rhs.word())?)
        }
    }
}
impl std::ops::Mul for Braid {
    type Output = Result<Self, BraidValidationError>;
    fn mul(self, rhs: Self) -> Self::Output {
        if self.braid_index() != rhs.braid_index() {
            Err(BraidValidationError::UnequalIndices {
                left: self.braid_index(),
                right: rhs.braid_index(),
            })
        } else {
            Self::new(Some(self.braid_index()), (self.word() * rhs.word())?)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Braid, BraidIndex, BraidValidationError, Letter, Sign, Word};
    use googletest::matchers::{eq, err};
    use googletest::{expect_that, gtest};

    #[gtest]
    fn valid_multiplication_with_letter_succeeds_and_computes_as_expected() {
        let letters = vec![
            Letter::new(1, Some(3), Sign::Positive).unwrap(),
            Letter::new(2, None::<u16>, Sign::Negative).unwrap(),
            Letter::new(2, Some(8), Sign::Positive).unwrap(),
        ];
        let braid = Braid::try_from(&letters[..]).unwrap();
        let other_letter = Letter::new(3, Some(7), Sign::Negative).unwrap();

        expect_that!(
            braid.clone() * other_letter,
            eq(&Braid::try_from(
                [letters.clone(), vec![other_letter]].concat()
            ))
        );
        expect_that!(
            other_letter * braid,
            eq(&Braid::try_from([vec![other_letter], letters].concat()))
        );
    }

    #[gtest]
    fn valid_multiplication_with_word_succeeds_and_computes_as_expected() {
        let braid = Braid::try_from(vec![
            Letter::new(1, Some(3), Sign::Positive).unwrap(),
            Letter::new(2, None::<u16>, Sign::Negative).unwrap(),
            Letter::new(2, Some(8), Sign::Positive).unwrap(),
        ])
        .unwrap();
        let word = Word::try_from(vec![
            Letter::new(3, None::<u16>, Sign::Negative).unwrap(),
            Letter::new(2, Some(7), Sign::Negative).unwrap(),
        ])
        .unwrap();

        expect_that!(
            braid.clone() * word.clone(),
            eq(&Braid::new(
                None::<u16>,
                (braid.word() * word.clone()).unwrap()
            )),
        );
        expect_that!(
            word.clone() * braid.clone(),
            eq(&Braid::new(None::<u16>, (word * braid.word()).unwrap()))
        );
    }

    #[gtest]
    fn valid_multiplication_with_braid_succeeds_and_computes_as_expected() {
        let braid1 = Braid::try_from(vec![
            Letter::new(1, Some(3), Sign::Positive).unwrap(),
            Letter::new(2, None::<u16>, Sign::Negative).unwrap(),
            Letter::new(2, Some(8), Sign::Positive).unwrap(),
        ])
        .unwrap();
        let braid2 = Braid::new(
            Some(8),
            Word::try_from(vec![
                Letter::new(3, None::<u16>, Sign::Negative).unwrap(),
                Letter::new(2, Some(7), Sign::Negative).unwrap(),
            ])
            .unwrap(),
        )
        .unwrap();

        expect_that!(
            braid1.clone() * braid2.clone(),
            eq(&Braid::new(
                None::<u16>,
                (braid1.word() * braid2.word()).unwrap()
            )),
        );
        expect_that!(
            braid2.clone() * braid1.clone(),
            eq(&Braid::new(
                None::<u16>,
                (braid2.word() * braid1.word()).unwrap()
            ))
        );
    }

    #[gtest]
    fn invalid_multiplication_fails_as_expected() {
        let letter = Letter::new(7, None::<u16>, Sign::Positive).unwrap();
        let word = Word::new(vec![
            (2, Some(8), Sign::Negative),
            (1, None::<u16>, Sign::Positive),
        ])
        .unwrap();
        let invalid_braids: Vec<(
            Result<Braid, BraidValidationError>,
            BraidValidationError,
            &'static str,
        )> = vec![
            (
                Braid::from_data(
                    None::<u16>,
                    vec![
                        (1, None::<u16>, Sign::Positive),
                        (2, Some(5), Sign::Negative),
                        (3, None::<u16>, Sign::Negative),
                        (4, Some(5), Sign::Positive),
                    ],
                )
                .unwrap()
                    * letter,
                BraidValidationError::IndexTooSmall {
                    index: BraidIndex::new(5).unwrap(),
                    minimal_required_index: BraidIndex::new(8).unwrap(),
                },
                "index too small, braid * letter",
            ),
            (
                letter
                    * Braid::from_data(
                        None::<u16>,
                        vec![
                            (1, None::<u16>, Sign::Positive),
                            (2, Some(5), Sign::Negative),
                            (3, None::<u16>, Sign::Negative),
                            (4, Some(5), Sign::Positive),
                        ],
                    )
                    .unwrap(),
                BraidValidationError::IndexTooSmall {
                    index: BraidIndex::new(5).unwrap(),
                    minimal_required_index: BraidIndex::new(8).unwrap(),
                },
                "index too small, letter * braid",
            ),
            (
                Braid::from_data(
                    None::<u16>,
                    vec![
                        (1, None::<u16>, Sign::Positive),
                        (2, Some(5), Sign::Negative),
                        (3, None::<u16>, Sign::Negative),
                        (4, Some(5), Sign::Positive),
                    ],
                )
                .unwrap()
                    * word.clone(),
                BraidValidationError::IndexTooSmall {
                    index: BraidIndex::new(5).unwrap(),
                    minimal_required_index: BraidIndex::new(8).unwrap(),
                },
                "index too small, braid * word",
            ),
            (
                word.clone()
                    * Braid::from_data(
                        None::<u16>,
                        vec![
                            (1, None::<u16>, Sign::Positive),
                            (2, Some(5), Sign::Negative),
                            (3, None::<u16>, Sign::Negative),
                            (4, Some(5), Sign::Positive),
                        ],
                    )
                    .unwrap(),
                BraidValidationError::IndexTooSmall {
                    index: BraidIndex::new(5).unwrap(),
                    minimal_required_index: BraidIndex::new(8).unwrap(),
                },
                "index too small, word * braid",
            ),
            (
                Braid::from_data(Some(10), word.clone()).unwrap()
                    * Braid::from_data(Some(11), word.clone()).unwrap(),
                BraidValidationError::UnequalIndices {
                    left: BraidIndex::new(10).unwrap(),
                    right: BraidIndex::new(11).unwrap(),
                },
                "unequal indices",
            ),
            (
                Braid::from_data(
                    Some(10),
                    vec![(1, None::<u16>, Sign::Positive); u16::MAX as usize],
                )
                .unwrap()
                    * letter,
                BraidValidationError::from(
                    Word::new(vec![
                        (1, None::<u16>, Sign::Positive);
                        u16::MAX as usize + 1
                    ])
                    .err()
                    .unwrap(),
                ),
                "word failed validation, braid * letter",
            ),
            (
                letter
                    * Braid::from_data(
                        Some(10),
                        vec![(1, None::<u16>, Sign::Positive); u16::MAX as usize],
                    )
                    .unwrap(),
                BraidValidationError::from(
                    Word::new(vec![
                        (1, None::<u16>, Sign::Positive);
                        u16::MAX as usize + 1
                    ])
                    .err()
                    .unwrap(),
                ),
                "word failed validation, letter * braid",
            ),
            (
                Braid::from_data(
                    Some(10),
                    vec![(1, None::<u16>, Sign::Positive); u16::MAX as usize],
                )
                .unwrap()
                    * word.clone(),
                BraidValidationError::from(
                    Word::new(vec![
                        (1, None::<u16>, Sign::Positive);
                        u16::MAX as usize + 12
                    ])
                    .err()
                    .unwrap(),
                ),
                "word failed validation, braid * word",
            ),
            (
                word * Braid::from_data(
                    Some(10),
                    vec![(1, None::<u16>, Sign::Positive); u16::MAX as usize],
                )
                .unwrap(),
                BraidValidationError::from(
                    Word::new(vec![
                        (1, None::<u16>, Sign::Positive);
                        u16::MAX as usize + 12
                    ])
                    .err()
                    .unwrap(),
                ),
                "word failed validation, word * braid",
            ),
            (
                Braid::from_data(
                    None::<u16>,
                    vec![(1, None::<u16>, Sign::Positive); u16::MAX as usize],
                )
                .unwrap()
                    * Braid::from_data(None::<u16>, vec![(1, None::<u16>, Sign::Positive)])
                        .unwrap(),
                BraidValidationError::from(
                    Word::new(vec![
                        (1, None::<u16>, Sign::Positive);
                        u16::MAX as usize + 1
                    ])
                    .err()
                    .unwrap(),
                ),
                "word failed validation, long_braid * short_braid",
            ),
            (
                Braid::from_data(None::<u16>, vec![(1, None::<u16>, Sign::Positive)]).unwrap()
                    * Braid::from_data(
                        None::<u16>,
                        vec![(1, None::<u16>, Sign::Positive); u16::MAX as usize],
                    )
                    .unwrap(),
                BraidValidationError::from(
                    Word::new(vec![
                        (1, None::<u16>, Sign::Positive);
                        u16::MAX as usize + 1
                    ])
                    .err()
                    .unwrap(),
                ),
                "word failed validation, short_braid * long_braid",
            ),
        ];

        for (invalid_braid, error, label) in invalid_braids {
            expect_that!(invalid_braid, err(eq(&error)), "{label}")
        }
    }
}
