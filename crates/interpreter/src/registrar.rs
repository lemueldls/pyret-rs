use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    value::{function::FunctionSignature, PyretFunction, PyretValue, PyretValueScoped},
    Context, PyretResult,
};

type TypePredicate = Box<dyn Fn(&PyretValue, Rc<RefCell<Context>>) -> bool>;

enum RegistrarDeclaration {
    Value(PyretValueScoped),
    Type(TypePredicate),
}

#[derive(Default)]
pub struct Registrar {
    declarations: HashMap<Box<str>, RegistrarDeclaration>,
}

impl Registrar {
    pub fn register_builtin_expr(&mut self, name: &'static str, value: PyretValue) {
        self.declarations.insert(
            Box::from(name),
            RegistrarDeclaration::Value(PyretValueScoped::new_builtin(Rc::new(value))),
        );
    }

    pub fn register_local_expr(&mut self, name: Box<str>, value: Rc<PyretValue>, depth: usize) {
        if let Some(shadowing) = self.declarations.get(&name) {
            if let RegistrarDeclaration::Value(value) = shadowing {
                if value.is_builtin {
                    todo!(
                        "The declaration of {name} shadows a built-in declaration of the same name."
                    );
                } else {
                    todo!(
                        "This declaration of a name conflicts with an earlier declaration of the same name:"
                    );
                }
            } else {
                todo!("The declaration of {name} is not a value.");
            }
        } else {
            self.declarations.insert(
                name,
                RegistrarDeclaration::Value(PyretValueScoped::new_local(value, depth)),
            );
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
            RegistrarDeclaration::Value(PyretValueScoped::new_builtin(Rc::new(
                PyretValue::Function(PyretFunction::new(name, length, body)),
            ))),
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
            RegistrarDeclaration::Value(PyretValueScoped::new_local(
                Rc::new(PyretValue::Function(PyretFunction::new(name, length, body))),
                depth,
            )),
        );
    }

    pub fn register_type(&mut self, name: Box<str>, predicate: TypePredicate) {
        self.declarations
            .insert(name, RegistrarDeclaration::Type(predicate));
    }

    pub fn get_value(&self, name: &str) -> PyretResult<Option<&PyretValueScoped>> {
        if let Some(declaration) = self.declarations.get(name) {
            if let RegistrarDeclaration::Value(value) = declaration {
                Ok(Some(value))
            } else {
                Err(todo!("The declaration of {name} is not a value."))
            }
        } else {
            Ok(None)
        }
    }

    pub fn get_type(&self, name: &str) -> PyretResult<Option<&TypePredicate>> {
        if let Some(declaration) = self.declarations.get(name) {
            if let RegistrarDeclaration::Type(predicate) = declaration {
                Ok(Some(predicate))
            } else {
                Err(todo!("The declaration of {name} is not a type."))
            }
        } else {
            Ok(None)
        }
    }

    pub fn pop_scope(&mut self, depth: usize) {
        self.declarations.retain(|_, declaration| {
            if let RegistrarDeclaration::Value(value) = declaration {
                value.depth != depth
            } else {
                true
            }
        });
    }
}
