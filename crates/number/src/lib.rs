pub mod macros;
pub mod math;
pub mod num;
pub mod ops;
pub mod str;

use std::{cmp::Ordering, result};

pub use num_bigint::{BigInt, Sign};
pub use num_rational::BigRational;
use num_traits::FromPrimitive;
pub use num_traits::{One, Signed, ToPrimitive, Zero};

type Result<T> = result::Result<T, Box<str>>;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PyretNumber {
    Exact(BigRational),
    Rough(f64),
}

impl PyretNumber {
    #[must_use]
    pub fn to_exact(&self) -> Self {
        match self {
            exact @ Self::Exact(..) => exact.clone(),
            Self::Rough(number) => Self::Exact(BigRational::from_f64(*number).unwrap()),
        }
    }

    pub fn to_rough(&self) -> Result<Self> {
        Ok(match self {
            Self::Exact(number) => Self::Rough(number.to_f64().ok_or("roughnum overflow")?),
            rough @ Self::Rough(..) => rough.clone(),
        })
    }

    #[must_use]
    pub fn to_f64(&self) -> Option<f64> {
        match self {
            Self::Exact(exact) => exact.to_f64(),
            Self::Rough(rough) => Some(*rough),
        }
    }

    #[must_use]
    pub fn is_integer(&self) -> bool {
        match self {
            Self::Exact(exact) => exact.is_integer(),
            Self::Rough(..) => false,
        }
    }

    #[must_use]
    pub const fn is_rational(&self) -> bool {
        match self {
            Self::Exact(..) => true,
            Self::Rough(..) => false,
        }
    }

    pub fn is_equal(&self, other: &Self) -> Result<bool> {
        match (self, other) {
            (Self::Exact(exact), Self::Exact(other_exact)) => Ok(exact == other_exact),
            _ => Err(Box::from("roughnums cannot be compared for equality")),
        }
    }

    pub fn min<'a>(&'a self, other: &'a Self) -> Result<&'a Self> {
        match self.partial_cmp(other) {
            Some(Ordering::Less | Ordering::Equal) => Ok(self),
            Some(Ordering::Greater) => Ok(other),
            None => Err(Box::from("roughnum overflow")),
        }
    }

    pub fn max<'a>(&'a self, other: &'a Self) -> Result<&'a Self> {
        match self.partial_cmp(other) {
            Some(Ordering::Less | Ordering::Equal) => Ok(other),
            Some(Ordering::Greater) => Ok(self),
            None => Err(Box::from("roughnum overflow")),
        }
    }

    #[must_use]
    pub fn is_within_abs(&self, other: &Self, tol: &Self) -> bool {
        (self.to_exact() - other.to_exact()).abs() <= tol.to_exact()
    }

    #[must_use]
    pub fn is_within_rel(&self, other: &Self, tol: &Self) -> bool {
        let diff = (self.to_exact() - other.to_exact()).abs();

        let self_abs = self.to_exact().abs();
        let other_abs = other.to_exact().abs();

        let max = self_abs.max(&other_abs).unwrap();

        diff <= max * &tol.to_exact()
    }

    pub fn is(&self, other: &Self) -> Result<bool> {
        match (self, other) {
            (Self::Exact(exact), Self::Exact(other_exact)) => Ok(exact == other_exact),
            _ => Err(Box::from("roughnums cannot be compared for equality")),
        }
    }

    #[must_use]
    pub fn is_roughly(&self, other: &Self) -> bool {
        let tol = &Self::Exact(BigRational::from_float(0.000_001).unwrap());

        self.is_within_rel(other, tol)
    }
}
