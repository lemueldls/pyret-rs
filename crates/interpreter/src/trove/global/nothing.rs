use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
    sync::Arc,
};

use pyret_error::PyretResult;

use super::Any;
use crate::{
    value::context::{Context, Register},
    PyretValue,
};

pub fn register(context: Rc<RefCell<Context>>) -> PyretResult<()> {
    let any = &Any::predicate();

    context.register_builtin_type(
        "Nothing",
        Arc::new(|value, _context| value.as_ref() == &PyretValue::Nothing),
    )?;

    context.register_builtin_expr("nothing", PyretValue::Nothing);

    context.register_builtin_function(
        "is-nothing",
        [any],
        Rc::new(|args, _context| {
            Ok(Rc::new(PyretValue::Boolean(
                args[0].as_ref() == &PyretValue::Nothing,
            )))
        }),
    )?;

    Ok(())
}
