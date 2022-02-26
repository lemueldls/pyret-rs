pub mod ast;
mod comment_remover;
mod error;
mod regex;

#[macro_use]
mod macros;

use std::cmp::max;

use comment_remover::CommentRemover;
pub use error::*;
use regex::*;

use fancy_regex::Match;

#[derive(Debug)]
pub struct Lexer {
    location: String,
    input: String,
    /// Full, unmodified, code for error reporting purposes.
    full_code: String,
}

impl Lexer {
    pub fn new(location: String, code: String) -> Result<Self> {
        let mut comment_remover = CommentRemover::new(location.clone(), code.clone());

        Ok(Self {
            location,
            full_code: code,
            input: comment_remover.remove()?,
        })
    }

    fn input(&self) -> String {
        self.input.to_owned()
    }

    pub fn lex(&mut self) -> Result<Vec<ast::Stmt>> {
        let mut statements = Vec::new();

        // Start without leading whitespace.
        self.eat(0);
        // Don't parse an empty program.
        if self.input.is_empty() {
            return Ok(statements);
        }

        if let Some(stmt) = self.next_statement()? {
            // if let Some(next) = self.input.chars().next() {
            //     dbg!(next);
            // }

            // if matches!(self.code.chars().next(), Some(next) if next == '\n') {
            //     panic!("k")
            // }

            statements.push(stmt);
        }
        statements.extend(self.lex()?);

        Ok(statements)
    }

    /// Returns a [Result] with [None] if it reached the end of the program.
    fn next_statement(&mut self) -> Result<Option<ast::Stmt>> {
        let start = self.position();

        use ast::Stmt::*;
        let search = search_next_token! {
            Decl |> self.next_declaration()?,
            Expr |> self.next_expression()?
        };

        Ok(if let Some(stmt) = search {
            Some(stmt)
        } else if self.input.is_empty() {
            None
        } else {
            // match self.code.chars().next().unwrap() {
            //     _ =>
            self.throw(CompileError::DidNotUnderstand, start, self.position())?
            //     ,
            // }
        })
    }

    fn position(&self) -> usize {
        self.full_code.chars().count() - self.input.chars().count()
    }

    fn next_declaration(&mut self) -> Result<Option<ast::Decl>> {
        let start = self.position();

        capture_token!(&self.input() => {
            VAR_DECLARATION => var = {
                self.eat(get_length(var.name("capture")));

                let type_ann = var.name("type");
                let type_ann = type_ann.map(|_| get_string(type_ann));

                ast::Decl::Var(ast::VarDecl {
                    name: get_string(var.name("name")),
                    init: self.try_expression()?,
                    mutable: var.name("var").is_some(),
                    recursive: var.name("rec").is_some(),
                    type_ann,
                })
            },
            FUNCTION => function = {
                let name = function.name("name").unwrap().as_str().to_owned();

                self.eat(get_length(function.name("capture")));

                let mut params = Vec::new();

                for param in function
                    .name("params")
                    .unwrap()
                    .as_str()
                    .split_terminator(',')
                {
                    let param: Vec<String> = param
                        .split_terminator("::")
                        .map(|str| str.trim().to_owned())
                        .collect();

                    let name = param[0].to_owned();
                    if !name.is_empty() {
                        let token = ast::Param {
                            name,
                            type_ann: param.get(1).map(|type_ann| type_ann.to_owned()),
                        };

                        params.push(token);
                    };
                }

                let body = self.next_block()?;

                if body.is_empty() {
                    self.throw(CompileError::EmptyBlock, start + 4, start + name.len() + 3)?
                }

                {
                    for (i, stmt) in body.iter().enumerate() {
                        let last = i == body.len() - 1;

                        match stmt {
                            ast::Stmt::Decl(decl) if last => todo!("no decl at end:\n{:#?}", *decl),
                            ast::Stmt::Expr(expr) if !last =>
                                match expr {
                                    ast::Expr::Ident(..) | ast::Expr::Lit(..) => todo!("no standalone expr:\n{expr:#?}"),
                                    _ => {}
                                }
                            _ => {}
                        }
                    }
                }

                ast::Decl::Fn(ast::FnDecl {
                    name,
                    function: ast::Function {
                        params,
                        body,
                    }
                })
            }
        })
    }

    /// Handles blocks with an `end` keyword.
    fn next_block(&mut self) -> Result<Vec<ast::Stmt>> {
        let mut stmts = Vec::new();

        if let Some(token) = self.next_statement()? {
            if let ast::Stmt::Expr(ast::Expr::Ident(ref ident)) = token {
                if ident == "end" {
                    return Ok(stmts);
                }
            }

            stmts.push(token);
            stmts.extend(self.next_block()?);
        };

        Ok(stmts)
    }

