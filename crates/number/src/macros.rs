#[macro_export]
macro_rules! n {
    ($n:literal) => {{
        use ::core::str::FromStr;

        $crate::PyretNumber::Exact($crate::BigRational::from_str(stringify!($n)).unwrap())
    }};

    ($n:literal / $d:literal) => {{
        use ::core::str::FromStr;

        $crate::PyretNumber::Exact(
            $crate::BigRational::from_str(&format!("{}/{}", stringify!($n), stringify!($d)))
                .unwrap(),
        )
    }};

    (~ $n:literal) => {
        $crate::PyretNumber::Rough($n as f64)
    };
}
