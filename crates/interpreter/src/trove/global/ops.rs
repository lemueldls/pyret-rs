use std::{cell::RefMut, rc::Rc};

use pyret_error::PyretResult;
use pyret_lexer::ast::ExpressionStatement;

use crate::{
    ops,
    value::{registrar::Registrar, PyretValue},
    Context, Interpreter,
};

pub fn register(registrar: &mut Registrar) -> PyretResult<()> {
    let any = &registrar.get_type("Any")?.unwrap();

    registrar.register_builtin_function(
        "_plus",
        [any, any],
        Rc::new(|args, _context| ops::plus(&args[0], &args[1])),
    )?;

    registrar.register_builtin_function(
        "_minus",
        [any, any],
        Rc::new(|args, _context| ops::minus(&args[0], &args[1])),
    )?;

    registrar.register_builtin_function(
        "_times",
        [any, any],
        Rc::new(|args, _context| ops::times(&args[0], &args[1])),
    )?;

    registrar.register_builtin_function(
        "_divide",
        [any, any],
        Rc::new(|args, _context| ops::divide(&args[0], &args[1])),
    )?;

    Ok(())
}
