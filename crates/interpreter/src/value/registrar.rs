use std::{collections::HashMap, rc::Rc, sync::Arc};

use super::TypePredicate;
use crate::{
    value::{function::FunctionSignature, PyretFunction, PyretValue, PyretValueScoped},
    PyretResult,
};

enum RegisteredDeclaration {
    Value(Option<PyretValueScoped>),
    Type(TypePredicate),
}

#[derive(Default)]
pub struct Registrar {
    pub depth: usize,
    declarations: HashMap<Box<str>, RegisteredDeclaration>,
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
            RegisteredDeclaration::Value(Some(PyretValueScoped::new_builtin(Rc::clone(&value)))),
        );

        value
    }

    pub fn register_local_expr(&mut self, name: Box<str>, value: Option<Rc<PyretValue>>) {
        if let Some(shadowing) = self.declarations.get(&name) {
            if let RegisteredDeclaration::Value(shadowing) = shadowing {
                if let Some(shadowing) = shadowing {
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
            RegisteredDeclaration::Value(
                value.map(|value| PyretValueScoped::new_local(value, self.depth)),
            ),
        );

        self.depth += 1;
    }

    pub fn register_builtin_function<const N: usize>(
        &mut self,
        name: &'static str,
        param_types: [&TypePredicate; N],
        body: FunctionSignature,
    ) -> PyretResult<FunctionSignature> {
        self.declarations.insert(
            Box::from(name),
            RegisteredDeclaration::Value(Some(PyretValueScoped::new_builtin(Rc::new(
                PyretValue::Function(PyretFunction::new(
                    Box::from(name),
                    Box::from_iter([]),
                    Box::from_iter(param_types.map(Arc::clone)),
                    self.get_type("Any")?.unwrap(),
                    Rc::clone(&body),
                )),
            )))),
        );

        Ok(body)
    }

    pub fn register_function(
        &mut self,
        name: Box<str>,
        param_types: Box<[TypePredicate]>,
        return_type: TypePredicate,
        body: FunctionSignature,
    ) -> PyretResult<FunctionSignature> {
        self.declarations.insert(
            name.clone(),
            RegisteredDeclaration::Value(Some(PyretValueScoped::new_builtin(Rc::new(
                PyretValue::Function(PyretFunction::new(
                    name,
                    Box::from_iter([]),
                    param_types,
                    return_type,
                    Rc::clone(&body),
                )),
            )))),
        );

        Ok(body)
    }

    pub fn register_local_function(
        &mut self,
        name: Box<str>,
        param_types: &[Box<str>],
        return_type: &str,
        body: FunctionSignature,
    ) -> PyretResult<FunctionSignature> {
        let param_types = param_types
            .iter()
            .map(|name| {
                self.get_type(name)?
                    .ok_or_else(|| todo!("The type {name} has not been previously defined."))
            })
            .collect::<PyretResult<_>>()?;

        let return_type = self
            .get_type(return_type)?
            .ok_or_else(|| todo!("The type {return_type} has not been previously defined."))?;

        self.declarations.insert(
            name.clone(),
            RegisteredDeclaration::Value(Some(PyretValueScoped::new_local(
                Rc::new(PyretValue::Function(PyretFunction::new(
                    name,
                    Box::from_iter([]),
                    param_types,
                    return_type,
                    Rc::clone(&body),
                ))),
                self.depth,
            ))),
        );

        self.depth += 1;

        Ok(body)
    }

    pub fn register_builtin_type(
        &mut self,
        name: &'static str,
        predicate: TypePredicate,
    ) -> PyretResult<TypePredicate> {
        self.declarations.insert(
            Box::from(name),
            RegisteredDeclaration::Type(Arc::clone(&predicate)),
        );

        Ok(predicate)
    }

    pub fn register_local_type(&mut self, name: Box<str>, predicate: TypePredicate) {
        if let Some(shadowing) = self.declarations.get(&name) {
            if let RegisteredDeclaration::Type(predicate) = shadowing {
                todo!(
                    "This declaration of a name conflicts with an earlier declaration of the same name:"
                );
            } else {
                todo!("The declaration of {name} is not a type.");
            }
        } else {
            self.declarations
                .insert(name, RegisteredDeclaration::Type(predicate));
        }
    }

    pub fn get_value(&self, name: &str) -> PyretResult<&PyretValueScoped> {
        if let Some(declaration) = self.declarations.get(name) {
            if let RegisteredDeclaration::Value(declaration) = declaration {
                if let Some(value) = &declaration {
                    Ok(value)
                } else {
                    Err(todo!(
                        "The identifier is unbound. Although it has been previously defined, it is being used before it has been is initialized to a value."
                    ))
                }
            } else {
                Err(todo!("The declaration of {name} is not a value."))
            }
        } else {
            Err(todo!(
                "The identifier is unbound. It has not been previously defined."
            ))
        }
    }

    pub fn get_type(&self, name: &str) -> PyretResult<Option<TypePredicate>> {
        if let Some(declaration) = self.declarations.get(name) {
            if let RegisteredDeclaration::Type(predicate) = declaration {
                Ok(Some(Arc::clone(predicate)))
            } else {
                Err(todo!("The declaration of {name} is not a type."))
            }
        } else {
            Ok(None)
        }
    }

    pub fn pop_depth(&mut self, depth: usize) {
        self.declarations.retain(|_, declaration| {
            if let RegisteredDeclaration::Value(declaration) = declaration {
                if declaration.as_ref().unwrap().depth > depth {
                    self.depth -= 1;

                    false
                } else {
                    true
                }
            } else {
                true
            }
        });
    }
}
