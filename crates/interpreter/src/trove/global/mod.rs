pub mod boolean;
pub mod nothing;
pub mod number;
pub mod ops;
pub mod string;

use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
    sync::Arc,
};

use pyret_error::{PyretErrorKind, PyretResult};

use super::import_trove;
use crate::{
    io::Output,
    trove::Trove,
    ty,
    value::{
        context::{Context, Register},
        PyretValue,
    },
    Interpreter,
};

ty!(Any, |_value, _context| true);

pub fn register(context: Rc<RefCell<Context>>) -> PyretResult<()> {
    {
        import_trove("constants", Rc::clone(&context))?;

        let any = &Any::predicate();

        context.register_builtin_function(
            "display",
            [any],
            Rc::new(|args, context| {
                let value = &args[0];

                context
                    .borrow_mut()
                    .io
                    .write(Output::Display(Rc::clone(value)));

                Ok(Rc::clone(value))
            }),
        )?;

        context.register_builtin_function(
            "print",
            [any],
            Rc::new(|args, context| {
                let value = &args[0];

                context.borrow_mut().io.write(Output::Print(to_repr(value)));

                Ok(Rc::clone(value))
            }),
        )?;

        context.register_builtin_function(
            "raise",
            [any],
            Rc::new(|args, _context| {
                Err(PyretErrorKind::RaiseRuntime(
                    args[0].to_string().into_boxed_str(),
                ))
            }),
        )?;
    }

    boolean::register(Rc::clone(&context))?;
    nothing::register(Rc::clone(&context))?;
    number::register(Rc::clone(&context))?;
    ops::register(Rc::clone(&context))?;
    string::register(Rc::clone(&context))?;

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
