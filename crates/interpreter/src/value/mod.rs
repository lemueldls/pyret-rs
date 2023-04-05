pub mod context;
pub mod function;

use std::{cell::RefCell, fmt, ops::Range, rc::Rc, sync::Arc};

pub use function::PyretFunction;
use pyret_number::PyretNumber;

use crate::Context;

pub type TypePredicate = Arc<dyn Fn(PyretValue, Rc<RefCell<Context>>) -> bool + Send + Sync>;

#[derive(Clone)]
pub struct PyretValueScoped {
    pub value: PyretValue,
    pub is_builtin: bool,
}

impl PyretValueScoped {
    #[must_use]
    pub const fn new_local(value: PyretValue) -> Self {
        Self {
            value,
            is_builtin: false,
        }
    }

    #[must_use]
    pub fn new_builtin(value: PyretValue) -> Self {
        Self {
            value,
            is_builtin: true,
        }
    }
}

#[derive(Clone)]
pub struct PyretValue {
    pub span: Option<Range<usize>>,
    pub kind: Rc<PyretValueKind>,
}

impl PyretValue {
    #[must_use]
    pub const fn new(span: Range<usize>, kind: Rc<PyretValueKind>) -> Self {
        Self {
            span: Some(span),
            kind,
        }
    }
}

impl From<PyretValueKind> for PyretValue {
    fn from(kind: PyretValueKind) -> Self {
        Self {
            span: None,
            kind: Rc::new(kind),
        }
    }
}

pub enum PyretValueKind {
    Number(PyretNumber),
    String(Box<str>),
    Boolean(bool),
    Function(PyretFunction),
    Nothing,
}

impl fmt::Display for PyretValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self.kind {
            PyretValueKind::Number(number) => write!(f, "{number}"),
            PyretValueKind::String(string) => write!(f, "\"{}\"", string.escape_debug()),
            PyretValueKind::Boolean(boolean) => write!(f, "{boolean}"),
            PyretValueKind::Function(PyretFunction { name, .. }) => write!(f, "<function:{name}>"),
            PyretValueKind::Nothing => Ok(()),
        }
    }
}

impl PartialEq for PyretValueKind {
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
