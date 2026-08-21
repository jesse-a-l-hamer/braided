use proptest::prelude::*;

use crate::arbitrary::valid;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FailedU16ConversionData {
    NegOverflowI8(i8),
    NegOverflowI16(i16),
    NegOverflowI32(i32),
    NegOverflowI64(i64),
    NegOverflowI128(i128),
    NegOverflowISize(isize),
    PosOverflowI32(i32),
    PosOverflowI64(i64),
    PosOverflowI128(i128),
    PosOverflowISize(isize),
    PosOverflowU32(u32),
    PosOverflowU64(u64),
    PosOverflowU128(u128),
    PosOverflowUSize(usize),
    AbsOverflowI32(i32),
    AbsOverflowI64(i64),
    AbsOverflowI128(i128),
    AbsOverflowISize(isize),
}

impl TryFrom<FailedU16ConversionData> for u16 {
    type Error = std::num::TryFromIntError;
    fn try_from(value: FailedU16ConversionData) -> Result<Self, Self::Error> {
        match value {
            FailedU16ConversionData::NegOverflowI8(val) => val.try_into(),
            FailedU16ConversionData::NegOverflowI16(val) => val.try_into(),
            FailedU16ConversionData::NegOverflowI32(val) => val.try_into(),
            FailedU16ConversionData::NegOverflowI64(val) => val.try_into(),
            FailedU16ConversionData::NegOverflowI128(val) => val.try_into(),
            FailedU16ConversionData::NegOverflowISize(val) => val.try_into(),
            FailedU16ConversionData::PosOverflowI32(val) => val.try_into(),
            FailedU16ConversionData::PosOverflowI64(val) => val.try_into(),
            FailedU16ConversionData::PosOverflowI128(val) => val.try_into(),
            FailedU16ConversionData::PosOverflowISize(val) => val.try_into(),
            FailedU16ConversionData::PosOverflowU32(val) => val.try_into(),
            FailedU16ConversionData::PosOverflowU64(val) => val.try_into(),
            FailedU16ConversionData::PosOverflowU128(val) => val.try_into(),
            FailedU16ConversionData::PosOverflowUSize(val) => val.try_into(),
            FailedU16ConversionData::AbsOverflowI32(val) => val.unsigned_abs().try_into(),
            FailedU16ConversionData::AbsOverflowI64(val) => val.unsigned_abs().try_into(),
            FailedU16ConversionData::AbsOverflowI128(val) => val.unsigned_abs().try_into(),
            FailedU16ConversionData::AbsOverflowISize(val) => val.unsigned_abs().try_into(),
        }
    }
}

impl TryFrom<FailedU16ConversionData> for isize {
    type Error = std::num::TryFromIntError;
    fn try_from(value: FailedU16ConversionData) -> Result<Self, Self::Error> {
        match value {
            FailedU16ConversionData::NegOverflowI8(val) => Ok(val.into()),
            FailedU16ConversionData::NegOverflowI16(val) => Ok(val.into()),
            FailedU16ConversionData::NegOverflowI32(val) => val.try_into(),
            FailedU16ConversionData::NegOverflowI64(val) => val.try_into(),
            FailedU16ConversionData::NegOverflowI128(val) => val.try_into(),
            FailedU16ConversionData::NegOverflowISize(val) => Ok(val),
            FailedU16ConversionData::PosOverflowI32(val) => val.try_into(),
            FailedU16ConversionData::PosOverflowI64(val) => val.try_into(),
            FailedU16ConversionData::PosOverflowI128(val) => val.try_into(),
            FailedU16ConversionData::PosOverflowISize(val) => Ok(val),
            FailedU16ConversionData::PosOverflowU32(val) => val.try_into(),
            FailedU16ConversionData::PosOverflowU64(val) => val.try_into(),
            FailedU16ConversionData::PosOverflowU128(val) => val.try_into(),
            FailedU16ConversionData::PosOverflowUSize(val) => val.try_into(),
            FailedU16ConversionData::AbsOverflowI32(val) => val.unsigned_abs().try_into(),
            FailedU16ConversionData::AbsOverflowI64(val) => val.unsigned_abs().try_into(),
            FailedU16ConversionData::AbsOverflowI128(val) => val.unsigned_abs().try_into(),
            FailedU16ConversionData::AbsOverflowISize(val) => val.unsigned_abs().try_into(),
        }
    }
}

