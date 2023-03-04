use std::{
    cmp::Ordering,
    ops::{Add, Div, Mul, Neg, Rem, Sub},
};

use num_traits::{CheckedAdd, CheckedDiv, CheckedMul, CheckedRem, CheckedSub, ToPrimitive};

use crate::PyretNumber;

impl Add for PyretNumber {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Exact(left_exact), Self::Exact(right_exact)) => {
                Self::Exact(left_exact + right_exact)
            }
            (Self::Exact(left_exact), Self::Rough(right_rough)) => {
                Self::Rough(left_exact.to_f64().unwrap() + right_rough)
            }
            (Self::Rough(left_rough), Self::Exact(right_exact)) => {
                Self::Rough(left_rough + right_exact.to_f64().unwrap())
            }
            (Self::Rough(left_rough), Self::Rough(right_rough)) => {
                Self::Rough(left_rough + right_rough)
            }
        }
    }
}

impl Add for &PyretNumber {
    type Output = PyretNumber;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (PyretNumber::Exact(left_exact), PyretNumber::Exact(right_exact)) => {
                PyretNumber::Exact(left_exact + right_exact)
            }
            (PyretNumber::Exact(left_exact), PyretNumber::Rough(right_rough)) => {
                PyretNumber::Rough(left_exact.to_f64().unwrap() + right_rough)
            }
            (PyretNumber::Rough(left_rough), PyretNumber::Exact(right_exact)) => {
                PyretNumber::Rough(left_rough + right_exact.to_f64().unwrap())
            }
            (PyretNumber::Rough(left_rough), PyretNumber::Rough(right_rough)) => {
                PyretNumber::Rough(left_rough + right_rough)
            }
        }
    }
}

impl CheckedAdd for PyretNumber {
    fn checked_add(&self, rhs: &Self) -> Option<Self::Output> {
        Some(match (self, rhs) {
            (Self::Exact(left_exact), Self::Exact(right_exact)) => {
                Self::Exact(left_exact + right_exact)
            }
            (Self::Exact(left_exact), Self::Rough(right_rough)) => {
                Self::Rough(left_exact.to_f64()? + right_rough)
            }
            (Self::Rough(left_rough), Self::Exact(right_exact)) => {
                Self::Rough(left_rough + right_exact.to_f64()?)
            }
            (Self::Rough(left_rough), Self::Rough(right_rough)) => {
                Self::Rough(left_rough + right_rough)
            }
        })
    }
}

impl Sub for PyretNumber {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Exact(left_exact), Self::Exact(right_exact)) => {
                Self::Exact(left_exact - right_exact)
            }
            (Self::Exact(left_exact), Self::Rough(right_rough)) => {
                Self::Rough(left_exact.to_f64().unwrap() - right_rough)
            }
            (Self::Rough(left_rough), Self::Exact(right_exact)) => {
                Self::Rough(left_rough - right_exact.to_f64().unwrap())
            }
            (Self::Rough(left_rough), Self::Rough(right_rough)) => {
                Self::Rough(left_rough - right_rough)
            }
        }
    }
}

impl Sub for &PyretNumber {
    type Output = PyretNumber;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (PyretNumber::Exact(left_exact), PyretNumber::Exact(right_exact)) => {
                PyretNumber::Exact(left_exact - right_exact)
            }
            (PyretNumber::Exact(left_exact), PyretNumber::Rough(right_rough)) => {
                PyretNumber::Rough(left_exact.to_f64().unwrap() - right_rough)
            }
            (PyretNumber::Rough(left_rough), PyretNumber::Exact(right_exact)) => {
                PyretNumber::Rough(left_rough - right_exact.to_f64().unwrap())
            }
            (PyretNumber::Rough(left_rough), PyretNumber::Rough(right_rough)) => {
                PyretNumber::Rough(left_rough - right_rough)
            }
        }
    }
}

impl CheckedSub for PyretNumber {
    fn checked_sub(&self, rhs: &Self) -> Option<Self::Output> {
        Some(match (self, rhs) {
            (Self::Exact(left_exact), Self::Exact(right_exact)) => {
                Self::Exact(left_exact - right_exact)
            }
            (Self::Exact(left_exact), Self::Rough(right_rough)) => {
                Self::Rough(left_exact.to_f64()? - right_rough)
            }
            (Self::Rough(left_rough), Self::Exact(right_exact)) => {
                Self::Rough(left_rough - right_exact.to_f64()?)
            }
            (Self::Rough(left_rough), Self::Rough(right_rough)) => {
                Self::Rough(left_rough - right_rough)
            }
        })
    }
}

impl Mul for PyretNumber {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Exact(left_exact), Self::Exact(right_exact)) => {
                Self::Exact(left_exact * right_exact)
            }
            (Self::Exact(left_exact), Self::Rough(right_rough)) => {
                Self::Rough(left_exact.to_f64().unwrap() * right_rough)
            }
            (Self::Rough(left_rough), Self::Exact(right_exact)) => {
                Self::Rough(left_rough * right_exact.to_f64().unwrap())
            }
            (Self::Rough(left_rough), Self::Rough(right_rough)) => {
                Self::Rough(left_rough * right_rough)
            }
        }
    }
}

