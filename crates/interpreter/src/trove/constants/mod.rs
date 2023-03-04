use std::{cell::RefMut, f64::consts};

use pyret_error::PyretResult;
use pyret_number::PyretNumber;

use crate::{value::PyretValue, Context};

pub fn register(context: &mut RefMut<Context>) -> PyretResult<()> {
    context
        .registrar
        .register_builtin_expr("PI", PyretValue::Number(PyretNumber::Rough(consts::PI)));

    Ok(())
}
