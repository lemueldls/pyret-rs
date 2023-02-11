use std::{cell::RefCell, f64::consts};

use pyret_number::PyretNumber;

use crate::{context::Context, PyretResult, PyretValue, Rc};

pub fn register(context: &Rc<RefCell<Context>>) {
    let mut context = context.as_ref().borrow_mut();

    context
        .registrar
        .register_builtin_expr("PI", PyretValue::Number(PyretNumber::Rough(consts::PI)));

    context.registrar.register_builtin_function(
        "_plus",
        2,
        Box::new(|args, _context| plus(&args[0], &args[1])),
    );

    context.registrar.register_builtin_function(
        "_minus",
        2,
        Box::new(|args, _context| minus(&args[0], &args[1])),
    );

    context.registrar.register_builtin_function(
        "_times",
        2,
        Box::new(|args, _context| times(&args[0], &args[1])),
    );

    context.registrar.register_builtin_function(
        "_divide",
        2,
        Box::new(|args, _context| divide(&args[0], &args[1])),
    );
}

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
