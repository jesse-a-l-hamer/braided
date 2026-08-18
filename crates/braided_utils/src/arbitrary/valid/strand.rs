use crate::arbitrary::valid;
use braided::{Strand, StrandResult};
use proptest::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AdditionOperand {
    U16(u16),
    Strand(Strand),
}

impl From<AdditionOperand> for u16 {
    fn from(value: AdditionOperand) -> Self {
        match value {
            AdditionOperand::U16(val) => val,
            AdditionOperand::Strand(val) => val.into(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SubtractionOperand {
    U16(u16),
    Strand(Strand),
}

impl From<SubtractionOperand> for u16 {
    fn from(value: SubtractionOperand) -> Self {
        match value {
            SubtractionOperand::U16(val) => val,
            SubtractionOperand::Strand(val) => val.into(),
        }
    }
}

pub fn data(min: Option<u16>, max: Option<u16>) -> impl Strategy<Value = valid::u16::Data> {
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
    valid::u16::data(min, max)
}

pub fn new(min: Option<u16>, max: Option<u16>) -> impl Strategy<Value = Strand> {
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
    data(min, max).prop_map(|data| Strand::try_new(data).unwrap())
}

pub fn addition_data(
    max_sum: Option<u16>,
) -> impl Strategy<Value = (AdditionOperand, AdditionOperand, StrandResult)> {
    if let Some(max_sum) = max_sum
        && max_sum < 1
    {
        panic!("Sum must be positive.");
    }
    (1..=max_sum.unwrap_or(u16::MAX))
        .prop_flat_map(|sum| (Just(sum), 0..=sum))
        .prop_perturb(|(sum, summand), mut rng| {
            let left = if summand == 0 || (summand < sum && rng.random_bool(0.5)) {
                AdditionOperand::U16(summand)
            } else {
                AdditionOperand::Strand(Strand::try_new(summand).unwrap())
            };
            let right = match left {
                AdditionOperand::U16(_) => {
                    AdditionOperand::Strand(Strand::try_new(sum - summand).unwrap())
                }
                AdditionOperand::Strand(_) => {
                    if sum - summand == 0 || rng.random_bool(0.5) {
                        AdditionOperand::U16(sum - summand)
                    } else {
                        AdditionOperand::Strand(Strand::try_new(sum - summand).unwrap())
                    }
                }
            };

            (left, right, Strand::try_new(sum))
        })
}

pub fn subtraction_data(
    max_left: Option<u16>,
) -> impl Strategy<Value = (SubtractionOperand, SubtractionOperand, StrandResult)> {
    (1..=max_left.unwrap_or(u16::MAX))
        .prop_flat_map(|left| (Just(left), 1..=left))
        .prop_perturb(|(left, difference), mut rng| {
            let right = if left == difference || rng.random_bool(0.5) {
                SubtractionOperand::U16(left - difference)
            } else {
                SubtractionOperand::Strand(Strand::try_new(left - difference).unwrap())
            };
            let left = match right {
                SubtractionOperand::U16(_) => {
                    SubtractionOperand::Strand(Strand::try_new(left).unwrap())
                }
                SubtractionOperand::Strand(_) => {
                    if rng.random_bool(0.5) {
                        SubtractionOperand::U16(left - difference)
                    } else {
                        SubtractionOperand::Strand(Strand::try_new(left - difference).unwrap())
                    }
                }
            };

            (left, right, Strand::try_new(difference))
        })
}

pub mod test_cases {
    use super::*;

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct TryNewData(pub valid::u16::Data);

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct AdditionData {
        pub left: AdditionOperand,
        pub right: AdditionOperand,
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct SubtractionData {
        pub left: SubtractionOperand,
        pub right: SubtractionOperand,
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct TryNew {
        pub data: TryNewData,
        pub expected_index: u16,
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct Addition {
        pub data: AdditionData,
        pub expected: StrandResult,
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct Subtraction {
        pub data: SubtractionData,
        pub expected: StrandResult,
    }

    pub fn try_new(min: Option<u16>, max: Option<u16>) -> impl Strategy<Value = TryNew> {
        data(min, max).prop_map(|valid_u16_data| TryNew {
            data: TryNewData(valid_u16_data),
            expected_index: valid_u16_data.try_into().unwrap(),
        })
    }

    pub fn addition(max_sum: Option<u16>) -> impl Strategy<Value = Addition> {
        addition_data(max_sum).prop_map(|(left, right, expected)| Addition {
            data: AdditionData { left, right },
            expected,
        })
    }

    pub fn subtraction(max_left: Option<u16>) -> impl Strategy<Value = Subtraction> {
        subtraction_data(max_left).prop_map(|(left, right, expected)| Subtraction {
            data: SubtractionData { left, right },
            expected,
        })
    }
}
