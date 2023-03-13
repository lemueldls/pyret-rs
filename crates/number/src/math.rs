use std::f64::consts::PI;

pub use num_bigint::BigInt;
pub use num_rational::BigRational;
use num_traits::{CheckedAdd, CheckedMul, FromPrimitive};
pub use num_traits::{One, Signed, ToPrimitive, Zero};

use crate::{PyretNumber, Result};

impl PyretNumber {
    /// # Examples
    ///
    /// ```
    /// # use pyret_number::n;
    /// assert!(n!(0).sin().unwrap().is(n!(0)).unwrap());
    /// assert!(n!(1).sin().unwrap().is_within_abs(n!(0.84), n!(0.01)));
    /// ```
    pub fn sin(&self) -> Result<Self> {
        Ok(match self {
            Self::Exact(exact) => {
                if exact.is_zero() {
                    Self::Exact(BigRational::zero())
                } else {
                    Self::Rough(
                        exact
                            .to_f64()
                            .ok_or_else(|| Box::from("roughnum overflow"))?
                            .sin(),
                    )
                }
            }
            Self::Rough(rough) => Self::Rough(rough.sin()),
        })
    }

    /// # Examples
    ///
    /// ```
    /// # use pyret_number::n;
    /// assert!(n!(0).cos().unwrap().is(n!(1)).unwrap());
    /// assert!(n!(1).cos().unwrap().is_within_abs(n!(0.54), n!(0.01)));
    /// ```
    pub fn cos(&self) -> Result<Self> {
        Ok(match self {
            Self::Exact(exact) => {
                if exact.is_zero() {
                    Self::Exact(BigRational::one())
                } else {
                    Self::Rough(
                        exact
                            .to_f64()
                            .ok_or_else(|| Box::from("roughnum overflow"))?
                            .cos(),
                    )
                }
            }
            Self::Rough(rough) => Self::Rough(rough.cos()),
        })
    }

    /// # Examples
    ///
    /// ```
    /// # use pyret_number::n;
    /// assert!(n!(0).tan().unwrap().is(n!(0)).unwrap());
    /// assert!(n!(1).tan().unwrap().is_within_abs(n!(1.56), n!(0.01)));
    /// ```
    pub fn tan(&self) -> Result<Self> {
        Ok(match self {
            Self::Exact(exact) => {
                if exact.is_zero() {
                    Self::Exact(BigRational::zero())
                } else {
                    Self::Rough(
                        exact
                            .to_f64()
                            .ok_or_else(|| Box::from("roughnum overflow"))?
                            .tan(),
                    )
                }
            }
            Self::Rough(rough) => Self::Rough(rough.tan()),
        })
    }

    /// # Examples
    ///
    /// ```
    /// # use pyret_number::n;
    /// assert!(n!(0).asin().unwrap().is(n!(0)).unwrap());
    /// assert!(n!(0.84).asin().unwrap().is_within_abs(n!(1), n!(0.01)));
    /// ```
    pub fn asin(&self) -> Result<Self> {
        Ok(match self {
            Self::Exact(exact) => {
                if exact.is_zero() {
                    Self::Exact(BigRational::zero())
                } else {
                    Self::Rough(
                        exact
                            .to_f64()
                            .ok_or_else(|| Box::from("roughnum overflow"))?
                            .asin(),
                    )
                }
            }
            Self::Rough(rough) => Self::Rough(rough.asin()),
        })
    }

    /// # Examples
    ///
    /// ```
    /// # use pyret_number::n;
    /// assert!(n!(1).acos().unwrap().is(n!(0)).unwrap());
    /// assert!(n!(0.54).acos().unwrap().is_within_abs(n!(1), n!(0.01)));
    /// ```
    pub fn acos(&self) -> Result<Self> {
        Ok(match self {
            Self::Exact(exact) => {
                if exact.is_one() {
                    Self::Exact(BigRational::zero())
                } else {
                    Self::Rough(
                        exact
                            .to_f64()
                            .ok_or_else(|| Box::from("roughnum overflow"))?
                            .acos(),
                    )
                }
            }
            Self::Rough(rough) => Self::Rough(rough.acos()),
        })
    }

