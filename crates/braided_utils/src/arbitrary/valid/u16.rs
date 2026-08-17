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

impl TryInto<u16> for Data {
    type Error = std::convert::Infallible;
    fn try_into(self) -> Result<u16, Self::Error> {
        match self {
            Self::U8(val) => Ok(val.into()),
            Self::U16(val) => Ok(val),
            Self::U32(val) => Ok(val.try_into().unwrap()),
            Self::U64(val) => Ok(val.try_into().unwrap()),
            Self::U128(val) => Ok(val.try_into().unwrap()),
            Self::USize(val) => Ok(val.try_into().unwrap()),
            Self::I8(val) => Ok(val.try_into().unwrap()),
            Self::I16(val) => Ok(val.try_into().unwrap()),
            Self::I32(val) => Ok(val.try_into().unwrap()),
            Self::I64(val) => Ok(val.try_into().unwrap()),
            Self::I128(val) => Ok(val.try_into().unwrap()),
            Self::ISize(val) => Ok(val.try_into().unwrap()),
            Self::Strand(val) => Ok(val.into()),
            Self::BraidIndex(val) => Ok(val.into()),
        }
    }
}

pub fn data(min: Option<u16>, max: Option<u16>) -> impl Strategy<Value = Data> {
    let min = min.unwrap_or(1u16);
    let weight_u8 = if min <= u8::MAX as u16 { 1u32 } else { 0 };
    let weight_i8 = if min <= i8::MAX as u16 { 1u32 } else { 0 };
    let weight_i16 = if min <= i16::MAX as u16 { 1u32 } else { 0 };
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
    let weight: u32 = 1;
    prop_oneof![
        weight_u8 => ((min as u8)..max_u8).prop_map(Data::U8),
        weight => (min..max).prop_map(Data::U16),
        weight => ((min as u32)..(max as u32)).prop_map(Data::U32),
        weight => ((min as u64)..(max as u64)).prop_map(Data::U64),
        weight => ((min as u128)..(max as u128)).prop_map(Data::U128),
        weight => ((min as usize)..(max as usize)).prop_map(Data::USize),
        weight_i8 => ((min as i8)..max_i8).prop_map(Data::I8),
        weight_i16 => ((min as i16)..max_i16).prop_map(Data::I16),
        weight => ((min as i32)..(max as i32)).prop_map(Data::I32),
        weight => ((min as i64)..(max as i64)).prop_map(Data::I64),
        weight => ((min as i128)..(max as i128)).prop_map(Data::I128),
        weight => ((min as isize)..(max as isize)).prop_map(Data::ISize),
        weight => (min..max)
            .prop_map(|val| Data::Strand(Strand::try_new(val).unwrap())),
        weight => (min..max)
            .prop_map(|val| Data::BraidIndex(BraidIndex::try_new(val).unwrap())),
    ]
}
