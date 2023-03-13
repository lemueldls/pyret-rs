use std::rc::Rc;

use pyret_error::PyretResult;

use crate::{
    trove::{Any, Trove},
    ty,
    value::registrar::Registrar,
    PyretValue,
};

pub fn register(registrar: &mut Registrar) -> PyretResult<()> {
    ModBoolean::register(registrar)
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