    fn next_expression(&mut self) -> Result<Option<ast::Expr>> {
        let code = &self.input();

        let expr = capture_token!(code => {
            NUMBER => number = {
                self.eat(get_string(number.get(0)).len());

                let n = get_string(number.name("value")).parse().unwrap();
                let value = if number.name("rough").is_some() {
                    ast::Number::Rough(n)
                } else {
                    ast::Number::Exact(n)
                };

                ast::Expr::Lit(ast::Lit::Num(value))
            },
            SINGLE_QUOTES => string if '\'' {
                self.eat(get_length(string.get(0)));

                ast::Expr::Lit(ast::Lit::Str(get_string(string.name("value")).replace('"', "\\\"")))
            } else {
                let position = self.position();

                self.throw(CompileError::UnfinishedString, position, position)?
            },
            DOUBLE_QUOTES => string if '"' {
                self.eat(get_length(string.get(0)));

                ast::Expr::Lit(ast::Lit::Str(get_string(string.name("value"))))
            } else {
                let position = self.position();

                self.throw(CompileError::UnfinishedString, position, position)?
            },
            MULTILINE_QUOTES => string if "```" {
                self.eat(get_length(string.get(0)));

                ast::Expr::Lit(ast::Lit::Str(
                    get_str(string.name("value"))
                        .trim()
                        .replace("\"", "\\\"")
                        .replace("\n", "\\n")
                ))
            } else {
                let position = self.position();

                self.throw(CompileError::UnfinishedString, position, position + 2)?
            },
            PARENTHESIS => _ if '(' {
                // Opening parenthesis
                self.eat(1);

                let expr = self.try_expression()?;

                // Closing parenthesis
                self.eat(1);

                ast::Expr::Paren(Box::new(expr))
            } else todo!("missing closing parenthesis:\n{}", self.input),
            CALL => call = {
                let callee = get_string(call.name("callee"));

                // Callee name + opening parenthesis.
                self.eat(callee.len() + 1);

                let arguments = self.separate_by_comma(')')?;

                ast::Expr::Call(ast::CallExpr { callee, arguments })
            },
            IDENTIFIER => variable = {
                self.eat(get_length(variable.get(0)));

                ast::Expr::Ident(get_string(variable.name("name")))
            }
        })?;

        // Check if there is an operator ahead of this expression.
        // If there is an operator, then replace the current
        // expression with a binary expression with both the current,
        // // and expression after the operator, to create a token.

        capture_token!(code => {
            OPERATOR => operator = {
                dbg!(operator.get(0));
            }
        })?;

        // let _ = match_start!(code => {
        //     "+",
        //     operator {
        //         self.eat(operator.len());

        //             Some(ast::Expr::Binary(ast::BinaryExpr {
        //                 left: Box::new(expr.unwrap()),
        //                 operator: String::from(operator),
        //                 right: Box::new(self.try_expression()?),
        //             }))
        //     }
        // });

        // if let Some(next) = self.code.chars().next() {
        //     match next {
        //         operator if "+" => {
        //             self.eat(operator.len());

        //             Some(ast::Expr::Binary(ast::BinaryExpr {
        //                 left: Box::new(expr.unwrap()),
        //                 operator: String::from(operator),
        //                 right: Box::new(self.try_expression()?),
        //             }))
        //         }
        //         _ => {}
        //     }
        // }

        // let expr = if matches!(expr, Some(_) if '+' == self.code.chars().next().unwrap_or('\0')) {
        //     self.eat(1);

        //     Some(ast::Expr::Binary(ast::BinaryExpr {
        //         left: Box::new(expr.unwrap()),
        //         operator: String::from("+"),
        //         right: Box::new(self.try_expression()?),
        //     }))
        // } else {
        //     expr
        // };

        Ok(expr)
    }

    fn try_expression(&mut self) -> Result<ast::Expr> {
        if let Some(expr) = self.next_expression()? {
            Ok(expr)
        } else {
            self.throw(
                CompileError::ExpectedExpression,
                self.position(),
                self.position(),
            )
        }
    }

    fn throw<E>(&self, error: CompileError, start: usize, end: usize) -> Result<E> {
        let filename = self.location.to_owned();

        let split = self.full_code[..start].split('\n');

        let line_number = split.clone().count();

        let code: Vec<String> = {
            let lines: Vec<&str> = self.full_code.split('\n').collect();

            let start_line = line_number - 1;
            let end = max(self.full_code[start..end].split('\n').count() - 1, 1);

            lines[start_line..start_line + end]
                .join("\n")
                .trim_end()
                .split('\n')
                .map(String::from)
                .collect()
        };

        let from_col = split.clone().last().unwrap().len() + 1;

        // TODO :(
        let to_row = line_number;
        let to_col = from_col + (end - start);

        // to_col -= code[..code.len() - 1].join("\n").len();
        // to_row += code.len() - 1;

        // for line in &code[..code.len() - 1] {
        //     to_col -= line.len() + 2;

        //     to_row += 1;
        // }

        let from = (line_number, from_col);
        let to = (to_row, to_col);

        // println!("{:?}:{:?}", from, to);

        Err(Error {
            error,
            filename,
            code,
            from,
            to,
        })
    }

    /// CONSUME
    fn eat(&mut self, amount: usize) {
        self.input = self.input[amount..].trim_start().to_owned();
    }

    // Capture a list of expressions separated by commas.
    fn separate_by_comma(&mut self, end: char) -> Result<Vec<ast::Expr>> {
        let mut exprs = Vec::new();

        if self.input.chars().next().unwrap() == end {
            self.eat(1);

            return Ok(exprs);
        }

        let start = self.position();

        exprs.push(self.try_expression()?);

        match self.input.chars().next().unwrap() {
            ',' => {
                self.eat(1);

                exprs.extend(self.separate_by_comma(end)?)
            }
            next if next == end => self.eat(1),
            _ => self.throw(CompileError::MissingComma, start + 1, self.position())?,
        }

        Ok(exprs)
    }
}

fn get_str(value: Option<Match>) -> &str {
    value.unwrap().as_str()
}
fn get_string(value: Option<Match>) -> String {
    get_str(value).to_owned()
}
fn get_length(value: Option<Match>) -> usize {
    get_str(value).len()
}
