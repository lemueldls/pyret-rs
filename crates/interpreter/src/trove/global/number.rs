use std::{cell::RefCell, cmp::Ordering, rc::Rc, sync::Arc};

use pyret_error::PyretErrorKind;
use pyret_number::{PyretNumber, Signed};

use super::{boolean::Boolean, Any};
use crate::{
    ty,
    value::{
        context::{Context, Register},
        PyretValueKind,
    },
    PyretResult, PyretValue,
};

pub fn register(context: Context) -> PyretResult<()> {
    let any = &Any::predicate();

    let number = &context.register_builtin_type(
        "Number",
        Arc::new(|value, _context| matches!(*value.kind, PyretValueKind::Number(..))),
    )?;

    context.register_builtin_type(
        "Exactnum",
        Arc::new(|value, _context| {
            matches!(*value.kind, PyretValueKind::Number(PyretNumber::Exact(..)))
        }),
    )?;

    context.register_builtin_type(
        "Roughnum",
        Arc::new(|value, _context| {
            matches!(*value.kind, PyretValueKind::Number(PyretNumber::Rough(..)))
        }),
    )?;

    let num_integer = &context.register_builtin_type(
        "NumInteger",
        Arc::new(|value, _context| {
            matches!(&*value.kind, PyretValueKind::Number(PyretNumber::Exact(number)) if number.is_integer())
        })
    )?;

    context.register_builtin_type(
        "NumRational",
        Arc::new(|value, _context| {
            matches!(*value.kind, PyretValueKind::Number(PyretNumber::Exact(..)))
        }),
    )?;

    context.register_builtin_type(
        "NumPositive",
        Arc::new(|value, _context| {
            matches!(&*value.kind, PyretValueKind::Number(number) if number.is_positive())
        })
    )?;

    context.register_builtin_type(
        "NumNegative",
        Arc::new(|value, _context| {
            matches!(&*value.kind, PyretValueKind::Number(number) if number.is_negative())
        })
    )?;

    context.register_builtin_type(
        "NumNonPositive",
        Arc::new(|value, _context| {
            matches!(&*value.kind, PyretValueKind::Number(number) if number.is_non_positive())
        })
    )?;

    context.register_builtin_type(
        "NumNonNegative",
        Arc::new(|value, _context| {
            matches!(&*value.kind, PyretValueKind::Number(number) if number.is_non_negative())
        })
    )?;

    context.register_builtin_function(
        "is-number",
        [any],
        Rc::new(|args, _context| {
            Ok(PyretValue::from(PyretValueKind::Boolean(matches!(
                &*args.next().unwrap().kind,
                PyretValueKind::Number(..)
            ))))
        }),
    )?;

    context.register_builtin_function(
        "is-equal",
        [number, number],
        Rc::new(|args, _context| {
            match (&*args.next().unwrap().kind, &*args.next().unwrap().kind) {
                (PyretValueKind::Number(left_number), PyretValueKind::Number(right_number)) => {
                    Ok(PyretValue::from(PyretValueKind::Boolean(
                        left_number
                            .is_equal(right_number)
                            .map_err(PyretErrorKind::RaiseRuntime)?,
                    )))
                }
                _ => unreachable!(),
            }
        }),
    )?;

    context.register_builtin_function(
        "num-max",
        [number, number],
        Rc::new(|args, _context| {
            let left = args.next().unwrap();
            let right = args.next().unwrap();

            match (&*left.kind, &*right.kind) {
                (PyretValueKind::Number(left_number), PyretValueKind::Number(right_number)) => {
                    match left_number.partial_cmp(right_number) {
                        Some(Ordering::Less | Ordering::Equal) => Ok(left),
                        Some(Ordering::Greater) => Ok(right),
                        None => Err(PyretErrorKind::RaiseRuntime(Box::from("roughnum overflow"))),
                    }
                }
                _ => unreachable!(),
            }
        }),
    )?;

    context.register_builtin_function(
        "num-min",
        [number, number],
        Rc::new(|args, _context| {
            let left = args.next().unwrap();
            let right = args.next().unwrap();

            match (&*left.kind, &*right.kind) {
                (PyretValueKind::Number(left_ref), PyretValueKind::Number(right_ref)) => {
                    match left_ref.partial_cmp(right_ref) {
                        Some(Ordering::Less | Ordering::Equal) => Ok(right),
                        Some(Ordering::Greater) => Ok(left),
                        None => Err(PyretErrorKind::RaiseRuntime(Box::from("roughnum overflow"))),
                    }
                }
                _ => unreachable!(),
            }
        }),
    )?;

    context.register_builtin_function(
        "num-abs",
        [number],
        Rc::new(|args, _context| match &*args.next().unwrap().kind {
            PyretValueKind::Number(number) => {
                Ok(PyretValue::from(PyretValueKind::Number(number.abs())))
            }
            _ => unreachable!(),
        }),
    )?;

    context.register_builtin_function(
        "num-sin",
        [number],
        Rc::new(|args, _context| match &*args.next().unwrap().kind {
            PyretValueKind::Number(number) => Ok(PyretValue::from(PyretValueKind::Number(
                number.sin().map_err(PyretErrorKind::RaiseRuntime)?,
            ))),
            _ => unreachable!(),
        }),
    )?;

    context.register_builtin_function(
        "num-cos",
        [number],
        Rc::new(|args, _context| match &*args.next().unwrap().kind {
            PyretValueKind::Number(number) => Ok(PyretValue::from(PyretValueKind::Number(
                number.cos().map_err(PyretErrorKind::RaiseRuntime)?,
            ))),
            _ => unreachable!(),
        }),
    )?;

    context.register_builtin_function(
        "num-tan",
        [number],
        Rc::new(|args, _context| match &*args.next().unwrap().kind {
            PyretValueKind::Number(number) => Ok(PyretValue::from(PyretValueKind::Number(
                number.tan().map_err(PyretErrorKind::RaiseRuntime)?,
            ))),
            _ => unreachable!(),
        }),
    )?;

    context.register_builtin_function(
        "num-asin",
        [number],
        Rc::new(|args, _context| match &*args.next().unwrap().kind {
            PyretValueKind::Number(number) => Ok(PyretValue::from(PyretValueKind::Number(
                number.asin().map_err(PyretErrorKind::RaiseRuntime)?,
            ))),
            _ => unreachable!(),
        }),
    )?;

    context.register_builtin_function(
        "num-acos",
        [number],
        Rc::new(|args, _context| match &*args.next().unwrap().kind {
            PyretValueKind::Number(number) => Ok(PyretValue::from(PyretValueKind::Number(
                number.acos().map_err(PyretErrorKind::RaiseRuntime)?,
            ))),
            _ => unreachable!(),
        }),
    )?;

    context.register_builtin_function(
        "num-atan",
        [number],
        Rc::new(|args, _context| match &*args.next().unwrap().kind {
            PyretValueKind::Number(number) => Ok(PyretValue::from(PyretValueKind::Number(
                number.atan().map_err(PyretErrorKind::RaiseRuntime)?,
            ))),
            _ => unreachable!(),
        }),
    )?;

    context.register_builtin_function(
        "num-atan2",
        [number, number],
        Rc::new(|args, _context| {
            match (&*args.next().unwrap().kind, &*args.next().unwrap().kind) {
                (PyretValueKind::Number(left), PyretValueKind::Number(right)) => {
                    Ok(PyretValue::from(PyretValueKind::Number(
                        left.atan2(right).map_err(PyretErrorKind::RaiseRuntime)?,
                    )))
                }
                _ => unreachable!(),
            }
        }),
    )?;

    context.register_builtin_function(
        "num-modulo",
        [num_integer, num_integer],
        Rc::new(|args, _context| {
            match (&*args.next().unwrap().kind, &*args.next().unwrap().kind) {
                (PyretValueKind::Number(left), PyretValueKind::Number(right)) => {
                    Ok(PyretValue::from(PyretValueKind::Number(
                        left.modulo(right).map_err(PyretErrorKind::RaiseRuntime)?,
                    )))
                }
                _ => unreachable!(),
            }
        }),
    )?;

    context.register_builtin_function(
        "num-truncate",
        [number],
        Rc::new(|args, _context| match &*args.next().unwrap().kind {
            PyretValueKind::Number(number) => Ok(PyretValue::from(PyretValueKind::Number(
                number.truncate().map_err(PyretErrorKind::RaiseRuntime)?,
            ))),
            _ => unreachable!(),
        }),
    )?;

    context.register_builtin_function(
        "num-sqrt",
        [number],
        Rc::new(|args, _context| match &*args.next().unwrap().kind {
            PyretValueKind::Number(number) => Ok(PyretValue::from(PyretValueKind::Number(
                number.sqrt().map_err(PyretErrorKind::RaiseRuntime)?,
            ))),
            _ => unreachable!(),
        }),
    )?;

    context.register_builtin_function(
        "num-sqr",
        [number],
        Rc::new(|args, _context| match &*args.next().unwrap().kind {
            PyretValueKind::Number(number) => Ok(PyretValue::from(PyretValueKind::Number(
                number.sqr().map_err(PyretErrorKind::RaiseRuntime)?,
            ))),
            _ => unreachable!(),
        }),
    )?;

    context.register_builtin_function(
        "num-ceiling",
        [number],
        Rc::new(|args, _context| match &*args.next().unwrap().kind {
            PyretValueKind::Number(number) => Ok(PyretValue::from(PyretValueKind::Number(
                number.ceiling().map_err(PyretErrorKind::RaiseRuntime)?,
            ))),
            _ => unreachable!(),
        }),
    )?;

    context.register_builtin_function(
        "num-floor",
        [number],
        Rc::new(|args, _context| match &*args.next().unwrap().kind {
            PyretValueKind::Number(number) => Ok(PyretValue::from(PyretValueKind::Number(
                number.floor().map_err(PyretErrorKind::RaiseRuntime)?,
            ))),
            _ => unreachable!(),
        }),
    )?;

    context.register_builtin_function(
        "num-round",
        [number],
        Rc::new(|args, _context| match &*args.next().unwrap().kind {
            PyretValueKind::Number(number) => Ok(PyretValue::from(PyretValueKind::Number(
                number.round().map_err(PyretErrorKind::RaiseRuntime)?,
            ))),
            _ => unreachable!(),
        }),
    )?;

    context.register_builtin_function(
        "num-round-even",
        [number],
        Rc::new(|args, _context| match &*args.next().unwrap().kind {
            PyretValueKind::Number(number) => Ok(PyretValue::from(PyretValueKind::Number(
                number.round_even().map_err(PyretErrorKind::RaiseRuntime)?,
            ))),
            _ => unreachable!(),
        }),
    )?;

    context.register_builtin_function(
        "num-log",
        [number],
        Rc::new(|args, _context| match &*args.next().unwrap().kind {
            PyretValueKind::Number(number) => Ok(PyretValue::from(PyretValueKind::Number(
                number.log().map_err(PyretErrorKind::RaiseRuntime)?,
            ))),
            _ => unreachable!(),
        }),
    )?;

    context.register_builtin_function(
        "num-exp",
        [number],
        Rc::new(|args, _context| match &*args.next().unwrap().kind {
            PyretValueKind::Number(number) => Ok(PyretValue::from(PyretValueKind::Number(
                number.exp().map_err(PyretErrorKind::RaiseRuntime)?,
            ))),
            _ => unreachable!(),
        }),
    )?;

    context.register_builtin_function(
        "num-expn",
        [number, number],
        Rc::new(|args, _context| {
            match (&*args.next().unwrap().kind, &*args.next().unwrap().kind) {
                (PyretValueKind::Number(left), PyretValueKind::Number(right)) => {
                    Ok(PyretValue::from(PyretValueKind::Number(
                        left.expt(right).map_err(PyretErrorKind::RaiseRuntime)?,
                    )))
                }
                _ => unreachable!(),
            }
        }),
    )?;

    context.register_builtin_function(
        "num-to-roughnum",
        [number],
        Rc::new(|args, _context| match &*args.next().unwrap().kind {
            PyretValueKind::Number(number) => Ok(PyretValue::from(PyretValueKind::Number(
                number.to_rough().map_err(PyretErrorKind::RaiseRuntime)?,
            ))),
            _ => unreachable!(),
        }),
    )?;

    context.register_builtin_function(
        "num-is-integer",
        [number],
        Rc::new(|args, _context| match &*args.next().unwrap().kind {
            PyretValueKind::Number(number) => Ok(PyretValue::from(PyretValueKind::Boolean(
                number.is_integer(),
            ))),
            _ => unreachable!(),
        }),
    )?;

    context.register_builtin_function(
        "num-is-rational",
        [number],
        Rc::new(|args, _context| match &*args.next().unwrap().kind {
            PyretValueKind::Number(number) => Ok(PyretValue::from(PyretValueKind::Boolean(
                number.is_rational(),
            ))),
            _ => unreachable!(),
        }),
    )?;

    context.register_builtin_function(
        "num-is-positive",
        [number],
        Rc::new(|args, _context| match &*args.next().unwrap().kind {
            PyretValueKind::Number(number) => Ok(PyretValue::from(PyretValueKind::Boolean(
                number.is_positive(),
            ))),
            _ => unreachable!(),
        }),
    )?;

    context.register_builtin_function(
        "num-is-negative",
        [number],
        Rc::new(|args, _context| match &*args.next().unwrap().kind {
            PyretValueKind::Number(number) => Ok(PyretValue::from(PyretValueKind::Boolean(
                number.is_negative(),
            ))),
            _ => unreachable!(),
        }),
    )?;

    context.register_builtin_function(
        "num-is-non-positive",
        [number],
        Rc::new(|args, _context| match &*args.next().unwrap().kind {
            PyretValueKind::Number(number) => Ok(PyretValue::from(PyretValueKind::Boolean(
                number.is_non_positive(),
            ))),
            _ => unreachable!(),
        }),
    )?;

    context.register_builtin_function(
        "num-is-non-negative",
        [number],
        Rc::new(|args, _context| match &*args.next().unwrap().kind {
            PyretValueKind::Number(number) => Ok(PyretValue::from(PyretValueKind::Boolean(
                number.is_non_negative(),
            ))),
            _ => unreachable!(),
        }),
    )?;

    context.register_builtin_function(
        "num-to-string",
        [number],
        Rc::new(|args, _context| match &*args.next().unwrap().kind {
            PyretValueKind::Number(number) => Ok(PyretValue::from(PyretValueKind::String(
                number.to_string().into_boxed_str(),
            ))),
            _ => unreachable!(),
        }),
    )?;

    Ok(())
}

