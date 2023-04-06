#![feature(lazy_cell)]

pub mod io;
pub mod ops;
pub mod trove;
pub mod value;
#[macro_use]
pub mod macros;

#[macro_use]
extern crate pyret_interpreter_macros;

use std::{cell::RefCell, collections::HashMap, ops::RangeInclusive, rc::Rc};

use io::Output;
use pyret_error::{PyretError, PyretResult};
pub use pyret_file::graph::PyretGraph;
use pyret_lexer::ast::LetDeclarationKind;
pub use pyret_lexer::{ast, lex, Token};
use value::{
    context::{Context, Declaration, Register, RegisteredDeclaration},
    PyretValue, PyretValueKind,
};

pub struct TestResult {
    pub passed: bool,
    pub left_span: RangeInclusive<usize>,
    pub left_value: PyretValue,
    pub right_span: RangeInclusive<usize>,
    pub right_value: PyretValue,
    pub test_span: RangeInclusive<usize>,
}

pub struct Interpreter<G: PyretGraph> {
    pub graph: G,
    pub context: Context,
    pub provide_values: ast::ProvideValues,
    pub provide_types: ast::ProvideTypes,
    scope_level: usize,
}

impl<G: PyretGraph> Interpreter<G> {
    #[must_use]
    pub fn new(graph: G) -> Self {
        Self {
            graph,
            context: Context::default(),
            provide_values: ast::ProvideValues::Identifiers(HashMap::new()),
            provide_types: ast::ProvideTypes::Wildcard,
            scope_level: 0,
        }
    }

    pub fn import_trove(&mut self, name: &str) -> PyretResult<()> {
        trove::import_trove(name, self.context.clone())?;

        Ok(())
    }

    #[must_use]
    pub fn get_provided(&self) -> Vec<RegisteredDeclaration> {
        self.context
            .as_ref()
            .borrow()
            .declarations
            .iter()
            .filter_map(|registered| match registered.declaration {
                Declaration::Value(..) => match &self.provide_values {
                    ast::ProvideValues::Wildcard => Some(registered.clone()),
                    ast::ProvideValues::Identifiers(identifiers) => identifiers
                        .get(&registered.name)
                        .map(|ident| registered.with_name(ident.name.clone())),
                },
                Declaration::Type(..) => match self.provide_types {
                    ast::ProvideTypes::Wildcard => Some(registered.clone()),
                },
            })
            .collect()
    }

    pub fn interpret(&mut self, file_id: usize) -> Result<Vec<PyretValue>, Vec<PyretError>> {
        let stmts = match lex(&self.graph.get(file_id).source) {
            Ok(tokens) => tokens,
            Err(errors) => {
                return Err(errors
                    .into_iter()
                    .map(|error| PyretError::new(error, file_id))
                    .collect());
            }
        };

        match self.interpret_block(stmts) {
            Ok(values) => Ok(values),
            Err(error) => Err(vec![PyretError::new(error, file_id)]),
        }
    }

    fn interpret_block_with<T>(
        &mut self,
        block: Vec<ast::Statement>,
        interpret: fn(&mut Self, ast::Statement) -> PyretResult<Option<T>>,
    ) -> PyretResult<Vec<T>> {
        self.scope_level += 1;

        let values = block
            .into_iter()
            .map(|token| interpret(self, token))
            .filter_map(Result::transpose)
            .collect();

        self.scope_level -= 1;

        if self.scope_level > 0 {
            self.context.pop_scope(self.scope_level);
        }

        values
    }

    pub fn interpret_block(&mut self, block: Vec<ast::Statement>) -> PyretResult<Vec<PyretValue>> {
        self.interpret_block_with(block, Self::interpret_statement)
    }

