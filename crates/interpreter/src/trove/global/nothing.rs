use std::{cell::RefCell, rc::Rc, sync::Arc};

use pyret_error::PyretResult;

use super::Any;
use crate::{
    value::{
        context::{Context, Register},
        PyretValueKind,
    },
    PyretValue,
};

pub fn register(context: Rc<RefCell<Context>>) -> PyretResult<()> {
    let any = &Any::predicate();

    context.register_builtin_type(
        "Nothing",
        Arc::new(|value, _context| *value.kind == PyretValueKind::Nothing),
    )?;

    context.register_builtin_expr("nothing", PyretValue::from(PyretValueKind::Nothing));

    context.register_builtin_function(
        "is-nothing",
        [any],
        Rc::new(|args, _context| {
            Ok(PyretValue::from(PyretValueKind::Boolean(
                *args.next().unwrap().kind == PyretValueKind::Nothing,
            )))
        }),
    )?;

    Ok(())
}
