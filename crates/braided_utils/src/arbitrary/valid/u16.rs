use braided::{BraidIndex, Strand};
use proptest::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Data {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    USize(usize),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    ISize(isize),
    Strand(Strand),
    BraidIndex(BraidIndex),
}

impl From<Data> for u16 {
    fn from(value: Data) -> Self {
        match value {
            Data::U8(val) => val.into(),
            Data::U16(val) => val,
            Data::U32(val) => val.try_into().unwrap(),
            Data::U64(val) => val.try_into().unwrap(),
            Data::U128(val) => val.try_into().unwrap(),
            Data::USize(val) => val.try_into().unwrap(),
            Data::I8(val) => val.try_into().unwrap(),
            Data::I16(val) => val.try_into().unwrap(),
            Data::I32(val) => val.try_into().unwrap(),
            Data::I64(val) => val.try_into().unwrap(),
            Data::I128(val) => val.try_into().unwrap(),
            Data::ISize(val) => val.try_into().unwrap(),
            Data::Strand(val) => val.into(),
            Data::BraidIndex(val) => val.into(),
        }
    }
}

impl std::ops::Add<u16> for Data {
    type Output = Data;
    fn add(self, rhs: u16) -> Self::Output {
        match self {
            Self::U8(val) => {
                if val as u16 > u16::MAX - rhs {
                    Self::U32(val as u32 + rhs as u32)
                } else if val as u16 + rhs > u8::MAX as u16 {
                    Self::U16(val as u16 + rhs)
                } else {
                    Self::U8(val + rhs as u8)
                }
            }
            Self::U16(val) => {
                if val > u16::MAX - rhs {
                    Self::U32(val as u32 + rhs as u32)
                } else {
                    Self::U16(val + rhs)
                }
            }
            Self::U32(val) => {
                if val > u32::MAX - (rhs as u32) {
                    Self::U64(val as u64 + rhs as u64)
                } else {
                    Self::U32(val + rhs as u32)
                }
            }
            Self::U64(val) => {
                if val > u64::MAX - (rhs as u64) {
                    Self::U128(val as u128 + rhs as u128)
                } else {
                    Self::U64(val + rhs as u64)
                }
            }
            Self::U128(val) => {
                if val > u128::MAX - (rhs as u128) {
                    panic!("Attempting to add {rhs} to {val} causes overflow.")
                } else {
                    Self::U128(val + rhs as u128)
                }
            }
            Self::USize(val) => {
                if val > usize::MAX - (rhs as usize) {
                    if usize::BITS == 32 {
                        Self::U64(val as u64 - rhs as u64)
                    } else {
                        Self::U128(val as u128 + rhs as u128)
                    }
                } else {
                    Self::USize(val + rhs as usize)
                }
            }
            Self::I8(val) => {
                if val as u16 > u16::MAX - rhs || val as u16 + rhs > i16::MAX as u16 {
                    Self::I32(val as i32 + rhs as i32)
                } else if val as u16 + rhs > i8::MAX as u16 {
                    Self::I16(val as i16 + rhs as i16)
                } else {
                    Self::I8(val + rhs as i8)
                }
            }
            Self::I16(val) => {
                if val as u16 > u16::MAX - rhs || val as u16 + rhs > i16::MAX as u16 {
                    Self::I32(val as i32 + rhs as i32)
                } else {
                    Self::I16(val + rhs as i16)
                }
            }
            Self::I32(val) => {
                if val > i32::MAX - rhs as i32 {
                    Self::I64(val as i64 + rhs as i64)
                } else {
                    Self::I32(val + rhs as i32)
                }
            }
            Self::I64(val) => {
                if val > i64::MAX - rhs as i64 {
                    Self::I128(val as i128 + rhs as i128)
                } else {
                    Self::I64(val + rhs as i64)
                }
            }
            Self::I128(val) => {
                if val > i128::MAX - rhs as i128 {
                    Self::U128(val as u128 + rhs as u128)
                } else {
                    Self::I128(val + rhs as i128)
                }
            }
            Self::ISize(val) => {
                if val > isize::MAX - (rhs as isize) {
                    if isize::BITS == 32 {
                        Self::I64(val as i64 - rhs as i64)
                    } else {
                        Self::I128(val as i128 + rhs as i128)
                    }
                } else {
                    Self::ISize(val + rhs as isize)
                }
            }
            Self::Strand(val) => {
                let idx: u16 = val.into();
                if idx > u16::MAX - rhs {
                    Self::U32(idx as u32 + rhs as u32)
                } else {
                    Self::Strand((val + rhs).unwrap())
                }
            }
            Self::BraidIndex(val) => {
                let idx: u16 = val.into();
                if idx > u16::MAX - rhs {
                    Self::U32(idx as u32 + rhs as u32)
                } else {
                    Self::BraidIndex(BraidIndex::try_new(idx + rhs).unwrap())
                }
            }
        }
    }
}

