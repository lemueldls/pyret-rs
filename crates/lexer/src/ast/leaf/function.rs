use crate::{ast::Statement, prelude::*};

#[derive(Debug, PartialEq)]
pub struct Function {
    pub params: Vec<Parameter>,
    pub body: Vec<Statement>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Parameter {
    pub name: Box<str>,
    pub type_ann: Option<Box<str>>,
}

/// <https://www.pyret.org/docs/latest/s_declarations.html#(part._s~3afun-decl)>
#[derive(Leaf, Debug, PartialEq)]
// #[regex(r".")]
pub struct FunctionDeclaration {
    span: (usize, usize),
    pub ident: Box<str>,
    pub function: Function,
}

impl TokenParser for FunctionDeclaration {
    #[inline]
    fn parse(_input: Box<str>, state: &mut LexerState) -> PyretResult<Self> {
        // let name = input.name("name").unwrap().as_str().to_owned();

        // self.eat(get_length(input.name("capture")));

        // let mut params = Vec::new();

        // for param in captures
        //     .name("params")
        //     .unwrap()
        //     .as_str()
        //     .split_terminator(',')
        // {
        //     let param: Vec<String> = param
        //         .split_terminator("::")
        //         .map(|str| str.trim().to_owned())
        //         .collect();

        //     let name = param[0].to_owned();
        //     if !name.is_empty() {
        //         let token = Parameter {
        //             name,
        //             type_ann: param.get(1).map(|type_ann| type_ann.to_owned()),
        //         };

        //         params.push(token);
        //     };
        // }

        // let body = self.next_block()?;

        // if body.is_empty() {
        //     self.throw(LexerError::EmptyBlock, start + 4, start + name.len() + 3)?
        // }

        // {
        //     for (i, stmt) in body.iter().enumerate() {
        //         let last = i == body.len() - 1;

        //         match stmt {
        //             Statement::Declaration(decl) if last => todo!("no decl at end:\n{:#?}", *decl),
        //             Statement::Expression(expr) if !last => match expr {
        //                 ast::Expr::Ident(..) | ast::Expr::Lit(..) => {
        //                     todo!("no standalone expr:\n{expr:#?}")
        //                 }
        //                 _ => {}
        //             },
        //             _ => {}
        //         }
        //     }
        // }

        Ok(Self {
            span: state.spanned(0),
            ident: Box::from("mock-fn"),
            function: Function {
                params: vec![],
                body: vec![],
            },
        })
    }
}
