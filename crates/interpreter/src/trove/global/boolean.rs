use std::{cell::RefCell, rc::Rc};

use pyret_error::PyretResult;

use super::Any;
use crate::{trove::Trove, ty, value::context::Context, PyretValue};

pub fn register(context: Rc<RefCell<Context>>) -> PyretResult<()> {
    ModBoolean::register(context)
}

ty!(Boolean, |value, _context| matches!(
    value.as_ref(),
    PyretValue::Boolean(..)
));

struct ModBoolean;

#[module]
impl ModBoolean {
    #[must_use]
    pub fn is_boolean(value: &Any) -> Boolean {
        Boolean(Rc::new(PyretValue::Boolean(matches!(
            value.as_ref(),
            PyretValue::Boolean(..)
        ))))
    }

    #[must_use]
    pub fn not(value: &Boolean) -> Boolean {
        match value.as_ref() {
            PyretValue::Boolean(value) => Boolean(Rc::new(PyretValue::Boolean(!value))),
            _ => unreachable!(),
        }
    }
}
