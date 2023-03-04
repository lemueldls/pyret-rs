use std::{cell::RefMut, rc::Rc};

use pyret_error::PyretResult;
use pyret_lexer::ast::ExpressionStatement;

use crate::{ops, value::PyretValue, Context, Interpreter};

pub fn register(context: &mut RefMut<Context>) -> PyretResult<()> {
    let any = &context.registrar.get_type("Any")?.unwrap();

    context.registrar.register_builtin_function(
        "_plus",
        [any, any],
        Rc::new(|args, _context| ops::plus(&args[0], &args[1])),
    )?;

    context.registrar.register_builtin_function(
        "_minus",
        [any, any],
        Rc::new(|args, _context| ops::minus(&args[0], &args[1])),
    )?;

    context.registrar.register_builtin_function(
        "_times",
        [any, any],
        Rc::new(|args, _context| ops::times(&args[0], &args[1])),
    )?;

    context.registrar.register_builtin_function(
        "_divide",
        [any, any],
        Rc::new(|args, _context| ops::divide(&args[0], &args[1])),
    )?;

    Ok(())
}
