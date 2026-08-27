use proptest::prelude::*;

/// Returns a partition of `partition_value` into a vector of odd numbers no greater than
/// `max_partition_elem`.
pub fn partition_into_odd_numbers(
    partition_value: u16,
    max_partition_elem: usize,
) -> impl Strategy<Value = Vec<u16>> {
    if partition_value == 0 {
        return Just(Vec::new()).boxed();
    }

    if max_partition_elem == 0 {
        panic!("Partition elements of a nonzero value must be positive.");
    }

    let max_partition_elem = if max_partition_elem.is_multiple_of(2) {
        max_partition_elem - 1
    } else {
        max_partition_elem
    };

    let max_partition_elem = if (partition_value as usize) < max_partition_elem {
        if partition_value.is_multiple_of(2) {
            partition_value - 1
        } else {
            partition_value
        }
    } else {
        max_partition_elem.try_into().unwrap()
    };

    if max_partition_elem == 1 {
        return vec![Just(1); partition_value as usize].boxed();
    }

    let partition_value_per_max_elem = partition_value.div_ceil(max_partition_elem);
    if partition_value as usize + 1
        < partition_value_per_max_elem as usize + max_partition_elem as usize
    {
        panic!(
            "The given value cannot be partitioned into odd numbers which are at most
             max_partition_elem."
        )
    }

    (partition_value_per_max_elem..=(partition_value - max_partition_elem + 1))
        .prop_map(move |partition_length| {
            if partition_length % 2 == partition_value % 2 {
                partition_length
            } else {
                partition_length + 1
            }
        })
        .prop_flat_map(move |partition_length| {
            let max_partition_elem =
                max_partition_elem.min(partition_value - (partition_length - 1));

            (
                Just(max_partition_elem),
                vec![1..=max_partition_elem; partition_length as usize],
            )
        })
        .prop_perturb(move |(max_partition_elem, mut partition), mut rng| {
            for elem in partition.iter_mut() {
                if elem.is_multiple_of(2) {
                    if rng.random_bool(0.5) {
                        *elem -= 1
                    } else {
                        *elem += 1
                    }
                }
            }

            let mut sum: usize = partition.iter().map(|&elem| elem as usize).sum();

            if sum > partition_value.into() {
                let mut partition_iter = partition.iter_mut();
                while sum > partition_value.into() {
                    match partition_iter.next() {
                        None => partition_iter = partition.iter_mut(),
                        Some(elem) => {
                            if *elem > 1 {
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
                            if *elem < elem_upper_bound {
                                *elem += 2;
                                sum += 2;
                            }
                        }
                    }
                }
            }

            partition
        })
        .boxed()
}