impl From<FailedU16ConversionData> for valid::u16::Data {
    fn from(value: FailedU16ConversionData) -> Self {
        match value {
            FailedU16ConversionData::NegOverflowI8(val) => valid::u16::Data::I8(val),
            FailedU16ConversionData::NegOverflowI16(val) => valid::u16::Data::I16(val),
            FailedU16ConversionData::NegOverflowI32(val)
            | FailedU16ConversionData::PosOverflowI32(val)
            | FailedU16ConversionData::AbsOverflowI32(val) => valid::u16::Data::I32(val),
            FailedU16ConversionData::NegOverflowI64(val)
            | FailedU16ConversionData::PosOverflowI64(val)
            | FailedU16ConversionData::AbsOverflowI64(val) => valid::u16::Data::I64(val),
            FailedU16ConversionData::NegOverflowI128(val)
            | FailedU16ConversionData::PosOverflowI128(val)
            | FailedU16ConversionData::AbsOverflowI128(val) => valid::u16::Data::I128(val),
            FailedU16ConversionData::NegOverflowISize(val)
            | FailedU16ConversionData::PosOverflowISize(val)
            | FailedU16ConversionData::AbsOverflowISize(val) => valid::u16::Data::ISize(val),
            FailedU16ConversionData::PosOverflowU32(val) => valid::u16::Data::U32(val),
            FailedU16ConversionData::PosOverflowU64(val) => valid::u16::Data::U64(val),
            FailedU16ConversionData::PosOverflowU128(val) => valid::u16::Data::U128(val),
            FailedU16ConversionData::PosOverflowUSize(val) => valid::u16::Data::USize(val),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct FailedU16Conversion {
    pub data: FailedU16ConversionData,
    pub error: std::num::TryFromIntError,
}

fn neg_overflow() -> impl Strategy<Value = FailedU16Conversion> {
    prop_oneof![
        (i8::MIN..0).prop_map(|data| {
            FailedU16Conversion {
                data: FailedU16ConversionData::NegOverflowI8(data),
                error: <u16 as TryFrom<i8>>::try_from(data).unwrap_err(),
            }
        }),
        (i16::MIN..0).prop_map(|data| {
            FailedU16Conversion {
                data: FailedU16ConversionData::NegOverflowI16(data),
                error: <u16 as TryFrom<i16>>::try_from(data).unwrap_err(),
            }
        }),
        (i32::MIN..0).prop_map(|data| {
            FailedU16Conversion {
                data: FailedU16ConversionData::NegOverflowI32(data),
                error: <u16 as TryFrom<i32>>::try_from(data).unwrap_err(),
            }
        }),
        (i64::MIN..0).prop_map(|data| {
            FailedU16Conversion {
                data: FailedU16ConversionData::NegOverflowI64(data),
                error: <u16 as TryFrom<i64>>::try_from(data).unwrap_err(),
            }
        }),
        (i128::MIN..0).prop_map(|data| {
            FailedU16Conversion {
                data: FailedU16ConversionData::NegOverflowI128(data),
                error: <u16 as TryFrom<i128>>::try_from(data).unwrap_err(),
            }
        }),
        (isize::MIN..0).prop_map(|data| {
            FailedU16Conversion {
                data: FailedU16ConversionData::NegOverflowISize(data),
                error: <u16 as TryFrom<isize>>::try_from(data).unwrap_err(),
            }
        }),
    ]
}

fn pos_overflow() -> impl Strategy<Value = FailedU16Conversion> {
    prop_oneof![
        ((u16::MAX as i32)..=i32::MAX).prop_map(|data| {
            FailedU16Conversion {
                data: FailedU16ConversionData::PosOverflowI32(data),
                error: <u16 as TryFrom<i32>>::try_from(data).unwrap_err(),
            }
        }),
        ((u16::MAX as i64)..=i64::MAX).prop_map(|data| {
            FailedU16Conversion {
                data: FailedU16ConversionData::PosOverflowI64(data),
                error: <u16 as TryFrom<i64>>::try_from(data).unwrap_err(),
            }
        }),
        ((u16::MAX as i128)..=i128::MAX).prop_map(|data| {
            FailedU16Conversion {
                data: FailedU16ConversionData::PosOverflowI128(data),
                error: <u16 as TryFrom<i128>>::try_from(data).unwrap_err(),
            }
        }),
        ((u16::MAX as isize)..=isize::MAX).prop_map(|data| {
            FailedU16Conversion {
                data: FailedU16ConversionData::PosOverflowISize(data),
                error: <u16 as TryFrom<isize>>::try_from(data).unwrap_err(),
            }
        }),
        ((u16::MAX as u32)..=u32::MAX).prop_map(|data| {
            FailedU16Conversion {
                data: FailedU16ConversionData::PosOverflowU32(data),
                error: <u16 as TryFrom<u32>>::try_from(data).unwrap_err(),
            }
        }),
        ((u16::MAX as u64)..=u64::MAX).prop_map(|data| {
            FailedU16Conversion {
                data: FailedU16ConversionData::PosOverflowU64(data),
                error: <u16 as TryFrom<u64>>::try_from(data).unwrap_err(),
            }
        }),
        ((u16::MAX as u128)..=u128::MAX).prop_map(|data| {
            FailedU16Conversion {
                data: FailedU16ConversionData::PosOverflowU128(data),
                error: <u16 as TryFrom<u128>>::try_from(data).unwrap_err(),
            }
        }),
        ((u16::MAX as usize)..=usize::MAX).prop_map(|data| {
            FailedU16Conversion {
                data: FailedU16ConversionData::PosOverflowUSize(data),
                error: <u16 as TryFrom<usize>>::try_from(data).unwrap_err(),
            }
        }),
    ]
}

fn abs_overflow() -> impl Strategy<Value = FailedU16Conversion> {
    prop_oneof![
        (i32::MIN..=-(u16::MAX as i32 + 1)).prop_map(|data| {
            FailedU16Conversion {
                data: FailedU16ConversionData::AbsOverflowI32(data),
                error: <u16 as TryFrom<u32>>::try_from(data.unsigned_abs()).unwrap_err(),
            }
        }),
        (i64::MIN..=-(u16::MAX as i64 + 1)).prop_map(|data| {
            FailedU16Conversion {
                data: FailedU16ConversionData::AbsOverflowI64(data),
                error: <u16 as TryFrom<u64>>::try_from(data.unsigned_abs()).unwrap_err(),
            }
        }),
        (i128::MIN..=-(u16::MAX as i128 + 1)).prop_map(|data| {
            FailedU16Conversion {
                data: FailedU16ConversionData::AbsOverflowI128(data),
                error: <u16 as TryFrom<u128>>::try_from(data.unsigned_abs()).unwrap_err(),
            }
        }),
        (isize::MIN..=-(u16::MAX as isize + 1)).prop_map(|data| {
            FailedU16Conversion {
                data: FailedU16ConversionData::AbsOverflowISize(data),
                error: <u16 as TryFrom<usize>>::try_from(data.unsigned_abs()).unwrap_err(),
            }
        }),
    ]
}

pub fn failed_u16_conversion() -> impl Strategy<Value = FailedU16Conversion> {
    prop_oneof![neg_overflow(), pos_overflow()]
}

pub fn bad_macro_exponent() -> impl Strategy<Value = FailedU16Conversion> {
    prop_oneof![abs_overflow(), pos_overflow()]
}
