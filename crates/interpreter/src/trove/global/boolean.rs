use std::cell::{RefCell, RefMut};

use pyret_error::PyretResult;

use crate::{context::Context, PyretValue, Rc};

pub fn register(context: &mut RefMut<Context>) -> PyretResult<()> {
    let any = &context.registrar.get_type("Any")?.unwrap();

    let boolean = &context.registrar.register_builtin_type(
        "Boolean",
        Rc::new(|value, _context| matches!(value.as_ref(), PyretValue::Boolean(..))),
    )?;

    context.registrar.register_builtin_function(
        "is-boolean",
        [any],
        Rc::new(|args, _context| {
            Ok(Rc::new(PyretValue::Boolean(matches!(
                args[0].as_ref(),
                PyretValue::Boolean(..)
            ))))
        }),
    )?;

    context.registrar.register_builtin_function(
        "not",
        [boolean],
        Rc::new(|args, _context| match args[0].as_ref() {
            PyretValue::Boolean(value) => Ok(Rc::new(PyretValue::Boolean(!value))),
            _ => unreachable!(),
        }),
    )?;

    Ok(())
}