    pub fn interpret_test_statement(
        &mut self,
        stmt: ast::Statement,
    ) -> PyretResult<Option<TestResult>> {
        Ok(match stmt {
            ast::Statement::Expression(ast::ExpressionStatement::BinaryOperator(binary_op))
                if binary_op.operator.is_testing() =>
            {
                let span = binary_op.start()..=binary_op.end();

                let left_span = binary_op.left.start()..=binary_op.left.end();
                let right_span = binary_op.right.start()..=binary_op.right.end();

                let left = self.interpret_expression(*binary_op.left)?;
                let left_kind = &*left.kind;

                let right = self.interpret_expression(*binary_op.right)?;
                let right_kind = &*right.kind;

                let result = match binary_op.operator {
                    ast::BinaryOperation::Is => left_kind == right_kind,
                    ast::BinaryOperation::IsRoughly => match (left_kind, right_kind) {
                        (
                            PyretValueKind::Number(left_number),
                            PyretValueKind::Number(right_number),
                        ) => left_number.is_roughly(right_number),
                        _ => todo!("Evaluating `is roughly` on non-number values"),
                    },
                    _ => unreachable!(),
                };

                Some(TestResult {
                    passed: result,
                    left_value: left,
                    left_span,
                    right_value: right,
                    right_span,
                    test_span: span,
                })
            }
            _ => {
                self.interpret_statement(stmt)?;

                None
            }
        })
    }

    fn interpret_statement(&mut self, stmt: ast::Statement) -> PyretResult<Option<PyretValue>> {
        match stmt {
            ast::Statement::Symbol(symbol) => todo!("Unexpected symbol: {symbol:?}"),
            ast::Statement::Declaration(decl) => {
                self.interpret_declaration(decl)?;

                Ok(None)
            }
            ast::Statement::Import(import) => match import.value {
                ast::Import::As {
                    source: _,
                    name: ident,
                } => {
                    if &*ident.name != "_" {
                        todo!()
                    }

                    Ok(None)
                }
                ast::Import::From {
                    names: _,
                    source: _,
                } => todo!(),
            },
            ast::Statement::Expression(expr) => {
                let expression = self.interpret_expression(expr)?;

                Ok(Some(expression))
            }
            ast::Statement::Provide(provide) => {
                match provide.value {
                    ast::Provide::Values(values) => match values {
                        ast::ProvideValues::Wildcard => {
                            self.provide_values = ast::ProvideValues::Wildcard;
                        }
                        ast::ProvideValues::Identifiers(idents) => match &mut self.provide_values {
                            ast::ProvideValues::Wildcard => {}
                            ast::ProvideValues::Identifiers(provided) => {
                                provided.extend(idents);
                            }
                        },
                    },
                    ast::Provide::Types(..) => todo!(),
                };

                Ok(None)
            }
        }
    }

    fn interpret_expression(&mut self, expr: ast::ExpressionStatement) -> PyretResult<PyretValue> {
        match expr {
            ast::ExpressionStatement::Application(app) => {
                let args = app
                    .args
                    .into_iter()
                    .map(|arg| self.interpret_expression(arg))
                    .collect::<PyretResult<Vec<_>>>()?;

                self.context
                    .call_function(app.ident, args, self.scope_level)
            }
            ast::ExpressionStatement::Block(block) => {
                let values = self.interpret_block(block.body)?.into_iter();

                Ok(values.last().unwrap())
            }
            ast::ExpressionStatement::Literal(literal) => match literal {
                ast::LiteralExpression::Number(number) => Ok(PyretValue::new(
                    number.span(),
                    Rc::new(PyretValueKind::Number(number.value)),
                )),
                ast::LiteralExpression::String(string) => Ok(PyretValue::new(
                    string.span(),
                    Rc::new(PyretValueKind::String(string.value)),
                )),
                ast::LiteralExpression::Boolean(boolean) => Ok(PyretValue::new(
                    boolean.span(),
                    Rc::new(PyretValueKind::Boolean(boolean.value)),
                )),
            },
            ast::ExpressionStatement::Identifier(ident) => {
                let name = &*ident.name;

                let declaration = self.context.get_value(name)?;

                Ok(PyretValue::new(
                    ident.span(),
                    Rc::clone(&declaration.value.kind),
                ))
            }
            ast::ExpressionStatement::BinaryOperator(binary_op) => {
                self.interpret_binary_operator(binary_op)
            }
            ast::ExpressionStatement::Parenthesis(paren) => self.interpret_expression(*paren.expr),
            ast::ExpressionStatement::Dot(..) => todo!(),
        }
    }

