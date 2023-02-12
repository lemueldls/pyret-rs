use std::{collections::HashMap, ops::Deref, rc::Rc};

use crate::value::{function::FunctionSignature, PyretFunction, PyretValue, PyretValueScoped};

#[derive(Default)]
pub struct Registrar {
    declarations: HashMap<Box<str>, PyretValueScoped>,
}

impl Registrar {
    pub fn register_builtin_expr(&mut self, name: &'static str, value: PyretValue) {
        self.declarations.insert(
            Box::from(name),
            PyretValueScoped::new_builtin(Rc::new(value)),
        );
    }

    pub fn register_local_expr(&mut self, name: Box<str>, value: Rc<PyretValue>, depth: usize) {
        if let Some(shadowing) = self.declarations.get(&name) {
            if shadowing.is_builtin {
                todo!("The declaration of {name} shadows a built-in declaration of the same name.");
            } else {
                todo!(
                    "This declaration of a name conflicts with an earlier declaration of the same name:"
                );
            }
        } else {
            self.declarations
                .insert(name, PyretValueScoped::new_local(value, depth));
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
            PyretValueScoped::new_builtin(Rc::new(PyretValue::Function(PyretFunction::new(
                name, length, body,
            )))),
        );
    }

    pub fn register_local_function(
        &mut self,
        name: &str,
        length: usize,
        body: FunctionSignature,
        depth: usize,
    ) {
        self.declarations.insert(
            Box::from(name),
            PyretValueScoped::new_local(
                Rc::new(PyretValue::Function(PyretFunction::new(name, length, body))),
                depth,
            ),
        );
    }

    pub fn pop_scope(&mut self, depth: usize) {
        self.declarations.retain(|_, scoped| scoped.depth != depth);
    }
}

impl Deref for Registrar {
    type Target = HashMap<Box<str>, PyretValueScoped>;

    fn deref(&self) -> &Self::Target {
        &self.declarations
    }
}
