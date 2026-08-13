use crate::{
    Braid, BraidResult, BraidValidationError, Letter, LetterResult, Word, WordResult,
    WordValidationError,
};

impl std::ops::Mul<Letter> for Braid {
    type Output = BraidResult;
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: Letter) -> Self::Output {
        tracing::debug!("Attempting to multiply {:?} x {:?}", self, rhs);
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
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: Braid) -> Self::Output {
        tracing::debug!("Attempting to multiply {:?} x {:?}", self, rhs);
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
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: Word) -> Self::Output {
        tracing::debug!("Attempting to multiply {:?} x {:?}", self, rhs);
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
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: Braid) -> Self::Output {
        tracing::debug!("Attempting to multiply {:?} x {:?}", self, rhs);
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
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: Self) -> Self::Output {
        tracing::debug!("Attempting to multiply {:?} x {:?}", self, rhs);
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
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: Letter) -> Self::Output {
        self.clone() * rhs
    }
}
impl std::ops::Mul<&Braid> for Letter {
    type Output = BraidResult;
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: &Braid) -> Self::Output {
        self * rhs.clone()
    }
}
impl std::ops::Mul<Word> for &Braid {
    type Output = BraidResult;
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: Word) -> Self::Output {
        self.clone() * rhs
    }
}
impl std::ops::Mul<&Braid> for Word {
    type Output = BraidResult;
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: &Braid) -> Self::Output {
        self * rhs.clone()
    }
}
impl std::ops::Mul<&Word> for Braid {
    type Output = BraidResult;
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: &Word) -> Self::Output {
        self * rhs.clone()
    }
}
impl std::ops::Mul<Braid> for &Word {
    type Output = BraidResult;
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: Braid) -> Self::Output {
        self.clone() * rhs
    }
}
impl std::ops::Mul<&Word> for &Braid {
    type Output = BraidResult;
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: &Word) -> Self::Output {
        self.clone() * rhs.clone()
    }
}
impl std::ops::Mul<&Braid> for &Word {
    type Output = BraidResult;
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: &Braid) -> Self::Output {
        self.clone() * rhs.clone()
    }
}
impl std::ops::Mul<Braid> for &Braid {
    type Output = BraidResult;
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: Braid) -> Self::Output {
        self.clone() * rhs
    }
}
impl std::ops::Mul<&Braid> for Braid {
    type Output = BraidResult;
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: &Braid) -> Self::Output {
        self * rhs.clone()
    }
}
impl std::ops::Mul<&Braid> for &Braid {
    type Output = BraidResult;
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: &Braid) -> Self::Output {
        self.clone() * rhs.clone()
    }
}