ty!(Number, |value, _context| matches!(
    *value.kind,
    PyretValueKind::Number(..)
));

ty!(Exactnum, |value, _context| matches!(
    *value.kind,
    PyretValueKind::Number(PyretNumber::Exact(..))
));

ty!(Roughnum, |value, _context| matches!(
    *value.kind,
    PyretValueKind::Number(PyretNumber::Rough(..))
));

ty!(NumInteger, |value, _context| matches!(
    &*value.kind,
    PyretValueKind::Number(PyretNumber::Exact(number)) if number.is_integer()
));

ty!(NumRational, |value, _context| matches!(
    *value.kind,
    PyretValueKind::Number(PyretNumber::Exact(..))
));

ty!(NumPositive, |value, _context| matches!(
    &*value.kind,
    PyretValueKind::Number(number) if number.is_positive()
));

ty!(NumNegative, |value, _context| matches!(
    &*value.kind,
    PyretValueKind::Number(number) if number.is_negative()
));

ty!(NumNonPositive, |value, _context| matches!(
    &*value.kind,
    PyretValueKind::Number(number) if number.is_non_positive()
));

ty!(NumNonNegative, |value, _context| matches!(
    &*value.kind,
    PyretValueKind::Number(number) if number.is_non_negative()
));

struct ModNumber;

#[module]
impl ModNumber {
    pub fn is_number(value: &Number) -> Boolean {
        Boolean(PyretValue::from(PyretValueKind::Boolean(matches!(
            *value.kind,
            PyretValueKind::Number(..)
        ))))
    }

    pub fn num_max(left: Number, right: Number) -> PyretResult<Number> {
        let rc_left = left.0;
        let rc_right = right.0;

        match (&*rc_left.kind, &*rc_right.kind) {
            (PyretValueKind::Number(left), PyretValueKind::Number(right)) => {
                match left.partial_cmp(right) {
                    Some(Ordering::Less | Ordering::Equal) => Ok(Number(rc_left)),
                    Some(Ordering::Greater) => Ok(Number(rc_right)),
                    None => Err(PyretErrorKind::RaiseRuntime(Box::from("roughnum overflow"))),
                }
            }
            _ => unreachable!(),
        }
    }
}
