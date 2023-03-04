use std::{cell::RefMut, cmp::Ordering};

use pyret_error::PyretErrorKind;
use pyret_number::{PyretNumber, Signed};

use crate::{context::Context, PyretResult, PyretValue, Rc};

pub fn register(context: &mut RefMut<Context>) -> PyretResult<()> {
    let any = &context.registrar.get_type("Any")?.unwrap();

    let number = &context.registrar.register_builtin_type(
        "Number",
        Rc::new(|value, _context| matches!(value.as_ref(), PyretValue::Number(..))),
    )?;

    context.registrar.register_builtin_type(
        "Exactnum",
        Rc::new(|value, _context| {
            matches!(value.as_ref(), PyretValue::Number(PyretNumber::Exact(..)))
        }),
    )?;

    context.registrar.register_builtin_type(
        "Roughnum",
        Rc::new(|value, _context| {
            matches!(value.as_ref(), PyretValue::Number(PyretNumber::Rough(..)))
        }),
    )?;

    let num_integer = &context.registrar.register_builtin_type(
        "NumInteger",
        Rc::new(|value, _context| {
            matches!(value.as_ref(), PyretValue::Number(PyretNumber::Exact(number)) if number.is_integer())
        }),
    )?;

    context.registrar.register_builtin_type(
        "NumRational",
        Rc::new(|value, _context| {
            matches!(value.as_ref(), PyretValue::Number(PyretNumber::Exact(..)))
        }),
    )?;

    context.registrar.register_builtin_type(
        "NumPositive",
        Rc::new(|value, _context| {
            matches!(value.as_ref(), PyretValue::Number(number) if number.is_positive())
        }),
    )?;

    context.registrar.register_builtin_type(
        "NumNegative",
        Rc::new(|value, _context| {
            matches!(value.as_ref(), PyretValue::Number(number) if number.is_negative())
        }),
    )?;

    context.registrar.register_builtin_type(
        "NumNonPositive",
        Rc::new(|value, _context| {
            matches!(value.as_ref(), PyretValue::Number(number) if number.is_non_positive())
        }),
    )?;

    context.registrar.register_builtin_type(
        "NumNonNegative",
        Rc::new(|value, _context| {
            matches!(value.as_ref(), PyretValue::Number(number) if number.is_non_negative())
        }),
    )?;

    context.registrar.register_builtin_function(
        "is-number",
        [any],
        Rc::new(|args, _context| {
            Ok(Rc::new(PyretValue::Boolean(matches!(
                args[0].as_ref(),
                PyretValue::Number(..)
            ))))
        }),
    )?;

    context.registrar.register_builtin_function(
        "is-equal",
        [number, number],
        Rc::new(
            |args, _context| match (args[0].as_ref(), args[1].as_ref()) {
                (PyretValue::Number(left), PyretValue::Number(right)) => {
                    Ok(Rc::new(PyretValue::Boolean(left.is_equal(right).map_err(
                        |err| PyretErrorKind::RaiseRuntime(Box::from(err)),
                    )?)))
                }
                _ => unreachable!(),
            },
        ),
    )?;

    context.registrar.register_builtin_function(
        "num-max",
        [number, number],
        Rc::new(|args, _context| {
            let rc_left = &args[0];
            let rc_right = &args[0];

            match (rc_left.as_ref(), rc_right.as_ref()) {
                (PyretValue::Number(left), PyretValue::Number(right)) => {
                    match left.partial_cmp(right) {
                        Some(Ordering::Less | Ordering::Equal) => Ok(Rc::clone(rc_left)),
                        Some(Ordering::Greater) => Ok(Rc::clone(rc_right)),
                        None => Err(PyretErrorKind::RaiseRuntime(Box::from("roughnum overflow"))),
                    }
                }
                _ => unreachable!(),
            }
        }),
    )?;

    context.registrar.register_builtin_function(
        "num-min",
        [number, number],
        Rc::new(|args, _context| {
            let rc_left = &args[0];
            let rc_right = &args[0];

            match (rc_left.as_ref(), rc_right.as_ref()) {
                (PyretValue::Number(left), PyretValue::Number(right)) => {
                    match left.partial_cmp(right) {
                        Some(Ordering::Less | Ordering::Equal) => Ok(Rc::clone(rc_right)),
                        Some(Ordering::Greater) => Ok(Rc::clone(rc_left)),
                        None => Err(PyretErrorKind::RaiseRuntime(Box::from("roughnum overflow"))),
                    }
                }
                _ => unreachable!(),
            }
        }),
    )?;

    context.registrar.register_builtin_function(
        "num-abs",
        [number],
        Rc::new(|args, _context| match args[0].as_ref() {
            PyretValue::Number(number) => Ok(Rc::new(PyretValue::Number(number.abs()))),
            _ => unreachable!(),
        }),
    )?;

    context.registrar.register_builtin_function(
        "num-sin",
        [number],
        Rc::new(|args, _context| match args[0].as_ref() {
            PyretValue::Number(number) => Ok(Rc::new(PyretValue::Number(
                number.sin().map_err(PyretErrorKind::from)?,
            ))),
            _ => unreachable!(),
        }),
    )?;

    context.registrar.register_builtin_function(
        "num-cos",
        [number],
        Rc::new(|args, _context| match args[0].as_ref() {
            PyretValue::Number(number) => Ok(Rc::new(PyretValue::Number(
                number.cos().map_err(PyretErrorKind::from)?,
            ))),
            _ => unreachable!(),
        }),
    )?;

    context.registrar.register_builtin_function(
        "num-tan",
        [number],
        Rc::new(|args, _context| match args[0].as_ref() {
            PyretValue::Number(number) => Ok(Rc::new(PyretValue::Number(
                number.tan().map_err(PyretErrorKind::from)?,
            ))),
            _ => unreachable!(),
        }),
    )?;

    context.registrar.register_builtin_function(
        "num-asin",
        [number],
        Rc::new(|args, _context| match args[0].as_ref() {
            PyretValue::Number(number) => Ok(Rc::new(PyretValue::Number(
                number.asin().map_err(PyretErrorKind::from)?,
            ))),
            _ => unreachable!(),
        }),
    )?;

    context.registrar.register_builtin_function(
        "num-acos",
        [number],
        Rc::new(|args, _context| match args[0].as_ref() {
            PyretValue::Number(number) => Ok(Rc::new(PyretValue::Number(
                number.acos().map_err(PyretErrorKind::from)?,
            ))),
            _ => unreachable!(),
        }),
    )?;

    context.registrar.register_builtin_function(
        "num-atan",
        [number],
        Rc::new(|args, _context| match args[0].as_ref() {
            PyretValue::Number(number) => Ok(Rc::new(PyretValue::Number(
                number.atan().map_err(PyretErrorKind::from)?,
            ))),
            _ => unreachable!(),
        }),
    )?;

    context.registrar.register_builtin_function(
        "num-atan2",
        [number, number],
        Rc::new(
            |args, _context| match (args[0].as_ref(), args[1].as_ref()) {
                (PyretValue::Number(left), PyretValue::Number(right)) => Ok(Rc::new(
                    PyretValue::Number(left.atan2(right).map_err(PyretErrorKind::from)?),
                )),
                _ => unreachable!(),
            },
        ),
    )?;

    context.registrar.register_builtin_function(
        "num-modulo",
        [num_integer, num_integer],
        Rc::new(
            |args, _context| match (args[0].as_ref(), args[1].as_ref()) {
                (PyretValue::Number(left), PyretValue::Number(right)) => Ok(Rc::new(
                    PyretValue::Number(left.modulo(right).map_err(PyretErrorKind::from)?),
                )),
                _ => unreachable!(),
            },
        ),
    )?;

    context.registrar.register_builtin_function(
        "num-truncate",
        [number],
        Rc::new(|args, _context| match args[0].as_ref() {
            PyretValue::Number(number) => Ok(Rc::new(PyretValue::Number(
                number.truncate().map_err(PyretErrorKind::from)?,
            ))),
            _ => unreachable!(),
        }),
    )?;

    context.registrar.register_builtin_function(
        "num-sqrt",
        [number],
        Rc::new(|args, _context| match args[0].as_ref() {
            PyretValue::Number(number) => Ok(Rc::new(PyretValue::Number(
                number.sqrt().map_err(PyretErrorKind::from)?,
            ))),
            _ => unreachable!(),
        }),
    )?;

    context.registrar.register_builtin_function(
        "num-sqr",
        [number],
        Rc::new(|args, _context| match args[0].as_ref() {
            PyretValue::Number(number) => Ok(Rc::new(PyretValue::Number(
                number.sqr().map_err(PyretErrorKind::from)?,
            ))),
            _ => unreachable!(),
        }),
    )?;

    context.registrar.register_builtin_function(
        "num-ceiling",
        [number],
        Rc::new(|args, _context| match args[0].as_ref() {
            PyretValue::Number(number) => Ok(Rc::new(PyretValue::Number(
                number.ceiling().map_err(PyretErrorKind::from)?,
            ))),
            _ => unreachable!(),
        }),
    )?;

    context.registrar.register_builtin_function(
        "num-floor",
        [number],
        Rc::new(|args, _context| match args[0].as_ref() {
            PyretValue::Number(number) => Ok(Rc::new(PyretValue::Number(
                number.floor().map_err(PyretErrorKind::from)?,
            ))),
            _ => unreachable!(),
        }),
    )?;

    context.registrar.register_builtin_function(
        "num-round",
        [number],
        Rc::new(|args, _context| match args[0].as_ref() {
            PyretValue::Number(number) => Ok(Rc::new(PyretValue::Number(
                number.round().map_err(PyretErrorKind::from)?,
            ))),
            _ => unreachable!(),
        }),
    )?;

    context.registrar.register_builtin_function(
        "num-round-even",
        [number],
        Rc::new(|args, _context| match args[0].as_ref() {
            PyretValue::Number(number) => Ok(Rc::new(PyretValue::Number(
                number.round_even().map_err(PyretErrorKind::from)?,
            ))),
            _ => unreachable!(),
        }),
    )?;

    context.registrar.register_builtin_function(
        "num-log",
        [number],
        Rc::new(|args, _context| match args[0].as_ref() {
            PyretValue::Number(number) => Ok(Rc::new(PyretValue::Number(
                number.log().map_err(PyretErrorKind::from)?,
            ))),
            _ => unreachable!(),
        }),
    )?;

    context.registrar.register_builtin_function(
        "num-exp",
        [number],
        Rc::new(|args, _context| match args[0].as_ref() {
            PyretValue::Number(number) => Ok(Rc::new(PyretValue::Number(
                number.exp().map_err(PyretErrorKind::from)?,
            ))),
            _ => unreachable!(),
        }),
    )?;

    context.registrar.register_builtin_function(
        "num-expn",
        [number, number],
        Rc::new(
            |args, _context| match (args[0].as_ref(), args[1].as_ref()) {
                (PyretValue::Number(left), PyretValue::Number(right)) => Ok(Rc::new(
                    PyretValue::Number(left.expt(right).map_err(PyretErrorKind::from)?),
                )),
                _ => unreachable!(),
            },
        ),
    )?;

    context.registrar.register_builtin_function(
        "num-to-roughnum",
        [number],
        Rc::new(|args, _context| match args[0].as_ref() {
            PyretValue::Number(number) => Ok(Rc::new(PyretValue::Number(
                number.to_rough().map_err(PyretErrorKind::from)?,
            ))),
            _ => unreachable!(),
        }),
    )?;

    context.registrar.register_builtin_function(
        "num-is-integer",
        [number],
        Rc::new(|args, _context| match args[0].as_ref() {
            PyretValue::Number(number) => Ok(Rc::new(PyretValue::Boolean(number.is_integer()))),
            _ => unreachable!(),
        }),
    )?;

    context.registrar.register_builtin_function(
        "num-is-rational",
        [number],
        Rc::new(|args, _context| match args[0].as_ref() {
            PyretValue::Number(number) => Ok(Rc::new(PyretValue::Boolean(number.is_rational()))),
            _ => unreachable!(),
        }),
    )?;

    context.registrar.register_builtin_function(
        "num-is-positive",
        [number],
        Rc::new(|args, _context| match args[0].as_ref() {
            PyretValue::Number(number) => Ok(Rc::new(PyretValue::Boolean(number.is_positive()))),
            _ => unreachable!(),
        }),
    )?;

    context.registrar.register_builtin_function(
        "num-is-negative",
        [number],
        Rc::new(|args, _context| match args[0].as_ref() {
            PyretValue::Number(number) => Ok(Rc::new(PyretValue::Boolean(number.is_negative()))),
            _ => unreachable!(),
        }),
    )?;

    context.registrar.register_builtin_function(
        "num-is-non-positive",
        [number],
        Rc::new(|args, _context| match args[0].as_ref() {
            PyretValue::Number(number) => {
                Ok(Rc::new(PyretValue::Boolean(number.is_non_positive())))
            }
            _ => unreachable!(),
        }),
    )?;

    context.registrar.register_builtin_function(
        "num-is-non-negative",
        [number],
        Rc::new(|args, _context| match args[0].as_ref() {
            PyretValue::Number(number) => {
                Ok(Rc::new(PyretValue::Boolean(number.is_non_negative())))
            }
            _ => unreachable!(),
        }),
    )?;

    context.registrar.register_builtin_function(
        "num-to-string",
        [number],
        Rc::new(|args, _context| match args[0].as_ref() {
            PyretValue::Number(number) => Ok(Rc::new(PyretValue::String(
                number.to_string().into_boxed_str(),
            ))),
            _ => unreachable!(),
        }),
    )?;

    Ok(())
}
