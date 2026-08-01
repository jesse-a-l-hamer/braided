/// Constructs a [`Braid`](crate::Braid) from an index and an arbitrary sequence of generators.
///
/// This macro exposes essentially the same functionality as the constructors
/// [`Braid::from_artin`](crate::Braid::from_artin) and
/// [`Braid::from_bands`](crate::Braid::from_bands), but with a less verbose syntax:
///
/// ```
/// use braided::{Braid, artin, band, braid};
///
/// # #[macro_use] extern crate braided;
/// # fn main() {
/// let braid_from_artins = braid![3; [1; 2], [2; -4], [1; -1], [2; -7]].unwrap();
/// let braid_from_bands = braid![4; [1 => 4; -1], [1 => 3; -3], [2 => 4; 5]].unwrap();
/// assert_eq!(braid_from_artins, Braid::from_artin(3, &[
///     artin![1; 2].unwrap(),
///     artin![2; -4].unwrap(),
///     artin![1; -1].unwrap(),
///     artin![2; -7].unwrap(),
/// ].concat()).unwrap());
/// assert_eq!(braid_from_bands, Braid::from_bands(4, &[
///     band![1 => 4; -1].unwrap(),
///     band![1 => 3; -3].unwrap(),
///     band![2 => 4; 5].unwrap(),
/// ].concat()).unwrap());
/// # }
/// ```
///
/// It is also possible to construct a [`Braid`](crate::Braid) using a heterogeneous sequence of
/// generators:
///
/// ```
/// use braided::{Braid, artin, band, braid};
///
/// # #[macro_use] extern crate braided;
/// # fn main() {
/// let braid_from_mixed = braid![4; [1; 2], [1 => 4; -1], [1 => 3; -3], [2; -7]].unwrap();
/// assert_eq!(braid_from_mixed, Braid::from_bands(4, &[
///     band![1 => 2; 2].unwrap(),
///     band![1 => 4; -1].unwrap(),
///     band![1 => 3; -3].unwrap(),
///     band![2 => 3; -7].unwrap(),
/// ].concat()).unwrap());
/// # }
/// ```
///
/// One may also construct a trivial braid of a given index by simply omitting any generators:
///
/// ```
/// use braided::{Braid, braid};
///
/// # #[macro_use] extern crate braided;
/// # fn main() {
/// let trivial_5_braid = braid![5].unwrap();
/// assert_eq!(trivial_5_braid, Braid::trivial(5).unwrap());
/// # }
/// ```
///
/// # Errors
///
/// Depending on the types of generators supplied to the macro, the generated code will make calls
/// to [`Braid::trivial`](crate::Braid::trivial), [`Braid::from_artin`](crate::Braid::from_artin) or
/// [`Braid::from_bands`](crate::Braid::from_bands). Please see the documentation for those
/// constructors to better understand the possible error types.
#[macro_export]
macro_rules! braid {
    ($index:expr $(;)?) => {
        $crate::Braid::trivial($index)
    };
    ($index:expr; [$foot:expr; $exp:expr]) => {
        $crate::Braid::from_artin($index, &$crate::artin![$foot; $exp].unwrap())
    };
    ($index:expr; [$foot:expr => $head:expr; $exp:expr]) => {
        $crate::Braid::from_bands($index, &$crate::band![$foot => $head; $exp].unwrap())
    };
    ($index:expr; [$foot:expr; $exp:expr], $($tail:tt)*) => {
        {
            match (braid![$index; [$foot; $exp]], braid![$index; $($tail)*]) {
                (Ok(head), Ok(tail)) => Ok(head * tail),
                (Err(head), _) => Err(head),
                (_, Err(tail)) => Err(tail)
            }
        }
    };
    ($index:expr; [$foot:expr => $head:expr; $exp:expr], $($tail:tt)*) => {
        {
            match (braid![$index; [$foot => $head; $exp]], braid![$index; $($tail)*]) {
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
