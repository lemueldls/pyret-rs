use std::{cell::RefCell, rc::Rc, sync::Arc};

use pyret_error::PyretErrorKind;
use pyret_lexer::{ast::IdentifierExpression, Token};

use super::{PyretValueKind, TypePredicate};
use crate::{
    io::Io,
    trove,
    value::{function::FunctionSignature, PyretFunction, PyretValue, PyretValueScoped},
    PyretResult,
};

#[derive(Clone)]
pub struct RegisteredDeclaration {
    pub name: Box<str>,
    pub declaration: Declaration,
    pub scope_level: usize,
}

impl RegisteredDeclaration {
    #[must_use]
    pub fn new_value(name: Box<str>, value: Option<PyretValueScoped>, scope_level: usize) -> Self {
        Self {
            name,
            declaration: Declaration::Value(value),
            scope_level,
        }
    }

    pub fn new_type(name: Box<str>, predicate: TypePredicate, scope_level: usize) -> Self {
        Self {
            name,
            declaration: Declaration::Type(predicate),
            scope_level,
        }
    }

    #[must_use]
    pub fn with_name(&self, name: Box<str>) -> Self {
        Self {
            name,
            declaration: self.declaration.clone(),
            scope_level: self.scope_level,
        }
    }
}

#[derive(Clone)]
pub enum Declaration {
    Value(Option<PyretValueScoped>),
    Type(TypePredicate),
}

#[derive(Default)]
pub struct Context {
    pub io: Io,
    pub declarations: Vec<RegisteredDeclaration>,
    depth_of_scope: Option<usize>,
}

impl Context {
    fn get_declaration(&mut self, name: &str) -> Option<&mut Declaration> {
        self.declarations
            .iter_mut()
            .enumerate()
            .rfind(|(depth, decl)| {
                name == &*decl.name
                    && match self.depth_of_scope {
                        None => true,
                        Some(depth_of_scope) if depth_of_scope > *depth => true,
                        _ => false,
                    }
            })
            .map(|(_, registered)| &mut registered.declaration)
    }
}

impl Register for Rc<RefCell<Context>> {
    fn register_builtin_expr(&self, name: &'static str, value: PyretValue) {
        self.borrow_mut()
            .declarations
            .push(RegisteredDeclaration::new_value(
                Box::from(name),
                Some(PyretValueScoped::new_builtin(value)),
                0,
            ));
    }

    fn register_local_expr(&self, name: Box<str>, value: Option<PyretValue>, scope_level: usize) {
        if let Some(declaration) = self.borrow_mut().get_declaration(&name) {
            if let Declaration::Value(shadowing) = &declaration {
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
                } else if let Some(value) = value {
                    *declaration = Declaration::Value(Some(PyretValueScoped::new_local(value)));

                    return;
                } else {
                    todo!(
                        "Contracts for functions can only be defined once, and the contract for {name} is already defined: "
                    );
                }
            } else {
                todo!("The declaration of {name} is not a value.");
            }
        }

        let declaration = RegisteredDeclaration::new_value(
            name,
            value.map(PyretValueScoped::new_local),
            scope_level,
        );

