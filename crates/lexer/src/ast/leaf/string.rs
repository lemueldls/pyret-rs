use crate::prelude::*;

/// <https://www.pyret.org/docs/latest/s_literals.html#(part._.String_.Literals)>
#[derive(Leaf, Debug, PartialEq, Eq)]
// Single quotes
#[regex(r"'(\\'|.)*?'")]
// Double quotes
#[regex(r#""(\\"|.)*?""#)]
// Multi-line quotes
#[regex(r"(?s)```(\\`|.)*?```")]
pub struct StringLiteral {
    span: (usize, usize),
    pub value: Box<str>,
}

impl TokenParser for StringLiteral {
    #[inline]
    fn parse(input: Box<str>, state: &mut LexerState) -> PyretResult<Self> {
        let offset = match input.chars().next().expect("input is not empty") {
            '\'' | '"' => 1,
            '`' => 3,
            _ => unreachable!(),
        };

        let end = input.len();
        let value = &input[offset..end - offset];

        let span = state.spanned(end);

        // TODO
        // let valid_regex = test_regex!(
        //     r#"(\\[0-7]{1,3}|\\x[[:xdigit:]]{1,2}|\\u[[:xdigit:]]{1,4}|\\[\\nrt"'
        // ]|[^\\\n])*$"#,     value,
        // );

        // if !valid_regex {
        //     state.throw_late(Error::InvalidString { span: span.clone() });
        // }

        Ok(Self {
            span,
            value: Box::from(value),
        })
    }
}