    fn interpret_binary_operator(
        &mut self,
        binary_op: ast::BinaryOperatorExpression,
    ) -> PyretResult<PyretValue> {
        match binary_op.operator {
            ast::BinaryOperation::And => ops::and(*binary_op.left, *binary_op.right, self),
            ast::BinaryOperation::Or => ops::or(*binary_op.left, *binary_op.right, self),
            _ => {
                let left = self.interpret_expression(*binary_op.left)?;
                let right = self.interpret_expression(*binary_op.right)?;

                match binary_op.operator {
                    ast::BinaryOperation::Plus => ops::plus(left, right),
                    ast::BinaryOperation::Minus => ops::minus(left, right),
                    ast::BinaryOperation::Times => ops::times(left, right),
                    ast::BinaryOperation::Divide => ops::divide(left, right),
                    ast::BinaryOperation::LessThan => ops::less_than(left, right),
                    ast::BinaryOperation::LessThanOrEqual => ops::less_than_or_equal(left, right),
                    ast::BinaryOperation::GreaterThan => ops::greater_than(left, right),
                    ast::BinaryOperation::GreaterThanOrEqual => {
                        ops::greater_than_or_equal(left, right)
                    }
                    ast::BinaryOperation::Equal => ops::equal(left, right),
                    ast::BinaryOperation::NotEqual => ops::not_equal(left, right),
                    ast::BinaryOperation::Is | ast::BinaryOperation::IsRoughly => {
                        todo!(
                            "The testing statement is not inside a check, where or examples block."
                        )
                    }
                    _ => unreachable!(),
                }
            }
        }
    }

    fn interpret_declaration(&mut self, decl: ast::DeclarationStatement) -> PyretResult<()> {
        match decl {
            ast::DeclarationStatement::Check(check) => {
                let label = check.label;

                let results = self
                    .interpret_block_with(check.body, Self::interpret_test_statement)?
                    .into_iter()
                    .collect::<Box<[TestResult]>>();

                self.context
                    .borrow_mut()
                    .io
                    .write(Output::Test { label, results });
            }
            ast::DeclarationStatement::Let(var) => {
                if var.kind == LetDeclarationKind::RecursiveLet {
                    self.context.register_local_expr(
                        var.ident.name.clone(),
                        None,
                        self.scope_level,
                    );
                }

                let value = var
                    .init
                    .map(|init| {
                        let expr = self.interpret_expression(init)?;

                        Ok(PyretValue::new(var.ident.span(), Rc::clone(&expr.kind)))
                    })
                    .transpose()?;

                if let (Some(annotation), Some(value)) = (var.annotation, &value) {
                    self.type_check(&annotation, value.clone())?;
                }

                self.context
                    .register_local_expr(var.ident.name, value, self.scope_level);
            }
        }

        Ok(())
    }

    fn type_check(&self, annotation: &ast::TypeAnnotation, value: PyretValue) -> PyretResult<()> {
        match &annotation.value {
            ast::AnnotationType::NameAnnotation {
                name,
                parameters: _,
                predicate: _,
            } => match name {
                ast::IdentifierAnnotation::Name(ident) => {
                    if let Some(r#type) = self.context.get_type(&ident.name)? {
                        if !r#type(value, self.context.clone()) {
                            todo!("Type error: {annotation:?}")
                        }
                    }

                    Ok(())
                }
                ast::IdentifierAnnotation::Dot(..) => todo!("Type annotation: {annotation:?}"),
            },
            _ => todo!("Type annotation: {annotation:?}"),
        }
    }
}
