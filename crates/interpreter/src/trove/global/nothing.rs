use std::cell::{RefCell, RefMut};

use pyret_error::PyretResult;

use crate::{context::Context, PyretValue, Rc};

pub fn register(context: &mut RefMut<Context>) -> PyretResult<()> {
    let any = &context.registrar.get_type("Any")?.unwrap();

    context.registrar.register_builtin_type(
        "Nothing",
        Rc::new(|value, _context| value.as_ref() == &PyretValue::Nothing),
    )?;

    context
        .registrar
        .register_builtin_expr("nothing", PyretValue::Nothing);

    context.registrar.register_builtin_function(
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
