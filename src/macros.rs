#[macro_export]
macro_rules! letter {
    ($foot:expr; +) => {
        $crate::Letter::new($foot, None, $crate::Sign::Positive)
    };
    ($foot:expr; -) => {
        $crate::Letter::new($foot, None, $crate::Sign::Negative)
    };
    ($foot:expr => $head:expr; +) => {
        $crate::Letter::new($foot, Some($head), $crate::Sign::Positive)
    };
    ($foot:expr => $head:expr; -) => {
        $crate::Letter::new($foot, Some($head), $crate::Sign::Negative)
    };
}

#[macro_export]
macro_rules! word {
    () => {
        $crate::Word::trivial()
    };
    ([$foot:expr; $exponent:expr]) => {{
        let letter = if $exponent < 0 {
            letter![$foot; -]
        } else {
            letter![$foot; +]
        };
        match letter {
            Ok(letter) => $crate::Word::from(vec![letter; $exponent.abs().try_into().unwrap()]),
            Err(e) => Err($crate::WordValidationError::from(e)),
        }
    }};
    ([$foot:expr => $head:expr; $exponent:expr]) => {{
        let letter = if $exponent < 0 {
            letter![$foot => $head; -]
        } else {
            letter![$foot => $head; +]
        };
        match letter {
            Ok(letter) => $crate::Word::from(vec![letter; $exponent.abs().try_into().unwrap()]),
            Err(e) => Err($crate::WordValidationError::from(e)),
        }
    }};
    ([$foot:expr; $exponent:expr], $($tail:tt)+) => {{
        match (word![[$foot; $exponent]], word![$($tail)+]) {
            (Ok(w1), Ok(w2)) => w1 * w2,
            (Err(e), _) => Err(e),
            (_, Err(e)) => Err(e),
        }
    }};
    ([$foot:expr => $head:expr; $exponent:expr], $($tail:tt)+) => {{
        match (word![[$foot => $head; $exponent]], word![$($tail)+]) {
            (Ok(w1), Ok(w2)) => w1 * w2,
            (Err(e), _) => Err(e),
            (_, Err(e)) => Err(e),
        }
    }};
}

#[macro_export]
macro_rules! braid {
    ($index:expr; $($tail:tt)*) => {
        Braid::new($index, word![$($tail)*])
    };
}
