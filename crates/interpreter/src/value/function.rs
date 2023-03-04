use std::{cell::RefCell, rc::Rc};

use super::TypePredicate;
use crate::{Context, PyretResult, PyretValue};

pub type FunctionSignature =
    Rc<dyn Fn(&[Rc<PyretValue>], Rc<RefCell<Context>>) -> PyretResult<Rc<PyretValue>>>;

pub struct PyretFunction {
    pub name: Box<str>,
    pub generic_types: Box<[Box<str>]>,
    pub param_types: Box<[TypePredicate]>,
    pub return_type: TypePredicate,
    pub body: FunctionSignature,
}

impl PyretFunction {
    #[must_use]
    pub fn new(
        name: Box<str>,
        generic_types: Box<[Box<str>]>,
        param_types: Box<[TypePredicate]>,
        return_type: TypePredicate,
        body: FunctionSignature,
    ) -> Self {
        Self {
            name,
            generic_types,
            param_types,
            return_type,
            body,
        }
    }

    pub fn call(
        &self,
        args: &[Rc<PyretValue>],
        context: Rc<RefCell<Context>>,
    ) -> PyretResult<Rc<PyretValue>> {
        if args.len() == self.param_types.len() {
            for generic in self.generic_types.iter() {
                let any = Rc::clone(&context.borrow().registrar.get_type("Any")?.unwrap());

                context
                    .borrow_mut()
                    .registrar
                    .register_local_type(generic.clone(), any);
            }

            for (arg, predicate) in args.iter().zip(self.param_types.iter()) {
                if !predicate(Rc::clone(arg), Rc::clone(&context)) {
                    todo!("Incorrect argument type.")
                }
            }

            let value = (self.body)(args, Rc::clone(&context));

            if let Ok(value) = &value {
                if !(self.return_type)(Rc::clone(value), Rc::clone(&context)) {
                    todo!("Incorrect return type.")
                }
            }

            value
        } else {
            todo!("Incorrect number of arguments.")
        }
    }
}
