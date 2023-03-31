use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};

use pyret_error::PyretResult;
use pyret_lexer::ast::ExpressionStatement;

use super::Any;
use crate::{ops, value::PyretValue, Context, Interpreter, Register};

pub fn register(context: Rc<RefCell<Context>>) -> PyretResult<()> {
    let any = &Any::predicate();

    context.register_builtin_function(
        "_plus",
        [any, any],
        Rc::new(|args, _context| ops::plus(&args[0], &args[1])),
    )?;

    context.register_builtin_function(
        "_minus",
        [any, any],
        Rc::new(|args, _context| ops::minus(&args[0], &args[1])),
    )?;

    context.register_builtin_function(
        "_times",
        [any, any],
        Rc::new(|args, _context| ops::times(&args[0], &args[1])),
    )?;

    context.register_builtin_function(
        "_divide",
        [any, any],
        Rc::new(|args, _context| ops::divide(&args[0], &args[1])),
    )?;

    Ok(())
}
