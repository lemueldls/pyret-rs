use std::{cell::RefCell, rc::Rc};

use pyret_error::PyretResult;

use super::Any;
use crate::{ops, Context, Register};

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
