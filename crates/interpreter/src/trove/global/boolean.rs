use std::rc::Rc;

use pyret_error::PyretResult;

use super::Any;
use crate::{
    trove::Trove,
    ty,
    value::{context::Context, PyretValueKind},
    PyretValue,
};

#[inline]
pub fn register(context: Context) -> PyretResult<()> {
    ModBoolean::register(context)
}

ty!(Boolean, |value, _context| matches!(
    *value.kind,
    PyretValueKind::Boolean(..)
));

struct ModBoolean;

#[module]
impl ModBoolean {
    #[inline]
    #[must_use]
    pub fn is_boolean(value: &Any) -> Boolean {
        Boolean(PyretValue::from(PyretValueKind::Boolean(matches!(
            *value.kind,
            PyretValueKind::Boolean(..)
        ))))
    }

    #[inline]
    #[must_use]
    pub fn not(value: &Boolean) -> Boolean {
        match *value.kind {
            PyretValueKind::Boolean(value) => {
                Boolean(PyretValue::from(PyretValueKind::Boolean(!value)))
            }
            _ => unreachable!(),
        }
    }
}
