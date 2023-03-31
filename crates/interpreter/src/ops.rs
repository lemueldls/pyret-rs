use std::{cell::RefMut, rc::Rc};

use pyret_error::PyretResult;
use pyret_lexer::ast::ExpressionStatement;

use crate::{value::PyretValue, Context, Interpreter};

pub fn plus(left: &PyretValue, right: &PyretValue) -> PyretResult<Rc<PyretValue>> {
    match (left, right) {
        (PyretValue::Number(left_number), PyretValue::Number(right_number)) => {
            Ok(Rc::new(PyretValue::Number(left_number + right_number)))
        }
        (PyretValue::String(left_string), PyretValue::String(right_string)) => {
            let mut string = String::with_capacity(left_string.len() + right_string.len());

            string.push_str(left_string);
            string.push_str(right_string);

            Ok(Rc::new(PyretValue::String(string.into_boxed_str())))
        }
        _ => todo!(),
    }
}

pub fn minus(left: &PyretValue, right: &PyretValue) -> PyretResult<Rc<PyretValue>> {
    match (left, right) {
        (PyretValue::Number(left_number), PyretValue::Number(right_number)) => {
            Ok(Rc::new(PyretValue::Number(left_number - right_number)))
        }
        _ => todo!(),
    }
}

pub fn times(left: &PyretValue, right: &PyretValue) -> PyretResult<Rc<PyretValue>> {
    match (left, right) {
        (PyretValue::Number(left_number), PyretValue::Number(right_number)) => {
            Ok(Rc::new(PyretValue::Number(left_number * right_number)))
        }
        _ => todo!(),
    }
}

pub fn divide(left: &PyretValue, right: &PyretValue) -> PyretResult<Rc<PyretValue>> {
    match (left, right) {
        (PyretValue::Number(left_number), PyretValue::Number(right_number)) => {
            Ok(Rc::new(PyretValue::Number(left_number / right_number)))
        }
        _ => todo!(),
    }
}

pub fn less_than(left: &PyretValue, right: &PyretValue) -> PyretResult<Rc<PyretValue>> {
    match (left, right) {
        (PyretValue::Number(left_number), PyretValue::Number(right_number)) => {
            Ok(Rc::new(PyretValue::Boolean(left_number < right_number)))
        }
        (PyretValue::String(left_string), PyretValue::String(right_string)) => {
            Ok(Rc::new(PyretValue::Boolean(left_string < right_string)))
        }
        _ => todo!(),
    }
}

pub fn less_than_or_equal(left: &PyretValue, right: &PyretValue) -> PyretResult<Rc<PyretValue>> {
    match (left, right) {
        (PyretValue::Number(left_number), PyretValue::Number(right_number)) => {
            Ok(Rc::new(PyretValue::Boolean(left_number <= right_number)))
        }
        (PyretValue::String(left_string), PyretValue::String(right_string)) => {
            Ok(Rc::new(PyretValue::Boolean(left_string <= right_string)))
        }
        _ => todo!(),
    }
}

pub fn greater_than(left: &PyretValue, right: &PyretValue) -> PyretResult<Rc<PyretValue>> {
    match (left, right) {
        (PyretValue::Number(left_number), PyretValue::Number(right_number)) => {
            Ok(Rc::new(PyretValue::Boolean(left_number > right_number)))
        }
        (PyretValue::String(left_string), PyretValue::String(right_string)) => {
            Ok(Rc::new(PyretValue::Boolean(left_string > right_string)))
        }
        _ => todo!(),
    }
}

pub fn greater_than_or_equal(left: &PyretValue, right: &PyretValue) -> PyretResult<Rc<PyretValue>> {
    match (left, right) {
        (PyretValue::Number(left_number), PyretValue::Number(right_number)) => {
            Ok(Rc::new(PyretValue::Boolean(left_number >= right_number)))
        }
        (PyretValue::String(left_string), PyretValue::String(right_string)) => {
            Ok(Rc::new(PyretValue::Boolean(left_string >= right_string)))
        }
        _ => todo!(),
    }
}

pub fn equal(left: &PyretValue, right: &PyretValue) -> PyretResult<Rc<PyretValue>> {
    match (left, right) {
        (PyretValue::Number(left_number), PyretValue::Number(right_number)) => {
            Ok(Rc::new(PyretValue::Boolean(left_number == right_number)))
        }
        (PyretValue::String(left_string), PyretValue::String(right_string)) => {
            Ok(Rc::new(PyretValue::Boolean(left_string == right_string)))
        }
        (PyretValue::Boolean(left_boolean), PyretValue::Boolean(right_boolean)) => {
            Ok(Rc::new(PyretValue::Boolean(left_boolean == right_boolean)))
        }
        _ => Ok(Rc::new(PyretValue::Boolean(false))),
    }
}

pub fn not_equal(left: &PyretValue, right: &PyretValue) -> PyretResult<Rc<PyretValue>> {
    match (left, right) {
        (PyretValue::Number(left_number), PyretValue::Number(right_number)) => {
            Ok(Rc::new(PyretValue::Boolean(left_number != right_number)))
        }
        (PyretValue::String(left_string), PyretValue::String(right_string)) => {
            Ok(Rc::new(PyretValue::Boolean(left_string != right_string)))
        }
        (PyretValue::Boolean(left_boolean), PyretValue::Boolean(right_boolean)) => {
            Ok(Rc::new(PyretValue::Boolean(left_boolean != right_boolean)))
        }
        _ => Ok(Rc::new(PyretValue::Boolean(true))),
    }
}

pub fn and(
    left: ExpressionStatement,
    right: ExpressionStatement,
    interpreter: &mut Interpreter,
) -> PyretResult<Rc<PyretValue>> {
    match interpreter.interpret_expression(left)?.as_ref() {
        PyretValue::Boolean(left) => Ok(Rc::new(PyretValue::Boolean(
            *left
                && match interpreter.interpret_expression(right)?.as_ref() {
                    PyretValue::Boolean(right) => *right,
                    _ => todo!("Evaluating `and` on non-boolean values"),
                },
        ))),
        _ => todo!("Evaluating `and` on non-boolean values"),
    }
}

pub fn or(
    left: ExpressionStatement,
    right: ExpressionStatement,
    interpreter: &mut Interpreter,
) -> PyretResult<Rc<PyretValue>> {
    match interpreter.interpret_expression(left)?.as_ref() {
        PyretValue::Boolean(left) => Ok(Rc::new(PyretValue::Boolean(
            *left
                || match interpreter.interpret_expression(right)?.as_ref() {
                    PyretValue::Boolean(right) => *right,
                    _ => todo!("Evaluating `or` on non-boolean values"),
                },
        ))),
        _ => todo!("Evaluating `or` on non-boolean values"),
    }
}