    /// # Examples
    ///
    /// ```
    /// # use pyret_number::n;
    /// assert!(n!(0).atan().unwrap().is(n!(0)).unwrap());
    /// assert!(
    ///     n!(1)
    ///         .atan()
    ///         .unwrap()
    ///         .is_roughly(&(n!(3.141592) * n!(1 / 4)))
    /// );
    /// assert!(
    ///     n!(-1)
    ///         .atan()
    ///         .unwrap()
    ///         .is_roughly(&(n!(-3.141592) * n!(1 / 4)))
    /// );
    /// assert!(
    ///     n!(100000000000)
    ///         .atan()
    ///         .unwrap()
    ///         .is_roughly(&(n!(3.141592) / n!(2)))
    /// );
    /// assert!(
    ///     n!(-100000000000)
    ///         .atan()
    ///         .unwrap()
    ///         .is_roughly(&(n!(-3.141592) / n!(2)))
    /// );
    /// ```
    pub fn atan(&self) -> Result<Self> {
        Ok(match self {
            Self::Exact(exact) => {
                if exact.is_zero() {
                    Self::Exact(BigRational::zero())
                } else {
                    Self::Rough(
                        exact
                            .to_f64()
                            .ok_or_else(|| Box::from("roughnum overflow"))?
                            .atan(),
                    )
                }
            }
            Self::Rough(rough) => Self::Rough(rough.atan()),
        })
    }

    /// # Examples
    ///
    /// ```
    /// # use pyret_number::n;
    /// assert!(n!(0).atan2(n!(1)).unwrap().is(n!(0)).unwrap());
    /// assert!(
    ///     n!(1)
    ///         .atan2(n!(1))
    ///         .unwrap()
    ///         .is_roughly(&(n!(3.141592) * n!(1 / 4)))
    /// );
    /// assert!(
    ///     n!(1)
    ///         .atan2(n!(-1))
    ///         .unwrap()
    ///         .is_roughly(&(n!(3.141592) * n!(3 / 4)))
    /// );
    /// assert!(
    ///     n!(-1)
    ///         .atan2(n!(-1))
    ///         .unwrap()
    ///         .is_roughly(&(n!(3.141592) * n!(5 / 4)))
    /// );
    /// assert!(
    ///     n!(-1)
    ///         .atan2(n!(1))
    ///         .unwrap()
    ///         .is_roughly(&(n!(3.141592) * n!(7 / 4)))
    /// );
    /// assert!(
    ///     n!(1)
    ///         .atan2(n!(0))
    ///         .unwrap()
    ///         .is_roughly(&(n!(3.141592) * n!(1 / 2)))
    /// );
    /// assert!(
    ///     n!(-1)
    ///         .atan2(n!(0))
    ///         .unwrap()
    ///         .is_roughly(&(n!(3.141592) * n!(3 / 2)))
    /// );
    /// ```
    pub fn atan2(&self, other: &Self) -> Result<Self> {
        let exact_zero = Self::Exact(BigRational::zero());

        if self == &exact_zero {
            Ok(exact_zero)
        } else {
            match (self.to_f64(), other.to_f64()) {
                (Some(self_rough), Some(other_rough)) => {
                    let atan2 = self_rough.atan2(other_rough);
                    Ok(if atan2 < 0.0 {
                        Self::Rough(atan2)
                            .checked_add(&Self::Rough(2.0 * PI))
                            .ok_or_else(|| Box::from("roughnum overflow"))?
                    } else {
                        Self::Rough(atan2)
                    })
                }
                _ => Err(Box::from("roughnum overflow")),
            }
        }
    }

