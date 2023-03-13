use std::{cell::RefMut, f64::consts};

use pyret_error::PyretResult;
use pyret_number::PyretNumber;

use crate::{
    value::{registrar::Registrar, PyretValue},
    Context,
};

pub fn register(registrar: &mut Registrar) -> PyretResult<()> {
    registrar.register_builtin_expr("PI", PyretValue::Number(PyretNumber::Rough(consts::PI)));

    Ok(())
}
