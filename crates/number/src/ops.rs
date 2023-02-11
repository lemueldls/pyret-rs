use std::ops::{Add, Div, Mul, Sub};

pub use num_traits::ToPrimitive;

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
