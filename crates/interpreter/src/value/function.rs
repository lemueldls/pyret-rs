use std::{cell::RefCell, rc::Rc};

use super::TypePredicate;
use crate::{trove, Context, PyretResult, PyretValue, Register};

pub type FunctionSignature = Rc<
    dyn Fn(&mut dyn Iterator<Item = PyretValue>, Rc<RefCell<Context>>) -> PyretResult<PyretValue>,
>;

#[derive(Clone)]
pub struct PyretFunction {
    pub name: Box<str>,
    pub generic_types: Box<[Box<str>]>,
    pub param_types: Box<[TypePredicate]>,
    pub return_type: TypePredicate,
    pub body: FunctionSignature,
    pub context: Rc<RefCell<Context>>,
}

impl PyretFunction {
    #[must_use]
    pub fn new(
        name: Box<str>,
        generic_types: Box<[Box<str>]>,
        param_types: Box<[TypePredicate]>,
        return_type: TypePredicate,
        body: FunctionSignature,
        context: Rc<RefCell<Context>>,
    ) -> Self {
        Self {
            name,
            generic_types,
            param_types,
            return_type,
            body,
            context,
        }
    }

    pub fn call(&self, args: Vec<PyretValue>, scope_level: usize) -> PyretResult<PyretValue> {
        if args.len() == self.param_types.len() {
            for generic in self.generic_types.iter() {
                let any = trove::global::Any::predicate();

                self.context
                    .register_local_type(generic.clone(), any, scope_level);
            }

            for (arg, predicate) in args.iter().zip(self.param_types.iter()) {
                if !predicate(arg.clone(), Rc::clone(&self.context)) {
                    todo!("Incorrect argument type.")
                }
            }

            let value = (self.body)(&mut args.into_iter(), Rc::clone(&self.context));

            if let Ok(value) = &value {
                if !(self.return_type)(value.clone(), Rc::clone(&self.context)) {
                    todo!("Incorrect return type.")
                }
            }

            value
        } else {
            todo!("Incorrect number of arguments.")
        }
    }
}
