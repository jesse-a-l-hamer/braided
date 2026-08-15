use proptest::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum InvalidU16Data {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    ISize(isize),
    U32(u32),
    U64(u64),
    U128(u128),
    USize(usize),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum InvalidU16 {
    NegOverflow {
        data: InvalidU16Data,
        error: std::num::TryFromIntError,
    },
    PosOverflow {
        data: InvalidU16Data,
        error: std::num::TryFromIntError,
    },
}

fn arbitrary_neg_overflow() -> impl Strategy<Value = InvalidU16> {
    prop_oneof![
        (i8::MIN..0).prop_map(|data| {
            InvalidU16::NegOverflow {
                data: InvalidU16Data::I8(data),
                error: <u16 as TryFrom<i8>>::try_from(data).unwrap_err(),
            }
        }),
        (i16::MIN..0).prop_map(|data| {
            InvalidU16::NegOverflow {
                data: InvalidU16Data::I16(data),
                error: <u16 as TryFrom<i16>>::try_from(data).unwrap_err(),
            }
        }),
        (i32::MIN..0).prop_map(|data| {
            InvalidU16::NegOverflow {
                data: InvalidU16Data::I32(data),
                error: <u16 as TryFrom<i32>>::try_from(data).unwrap_err(),
            }
        }),
        (i64::MIN..0).prop_map(|data| {
            InvalidU16::NegOverflow {
                data: InvalidU16Data::I64(data),
                error: <u16 as TryFrom<i64>>::try_from(data).unwrap_err(),
            }
        }),
        (i128::MIN..0).prop_map(|data| {
            InvalidU16::NegOverflow {
                data: InvalidU16Data::I128(data),
                error: <u16 as TryFrom<i128>>::try_from(data).unwrap_err(),
            }
        }),
        (isize::MIN..0).prop_map(|data| {
            InvalidU16::NegOverflow {
                data: InvalidU16Data::ISize(data),
                error: <u16 as TryFrom<isize>>::try_from(data).unwrap_err(),
            }
        }),
    ]
}

fn arbitrary_pos_overflow() -> impl Strategy<Value = InvalidU16> {
    prop_oneof![
        (1..=i8::MAX).prop_map(|data| {
            InvalidU16::PosOverflow {
                data: InvalidU16Data::I8(data),
                error: <u16 as TryFrom<i8>>::try_from(data).unwrap_err(),
            }
        }),
        (1..=i16::MAX).prop_map(|data| {
            InvalidU16::PosOverflow {
                data: InvalidU16Data::I16(data),
                error: <u16 as TryFrom<i16>>::try_from(data).unwrap_err(),
            }
        }),
        (1..=i32::MAX).prop_map(|data| {
            InvalidU16::PosOverflow {
                data: InvalidU16Data::I32(data),
                error: <u16 as TryFrom<i32>>::try_from(data).unwrap_err(),
            }
        }),
        (1..=i64::MAX).prop_map(|data| {
            InvalidU16::PosOverflow {
                data: InvalidU16Data::I64(data),
                error: <u16 as TryFrom<i64>>::try_from(data).unwrap_err(),
            }
        }),
        (1..=i128::MAX).prop_map(|data| {
            InvalidU16::PosOverflow {
                data: InvalidU16Data::I128(data),
                error: <u16 as TryFrom<i128>>::try_from(data).unwrap_err(),
            }
        }),
        (1..=isize::MAX).prop_map(|data| {
            InvalidU16::PosOverflow {
                data: InvalidU16Data::ISize(data),
                error: <u16 as TryFrom<isize>>::try_from(data).unwrap_err(),
            }
        }),
        (1..=u32::MAX).prop_map(|data| {
            InvalidU16::PosOverflow {
                data: InvalidU16Data::U32(data),
                error: <u16 as TryFrom<u32>>::try_from(data).unwrap_err(),
            }
        }),
        (1..=u64::MAX).prop_map(|data| {
            InvalidU16::PosOverflow {
                data: InvalidU16Data::U64(data),
                error: <u16 as TryFrom<u64>>::try_from(data).unwrap_err(),
            }
        }),
        (1..=u128::MAX).prop_map(|data| {
            InvalidU16::PosOverflow {
                data: InvalidU16Data::U128(data),
                error: <u16 as TryFrom<u128>>::try_from(data).unwrap_err(),
            }
        }),
        (1..=usize::MAX).prop_map(|data| {
            InvalidU16::PosOverflow {
                data: InvalidU16Data::USize(data),
                error: <u16 as TryFrom<usize>>::try_from(data).unwrap_err(),
            }
        }),
    ]
}

pub fn arbitrary_invalid_u16() -> impl Strategy<Value = InvalidU16> {
    prop_oneof![arbitrary_neg_overflow(), arbitrary_pos_overflow()]
}
