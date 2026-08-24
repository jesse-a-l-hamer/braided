use proptest::prelude::*;

/// Returns a partition of `partition_value` into a vector of odd numbers no greater than
/// `max_partition_elem`.
pub fn partition_into_odd_numbers(
    partition_value: u16,
    max_partition_elem: u16,
) -> impl Strategy<Value = Vec<u16>> {
    if max_partition_elem == 0 {
        panic!("Partition elements must be positive.");
    } else if max_partition_elem == 1 {
        return vec![Just(1); partition_value as usize].boxed();
    } else if partition_value == 0 {
        return Just(Vec::new()).boxed();
    } else if partition_value == 1 {
        return Just(vec![1]).boxed();
    } else if partition_value == 2 {
        return Just(vec![1, 1]).boxed();
    }
    let partition_length_strategy = if partition_value.is_multiple_of(2) {
        (1..=partition_value / 2).prop_map(|k| 2 * k).boxed()
    } else {
        (0..=partition_value.div_euclid(2))
            .prop_map(|k| 2 * k + 1)
            .boxed()
    };
    partition_length_strategy
        .prop_flat_map(move |partition_length| {
            let max_partition_elem =
                max_partition_elem.min(partition_value - (partition_length - 1));
            let half_elem_max = if max_partition_elem.is_multiple_of(2) {
                max_partition_elem / 2 - 1
            } else {
                max_partition_elem.div_euclid(2)
            };
            vec![0..=half_elem_max; partition_length as usize]
        })
        .prop_map(move |half_partition_elems| {
            let mut partition: Vec<usize> = half_partition_elems
                .iter()
                .map(|k| (2 * k + 1) as usize)
                .collect();
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
        .boxed()
}
