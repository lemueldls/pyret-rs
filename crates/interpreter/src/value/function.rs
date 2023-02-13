use std::{cell::RefCell, rc::Rc};

use crate::{Context, PyretResult, PyretValue};

pub type FunctionSignature =
    Box<dyn Fn(&[Rc<PyretValue>], Rc<RefCell<Context>>) -> PyretResult<Rc<PyretValue>>>;

pub struct PyretFunction {
    pub name: Box<str>,
    pub length: usize,
    pub body: FunctionSignature,
}

impl PyretFunction {
    #[must_use]
    pub fn new(name: &str, length: usize, body: FunctionSignature) -> Self {
        Self {
            name: Box::from(name),
            length,
            body,
        }
    }

    pub fn call(
        &self,
        args: &[Rc<PyretValue>],
        context: Rc<RefCell<Context>>,
    ) -> PyretResult<Rc<PyretValue>> {
        if args.len() == self.length {
            (self.body)(args, context)
        } else {
            todo!("Incorrect number of arguments.")
        }
    }
}
