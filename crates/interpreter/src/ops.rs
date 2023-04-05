use std::{convert::Into, rc::Rc};

use pyret_error::{PyretErrorKind, PyretGraph, PyretResult};
use pyret_lexer::ast::ExpressionStatement;
use pyret_number::Zero;

use crate::{
    value::{PyretValue, PyretValueKind},
    Interpreter,
};

pub fn plus(left: PyretValue, right: PyretValue) -> PyretResult<PyretValue> {
    match (&*left.kind, &*right.kind) {
        (PyretValueKind::Number(left_number), PyretValueKind::Number(right_number)) => Ok(
            PyretValue::from(PyretValueKind::Number(left_number + right_number)),
        ),
        (PyretValueKind::String(left_string), PyretValueKind::String(right_string)) => {
            let mut string = String::with_capacity(left_string.len() + right_string.len());

            string.push_str(left_string);
            string.push_str(right_string);

            Ok(PyretValue::from(PyretValueKind::String(
                string.into_boxed_str(),
            )))
        }
        _ => todo!(),
    }
}

pub fn minus(left: PyretValue, right: PyretValue) -> PyretResult<PyretValue> {
    match (&*left.kind, &*right.kind) {
        (PyretValueKind::Number(left_number), PyretValueKind::Number(right_number)) => {
            Ok(PyretValue::new(
                left.span.as_ref().unwrap().start..right.span.as_ref().unwrap().end,
                Rc::new(PyretValueKind::Number(left_number - right_number)),
            ))
        }
        _ => todo!(),
    }
}

pub fn times(left: PyretValue, right: PyretValue) -> PyretResult<PyretValue> {
    match (&*left.kind, &*right.kind) {
        (PyretValueKind::Number(left_number), PyretValueKind::Number(right_number)) => Ok(
            PyretValue::from(PyretValueKind::Number(left_number * right_number)),
        ),
        _ => todo!(),
    }
}

pub fn divide(left: PyretValue, right: PyretValue) -> PyretResult<PyretValue> {
    match (&*left.kind, &*right.kind) {
        (PyretValueKind::Number(left_number), PyretValueKind::Number(right_number)) => {
            if right_number.is_zero() {
                Err(PyretErrorKind::DivideByZero {
                    denominator: right.span.map(Into::into),
                })
            } else {
                Ok(PyretValue::from(PyretValueKind::Number(
                    left_number / right_number,
                )))
            }
        }
        _ => todo!(),
    }
}

pub fn less_than(left: PyretValue, right: PyretValue) -> PyretResult<PyretValue> {
    match (&*left.kind, &*right.kind) {
        (PyretValueKind::Number(left_number), PyretValueKind::Number(right_number)) => Ok(
            PyretValue::from(PyretValueKind::Boolean(left_number < right_number)),
        ),
        (PyretValueKind::String(left_string), PyretValueKind::String(right_string)) => Ok(
            PyretValue::from(PyretValueKind::Boolean(left_string < right_string)),
        ),
        _ => todo!(),
    }
}

pub fn less_than_or_equal(left: PyretValue, right: PyretValue) -> PyretResult<PyretValue> {
    match (&*left.kind, &*right.kind) {
        (PyretValueKind::Number(left_number), PyretValueKind::Number(right_number)) => Ok(
            PyretValue::from(PyretValueKind::Boolean(left_number <= right_number)),
        ),
        (PyretValueKind::String(left_string), PyretValueKind::String(right_string)) => Ok(
            PyretValue::from(PyretValueKind::Boolean(left_string <= right_string)),
        ),
        _ => todo!(),
    }
}

pub fn greater_than(left: PyretValue, right: PyretValue) -> PyretResult<PyretValue> {
    match (&*left.kind, &*right.kind) {
        (PyretValueKind::Number(left_number), PyretValueKind::Number(right_number)) => Ok(
            PyretValue::from(PyretValueKind::Boolean(left_number > right_number)),
        ),
        (PyretValueKind::String(left_string), PyretValueKind::String(right_string)) => Ok(
            PyretValue::from(PyretValueKind::Boolean(left_string > right_string)),
        ),
        _ => todo!(),
    }
}

pub fn greater_than_or_equal(left: PyretValue, right: PyretValue) -> PyretResult<PyretValue> {
    match (&*left.kind, &*right.kind) {
        (PyretValueKind::Number(left_number), PyretValueKind::Number(right_number)) => Ok(
            PyretValue::from(PyretValueKind::Boolean(left_number >= right_number)),
        ),
        (PyretValueKind::String(left_string), PyretValueKind::String(right_string)) => Ok(
            PyretValue::from(PyretValueKind::Boolean(left_string >= right_string)),
        ),
        _ => todo!(),
    }
}

pub fn equal(left: PyretValue, right: PyretValue) -> PyretResult<PyretValue> {
    match (&*left.kind, &*right.kind) {
        (PyretValueKind::Number(left_number), PyretValueKind::Number(right_number)) => Ok(
            PyretValue::from(PyretValueKind::Boolean(left_number == right_number)),
        ),
        (PyretValueKind::String(left_string), PyretValueKind::String(right_string)) => Ok(
            PyretValue::from(PyretValueKind::Boolean(left_string == right_string)),
        ),
        (PyretValueKind::Boolean(left_boolean), PyretValueKind::Boolean(right_boolean)) => Ok(
            PyretValue::from(PyretValueKind::Boolean(left_boolean == right_boolean)),
        ),
        _ => Ok(PyretValue::from(PyretValueKind::Boolean(false))),
    }
}

pub fn not_equal(left: PyretValue, right: PyretValue) -> PyretResult<PyretValue> {
    match (&*left.kind, &*right.kind) {
        (PyretValueKind::Number(left_number), PyretValueKind::Number(right_number)) => Ok(
            PyretValue::from(PyretValueKind::Boolean(left_number != right_number)),
        ),
        (PyretValueKind::String(left_string), PyretValueKind::String(right_string)) => Ok(
            PyretValue::from(PyretValueKind::Boolean(left_string != right_string)),
        ),
        (PyretValueKind::Boolean(left_boolean), PyretValueKind::Boolean(right_boolean)) => Ok(
            PyretValue::from(PyretValueKind::Boolean(left_boolean != right_boolean)),
        ),
        _ => Ok(PyretValue::from(PyretValueKind::Boolean(true))),
    }
}

pub fn and<G: PyretGraph>(
    left: ExpressionStatement,
    right: ExpressionStatement,
    interpreter: &mut Interpreter<G>,
) -> PyretResult<PyretValue> {
    match &*interpreter.interpret_expression(left)?.kind {
        PyretValueKind::Boolean(left) => Ok(PyretValue::from(PyretValueKind::Boolean(
            *left
                && match &*interpreter.interpret_expression(right)?.kind {
                    PyretValueKind::Boolean(right) => *right,
                    _ => todo!("Evaluating `and` on non-boolean values"),
                },
        ))),
        _ => todo!("Evaluating `and` on non-boolean values"),
    }
}

pub fn or<G: PyretGraph>(
    left: ExpressionStatement,
    right: ExpressionStatement,
    interpreter: &mut Interpreter<G>,
) -> PyretResult<PyretValue> {
    match &*interpreter.interpret_expression(left)?.kind {
        PyretValueKind::Boolean(left) => Ok(PyretValue::from(PyretValueKind::Boolean(
            *left
                || match &*interpreter.interpret_expression(right)?.kind {
                    PyretValueKind::Boolean(right) => *right,
                    _ => todo!("Evaluating `or` on non-boolean values"),
                },
        ))),
        _ => todo!("Evaluating `or` on non-boolean values"),
    }
}
