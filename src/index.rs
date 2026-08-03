use crate::Strand;

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum IndexValidationError {
    #[error("Braid index cannot be zero")]
    Zero,
    #[error(transparent)]
    FromInt(#[from] std::num::TryFromIntError),
    #[error(transparent)]
    Infallible(#[from] std::convert::Infallible),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BraidIndex(u16);

impl BraidIndex {
    pub fn new<N>(index: N) -> Result<Self, IndexValidationError>
    where
        N: TryInto<u16>,
        IndexValidationError: From<<N as TryInto<u16>>::Error>,
    {
        Self::try_from(index.try_into()?)
    }
}

impl TryFrom<u16> for BraidIndex {
    type Error = IndexValidationError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(IndexValidationError::Zero)
        } else {
            Ok(Self(value))
        }
    }
}
impl From<Strand> for BraidIndex {
    fn from(value: Strand) -> Self {
        Self(value.into())
    }
}
impl From<BraidIndex> for u16 {
    fn from(value: BraidIndex) -> Self {
        value.0
    }
}

impl std::ops::Deref for BraidIndex {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> AsRef<T> for BraidIndex
where
    T: ?Sized,
    <BraidIndex as std::ops::Deref>::Target: AsRef<T>,
{
    fn as_ref(&self) -> &T {
        <BraidIndex as std::ops::Deref>::deref(self).as_ref()
    }
}
