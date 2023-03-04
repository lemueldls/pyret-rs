use std::{collections::HashMap, rc::Rc};

use super::TypePredicate;
use crate::{
    value::{function::FunctionSignature, PyretFunction, PyretValue, PyretValueScoped},
    PyretResult,
};

enum RegistrarDeclaration {
    Value(PyretValueScoped),
    Type(TypePredicate),
}

#[derive(Default)]
pub struct Registrar {
    declarations: HashMap<Box<str>, RegistrarDeclaration>,
}

impl Registrar {
    pub fn register_builtin_expr(
        &mut self,
        name: &'static str,
        value: PyretValue,
    ) -> Rc<PyretValue> {
        let value = Rc::new(value);

        self.declarations.insert(
            Box::from(name),
            RegistrarDeclaration::Value(PyretValueScoped::new_builtin(Rc::clone(&value))),
        );

        value
    }

    pub fn register_local_expr(
        &mut self,
        name: Box<str>,
        value: Option<Rc<PyretValue>>,
        depth: usize,
    ) {
        if let Some(shadowing) = self.declarations.get(&name) {
            if let RegistrarDeclaration::Value(shadowing) = shadowing {
                if shadowing.value.is_some() {
                    if shadowing.is_builtin {
                        todo!(
                            "The declaration of {name} shadows a built-in declaration of the same name."
                        );
                    } else {
                        todo!(
                            "This declaration of a name conflicts with an earlier declaration of the same name:"
                        );
                    }
                }
            } else {
                todo!("The declaration of {name} is not a value.");
            }
        }

        self.declarations.insert(
            name,
            RegistrarDeclaration::Value(PyretValueScoped::new_local(value, depth)),
        );
    }

    pub fn register_builtin_function<const N: usize>(
        &mut self,
        name: &'static str,
        param_types: [&TypePredicate; N],
        body: FunctionSignature,
    ) -> PyretResult<FunctionSignature> {
        self.declarations.insert(
            Box::from(name),
            RegistrarDeclaration::Value(PyretValueScoped::new_builtin(Rc::new(
                PyretValue::Function(PyretFunction::new(
                    Box::from(name),
                    Box::from_iter([]),
                    Box::from_iter(param_types.map(Rc::clone)),
                    Rc::clone(&self.get_type("Any")?.unwrap()),
                    Rc::clone(&body),
                )),
            ))),
        );

        Ok(body)
    }

    pub fn register_builtin_type(
        &mut self,
        name: &'static str,
        predicate: TypePredicate,
    ) -> PyretResult<TypePredicate> {
        self.declarations.insert(
            Box::from(name),
            RegistrarDeclaration::Type(Rc::clone(&predicate)),
        );

        Ok(predicate)
    }

    pub fn register_local_type(&mut self, name: Box<str>, predicate: TypePredicate) {
        if let Some(shadowing) = self.declarations.get(&name) {
            if let RegistrarDeclaration::Type(predicate) = shadowing {
                todo!(
                    "This declaration of a name conflicts with an earlier declaration of the same name:"
                );
            } else {
                todo!("The declaration of {name} is not a type.");
            }
        } else {
            self.declarations
                .insert(Box::from(name), RegistrarDeclaration::Type(predicate));
        }
    }

    pub fn get_value(&self, name: &str) -> PyretResult<Option<&Rc<PyretValue>>> {
        if let Some(declaration) = self.declarations.get(name) {
            if let RegistrarDeclaration::Value(declaration) = declaration {
                if let Some(value) = &declaration.value {
                    Ok(Some(value))
                } else {
                    Err(todo!(
                        "The identifier is unbound. Although it has been previously defined, it is being used before it has been is initialized to a value."
                    ))
                }
            } else {
                Err(todo!("The declaration of {name} is not a value."))
            }
        } else {
            Ok(None)
        }
    }

    pub fn get_type(&self, name: &str) -> PyretResult<Option<TypePredicate>> {
        if let Some(declaration) = self.declarations.get(name) {
            if let RegistrarDeclaration::Type(predicate) = declaration {
                Ok(Some(Rc::clone(predicate)))
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
