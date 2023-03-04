pub mod macros;
pub mod math;
pub mod num;
pub mod ops;
pub mod str;

use std::cmp::Ordering;

pub use num_bigint::BigInt;
pub use num_rational::BigRational;
use num_traits::FromPrimitive;
pub use num_traits::{One, Signed, ToPrimitive, Zero};

type Result<T> = std::result::Result<T, &'static str>;

#[derive(Debug, Clone, PartialEq)]
pub enum PyretNumber {
    Exact(BigRational),
    Rough(f64),
}

impl PyretNumber {
    pub fn to_exact(&self) -> Result<Self> {
        Ok(match self {
            exact @ Self::Exact(..) => exact.clone(),
            Self::Rough(number) => Self::Exact(BigRational::from_f64(*number).unwrap()),
        })
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
            _ => Err("roughnums cannot be compared for equality"),
        }
    }

    pub fn min<'a>(&'a self, other: &'a Self) -> Result<&'a Self> {
        match self.partial_cmp(other) {
            Some(Ordering::Less | Ordering::Equal) => Ok(self),
            Some(Ordering::Greater) => Ok(other),
            None => Err("roughnum overflow"),
        }
    }

    pub fn max<'a>(&'a self, other: &'a Self) -> Result<&'a Self> {
        match self.partial_cmp(other) {
            Some(Ordering::Less | Ordering::Equal) => Ok(other),
            Some(Ordering::Greater) => Ok(self),
            None => Err("roughnum overflow"),
        }
    }
}