impl std::ops::Mul<Braid> for LetterResult {
    type Output = BraidResult;
    #[tracing::instrument(level = "info")]
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
    #[tracing::instrument(level = "info")]
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
    #[tracing::instrument(level = "info")]
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
    #[tracing::instrument(level = "info")]
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
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: Braid) -> Self::Output {
        match &*self {
            Ok(lhs) => lhs * rhs,
            Err(lhs) => BraidResult::from(Err(BraidValidationError::from(*lhs))),
        }
    }
}
impl std::ops::Mul<&Braid> for WordResult {
    type Output = BraidResult;
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: &Braid) -> Self::Output {
        match &*self {
            Ok(lhs) => lhs * rhs,
            Err(lhs) => BraidResult::from(Err(BraidValidationError::from(*lhs))),
        }
    }
}
impl std::ops::Mul<WordResult> for Braid {
    type Output = BraidResult;
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: WordResult) -> Self::Output {
        match &*rhs {
            Ok(rhs) => self * rhs,
            Err(rhs) => BraidResult::from(Err(BraidValidationError::from(*rhs))),
        }
    }
}
impl std::ops::Mul<WordResult> for &Braid {
    type Output = BraidResult;
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: WordResult) -> Self::Output {
        match &*rhs {
            Ok(rhs) => self * rhs,
            Err(rhs) => BraidResult::from(Err(BraidValidationError::from(*rhs))),
        }
    }
}
impl std::ops::Mul<Braid> for BraidResult {
    type Output = BraidResult;
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: Braid) -> Self::Output {
        match &*self {
            Ok(lhs) => lhs * rhs,
            Err(lhs) => BraidResult::from(Err(*lhs)),
        }
    }
}
impl std::ops::Mul<&Braid> for BraidResult {
    type Output = BraidResult;
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: &Braid) -> Self::Output {
        match &*self {
            Ok(lhs) => lhs * rhs,
            Err(lhs) => BraidResult::from(Err(*lhs)),
        }
    }
}
impl std::ops::Mul<BraidResult> for Braid {
    type Output = BraidResult;
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: BraidResult) -> Self::Output {
        match &*rhs {
            Ok(rhs) => self * rhs,
            Err(rhs) => BraidResult::from(Err(*rhs)),
        }
    }
}
impl std::ops::Mul<BraidResult> for &Braid {
    type Output = BraidResult;
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: BraidResult) -> Self::Output {
        match &*rhs {
            Ok(rhs) => self * rhs,
            Err(rhs) => BraidResult::from(Err(*rhs)),
        }
    }
}
impl std::ops::Mul<Word> for BraidResult {
    type Output = BraidResult;
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: Word) -> Self::Output {
        match &*self {
            Ok(lhs) => lhs * rhs,
            Err(lhs) => BraidResult::from(Err(*lhs)),
        }
    }
}
impl std::ops::Mul<&Word> for BraidResult {
    type Output = BraidResult;
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: &Word) -> Self::Output {
        match &*self {
            Ok(lhs) => lhs * rhs,
            Err(lhs) => BraidResult::from(Err(*lhs)),
        }
    }
}
impl std::ops::Mul<BraidResult> for Word {
    type Output = BraidResult;
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: BraidResult) -> Self::Output {
        match &*rhs {
            Ok(rhs) => self * rhs,
            Err(rhs) => BraidResult::from(Err(*rhs)),
        }
    }
}
impl std::ops::Mul<BraidResult> for &Word {
    type Output = BraidResult;
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: BraidResult) -> Self::Output {
        match &*rhs {
            Ok(rhs) => self * rhs,
            Err(rhs) => BraidResult::from(Err(*rhs)),
        }
    }
}
impl std::ops::Mul<Letter> for BraidResult {
    type Output = BraidResult;
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: Letter) -> Self::Output {
        match &*self {
            Ok(lhs) => lhs * rhs,
            Err(lhs) => BraidResult::from(Err(*lhs)),
        }
    }
}
impl std::ops::Mul<BraidResult> for Letter {
    type Output = BraidResult;
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: BraidResult) -> Self::Output {
        match &*rhs {
            Ok(rhs) => self * rhs,
            Err(rhs) => BraidResult::from(Err(*rhs)),
        }
    }
}
impl std::ops::Mul<LetterResult> for BraidResult {
    type Output = BraidResult;
    #[tracing::instrument(level = "info")]
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
    #[tracing::instrument(level = "info")]
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
    #[tracing::instrument(level = "info")]
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
    #[tracing::instrument(level = "info")]
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
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: BraidResult) -> Self::Output {
        match (&*self, &*rhs) {
            (Ok(lhs), Ok(rhs)) => lhs * rhs,
            (Err(lhs), _) => BraidResult::from(Err(*lhs)),
            (_, Err(rhs)) => BraidResult::from(Err(*rhs)),
        }
    }
}

impl std::ops::Mul<Braid> for &WordResult {
    type Output = BraidResult;
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: Braid) -> Self::Output {
        (*self).clone() * rhs
    }
}
impl std::ops::Mul<&WordResult> for Braid {
    type Output = BraidResult;
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: &WordResult) -> Self::Output {
        self * (*rhs).clone()
    }
}
impl std::ops::Mul<&Braid> for &WordResult {
    type Output = BraidResult;
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: &Braid) -> Self::Output {
        (*self).clone() * rhs
    }
}
impl std::ops::Mul<&WordResult> for &Braid {
    type Output = BraidResult;
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: &WordResult) -> Self::Output {
        self * (*rhs).clone()
    }
}
impl std::ops::Mul<BraidResult> for &WordResult {
    type Output = BraidResult;
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: BraidResult) -> Self::Output {
        (*self).clone() * rhs
    }
}
impl std::ops::Mul<&WordResult> for BraidResult {
    type Output = BraidResult;
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: &WordResult) -> Self::Output {
        self * (*rhs).clone()
    }
}

