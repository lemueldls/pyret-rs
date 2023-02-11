mod ops;

use std::str::FromStr;

use num_bigint::BigInt;
pub use num_rational::BigRational;
pub use num_traits::{One, ToPrimitive, Zero};

#[derive(Debug, PartialEq)]
pub enum PyretNumber {
    Exact(BigRational),
    Rough(f64),
}

impl FromStr for PyretNumber {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if let Some(number) = value.strip_prefix('~') {
            let rough_number = if let Some((numerator, denominator)) = number.split_once('/') {
                let numer = numerator.parse::<f64>().unwrap();
                let denom = denominator.parse::<f64>().unwrap();

                if denom == 0_f64 {
                    return Err(());
                }

                numer / denom
            } else {
                number.parse().unwrap()
            };

            Ok(Self::Rough(rough_number))
        } else {
            let exact_number = if let Some((numerator, denominator)) = value.split_once('/') {
                let numer = BigInt::from_str(numerator).unwrap();
                let denom = BigInt::from_str(denominator).unwrap();

                if denom.is_zero() {
                    return Err(());
                }

                BigRational::new(numer, denom)
            } else {
                let is_negative = value.starts_with('-');

                let mut split = if is_negative { &value[1..] } else { value }
                    .splitn(2, |c| c == 'e' || c == 'E');

                let base = split.next().unwrap();

                let (numerator, denominator) =
                    if let Some((before_decimal_str, after_decimal_str)) = base.split_once('.') {
                        let before_decimal = BigInt::from_str(before_decimal_str).unwrap();

                        let tens = String::from('1') + &"0".repeat(after_decimal_str.len());
                        let denominator = BigInt::from_str(&tens).unwrap();

                        let after_decimal = BigInt::from_str(after_decimal_str).unwrap();

                        let numerator = before_decimal * &denominator + after_decimal;

                        (numerator, denominator)
                    } else {
                        (BigInt::from_str(base).unwrap(), BigInt::one())
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
                            .unwrap(),
                        );

                    let exponent = BigInt::from_str(&expanded).unwrap();

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
