#[macro_export]
macro_rules! braid {
    ($index:expr $(;)?) => {
        Braid::trivial($index)
    };
    ($index:expr; [$foot:expr; $power:expr]) => {
        Braid::from_artin($index, &$crate::artin![$foot; $power].unwrap())
    };
    ($index:expr; [$foot:expr => $head:expr; $power:expr]) => {
        Braid::from_bands($index, &$crate::band![$foot => $head; $power].unwrap())
    };
    ($index:expr; [$foot:expr; $power:expr], $($tail:tt)+) => {
        {
            match (braid![$index; [$foot; $power]], braid![$index; $($tail)+]) {
                (Ok(head), Ok(tail)) => Ok(head * tail),
                (Err(head), _) => Err(head),
                (_, Err(tail)) => Err(tail)
            }
        }
    };
    ($index:expr; [$foot:expr => $head:expr; $power:expr], $($tail:tt)+) => {
        {
            match (braid![$index; [$foot => $head; $power]], braid![$index; $($tail)+]) {
                (Ok(head), Ok(tail)) => Ok(head * tail),
                (Err(head), _) => Err(head),
                (_, Err(tail)) => Err(tail)
            }
        }

    };
}

#[cfg(test)]
mod tests {
    use crate::{Braid, artin, band};
    use googletest::assert_that;
    use googletest::matchers::{eq, ok};

    #[test]
    fn macro_braid_with_only_index_produces_trivial_braid() {
        let braid = braid![3];
        assert_that!(braid, ok(eq(&Braid::trivial(3).unwrap())))
    }

    #[test]
    fn macro_braid_with_artin_generators_is_successful() {
        let braid = braid![3; [1; 2], [2; -3], [1; -1], [2; 1]];
        assert_that!(
            braid,
            ok(eq(&Braid::from_artin(
                3,
                &[
                    artin![1; 2].unwrap(),
                    artin![2; -3].unwrap(),
                    artin![1; -1].unwrap(),
                    artin![2; 1].unwrap()
                ]
                .concat()
            )
            .unwrap()))
        )
    }

    #[test]
    fn macro_braid_with_band_generators_is_successful() {
        let braid = braid![3; [1 => 3; 3], [1 => 2; 1], [2 => 3; -4], [1 => 3; -2]];
        assert_that!(
            braid,
            ok(eq(&Braid::from_bands(
                3,
                &[
                    band![1 => 3; 3].unwrap(),
                    band![1 => 2; 1].unwrap(),
                    band![2 => 3; -4].unwrap(),
                    band![1 => 3; -2].unwrap(),
                ]
                .concat()
            )
            .unwrap()))
        )
    }

    #[test]
    fn macro_braid_with_mixed_generators_is_successful() {
        let braid = braid![3;
            [1; -1],
            [1 => 3; 3],
            [1; 2],
            [2; -3],
            [1 => 2; 1],
            [2 => 3; -4],
            [1 => 3; -2],
            [2; 1]
        ];
        assert_that!(
            braid,
            ok(eq(&Braid::from_bands(
                3,
                &[
                    band![1 => 2; -1].unwrap(),
                    band![1 => 3; 3].unwrap(),
                    band![1 => 2; 2].unwrap(),
                    band![2 => 3; -3].unwrap(),
                    band![1 => 2; 1].unwrap(),
                    band![2 => 3; -4].unwrap(),
                    band![1 => 3; -2].unwrap(),
                    band![2 => 3; 1].unwrap(),
                ]
                .concat()
            )
            .unwrap()))
        )
    }
}
