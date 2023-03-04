pub mod boolean;
pub mod nothing;
pub mod number;
pub mod ops;
pub mod string;

use std::{cell::RefMut, rc::Rc};

use pyret_error::{PyretErrorKind, PyretResult};

use crate::{context::Context, io::Output, value::PyretValue};

pub fn register(context: &mut RefMut<Context>) -> PyretResult<()> {
    super::constants::register(context)?;

    let any = &context
        .registrar
        .register_builtin_type("Any", Rc::new(|_value, _context| true))?;

    context.registrar.register_builtin_function(
        "display",
        [any],
        Rc::new(|args, context| {
            let value = &args[0];

            context.borrow().io.write(Output::Display(value));

            Ok(Rc::clone(value))
        }),
    )?;

    context.registrar.register_builtin_function(
        "print",
        [any],
        Rc::new(|args, context| {
            let value = &args[0];

            context.borrow().io.write(Output::Print(to_repr(value)));

            Ok(Rc::clone(value))
        }),
    )?;

    context.registrar.register_builtin_function(
        "raise",
        [any],
        Rc::new(|args, _context| {
            Err(PyretErrorKind::RaiseRuntime(
                args[0].to_string().into_boxed_str(),
            ))
        }),
    )?;

    boolean::register(context)?;
    nothing::register(context)?;
    number::register(context)?;
    ops::register(context)?;
    string::register(context)?;

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