impl std::ops::Mul<Letter> for &BraidResult {
    type Output = BraidResult;
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: Letter) -> Self::Output {
        (*self).clone() * rhs
    }
}
impl std::ops::Mul<&BraidResult> for Letter {
    type Output = BraidResult;
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: &BraidResult) -> Self::Output {
        self * (*rhs).clone()
    }
}
impl std::ops::Mul<LetterResult> for &BraidResult {
    type Output = BraidResult;
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: LetterResult) -> Self::Output {
        (*self).clone() * rhs
    }
}
impl std::ops::Mul<&BraidResult> for LetterResult {
    type Output = BraidResult;
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: &BraidResult) -> Self::Output {
        self * (*rhs).clone()
    }
}
impl std::ops::Mul<Word> for &BraidResult {
    type Output = BraidResult;
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: Word) -> Self::Output {
        (*self).clone() * rhs
    }
}
impl std::ops::Mul<&BraidResult> for Word {
    type Output = BraidResult;
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: &BraidResult) -> Self::Output {
        self * (*rhs).clone()
    }
}
impl std::ops::Mul<&Word> for &BraidResult {
    type Output = BraidResult;
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: &Word) -> Self::Output {
        (*self).clone() * rhs
    }
}
impl std::ops::Mul<&BraidResult> for &Word {
    type Output = BraidResult;
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: &BraidResult) -> Self::Output {
        self * (*rhs).clone()
    }
}
impl std::ops::Mul<WordResult> for &BraidResult {
    type Output = BraidResult;
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: WordResult) -> Self::Output {
        (*self).clone() * rhs
    }
}
impl std::ops::Mul<&BraidResult> for WordResult {
    type Output = BraidResult;
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: &BraidResult) -> Self::Output {
        self * (*rhs).clone()
    }
}
impl std::ops::Mul<&WordResult> for &BraidResult {
    type Output = BraidResult;
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: &WordResult) -> Self::Output {
        (*self).clone() * rhs
    }
}
impl std::ops::Mul<&BraidResult> for &WordResult {
    type Output = BraidResult;
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: &BraidResult) -> Self::Output {
        self * (*rhs).clone()
    }
}
impl std::ops::Mul<Braid> for &BraidResult {
    type Output = BraidResult;
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: Braid) -> Self::Output {
        (*self).clone() * rhs
    }
}
impl std::ops::Mul<&BraidResult> for Braid {
    type Output = BraidResult;
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: &BraidResult) -> Self::Output {
        self * (*rhs).clone()
    }
}
impl std::ops::Mul<&Braid> for &BraidResult {
    type Output = BraidResult;
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: &Braid) -> Self::Output {
        (*self).clone() * rhs
    }
}
impl std::ops::Mul<&BraidResult> for &Braid {
    type Output = BraidResult;
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: &BraidResult) -> Self::Output {
        self * (*rhs).clone()
    }
}
impl std::ops::Mul<BraidResult> for &BraidResult {
    type Output = BraidResult;
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: BraidResult) -> Self::Output {
        (*self).clone() * rhs
    }
}
impl std::ops::Mul<&BraidResult> for BraidResult {
    type Output = BraidResult;
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: &BraidResult) -> Self::Output {
        self * (*rhs).clone()
    }
}
impl std::ops::Mul for &BraidResult {
    type Output = BraidResult;
    #[tracing::instrument(level = "info")]
    fn mul(self, rhs: Self) -> Self::Output {
        (*self).clone() * (*rhs).clone()
    }
}