        self.borrow_mut().declarations.push(declaration);
    }

    fn register_builtin_function<const N: usize>(
        &self,
        name: &'static str,
        param_types: [&TypePredicate; N],
        body: FunctionSignature,
    ) -> PyretResult<()> {
        let return_type = trove::global::Any::predicate();

        self.borrow_mut()
            .declarations
            .push(RegisteredDeclaration::new_value(
                Box::from(name),
                Some(PyretValueScoped::new_builtin(
                    PyretValue::from(PyretValueKind::Function(PyretFunction::new(
                        Box::from(name),
                        Box::from_iter([]),
                        Box::from_iter(param_types.map(Arc::clone)),
                        return_type,
                        body,
                        Self::clone(self),
                    ))),
                )),
                0,
            ));

        Ok(())
    }

    fn register_function(
        &self,
        name: Box<str>,
        param_types: Box<[TypePredicate]>,
        return_type: TypePredicate,
        body: FunctionSignature,
    ) -> PyretResult<()> {
        self.borrow_mut()
            .declarations
            .push(RegisteredDeclaration::new_value(
                name.clone(),
                Some(PyretValueScoped::new_builtin(
                    PyretValue::from(PyretValueKind::Function(PyretFunction::new(
                        name,
                        Box::from_iter([]),
                        param_types,
                        return_type,
                        body,
                        Self::clone(self),
                    ))),
                )),
                0,
            ));

        Ok(())
    }

    fn register_local_function(
        &self,
        name: Box<str>,
        param_types: &[Box<str>],
        return_type: &str,
        body: FunctionSignature,
        scope_level: usize,
    ) -> PyretResult<()> {
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

        self.borrow_mut()
            .declarations
            .push(RegisteredDeclaration::new_value(
                name.clone(),
                Some(PyretValueScoped::new_local(
                    PyretValue::from(PyretValueKind::Function(PyretFunction::new(
                        name,
                        Box::from_iter([]),
                        param_types,
                        return_type,
                        body,
                        Self::clone(self),
                    ))),
                )),
                scope_level,
            ));

        Ok(())
    }

    fn register_builtin_type(
        &self,
        name: &'static str,
        predicate: TypePredicate,
    ) -> PyretResult<TypePredicate> {
        self.borrow_mut()
            .declarations
            .push(RegisteredDeclaration::new_type(
                Box::from(name),
                Arc::clone(&predicate),
                0,
            ));

        Ok(predicate)
    }

    fn register_local_type(&self, name: Box<str>, predicate: TypePredicate, scope_level: usize) {
        if let Some(declaration) = self.borrow_mut().get_declaration(&name) {
            if let Declaration::Type(_predicate) = &declaration {
                todo!(
                    "This declaration of a name conflicts with an earlier declaration of the same name:"
                );
            } else {
                todo!("The declaration of {name} is not a type.");
            }
        }

        self.borrow_mut()
            .declarations
            .push(RegisteredDeclaration::new_type(
                name,
                predicate,
                scope_level,
            ));
    }

    fn get_value(&self, name: &str) -> PyretResult<PyretValueScoped> {
        if let Some(declaration) = self.borrow_mut().get_declaration(name) {
            if let Declaration::Value(value) = declaration {
                if let Some(value_scoped) = value {
                    Ok(value_scoped.clone())
                } else {
                    Err(todo!(
                        "The identifier is unbound. Although it has been previously defined, it is being used before it has been is initialized to a value: {name}"
                    ))
                }
            } else {
                Err(todo!("The declaration of {name} is not a value."))
            }
        } else {
            Err(todo!(
                "The identifier is unbound. It has not been previously defined: {name}"
            ))
        }
    }

    fn get_type(&self, name: &str) -> PyretResult<Option<TypePredicate>> {
        self.borrow_mut()
            .get_declaration(name)
            .map(|declaration| {
                if let Declaration::Type(predicate) = declaration {
                    Ok(Arc::clone(predicate))
                } else {
                    Err(todo!("The declaration of {name} is not a type."))
                }
            })
            .transpose()
    }

    fn call_function(
        &self,
        ident: IdentifierExpression,
        args: Vec<PyretValue>,
        scope_level: usize,
    ) -> PyretResult<PyretValue> {
        let Some(depth) = self
            .borrow()
            .declarations
            .iter()
            .position(|decl| decl.name == ident.name)
        else {
            todo!(
                "The identifier is unbound. It has not been previously defined: {}",
                ident.name
            )
        };

        self.borrow_mut().depth_of_scope = Some(depth);

        let function = {
            let binding = self.borrow();

            if let Declaration::Value(declaration) = &binding.declarations[depth].declaration {
                if let Some(declaration) = &declaration {
                    if let PyretValueKind::Function(function) = &*declaration.value.kind {
                        Ok(function.clone())
                    } else {
                        Err(PyretErrorKind::InvalidFunctionApplication {
                            span: ident.span().into(),
                        })
                    }
                } else {
                    Err(todo!(
                        "The identifier is unbound. Although it has been previously defined, it is being used before it has been is initialized to a value: {}",
                        ident.name
                    ))
                }
            } else {
                Err(todo!("The declaration of {} is not a value.", ident.name))
            }?
        };

        let result = function.call(args, scope_level)?;

        self.borrow_mut().depth_of_scope = None;

        Ok(result)
    }

    fn pop_scope(&self, scope_level: usize) {
        self.borrow_mut()
            .declarations
            .retain(|registered| registered.scope_level <= scope_level);
    }
}

pub trait Register {
    fn register_builtin_expr(&self, name: &'static str, value: PyretValue);
    fn register_local_expr(&self, name: Box<str>, value: Option<PyretValue>, scope_level: usize);
    fn register_builtin_function<const N: usize>(
        &self,
        name: &'static str,
        param_types: [&TypePredicate; N],
        body: FunctionSignature,
    ) -> PyretResult<()>;
    fn register_function(
        &self,
        name: Box<str>,
        param_types: Box<[TypePredicate]>,
        return_type: TypePredicate,
        body: FunctionSignature,
    ) -> PyretResult<()>;
    fn register_local_function(
        &self,
        name: Box<str>,
        param_types: &[Box<str>],
        return_type: &str,
        body: FunctionSignature,
        scope_level: usize,
    ) -> PyretResult<()>;
    fn register_builtin_type(
        &self,
        name: &'static str,
        predicate: TypePredicate,
    ) -> PyretResult<TypePredicate>;
    fn register_local_type(&self, name: Box<str>, predicate: TypePredicate, scope_level: usize);
    fn get_value(&self, name: &str) -> PyretResult<PyretValueScoped>;
    fn get_type(&self, name: &str) -> PyretResult<Option<TypePredicate>>;
    fn call_function(
        &self,
        ident: IdentifierExpression,
        args: Vec<PyretValue>,
        scope_level: usize,
    ) -> PyretResult<PyretValue>;
    fn pop_scope(&self, scope_level: usize);
}
