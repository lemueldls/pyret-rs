use std::{cell::RefMut, sync::Arc};

use pyret_error::PyretResult;
use pyret_number::{BigInt, BigRational, PyretNumber};

use crate::{context::Context, value::registrar::Registrar, PyretValue, Rc};

pub fn register(registrar: &mut Registrar) -> PyretResult<()> {
    let string = &registrar.register_builtin_type(
        "String",
        Arc::new(|value, _context| matches!(value.as_ref(), PyretValue::String(..))),
    )?;

    registrar.register_builtin_function(
        "string-equal",
        [string, string],
        Rc::new(|args, _context| {
            Ok(Rc::new(PyretValue::Boolean(
                args[0].as_ref() == args[1].as_ref(),
            )))
        }),
    )?;

    registrar.register_builtin_function(
        "string-contains",
        [string, string],
        Rc::new(|args, _context| {
            let (PyretValue::String(haystack), PyretValue::String(needle)) =
                (&args[0].as_ref(), &args[1].as_ref())
            else {
                unreachable!()
            };

            Ok(Rc::new(PyretValue::Boolean(haystack.contains(&**needle))))
        }),
    )?;

    registrar.register_builtin_function(
        "string-append",
        [string, string],
        Rc::new(|args, _context| {
            let (PyretValue::String(left), PyretValue::String(right)) =
                (&args[0].as_ref(), &args[1].as_ref())
            else {
                unreachable!()
            };

            Ok(Rc::new(PyretValue::String(
                format!("{left}{right}").into_boxed_str(),
            )))
        }),
    )?;

    registrar.register_builtin_function(
        "string-length",
        [string],
        Rc::new(|args, _context| {
            let PyretValue::String(string) = args[0].as_ref() else {
                unreachable!()
            };

            Ok(Rc::new(PyretValue::Number(PyretNumber::Exact(
                BigRational::from_integer(BigInt::from(string.len())),
            ))))
        }),
    )?;

    Ok(())
}