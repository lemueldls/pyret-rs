use num_rational::BigRational;
use num_traits::{One, Signed, ToPrimitive, Zero};

use crate::PyretNumber;

impl ToPrimitive for PyretNumber {
    fn to_i64(&self) -> Option<i64> {
        match self {
            Self::Exact(exact) => exact.to_i64(),
            Self::Rough(rough) => rough.to_i64(),
        }
    }

    fn to_u64(&self) -> Option<u64> {
        match self {
            Self::Exact(exact) => exact.to_u64(),
            Self::Rough(rough) => rough.to_u64(),
        }
    }

    fn to_f64(&self) -> Option<f64> {
        match self {
            Self::Exact(exact) => exact.to_f64(),
            Self::Rough(rough) => Some(*rough),
        }
    }
}

impl Zero for PyretNumber {
    fn zero() -> Self {
        Self::Exact(BigRational::zero())
    }

    fn is_zero(&self) -> bool {
        match self {
            Self::Exact(exact) => exact.is_zero(),
            Self::Rough(rough) => rough.is_zero(),
        }
    }
}

impl One for PyretNumber {
    fn one() -> Self {
        Self::Exact(BigRational::one())
    }
}

impl Signed for PyretNumber {
    fn abs(&self) -> Self {
        match self {
            Self::Exact(exact) => Self::Exact(exact.abs()),
            Self::Rough(rough) => Self::Rough(rough.abs()),
        }
    }

    fn abs_sub(&self, other: &Self) -> Self {
        (self - other).abs()
    }

    fn signum(&self) -> Self {
        match self {
            Self::Exact(exact) => Self::Exact(exact.signum()),
            Self::Rough(rough) => Self::Rough(rough.signum()),
        }
    }

    fn is_positive(&self) -> bool {
        match self {
            Self::Exact(exact) => exact.is_positive(),
            Self::Rough(rough) => rough.is_positive(),
        }
    }

    fn is_negative(&self) -> bool {
        match self {
            Self::Exact(exact) => exact.is_negative(),
            Self::Rough(rough) => rough.is_negative(),
        }
    }
}
