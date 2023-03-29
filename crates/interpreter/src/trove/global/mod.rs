pub mod boolean;
pub mod nothing;
pub mod number;
pub mod ops;
pub mod string;

use std::{cell::RefMut, rc::Rc, sync::Arc};

use pyret_error::{PyretErrorKind, PyretResult};

use crate::{
    context::Context,
    io::Output,
    trove::Trove,
    value::{registrar::Registrar, PyretValue},
};

pub fn register(registrar: &mut Registrar) -> PyretResult<()> {
    super::constants::register(registrar)?;

    let any = &registrar.register_builtin_type("Any", Arc::new(|_value, _context| true))?;

    registrar.register_builtin_function(
        "display",
        [any],
        Rc::new(|args, context| {
            let value = &args[0];

            context.borrow().io.write(Output::Display(value));

            Ok(Rc::clone(value))
        }),
    )?;

    registrar.register_builtin_function(
        "print",
        [any],
        Rc::new(|args, context| {
            let value = &args[0];

            context.borrow().io.write(Output::Print(to_repr(value)));

            Ok(Rc::clone(value))
        }),
    )?;

    registrar.register_builtin_function(
        "raise",
        [any],
        Rc::new(|args, _context| {
            Err(PyretErrorKind::RaiseRuntime(
                args[0].to_string().into_boxed_str(),
            ))
        }),
    )?;

    boolean::register(registrar)?;
    nothing::register(registrar)?;
    number::register(registrar)?;
    ops::register(registrar)?;
    string::register(registrar)?;

    Ok(())
}

fn to_repr(value: &Rc<PyretValue>) -> Box<str> {
    match value.as_ref() {
        PyretValue::String(string) => string.clone(),
        PyretValue::Function(..) => Box::from("<function>"),
        PyretValue::Nothing => Box::from("nothing"),
        _ => format!("{value}").into_boxed_str(),
    }
}
