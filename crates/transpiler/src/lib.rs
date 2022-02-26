pub mod bundler;
mod utils;

use crate::{bundler::Bundler, utils::*};

use pyret_lexer::{ast, Lexer, Result};

/// Transpiles a Pyret AST into JavaScript.
pub struct Transpiler {
    bundler: Bundler,
    scope_level: u32,
}

impl Default for Transpiler {
    fn default() -> Self {
        let bundler = Bundler::default();

        Self::new(bundler)
    }
}

impl Transpiler {
    pub fn new(bundler: Bundler) -> Self {
        Self {
            bundler,
            scope_level: 0,
        }
    }

    fn is_top_level(&self) -> bool {
        self.scope_level == 1
    }

    pub fn transpile(&mut self, location: String, code: String) -> Result<String> {
        let mut lexer = Lexer::new(location, code)?;

        let stmts = lexer.lex()?;

        let code = self.parse_block(stmts).join("");

        Ok(self.bundler.bundle(code).unwrap())
    }

    fn parse_block(&mut self, stmts: Vec<ast::Stmt>) -> Vec<String> {
        let mut output = Vec::new();

        self.scope_level += 1;

        for stmt in stmts {
            output.push(self.parse_statement(stmt));
        }

        self.scope_level -= 1;

        output
    }

    fn parse_statement(&mut self, token: ast::Stmt) -> String {
        use ast::Stmt::*;

        match token {
            Expr(expr) => {
                let expr = self.parse_expression(expr);

                if self.is_top_level() {
                    format!("display({});", expr)
                } else {
                    format!("{};", expr)
                }
            }
            Decl(decl) => self.parse_declaration(decl),
        }
    }

    fn parse_declaration(&mut self, decl: ast::Decl) -> String {
        use ast::Decl::*;

        match decl {
            Var(var) => format!(
                "{}{} {}={};",
                if self.is_top_level() { "export " } else { "" },
                if var.mutable { "let" } else { "const" },
                serialize_name(var.name),
                self.parse_expression(var.init)
            ),
            Fn(decl) => self.parse_function(Some(decl.name), decl.function),
        }
    }

    fn parse_function(&mut self, name: Option<String>, function: ast::Function) -> String {
        let params: Vec<String> = function
            .params
            .into_iter()
            .map(|parameter| {
                let ast::Param { name, type_ann: _ } = parameter;

                serialize_name(name)
            })
            .collect();

        let mut body = self.parse_block(function.body);

        let last = body.pop().unwrap();

        format!(
            "{}function {}({}){{{}return {}}}",
            if self.is_top_level() { "export " } else { "" },
            if let Some(name) = name {
                serialize_name(name)
            } else {
                String::new()
            },
            params.join(","),
            body.join(""),
            last
        )
    }

    fn parse_expression(&mut self, expr: ast::Expr) -> String {
        use ast::Expr::*;

        match expr {
            Block(block) => {
                let stmts = block.stmts;

                match stmts.len() {
                    1 => self.parse_block(stmts).join(""),
                    _ => todo!(),
                }
            }
            Call(call) => {
                let name = serialize_name(call.callee);

                let args: Vec<String> = call
                    .arguments
                    .into_iter()
                    .map(|arg| self.parse_expression(arg))
                    .collect();

                // self.bundler.import(name.clone());

                format!("{name}({})", args.join(","))
            }
            Binary(binary) => format!(
                "{}{}{}",
                self.parse_expression(*binary.left),
                binary.operator,
                self.parse_expression(*binary.right)
            ),
            Ident(ident) => serialize_name(ident),
            Lit(lit) => match lit {
                ast::Lit::Str(string) => format!("\"{string}\""),
                ast::Lit::Num(number) => match number {
                    ast::Number::Exact(number) => format!("new Exactnum({number})"),
                    ast::Number::Rough(number) => format!("new Roughnum({number})"),
                },
                ast::Lit::Bool(boolean) => boolean.to_string(),
            },
            Paren(expr) => format!("({})", self.parse_expression(*expr)),
        }
    }
}