pub fn data(min: Option<u16>, max: Option<u16>) -> impl Strategy<Value = Data> {
    if let Some(min) = min
        && min == 0
    {
        panic!("min must be positive to generate this data.");
    }
    if let (Some(min), Some(max)) = (min, max)
        && min > max
    {
        panic!("min may be no larger than max.");
    }
    let min = min.unwrap_or(1u16);
    let max = max.unwrap_or(u16::MAX);
    let max_u8 = if max <= u8::MAX as u16 {
        max as u8
    } else {
        u8::MAX
    };
    let max_i8 = if max <= i8::MAX as u16 {
        max as i8
    } else {
        i8::MAX
    };
    let max_i16 = if max <= i16::MAX as u16 {
        max as i16
    } else {
        i16::MAX
    };
    if min <= i8::MAX as u16 {
        prop_oneof![
            ((min as u8)..=max_u8).prop_map(Data::U8),
            (min..=max).prop_map(Data::U16),
            ((min as u32)..=(max as u32)).prop_map(Data::U32),
            ((min as u64)..=(max as u64)).prop_map(Data::U64),
            ((min as u128)..=(max as u128)).prop_map(Data::U128),
            ((min as usize)..=(max as usize)).prop_map(Data::USize),
            ((min as i8)..=max_i8).prop_map(Data::I8),
            ((min as i16)..=max_i16).prop_map(Data::I16),
            ((min as i32)..=(max as i32)).prop_map(Data::I32),
            ((min as i64)..=(max as i64)).prop_map(Data::I64),
            ((min as i128)..=(max as i128)).prop_map(Data::I128),
            ((min as isize)..=(max as isize)).prop_map(Data::ISize),
            (min..=max).prop_map(|val| Data::Strand(Strand::try_new(val).unwrap())),
            (min..=max).prop_map(|val| Data::BraidIndex(BraidIndex::try_new(val).unwrap())),
        ]
        .boxed()
    } else if min <= u8::MAX as u16 {
        prop_oneof![
            ((min as u8)..=max_u8).prop_map(Data::U8),
            (min..=max).prop_map(Data::U16),
            ((min as u32)..=(max as u32)).prop_map(Data::U32),
            ((min as u64)..=(max as u64)).prop_map(Data::U64),
            ((min as u128)..=(max as u128)).prop_map(Data::U128),
            ((min as usize)..=(max as usize)).prop_map(Data::USize),
            ((min as i16)..=max_i16).prop_map(Data::I16),
            ((min as i32)..=(max as i32)).prop_map(Data::I32),
            ((min as i64)..=(max as i64)).prop_map(Data::I64),
            ((min as i128)..=(max as i128)).prop_map(Data::I128),
            ((min as isize)..=(max as isize)).prop_map(Data::ISize),
            (min..=max).prop_map(|val| Data::Strand(Strand::try_new(val).unwrap())),
            (min..=max).prop_map(|val| Data::BraidIndex(BraidIndex::try_new(val).unwrap())),
        ]
        .boxed()
    } else if min <= i16::MAX as u16 {
        prop_oneof![
            (min..=max).prop_map(Data::U16),
            ((min as u32)..=(max as u32)).prop_map(Data::U32),
            ((min as u64)..=(max as u64)).prop_map(Data::U64),
            ((min as u128)..=(max as u128)).prop_map(Data::U128),
            ((min as usize)..=(max as usize)).prop_map(Data::USize),
            ((min as i16)..=max_i16).prop_map(Data::I16),
            ((min as i32)..=(max as i32)).prop_map(Data::I32),
            ((min as i64)..=(max as i64)).prop_map(Data::I64),
            ((min as i128)..=(max as i128)).prop_map(Data::I128),
            ((min as isize)..=(max as isize)).prop_map(Data::ISize),
            (min..=max).prop_map(|val| Data::Strand(Strand::try_new(val).unwrap())),
            (min..=max).prop_map(|val| Data::BraidIndex(BraidIndex::try_new(val).unwrap())),
        ]
        .boxed()
    } else {
        prop_oneof![
            (min..=max).prop_map(Data::U16),
            ((min as u32)..=(max as u32)).prop_map(Data::U32),
            ((min as u64)..=(max as u64)).prop_map(Data::U64),
            ((min as u128)..=(max as u128)).prop_map(Data::U128),
            ((min as usize)..=(max as usize)).prop_map(Data::USize),
            ((min as i32)..=(max as i32)).prop_map(Data::I32),
            ((min as i64)..=(max as i64)).prop_map(Data::I64),
            ((min as i128)..=(max as i128)).prop_map(Data::I128),
            ((min as isize)..=(max as isize)).prop_map(Data::ISize),
            (min..=max).prop_map(|val| Data::Strand(Strand::try_new(val).unwrap())),
            (min..=max).prop_map(|val| Data::BraidIndex(BraidIndex::try_new(val).unwrap())),
        ]
        .boxed()
    }
}