    /// # Examples
    ///
    /// ```
    /// # use pyret_number::n;
    /// assert!(n!(5).modulo(n!(2)).unwrap().is(n!(1)).unwrap());
    /// assert!(n!(-5).modulo(n!(2)).unwrap().is(n!(1)).unwrap());
    /// assert!(n!(-5).modulo(n!(-2)).unwrap().is(n!(-1)).unwrap());
    /// assert!(n!(7).modulo(n!(3)).unwrap().is(n!(1)).unwrap());
    /// assert!(n!(0).modulo(n!(2)).unwrap().is(n!(0)).unwrap());
    /// assert!(n!(-7).modulo(n!(3)).unwrap().is(n!(2)).unwrap());
    /// ```
    pub fn modulo(&self, other: &Self) -> Result<Self> {
        match (self, other) {
            (Self::Exact(dividend), Self::Exact(divisor)) => {
                if divisor.is_zero() {
                    Err(Box::from("the second argument is zero"))
                } else {
                    let exact_zero = BigRational::zero();
                    let result = dividend % divisor;

                    Ok(Self::Exact(if divisor < &exact_zero {
                        if result > exact_zero {
                            result + divisor
                        } else {
                            result
                        }
                    } else if result < exact_zero {
                        result + divisor
                    } else {
                        result
                    }))
                }
            }
            _ => Err(Box::from("roughnums cannot be moduloed")),
        }
    }

    /// # Examples
    ///
    /// ```
    /// # use pyret_number::n;
    /// assert!(n!(3.14).truncate().unwrap().is(n!(3)).unwrap());
    /// assert!(n!(-3.14).truncate().unwrap().is(n!(-3)).unwrap());
    /// assert!(n!(~3.14).truncate().unwrap().is_roughly(n!(~3)));
    /// assert!(n!(~-3.14).truncate().unwrap().is_roughly(n!(~-3)));
    /// ```
    pub fn truncate(&self) -> Result<Self> {
        match self {
            Self::Exact(exact) => Ok(Self::Exact(exact.trunc())),
            Self::Rough(rough) => Ok(Self::Rough(rough.trunc())),
        }
    }

    /// # Examples
    ///
    /// ```
    /// # use pyret_number::n;
    /// assert!(n!(4).sqrt().unwrap().is(n!(2)).unwrap());
    /// assert!(n!(5).sqrt().unwrap().is_within_abs(n!(~2.236), n!(0.001)));
    /// assert!(n!(~4).sqrt().unwrap().is_within_abs(n!(~2), n!(0.001)));
    /// assert!(n!(~5).sqrt().unwrap().is_within_abs(n!(~2.236), n!(0.001)));
    /// assert!(n!(0.04).sqrt().unwrap().is(n!(1/5)).unwrap());
    /// assert_eq!(n!(-1).sqrt(), Err(Box::from("negative argument")));
    /// ```
    pub fn sqrt(&self) -> Result<Self> {
        if self.is_negative() {
            Err(Box::from("negative argument"))
        } else {
            Ok(match self {
                Self::Exact(exact) => {
                    let numer = exact
                        .numer()
                        .to_f64()
                        .ok_or_else(|| Box::from("roughnum overflow"))?
                        .sqrt();
                    let denom = exact
                        .denom()
                        .to_f64()
                        .ok_or_else(|| Box::from("roughnum overflow"))?
                        .sqrt();

                    if numer.fract() == 0.0 && denom.fract() == 0.0 {
                        Self::Exact(BigRational::new(
                            BigInt::from_f64(numer).unwrap(),
                            BigInt::from_f64(denom).unwrap(),
                        ))
                    } else {
                        Self::Rough(numer / denom)
                    }
                }
                Self::Rough(rough) => Self::Rough(rough.sqrt()),
            })
        }
    }

