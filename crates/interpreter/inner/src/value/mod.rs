pub mod function;
pub mod registrar;

use std::{
    cell::{Ref, RefCell},
    fmt,
    rc::Rc,
    sync::Arc,
};

pub use function::PyretFunction;
use pyret_error::PyretResult;
use pyret_number::PyretNumber;

use crate::Context;

pub type TypePredicate = Arc<dyn Fn(Rc<PyretValue>, Rc<RefCell<Context>>) -> bool + Send + Sync>;

pub struct PyretValueScoped {
    pub value: Rc<PyretValue>,
    pub depth: usize,
    pub is_builtin: bool,
}

impl PyretValueScoped {
    #[must_use]
    pub const fn new_local(value: Rc<PyretValue>, depth: usize) -> Self {
        Self {
            value,
            depth,
            is_builtin: false,
        }
    }

    #[must_use]
    pub fn new_builtin(value: Rc<PyretValue>) -> Self {
        Self {
            value,
            depth: 0,
            is_builtin: true,
        }
    }
}

pub enum PyretValue {
    Number(PyretNumber),
    String(Box<str>),
    Boolean(bool),
    Function(PyretFunction),
    Nothing,
}

impl fmt::Display for PyretValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(number) => write!(f, "{number}"),
            Self::String(string) => write!(f, "\"{}\"", string.escape_debug()),
            Self::Boolean(boolean) => write!(f, "{boolean}"),
            Self::Function(PyretFunction { name, .. }) => write!(f, "<function:{name}>"),
            Self::Nothing => Ok(()),
        }
    }
}

impl PartialEq for PyretValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Number(left_number), Self::Number(right_number)) => left_number == right_number,
            (Self::String(left_string), Self::String(right_string)) => left_string == right_string,
            (Self::Boolean(left_boolean), Self::Boolean(right_boolean)) => {
                left_boolean == right_boolean
            }
            (Self::Nothing, Self::Nothing) => true,
            _ => false,
        }
    }
}
