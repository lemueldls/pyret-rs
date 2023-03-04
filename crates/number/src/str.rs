use std::{fmt::Display, str::FromStr};

use num_bigint::{BigInt, ParseBigIntError};
use num_rational::BigRational;
use num_traits::{Num, One, ParseFloatError, Zero};

use crate::PyretNumber;

impl Display for PyretNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Exact(exact) => write!(f, "{exact}"),
            Self::Rough(rough) => {
                let abs = rough.abs();

                if abs > 99_999_999_999_999_999_999.0 {
                    write!(f, "~{}", format!("{rough:e}").replacen('e', "e+", 1))
                } else if abs < 0.000_001 {
                    write!(f, "~{rough:e}")
                } else {
                    write!(f, "~{rough}")
                }
            }
        }
    }
}

pub enum PyretNumberParseError {
    InvalidNumber,
    DivideByZero,
    ParseFloat(ParseFloatError),
    ParseBigInt(ParseBigIntError),
}

impl FromStr for PyretNumber {
    type Err = PyretNumberParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::from_str_radix(value, 10)
    }
}

impl Num for PyretNumber {
    type FromStrRadixErr = PyretNumberParseError;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        if let Some(number) = str.strip_prefix('~') {
            let rough_number = if let Some((numerator, denominator)) = number.split_once('/') {
                let numerator = f64::from_str_radix(numerator, radix)
                    .map_err(PyretNumberParseError::ParseFloat)?;
                let denominator = f64::from_str_radix(denominator, radix)
                    .map_err(PyretNumberParseError::ParseFloat)?;

                if denominator == 0.0 {
                    return Err(PyretNumberParseError::DivideByZero);
                }

                numerator / denominator
            } else {
                f64::from_str_radix(number, radix).map_err(PyretNumberParseError::ParseFloat)?
            };

            Ok(Self::Rough(rough_number))
        } else {
            let exact_number = if let Some((numerator, denominator)) = str.split_once('/') {
                let numer = BigInt::from_str_radix(numerator, radix)
                    .map_err(PyretNumberParseError::ParseBigInt)?;
                let denom = BigInt::from_str_radix(denominator, radix)
                    .map_err(PyretNumberParseError::ParseBigInt)?;

                if denom.is_zero() {
                    return Err(PyretNumberParseError::DivideByZero);
                }

                BigRational::new(numer, denom)
            } else {
                let is_negative = str.starts_with('-');

                let mut split =
                    if is_negative { &str[1..] } else { str }.splitn(2, |c| c == 'e' || c == 'E');

                let base = split.next().ok_or(PyretNumberParseError::InvalidNumber)?;

                let (numerator, denominator) =
                    if let Some((before_decimal_str, after_decimal_str)) = base.split_once('.') {
                        let before_decimal = BigInt::from_str_radix(before_decimal_str, radix)
                            .map_err(PyretNumberParseError::ParseBigInt)?;

                        let tens = String::from('1') + &"0".repeat(after_decimal_str.len());
                        let denominator = BigInt::from_str_radix(&tens, radix)
                            .map_err(PyretNumberParseError::ParseBigInt)?;

                        let after_decimal = BigInt::from_str_radix(after_decimal_str, radix)
                            .map_err(PyretNumberParseError::ParseBigInt)?;

                        let numerator = before_decimal * &denominator + after_decimal;

                        (numerator, denominator)
                    } else {
                        (
                            BigInt::from_str_radix(base, radix)
                                .map_err(PyretNumberParseError::ParseBigInt)?,
                            BigInt::one(),
                        )
                    };

                let (numerator, denominator) = if let Some(exp) = split.next() {
                    let is_exponent_negative = exp.starts_with('-');

                    let expanded = String::from('1')
                        + &"0".repeat(
                            if is_exponent_negative || exp.starts_with('+') {
                                &exp[1..]
                            } else {
                                exp
                            }
                            .parse()
                            .map_err(|_| PyretNumberParseError::InvalidNumber)?,
                        );

                    let exponent = BigInt::from_str_radix(&expanded, radix)
                        .map_err(PyretNumberParseError::ParseBigInt)?;

                    if exponent.is_one() {
                        (numerator, denominator)
                    } else if is_exponent_negative {
                        (numerator, denominator * exponent)
                    } else {
                        (numerator * exponent, denominator)
                    }
                } else {
                    (numerator, denominator)
                };

                BigRational::new(
                    if is_negative { -numerator } else { numerator },
                    denominator,
                )
            };

            Ok(Self::Exact(exact_number))
        }
    }
}