    /// # Examples
    ///
    /// ```
    /// # use pyret_number::n;
    /// assert!(n!(4).sqr().unwrap().is(n!(16)).unwrap());
    /// assert!(n!(5).sqr().unwrap().is(n!(25)).unwrap());
    /// assert!(n!(-4).sqr().unwrap().is(n!(16)).unwrap());
    /// assert!(n!(~4).sqr().unwrap().is_roughly(n!(~16)));
    /// assert!(n!(0.04).sqr().unwrap().is(n!(1/625)).unwrap());
    /// ```
    pub fn sqr(&self) -> Result<Self> {
        self.checked_mul(self)
            .ok_or_else(|| Box::from("roughnum overflow"))
    }

    /// # Examples
    ///
    /// ```
    /// # use pyret_number::n;
    /// assert!(n!(4.2).ceiling().unwrap().is(n!(5)).unwrap());
    /// assert!(n!(-4.2).ceiling().unwrap().is(n!(-4)).unwrap());
    /// ```
    pub fn ceiling(&self) -> Result<Self> {
        Ok(match self {
            Self::Exact(exact) => Self::Exact(exact.ceil()),
            Self::Rough(rough) => Self::Rough(rough.ceil()),
        })
    }

    /// # Examples
    ///
    /// ```
    /// # use pyret_number::n;
    /// assert!(n!(4.2).floor().unwrap().is(n!(4)).unwrap());
    /// assert!(n!(-4.2).floor().unwrap().is(n!(-5)).unwrap());
    /// ```
    pub fn floor(&self) -> Result<Self> {
        Ok(match self {
            Self::Exact(exact) => Self::Exact(exact.floor()),
            Self::Rough(rough) => Self::Rough(rough.floor()),
        })
    }

    /// # Examples
    ///
    /// ```
    /// # use pyret_number::n;
    /// assert!(n!(4.2).round().unwrap().is(n!(4)).unwrap());
    /// assert!(n!(4.8).round().unwrap().is(n!(5)).unwrap());
    /// assert!(n!(-4.2).round().unwrap().is(n!(-4)).unwrap());
    /// assert!(n!(-4.8).round().unwrap().is(n!(-5)).unwrap());
    /// assert!(n!(3.5).round().unwrap().is(n!(4)).unwrap());
    /// assert!(n!(2.5).round().unwrap().is(n!(3)).unwrap());
    /// ```
    pub fn round(&self) -> Result<Self> {
        Ok(match self {
            Self::Exact(exact) => Self::Exact(exact.round()),
            Self::Rough(rough) => Self::Rough(rough.round()),
        })
    }

    /// # Examples
    ///
    /// ```
    /// # use pyret_number::n;
    /// assert!(n!(3.5).round_even().unwrap().is(n!(4)).unwrap());
    /// assert!(n!(2.5).round_even().unwrap().is(n!(2)).unwrap());
    /// ```
    pub fn round_even(&self) -> Result<Self> {
        Ok(match self {
            Self::Exact(exact) => {
                let big_two = &BigRational::from_u8(2).unwrap();

                Self::Exact(big_two * (exact / big_two).round())
            }
            Self::Rough(rough) => Self::Rough(2.0 * (rough / 2.0).round()),
        })
    }

    #[must_use]
    pub fn is_non_positive(&self) -> bool {
        self.is_negative() || self.is_zero()
    }

    #[must_use]
    pub fn is_non_negative(&self) -> bool {
        self.is_positive() || self.is_zero()
    }

    /// # Examples
    ///
    /// ```
    /// # use pyret_number::n;
    /// assert!(n!(1).log().unwrap().is(n!(0)).unwrap());
    /// assert_eq!(n!(0).log(), Err(Box::from("non-positive argument")));
    /// assert_eq!(n!(-1).log(), Err(Box::from("non-positive argument")));
    /// assert!(
    ///     n!(2.718281828)
    ///         .log()
    ///         .unwrap()
    ///         .is_within_abs(n!(1), n!(0.01))
    /// );
    /// assert!(n!(10).log().unwrap().is_within_abs(n!(2.3), n!(0.1)));
    /// ```
    pub fn log(&self) -> Result<Self> {
        if self.is_non_positive() {
            Err(Box::from("non-positive argument"))
        } else {
            Ok(match self {
                Self::Exact(exact) => {
                    if exact.is_one() {
                        Self::Exact(BigRational::zero())
                    } else {
                        Self::Rough(
                            exact
                                .to_f64()
                                .ok_or_else(|| Box::from("roughnum overflow"))?
                                .ln(),
                        )
                    }
                }
                Self::Rough(rough) => Self::Rough(rough.ln()),
            })
        }
    }

