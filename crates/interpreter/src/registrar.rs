use std::{collections::HashMap, ops::Deref, rc::Rc};

use crate::value::{function::FunctionSignature, PyretFunction, PyretValue, PyretValueScoped};

#[derive(Default)]
pub struct Registrar {
    declarations: HashMap<Box<str>, PyretValueScoped>,
}

impl Registrar {
    pub fn register_builtin_expr(&mut self, name: &'static str, value: PyretValue) {
        self.declarations
            .insert(Box::from(name), PyretValueScoped::Builtin(Rc::new(value)));
    }

    pub fn register_local_expr(&mut self, name: Box<str>, value: Rc<PyretValue>) {
        if let Some(shadowing) = self.declarations.get(&name) {
            match shadowing {
                PyretValueScoped::Local(..) => todo!(
                    "This declaration of a name conflicts with an earlier declaration of the same name:"
                ),
                PyretValueScoped::Builtin(..) => todo!(
                    "The declaration of {name} shadows a built-in declaration of the same name."
                ),
            }
        } else {
            self.declarations
                .insert(name, PyretValueScoped::Local(value));
        }
    }

    pub fn register_builtin_function(
        &mut self,
        name: &'static str,
        length: usize,
        body: FunctionSignature,
    ) {
        self.declarations.insert(
            Box::from(name),
            PyretValueScoped::Builtin(Rc::new(PyretValue::Function(PyretFunction::new(
                name, length, body,
            )))),
        );
    }

    pub fn register_local_function(&mut self, name: &str, length: usize, body: FunctionSignature) {
        self.declarations.insert(
            Box::from(name),
            PyretValueScoped::Local(Rc::new(PyretValue::Function(PyretFunction::new(
                name, length, body,
            )))),
        );
    }
}

impl Deref for Registrar {
    type Target = HashMap<Box<str>, PyretValueScoped>;

    fn deref(&self) -> &Self::Target {
        &self.declarations
    }
}
