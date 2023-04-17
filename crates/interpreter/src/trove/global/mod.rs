pub mod boolean;
pub mod nothing;
pub mod number;
pub mod ops;
pub mod string;

use std::rc::Rc;

use pyret_error::{PyretErrorKind, PyretResult};

use super::import_trove;
use crate::{
    io::Output,
    ty,
    value::{context::Context, PyretValue, PyretValueKind},
};

ty!(Any, |_value, _context| true);

#[inline]
pub fn register(context: Context) -> PyretResult<()> {
    {
        Any::register(context.clone())?;

        let any = &Any::predicate();

        context.register_builtin_function(
            "display",
            [any],
            Rc::new(|args, context| {
                let value = args.next().unwrap();

                context
                    .borrow_mut()
                    .io
                    .write(Output::Display(value.clone()));

                Ok(value)
            }),
        )?;

        context.register_builtin_function(
            "print",
            [any],
            Rc::new(|args, context| {
                let value = args.next().unwrap();

                context
                    .borrow_mut()
                    .io
                    .write(Output::Print(to_repr(&value)));

                Ok(value)
            }),
        )?;

        context.register_builtin_function(
            "raise",
            [any],
            Rc::new(|args, _context| {
                Err(PyretErrorKind::RaiseRuntime(
                    args.next().unwrap().to_string().into_boxed_str(),
                ))
            }),
        )?;
    }

    boolean::register(context.clone())?;
    nothing::register(context.clone())?;
    number::register(context.clone())?;
    ops::register(context.clone())?;
    string::register(context.clone())?;

    import_trove("constants", context)?;

    Ok(())
}

fn to_repr(value: &PyretValue) -> Box<str> {
    match &*value.kind {
        PyretValueKind::String(string) => string.clone(),
        PyretValueKind::Function(..) => Box::from("<function>"),
        PyretValueKind::Nothing => Box::from("nothing"),
        _ => format!("{value}").into_boxed_str(),
    }
}