    /// # Examples
    ///
    /// ```
    /// # use pyret_number::n;
    /// assert!(
    ///     n!(-1)
    ///         .exp()
    ///         .unwrap()
    ///         .is_within_abs(&(n!(1) / &n!(1).exp().unwrap()), n!(0.0001))
    /// );
    /// assert!(n!(0).exp().unwrap().is(n!(1)).unwrap());
    /// assert!(
    ///     n!(1)
    ///         .exp()
    ///         .unwrap()
    ///         .is_within_abs(n!(2.718281828), n!(0.0001))
    /// );
    /// assert!(
    ///     n!(3)
    ///         .exp()
    ///         .unwrap()
    ///         .is_within_abs(&n!(2.718281828).expt(n!(3)).unwrap(), n!(0.0001))
    /// );
    /// assert_eq!(
    ///     n!(710).exp(),
    ///     Err(Box::from("exp: argument too large: 710"))
    /// );
    /// ```
    pub fn exp(&self) -> Result<Self> {
        match self {
            Self::Exact(exact) => {
                if exact.is_zero() {
                    Ok(Self::Exact(BigRational::one()))
                } else {
                    let exp = exact
                        .to_f64()
                        .ok_or_else(|| Box::from("roughnum overflow"))?
                        .exp();

                    if exp.is_infinite() {
                        Err(format!("exp: argument too large: {self}").into_boxed_str())
                    } else {
                        Ok(Self::Rough(exp))
                    }
                }
            }
            Self::Rough(rough) => Ok(Self::Rough(rough.exp())),
        }
    }

    /// # Examples
    ///
    /// ```
    /// # use pyret_number::n;
    /// assert!(n!(3).expt(n!(0)).unwrap().is(n!(1)).unwrap());
    /// assert!(n!(1).expt(n!(3)).unwrap().is(n!(1)).unwrap());
    /// assert!(n!(0).expt(n!(0)).unwrap().is(n!(1)).unwrap());
    /// assert!(n!(0).expt(n!(3)).unwrap().is(n!(0)).unwrap());
    /// assert_eq!(n!(0).expt(n!(-3)), Err(Box::from("division by zero")));
    /// assert!(n!(2).expt(n!(3)).unwrap().is(n!(8)).unwrap());
    /// assert!(n!(2).expt(n!(-3)).unwrap().is(n!(1 / 8)).unwrap());
    /// ```
    pub fn expt(&self, exponent: &Self) -> Result<Self> {
        if self.is_zero() && exponent.is_negative() {
            Err(Box::from("division by zero"))
        } else {
            let exact_zero = Self::Exact(BigRational::zero());
            let exact_one = Self::Exact(BigRational::one());

            Ok(if self == &exact_one || exponent == &exact_zero {
                exact_one
            } else if self == &exact_zero {
                exact_zero
            } else {
                match self {
                    Self::Exact(base) => {
                        let result = base.pow(
                            exponent
                                .to_i32()
                                .ok_or_else(|| Box::from("roughnum overflow"))?,
                        );

                        if exponent.is_integer() {
                            Self::Exact(result)
                        } else {
                            Self::Rough(
                                result
                                    .to_f64()
                                    .ok_or_else(|| Box::from("roughnum overflow"))?,
                            )
                        }
                    }
                    Self::Rough(base) => Self::Rough(
                        base.powf(
                            exponent
                                .to_f64()
                                .ok_or_else(|| Box::from("roughnum overflow"))?,
                        ),
                    ),
                }
            })
        }
    }
}
