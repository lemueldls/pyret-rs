#![feature(once_cell, array_try_map)]

mod context;
pub mod io;
pub mod ops;
pub mod trove;
pub mod value;
#[macro_use]
pub mod macros;

#[macro_use]
extern crate pyret_interpreter_macros;

use std::{borrow::Borrow, cell::RefCell, rc::Rc};

pub use context::Context;
use pyret_error::{PyretError, PyretErrorKind, PyretResult};
pub use pyret_file::graph::PyretGraph;
use pyret_lexer::ast::LetDeclarationKind;
pub use pyret_lexer::{ast, lex, Token};
use value::PyretValue;

pub struct Interpreter {
    pub graph: Box<dyn PyretGraph>,
    pub context: Rc<RefCell<Context>>,
}

impl Interpreter {
    #[must_use]
    pub fn new(graph: impl PyretGraph + 'static) -> Self {
        Self {
            graph: Box::new(graph),
            context: Rc::new(RefCell::new(Context::default())),
        }
    }

    #[must_use]
    pub fn depth(&self) -> usize {
        self.context.as_ref().borrow().registrar.depth
    }

    pub fn use_context(&self, name: &str) -> PyretResult<()> {
        trove::import_trove(name, &mut self.context.as_ref().borrow_mut().registrar)?;

        Ok(())
    }

    pub fn interpret(&mut self, file_id: usize) -> Result<Box<[Rc<PyretValue>]>, Vec<PyretError>> {
        let stmts = match lex(&self.graph.get(file_id).source) {
            Ok(tokens) => tokens,
            Err(errors) => {
                return Err(errors
                    .into_iter()
                    .map(|error| PyretError::new(error, file_id))
                    .collect());
            }
        };

        match self.interpret_block(stmts, self.depth()) {
            Ok(values) => Ok(values),
            Err(error) => Err(vec![PyretError::new(error, file_id)]),
        }
    }

    fn interpret_block(
        &mut self,
        block: Vec<ast::Statement>,
        depth: usize,
    ) -> PyretResult<Box<[Rc<PyretValue>]>> {
        let values = block
            .into_iter()
            .map(|token| match token {
                ast::Statement::Symbol(symbol) => todo!("Unexpected symbol: {symbol:?}"),
                ast::Statement::Declaration(decl) => {
                    self.interpret_declaration(decl)?;

                    Ok(None)
                }
                ast::Statement::Import(import) => todo!("Import: {import:?}"),
                ast::Statement::Expression(expr) => {
                    let expression = self.interpret_expression(expr)?;

                    Ok(Some(expression))
                }
            })
            .filter_map(Result::transpose)
            .collect();

        self.context
            .as_ref()
            .borrow_mut()
            .registrar
            .pop_depth(depth);

        values
    }