impl Mul for &PyretNumber {
    type Output = PyretNumber;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (PyretNumber::Exact(left_exact), PyretNumber::Exact(right_exact)) => {
                PyretNumber::Exact(left_exact * right_exact)
            }
            (PyretNumber::Exact(left_exact), PyretNumber::Rough(right_rough)) => {
                PyretNumber::Rough(left_exact.to_f64().unwrap() * right_rough)
            }
            (PyretNumber::Rough(left_rough), PyretNumber::Exact(right_exact)) => {
                PyretNumber::Rough(left_rough * right_exact.to_f64().unwrap())
            }
            (PyretNumber::Rough(left_rough), PyretNumber::Rough(right_rough)) => {
                PyretNumber::Rough(left_rough * right_rough)
            }
        }
    }
}

impl CheckedMul for PyretNumber {
    fn checked_mul(&self, rhs: &Self) -> Option<Self::Output> {
        Some(match (self, rhs) {
            (Self::Exact(left_exact), Self::Exact(right_exact)) => {
                Self::Exact(left_exact * right_exact)
            }
            (Self::Exact(left_exact), Self::Rough(right_rough)) => {
                Self::Rough(left_exact.to_f64()? * right_rough)
            }
            (Self::Rough(left_rough), Self::Exact(right_exact)) => {
                Self::Rough(left_rough * right_exact.to_f64()?)
            }
            (Self::Rough(left_rough), Self::Rough(right_rough)) => {
                Self::Rough(left_rough * right_rough)
            }
        })
    }
}

impl Div for PyretNumber {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Exact(left_exact), Self::Exact(right_exact)) => {
                Self::Exact(left_exact / right_exact)
            }
            (Self::Exact(left_exact), Self::Rough(right_rough)) => {
                Self::Rough(left_exact.to_f64().unwrap() / right_rough)
            }
            (Self::Rough(left_rough), Self::Exact(right_exact)) => {
                Self::Rough(left_rough / right_exact.to_f64().unwrap())
            }
            (Self::Rough(left_rough), Self::Rough(right_rough)) => {
                Self::Rough(left_rough / right_rough)
            }
        }
    }
}

impl Div for &PyretNumber {
    type Output = PyretNumber;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (PyretNumber::Exact(left_exact), PyretNumber::Exact(right_exact)) => {
                PyretNumber::Exact(left_exact / right_exact)
            }
            (PyretNumber::Exact(left_exact), PyretNumber::Rough(right_rough)) => {
                PyretNumber::Rough(left_exact.to_f64().unwrap() / right_rough)
            }
            (PyretNumber::Rough(left_rough), PyretNumber::Exact(right_exact)) => {
                PyretNumber::Rough(left_rough / right_exact.to_f64().unwrap())
            }
            (PyretNumber::Rough(left_rough), PyretNumber::Rough(right_rough)) => {
                PyretNumber::Rough(left_rough / right_rough)
            }
        }
    }
}

impl CheckedDiv for PyretNumber {
    fn checked_div(&self, rhs: &Self) -> Option<Self::Output> {
        Some(match (self, rhs) {
            (Self::Exact(left_exact), Self::Exact(right_exact)) => {
                Self::Exact(left_exact / right_exact)
            }
            (Self::Exact(left_exact), Self::Rough(right_rough)) => {
                Self::Rough(left_exact.to_f64()? / right_rough)
            }
            (Self::Rough(left_rough), Self::Exact(right_exact)) => {
                Self::Rough(left_rough / right_exact.to_f64()?)
            }
            (Self::Rough(left_rough), Self::Rough(right_rough)) => {
                Self::Rough(left_rough / right_rough)
            }
        })
    }
}

impl Rem for PyretNumber {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Exact(left_exact), Self::Exact(right_exact)) => {
                Self::Exact(left_exact % right_exact)
            }
            _ => unimplemented!("roughnums cannot be moduloed"),
        }
    }
}

impl Rem for &PyretNumber {
    type Output = PyretNumber;

    fn rem(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (PyretNumber::Exact(left_exact), PyretNumber::Exact(right_exact)) => {
                PyretNumber::Exact(left_exact % right_exact)
            }
            _ => unimplemented!("roughnums cannot be moduloed"),
        }
    }
}

impl CheckedRem for PyretNumber {
    fn checked_rem(&self, rhs: &Self) -> Option<Self::Output> {
        match (self, rhs) {
            (Self::Exact(left_exact), Self::Exact(right_exact)) => {
                Some(Self::Exact(left_exact % right_exact))
            }
            _ => None,
        }
    }
}

// impl Eq for PyretNumber {}

impl PartialOrd for PyretNumber {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Exact(left_exact), Self::Exact(right_exact)) => {
                left_exact.partial_cmp(right_exact)
            }
            (Self::Exact(left_exact), Self::Rough(right_rough)) => {
                left_exact.to_f64()?.partial_cmp(right_rough)
            }
            (Self::Rough(left_rough), Self::Exact(right_exact)) => {
                left_rough.partial_cmp(&right_exact.to_f64()?)
            }
            (Self::Rough(left_rough), Self::Rough(right_rough)) => {
                left_rough.partial_cmp(right_rough)
            }
        }
    }
}

impl Neg for PyretNumber {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Self::Exact(exact) => Self::Exact(-exact),
            Self::Rough(rough) => Self::Rough(-rough),
        }
    }
}
