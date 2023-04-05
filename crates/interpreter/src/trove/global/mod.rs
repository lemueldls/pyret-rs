pub mod boolean;
pub mod nothing;
pub mod number;
pub mod ops;
pub mod string;

use std::{cell::RefCell, rc::Rc};

use pyret_error::{PyretErrorKind, PyretResult};

use super::import_trove;
use crate::{
    io::Output,
    ty,
    value::{
        context::{Context, Register},
        PyretValue, PyretValueKind,
    },
};

ty!(Any, |_value, _context| true);

pub fn register(context: Rc<RefCell<Context>>) -> PyretResult<()> {
    {
        Any::register(Rc::clone(&context))?;

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

    boolean::register(Rc::clone(&context))?;
    nothing::register(Rc::clone(&context))?;
    number::register(Rc::clone(&context))?;
    ops::register(Rc::clone(&context))?;
    string::register(Rc::clone(&context))?;

    import_trove("constants", Rc::clone(&context))?;

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
