pub use num_bigint::BigInt;
pub use num_rational::BigRational;
use num_traits::{CheckedMul, FromPrimitive};
pub use num_traits::{One, Signed, ToPrimitive, Zero};

use crate::{PyretNumber, Result};

impl PyretNumber {
    pub fn sin(&self) -> Result<Self> {
        Ok(match self {
            Self::Exact(exact) => {
                if exact.is_zero() {
                    Self::Exact(BigRational::zero())
                } else {
                    Self::Rough(exact.to_f64().ok_or("roughnum overflow")?.sin())
                }
            }
            Self::Rough(rough) => Self::Rough(rough.sin()),
        })
    }

    pub fn cos(&self) -> Result<Self> {
        Ok(match self {
            Self::Exact(exact) => {
                if exact.is_zero() {
                    Self::Exact(BigRational::one())
                } else {
                    Self::Rough(exact.to_f64().ok_or("roughnum overflow")?.cos())
                }
            }
            Self::Rough(rough) => Self::Rough(rough.cos()),
        })
    }

    pub fn tan(&self) -> Result<Self> {
        Ok(match self {
            Self::Exact(exact) => {
                if exact.is_zero() {
                    Self::Exact(BigRational::zero())
                } else {
                    Self::Rough(exact.to_f64().ok_or("roughnum overflow")?.tan())
                }
            }
            Self::Rough(rough) => Self::Rough(rough.tan()),
        })
    }

    pub fn asin(&self) -> Result<Self> {
        Ok(match self {
            Self::Exact(exact) => {
                if exact.is_zero() {
                    Self::Exact(BigRational::zero())
                } else {
                    Self::Rough(exact.to_f64().ok_or("roughnum overflow")?.asin())
                }
            }
            Self::Rough(rough) => Self::Rough(rough.asin()),
        })
    }

    pub fn acos(&self) -> Result<Self> {
        Ok(match self {
            Self::Exact(exact) => {
                if exact.is_one() {
                    Self::Exact(BigRational::zero())
                } else {
                    Self::Rough(exact.to_f64().ok_or("roughnum overflow")?.acos())
                }
            }
            Self::Rough(rough) => Self::Rough(rough.acos()),
        })
    }

    pub fn atan(&self) -> Result<Self> {
        Ok(match self {
            Self::Exact(exact) => {
                if exact.is_zero() {
                    Self::Exact(BigRational::zero())
                } else {
                    Self::Rough(exact.to_f64().ok_or("roughnum overflow")?.atan())
                }
            }
            Self::Rough(rough) => Self::Rough(rough.atan()),
        })
    }

    pub fn atan2(&self, other: &Self) -> Result<Self> {
        let exact_zero = Self::Exact(BigRational::zero());

        if self == &exact_zero {
            Ok(exact_zero)
        } else {
            match (self.to_f64(), other.to_f64()) {
                (Some(self_rough), Some(other_rough)) => {
                    Ok(Self::Rough(self_rough.atan2(other_rough)))
                }
                _ => Err("roughnum overflow"),
            }
        }
    }

    pub fn modulo(&self, other: &Self) -> Result<Self> {
        match (self, other) {
            (Self::Exact(dividend), Self::Exact(divisor)) => {
                if divisor.is_zero() {
                    Err("the second argument is zero")
                } else {
                    Ok(Self::Exact(dividend % divisor))
                }
            }
            _ => Err("roughnums cannot be moduloed"),
        }
    }

    pub fn truncate(&self) -> Result<Self> {
        match self {
            Self::Exact(exact) => Ok(Self::Exact(exact.trunc())),
            Self::Rough(rough) => Ok(Self::Rough(rough.trunc())),
        }
    }

    pub fn sqrt(&self) -> Result<Self> {
        if self.is_negative() {
            Err("negative argument")
        } else {
            Ok(match self {
                Self::Exact(exact) => {
                    let sqrt = exact.to_f64().ok_or("roughnum overflow")?.sqrt();

                    if sqrt.fract() == 0.0 {
                        Self::Exact(BigRational::from_f64(sqrt).unwrap())
                    } else {
                        Self::Rough(sqrt)
                    }
                }
                Self::Rough(rough) => Self::Rough(rough.sqrt()),
            })
        }
    }

    pub fn sqr(&self) -> Result<Self> {
        self.checked_mul(self).ok_or("roughnum overflow")
    }

    pub fn ceiling(&self) -> Result<Self> {
        Ok(match self {
            Self::Exact(exact) => Self::Exact(exact.ceil()),
            Self::Rough(rough) => Self::Rough(rough.ceil()),
        })
    }

    pub fn floor(&self) -> Result<Self> {
        Ok(match self {
            Self::Exact(exact) => Self::Exact(exact.floor()),
            Self::Rough(rough) => Self::Rough(rough.floor()),
        })
    }

    pub fn round(&self) -> Result<Self> {
        Ok(match self {
            Self::Exact(exact) => Self::Exact(exact.round()),
            Self::Rough(rough) => Self::Rough(rough.round()),
        })
    }

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

    pub fn log(&self) -> Result<Self> {
        if self.is_non_positive() {
            Err("non-positive argument")
        } else {
            Ok(match self {
                Self::Exact(exact) => {
                    if exact.is_one() {
                        Self::Exact(BigRational::zero())
                    } else {
                        Self::Rough(exact.to_f64().ok_or("roughnum overflow")?.ln())
                    }
                }
                Self::Rough(rough) => Self::Rough(rough.ln()),
            })
        }
    }

    pub fn exp(&self) -> Result<Self> {
        Ok(match self {
            Self::Exact(exact) => {
                if exact.is_zero() {
                    Self::Exact(BigRational::one())
                } else {
                    Self::Rough(exact.to_f64().ok_or("roughnum overflow")?.exp())
                }
            }
            Self::Rough(rough) => Self::Rough(rough.exp()),
        })
    }

    /// # Examples
    ///
    /// ```
    /// # use pyret_number::n;
    /// assert_eq!(n!(2).expt(&n!(~0)), Ok(n!(~1)));
    /// assert_eq!(n!(3).expt(&n!(0)), Ok(n!(1)));
    /// assert_eq!(n!(1).expt(&n!(3)), Ok(n!(1)));
    /// assert_eq!(n!(0).expt(&n!(0)), Ok(n!(1)));
    /// assert_eq!(n!(0).expt(&n!(3)), Ok(n!(0)));
    /// assert_eq!(n!(0).expt(&n!(-3)), Err("division by zero"));
    /// assert_eq!(n!(2).expt(&n!(3)), Ok(n!(8)));
    /// assert_eq!(n!(2).expt(&n!(-3)), Ok(n!(1/8)));
    /// ```
    pub fn expt(&self, exponent: &Self) -> Result<Self> {
        if self.is_zero() && exponent.is_negative() {
            Err("division by zero")
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
                        let result = base.pow(exponent.to_i32().ok_or("roughnum overflow")?);

                        if exponent.is_integer() {
                            Self::Exact(result)
                        } else {
                            Self::Rough(result.to_f64().ok_or("roughnum overflow")?)
                        }
                    }
                    Self::Rough(base) => {
                        Self::Rough(base.powf(exponent.to_f64().ok_or("roughnum overflow")?))
                    }
                }
            })
        }
    }
}
