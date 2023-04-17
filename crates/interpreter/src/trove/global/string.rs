use std::{rc::Rc, sync::Arc};

use pyret_error::PyretResult;
use pyret_number::{BigInt, BigRational, PyretNumber};

use crate::{
    value::{context::Context, PyretValueKind},
    PyretValue,
};

#[inline]
pub fn register(context: Context) -> PyretResult<()> {
    let string = &context.register_builtin_type(
        "String",
        Arc::new(|value, _context| matches!(*value.kind, PyretValueKind::String(..))),
    )?;

    context.register_builtin_function(
        "string-equal",
        [string, string],
        Rc::new(|args, _context| {
            Ok(PyretValue::from(PyretValueKind::Boolean(
                args.next().unwrap().kind == args.next().unwrap().kind,
            )))
        }),
    )?;

    context.register_builtin_function(
        "string-contains",
        [string, string],
        Rc::new(|args, _context| {
            let (PyretValueKind::String(haystack), PyretValueKind::String(needle)) =
                (&*args.next().unwrap().kind, &*args.next().unwrap().kind)
            else {
                unreachable!()
            };

            Ok(PyretValue::from(PyretValueKind::Boolean(
                haystack.contains(&**needle),
            )))
        }),
    )?;

    context.register_builtin_function(
        "string-append",
        [string, string],
        Rc::new(|args, _context| {
            let (PyretValueKind::String(left), PyretValueKind::String(right)) =
                (&*args.next().unwrap().kind, &*args.next().unwrap().kind)
            else {
                unreachable!()
            };

            Ok(PyretValue::from(PyretValueKind::String(
                format!("{left}{right}").into_boxed_str(),
            )))
        }),
    )?;

    context.register_builtin_function(
        "string-length",
        [string],
        Rc::new(|args, _context| {
            let PyretValueKind::String(string) = &*args.next().unwrap().kind else {
                unreachable!()
            };

            Ok(PyretValue::from(PyretValueKind::Number(
                PyretNumber::Exact(BigRational::from_integer(BigInt::from(string.len()))),
            )))
        }),
    )?;

    Ok(())
}
