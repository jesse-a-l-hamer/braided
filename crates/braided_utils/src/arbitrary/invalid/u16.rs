use proptest::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum InvalidU16Data {
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
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct InvalidU16 {
    pub data: InvalidU16Data,
    pub error: std::num::TryFromIntError,
}

fn arbitrary_neg_overflow() -> impl Strategy<Value = InvalidU16> {
    prop_oneof![
        (i8::MIN..0).prop_map(|data| {
            InvalidU16 {
                data: InvalidU16Data::NegOverflowI8(data),
                error: <u16 as TryFrom<i8>>::try_from(data).unwrap_err(),
            }
        }),
        (i16::MIN..0).prop_map(|data| {
            InvalidU16 {
                data: InvalidU16Data::NegOverflowI16(data),
                error: <u16 as TryFrom<i16>>::try_from(data).unwrap_err(),
            }
        }),
        (i32::MIN..0).prop_map(|data| {
            InvalidU16 {
                data: InvalidU16Data::NegOverflowI32(data),
                error: <u16 as TryFrom<i32>>::try_from(data).unwrap_err(),
            }
        }),
        (i64::MIN..0).prop_map(|data| {
            InvalidU16 {
                data: InvalidU16Data::NegOverflowI64(data),
                error: <u16 as TryFrom<i64>>::try_from(data).unwrap_err(),
            }
        }),
        (i128::MIN..0).prop_map(|data| {
            InvalidU16 {
                data: InvalidU16Data::NegOverflowI128(data),
                error: <u16 as TryFrom<i128>>::try_from(data).unwrap_err(),
            }
        }),
        (isize::MIN..0).prop_map(|data| {
            InvalidU16 {
                data: InvalidU16Data::NegOverflowISize(data),
                error: <u16 as TryFrom<isize>>::try_from(data).unwrap_err(),
            }
        }),
    ]
}

fn arbitrary_pos_overflow() -> impl Strategy<Value = InvalidU16> {
    prop_oneof![
        ((u16::MAX as i32)..=i32::MAX).prop_map(|data| {
            InvalidU16 {
                data: InvalidU16Data::PosOverflowI32(data),
                error: <u16 as TryFrom<i32>>::try_from(data).unwrap_err(),
            }
        }),
        ((u16::MAX as i64)..=i64::MAX).prop_map(|data| {
            InvalidU16 {
                data: InvalidU16Data::PosOverflowI64(data),
                error: <u16 as TryFrom<i64>>::try_from(data).unwrap_err(),
            }
        }),
        ((u16::MAX as i128)..=i128::MAX).prop_map(|data| {
            InvalidU16 {
                data: InvalidU16Data::PosOverflowI128(data),
                error: <u16 as TryFrom<i128>>::try_from(data).unwrap_err(),
            }
        }),
        ((u16::MAX as isize)..=isize::MAX).prop_map(|data| {
            InvalidU16 {
                data: InvalidU16Data::PosOverflowISize(data),
                error: <u16 as TryFrom<isize>>::try_from(data).unwrap_err(),
            }
        }),
        ((u16::MAX as u32)..=u32::MAX).prop_map(|data| {
            InvalidU16 {
                data: InvalidU16Data::PosOverflowU32(data),
                error: <u16 as TryFrom<u32>>::try_from(data).unwrap_err(),
            }
        }),
        ((u16::MAX as u64)..=u64::MAX).prop_map(|data| {
            InvalidU16 {
                data: InvalidU16Data::PosOverflowU64(data),
                error: <u16 as TryFrom<u64>>::try_from(data).unwrap_err(),
            }
        }),
        ((u16::MAX as u128)..=u128::MAX).prop_map(|data| {
            InvalidU16 {
                data: InvalidU16Data::PosOverflowU128(data),
                error: <u16 as TryFrom<u128>>::try_from(data).unwrap_err(),
            }
        }),
        ((u16::MAX as usize)..=usize::MAX).prop_map(|data| {
            InvalidU16 {
                data: InvalidU16Data::PosOverflowUSize(data),
                error: <u16 as TryFrom<usize>>::try_from(data).unwrap_err(),
            }
        }),
    ]
}

pub fn arbitrary_invalid_u16() -> impl Strategy<Value = InvalidU16> {
    prop_oneof![arbitrary_neg_overflow(), arbitrary_pos_overflow()]
}
