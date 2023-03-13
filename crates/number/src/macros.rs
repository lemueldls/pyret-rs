#[macro_export]
macro_rules! n {
    ($n:literal) => {{
        use ::std::str::FromStr;

        &$crate::PyretNumber::from_str(stringify!($n)).unwrap()
    }};

    ($n:literal / $d:literal) => {{
        use ::std::str::FromStr;

        &$crate::PyretNumber::from_str(&format!("{}/{}", stringify!($n), stringify!($d))).unwrap()
    }};

    (~ $n:literal) => {
        &$crate::PyretNumber::Rough($n as f64)
    };
}
