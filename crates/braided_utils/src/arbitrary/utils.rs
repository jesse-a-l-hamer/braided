use proptest::prelude::*;

/// Returns a partition of `partition_value` into a vector of odd numbers no greater than
/// `max_partition_elem`.
pub fn partition_into_odd_numbers(
    partition_value: u16,
    max_partition_elem: u16,
) -> impl Strategy<Value = Vec<u16>> {
    let partition_length_strategy = if partition_value == 0 {
        (0u16..=0).boxed()
    } else if partition_value.is_multiple_of(2) {
        (1..=partition_value / 2).prop_map(|k| 2 * k).boxed()
    } else {
        (1..=partition_value.div_euclid(2))
            .prop_map(|k| 2 * k + 1)
            .boxed()
    };
    partition_length_strategy
        .prop_flat_map(move |partition_length| {
            let elem_upper_bound = if max_partition_elem.is_multiple_of(2) {
                max_partition_elem / 2 - 1
            } else {
                max_partition_elem.div_euclid(2)
            };
            vec![1..=elem_upper_bound; partition_length as usize]
        })
        .prop_map(move |partition| {
            let mut partition: Vec<usize> =
                partition.iter().map(|k| (2 * k + 1) as usize).collect();
            let mut sum = partition.iter().sum::<usize>();

            if sum > partition_value.into() {
                let mut partition_iter = partition.iter_mut();
                while sum > partition_value.into() {
                    match partition_iter.next() {
                        None => partition_iter = partition.iter_mut(),
                        Some(elem) => {
                            if *elem > 1usize {
                                *elem -= 2;
                                sum -= 2;
                            }
                        }
                    }
                }
            } else if sum < partition_value.into() {
                let elem_upper_bound = if max_partition_elem.is_multiple_of(2) {
                    max_partition_elem - 1
                } else {
                    max_partition_elem
                };
                let mut partition_iter = partition.iter_mut();
                while sum < partition_value.into() {
                    match partition_iter.next() {
                        None => partition_iter = partition.iter_mut(),
                        Some(elem) => {
                            if *elem < elem_upper_bound.into() {
                                *elem += 2;
                                sum += 2;
                            }
                        }
                    }
                }
            }

            partition
                .iter()
                .map(|&elem| <usize as TryInto<u16>>::try_into(elem).unwrap())
                .collect()
        })
}