    fn interpret_expression(
        &mut self,
        expr: ast::ExpressionStatement,
    ) -> PyretResult<Rc<PyretValue>> {
        match expr {
            ast::ExpressionStatement::Application(app) => {
                let args = app
                    .args
                    .into_iter()
                    .map(|arg| self.interpret_expression(arg))
                    .collect::<PyretResult<Vec<_>>>()?;

                let context = self.context.as_ref().borrow();

                let declaration = context.registrar.get_value(&app.ident.name)?;

                match declaration.value.as_ref().borrow() {
                    PyretValue::Function(function) => {
                        function.call(&args, Rc::clone(&self.context))
                    }
                    _ => Err(PyretErrorKind::InvalidFunctionApplication {
                        span: app.ident.span().into(),
                    }),
                }
            }
            ast::ExpressionStatement::Block(block) => {
                let values = self.interpret_block(block.body, self.depth())?;

                Ok(Rc::clone(values.last().unwrap()))
            }
            ast::ExpressionStatement::Literal(literal) => match literal {
                ast::LiteralExpression::Number(number) => {
                    Ok(Rc::new(PyretValue::Number(number.value)))
                }
                ast::LiteralExpression::String(string) => {
                    Ok(Rc::new(PyretValue::String(string.value)))
                }
                ast::LiteralExpression::Boolean(boolean) => {
                    Ok(Rc::new(PyretValue::Boolean(boolean.value)))
                }
            },
            ast::ExpressionStatement::Identifier(ident) => {
                let name = &*ident.name;

                let context = self.context.borrow_mut();
                let declaration = context.registrar.get_value(name)?;

                Ok(Rc::clone(&declaration.value))
            }
            ast::ExpressionStatement::BinaryOperator(binary_op) => match binary_op.operator {
                ast::BinaryOperation::And => ops::and(*binary_op.left, *binary_op.right, self),
                ast::BinaryOperation::Or => ops::or(*binary_op.left, *binary_op.right, self),
                _ => {
                    let left = self.interpret_expression(*binary_op.left)?;
                    let left = left.as_ref();

                    let right = self.interpret_expression(*binary_op.right)?;
                    let right = right.as_ref();

                    match binary_op.operator {
                        ast::BinaryOperation::Plus => ops::plus(left, right),
                        ast::BinaryOperation::Minus => ops::minus(left, right),
                        ast::BinaryOperation::Times => ops::times(left, right),
                        ast::BinaryOperation::Divide => ops::divide(left, right),
                        ast::BinaryOperation::LessThan => ops::less_than(left, right),
                        ast::BinaryOperation::LessThanOrEqual => {
                            ops::less_than_or_equal(left, right)
                        }
                        ast::BinaryOperation::GreaterThan => ops::greater_than(left, right),
                        ast::BinaryOperation::GreaterThanOrEqual => {
                            ops::greater_than_or_equal(left, right)
                        }
                        ast::BinaryOperation::Equal => ops::equal(left, right),
                        ast::BinaryOperation::NotEqual => ops::not_equal(left, right),
                        ast::BinaryOperation::Is => ops::is(left, right),
                        ast::BinaryOperation::IsRoughly => ops::is_roughly(left, right),
                        _ => unreachable!(),
                    }
                }
            },
            ast::ExpressionStatement::Parenthesis(paren) => self.interpret_expression(*paren.expr),
            ast::ExpressionStatement::Dot(..) => todo!(),
        }
    }

    fn interpret_declaration(&mut self, decl: ast::DeclarationStatement) -> PyretResult<()> {
        match decl {
            ast::DeclarationStatement::Check(check) => {
                if let Some(label) = check.label {
                    println!("Check block: {label:?}");
                }

                self.interpret_block(check.body, self.depth())?;

                println!();
            }
            ast::DeclarationStatement::Let(var) => {
                if var.kind == LetDeclarationKind::RecursiveLet {
                    self.context
                        .borrow_mut()
                        .registrar
                        .register_local_expr(var.ident.name.clone(), None);
                }

                let value = self.interpret_expression(var.init)?;

                self.type_check_identifier(&var.ident, &value)?;

                self.context
                    .borrow_mut()
                    .registrar
                    .register_local_expr(var.ident.name, Some(value));
            }
        }

        Ok(())
    }

    fn type_check_identifier(
        &self,
        ident: &ast::IdentifierExpression,
        value: &Rc<PyretValue>,
    ) -> PyretResult<()> {
        if let Some(annotation) = &ident.annotation {
            match &annotation.value {
                ast::AnnotationType::NameAnnotation {
                    name,
                    parameters,
                    predicate,
                } => match name {
                    ast::IdentifierAnnotation::Name(ident) => {
                        if let Some(r#type) = self
                            .context
                            .as_ref()
                            .borrow()
                            .registrar
                            .get_type(&ident.name)?
                        {
                            if !r#type(Rc::clone(value), Rc::clone(&self.context)) {
                                todo!("Type error: {annotation:?}")
                            }
                        }

                        Ok(())
                    }
                    ast::IdentifierAnnotation::Dot(..) => todo!("Type annotation: {annotation:?}"),
                },
                _ => todo!("Type annotation: {annotation:?}"),
            }
        } else {
            Ok(())
        }
    }
}
