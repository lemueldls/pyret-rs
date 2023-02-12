pub mod trove;
pub mod value;

mod context;
mod io;
mod registrar;

use std::{borrow::Borrow, cell::RefCell, rc::Rc};

pub use context::Context;
pub use pyret_file::graph::PyretGraph;
pub use pyret_lexer::{ast, lex, prelude::*};
use trove::{global::number, Trove};
use value::PyretValue;

pub struct Interpreter {
    pub graph: Box<dyn PyretGraph>,
    pub context: Rc<RefCell<Context>>,
    scope_depth: usize,
}

impl Interpreter {
    #[must_use]
    pub fn new(graph: impl PyretGraph + 'static) -> Self {
        Self {
            graph: Box::new(graph),
            context: Rc::new(RefCell::new(Context::default())),
            scope_depth: 0,
        }
    }

    pub fn use_context<T: Trove>(&self) {
        T::register(Rc::clone(&self.context));
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

        match self.interpret_block(stmts) {
            Ok(values) => Ok(values),
            Err(error) => Err(vec![PyretError::new(error, file_id)]),
        }
    }

    fn interpret_block(
        &mut self,
        block: Vec<ast::Statement>,
    ) -> PyretResult<Box<[Rc<PyretValue>]>> {
        self.scope_depth += 1;

        let depth = self.scope_depth;

        let values = block
            .into_iter()
            .map(|token| match token {
                ast::Statement::Keyword(keyword) => todo!("Unexpected keyword: {keyword:?}"),
                ast::Statement::Expression(expr) => {
                    let expression = self.interpret_expression(expr)?;

                    Ok(Some(expression))
                }
                ast::Statement::Declaration(decl) => {
                    self.interpret_declaration(decl)?;

                    Ok(None)
                }
            })
            .filter_map(Result::transpose)
            .collect();

        self.context
            .as_ref()
            .borrow_mut()
            .registrar
            .pop_scope(depth);

        self.scope_depth -= 1;

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

                if let Some(value) = self
                    .context
                    .as_ref()
                    .borrow()
                    .registrar
                    .get(&app.ident.name)
                {
                    match value.as_ref().borrow() {
                        PyretValue::Function(function) => {
                            function.call(&args, Rc::clone(&self.context))
                        }
                        _ => Err(PyretErrorKind::InvalidFunctionApplication {
                            span: app.ident.span(),
                        }),
                    }
                } else {
                    todo!()
                }
            }
            ast::ExpressionStatement::Block(block) => {
                let values = self.interpret_block(block.body)?;

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

                if let Some(value) = self.context.borrow_mut().registrar.get(name) {
                    Ok(Rc::clone(value))
                } else {
                    Err(PyretErrorKind::UnboundIdentifier {
                        ident: Box::from(name),
                        span: ident.span(),
                    })
                }
            }
            ast::ExpressionStatement::BinaryOperator(binary_op) => {
                let left = self.interpret_expression(*binary_op.left)?;
                let right = self.interpret_expression(*binary_op.right)?;

                match binary_op.operator {
                    ast::BinaryOperation::Plus => number::plus(left.as_ref(), right.as_ref()),
                    ast::BinaryOperation::Minus => number::minus(left.as_ref(), right.as_ref()),
                    ast::BinaryOperation::Times => number::times(left.as_ref(), right.as_ref()),
                    ast::BinaryOperation::Divide => number::divide(left.as_ref(), right.as_ref()),
                }
            }
            ast::ExpressionStatement::Parenthesis(paren) => self.interpret_expression(*paren.expr),
            ast::ExpressionStatement::Dot(..) => todo!(),
        }
    }

    fn interpret_declaration(&mut self, decl: ast::DeclarationStatement) -> PyretResult<()> {
        match decl {
            ast::DeclarationStatement::Type(r#type) => {
                dbg!(r#type);

                todo!()
            }
            ast::DeclarationStatement::Variable(var) => {
                let value = self.interpret_expression(var.init)?;

                self.context.borrow_mut().registrar.register_local_expr(
                    var.ident.name,
                    value,
                    self.scope_depth,
                );
            }
        }

        Ok(())
    }
}
