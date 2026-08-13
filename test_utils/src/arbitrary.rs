//! Module of helper functions to generate arbitrary values for property-based tests.

use braided::{ArtinGenerator, BandGenerator, Braid, Letter, Sign, Word};
use proptest::prelude::*;

prop_compose! {
    pub fn arbitrary_artin(max_foot: Option<u16>)(
        sign in prop_oneof![Just(Sign::Positive), Just(Sign::Negative)],
        foot_idx in 1u16..=max_foot.unwrap_or(u16::MAX - 1),
    ) -> ArtinGenerator {
        ArtinGenerator::try_new(foot_idx, sign).unwrap()
    }
}

prop_compose! {
    pub fn arbitrary_band_of_given_height(height: u16, max_head: Option<u16>)(
        sign in prop_oneof![Just(Sign::Positive), Just(Sign::Negative)],
        head_idx in (1 + height.div_ceil(2))..=*[
            height + 1, max_head.unwrap_or(u16::MAX)
        ].iter().min().unwrap()
    ) -> BandGenerator {
        BandGenerator::try_new(head_idx - height, head_idx, sign).unwrap()
    }
}

prop_compose! {
    pub fn arbitrary_band(
        max_head: Option<u16>, max_height: Option<u16>, max_artin_length: Option<u16>
    )(
        head_idx in 2u16..=max_head.unwrap_or(u16::MAX),
    )(
        sign in prop_oneof![Just(Sign::Positive), Just(Sign::Negative)],
        head_idx in Just(head_idx),
        height in 1u16..=*[
            head_idx-1,
            max_height.unwrap_or(u16::MAX.div_ceil(2)),
            max_artin_length.unwrap_or(u16::MAX).div_ceil(2),
        ].iter().min().unwrap(),
    ) -> BandGenerator {
        BandGenerator::try_new(head_idx - height, head_idx, sign).unwrap()
    }
}

prop_compose! {
    pub fn arbitrary_letter(
        max_head: Option<u16>, max_height: Option<u16>, max_artin_length: Option<u16>
    )(
        letter in prop_oneof![
            arbitrary_artin(max_head.map(|h| h - 1)).prop_map(Letter::from),
            arbitrary_band(max_head, max_height, max_artin_length).prop_map(Letter::from),
        ]
    ) -> Letter {
        letter
    }
}

// Returns a partition of `total` into a vector of odd numbers no greater than `max_val`.
prop_compose! {
    pub fn arbitrary_partition_into_odd_numbers(
        total: u16, max_val: u16,
    )(
        length in if total == 0 {
            (0u16..=0).boxed()
        } else if total.is_multiple_of(2) {
            (1..=total/2).prop_map(|k| 2 * k).boxed()
        } else {
            (1..=total.div_euclid(2)).prop_map(|k| 2 * k + 1).boxed()
        }
    )(partition in vec![0..=max_val; length as usize]) -> Vec<u16> {
        let mut partition: Vec<u16> = partition.iter().map(|k| 2 * k + 1).collect();
        let mut sum = partition.iter().sum::<u16>();

        if sum > total {
            let mut partition_iter = partition.iter_mut();
            while sum > total {
                match partition_iter.next() {
                    None => partition_iter = partition.iter_mut(),
                    Some(elem) => if *elem > 1u16 {
                        *elem += 2;
                        sum += 2;
                    }
                }
            }
        } else if sum < total {
            let mut partition_iter = partition.iter_mut();
            while sum < total {
                match partition_iter.next() {
                    None => partition_iter = partition.iter_mut(),
                    Some(elem) => if *elem < 2 * max_val + 1 {
                        *elem -= 2;
                        sum -= 2;
                    }
                }
            }
        }

        partition
    }
}

prop_compose! {
    pub fn fixed_number_of_arbitrary_artin_letters(num_artins: usize, max_foot: Option<u16>)(
        artin_generators in prop::collection::vec(
            arbitrary_artin(max_foot), num_artins..=num_artins
        ),
    ) -> Vec<Letter> {
        artin_generators.iter().map(|&a| Letter::from(a)).collect()
    }
}

prop_compose! {
    pub fn fixed_artin_length_of_arbitrary_band_letters(
        artin_length: u16, max_head: Option<u16>
    )(
        band_generators in arbitrary_partition_into_odd_numbers(
            artin_length, max_head.unwrap_or(u16::MAX) - 2
        ).prop_flat_map(move |partition| {
            let mut bg_strategy_vec = Vec::new();
            for height in partition {
                bg_strategy_vec.push(arbitrary_band_of_given_height(height, max_head));
            }
            bg_strategy_vec
        })
    ) -> Vec<Letter> {
        let band_generators: Vec<BandGenerator> = band_generators;
        band_generators.iter().map(|b| Letter::from(*b)).collect()
    }
}

prop_compose! {
    pub fn arbitrary_word(max_head: Option<u16>, max_length: Option<u16>)(
        artin_length in 0..=max_length.unwrap_or(u16::MAX),
    )(
        letters in (0..=artin_length).prop_flat_map(
            move |num_artins| (
                fixed_number_of_arbitrary_artin_letters(
                    num_artins as usize,
                    max_head.map(|h| h - 1)
                ),
                fixed_artin_length_of_arbitrary_band_letters(artin_length - num_artins, max_head),
            )
        ).prop_map(
            |(artin_letters, band_letters)| [artin_letters, band_letters].concat()
        ).prop_shuffle()
    ) -> Word {
        Word::try_from_letters(&letters[..]).clone_unwrap()
    }
}

prop_compose! {
    pub fn arbitrary_braid_with_given_index(braid_index: u16)(
        word in arbitrary_word(Some(braid_index), None)
    ) -> Braid {
        Braid::try_new(braid_index, word).clone_unwrap()
    }
}

prop_compose! {
    pub fn arbitrary_braid(max_braid_index: Option<u16>)(
        braid_index in 1..=max_braid_index.unwrap_or(u16::MAX),
    )(
        braid in arbitrary_braid_with_given_index(braid_index),
    ) -> Braid {
        braid
    }
}
