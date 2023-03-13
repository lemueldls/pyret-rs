use std::{
    cell::{RefCell, RefMut},
    sync::Arc,
};

use pyret_error::PyretResult;

use crate::{context::Context, value::registrar::Registrar, PyretValue, Rc};

pub fn register(registrar: &mut Registrar) -> PyretResult<()> {
    let any = &registrar.get_type("Any")?.unwrap();

    registrar.register_builtin_type(
        "Nothing",
        Arc::new(|value, _context| value.as_ref() == &PyretValue::Nothing),
    )?;

    registrar.register_builtin_expr("nothing", PyretValue::Nothing);

    registrar.register_builtin_function(
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
