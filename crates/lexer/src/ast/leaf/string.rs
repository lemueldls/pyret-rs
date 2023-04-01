use crate::prelude::*;

/// <https://www.pyret.org/docs/latest/s_literals.html#(part._.String_.Literals)>
#[common]
#[derive(Leaf)]
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
    fn parse_token(input: Box<str>, state: &mut LexerState) -> PyretResult<Self> {
        let offset = match input.chars().next().expect("input is not empty") {
            '\'' | '"' => 1,
            '`' => 3,
            _ => unreachable!(),
        };

        let end = input.len();
        let content = &input[offset..end - offset];

        let span = state.spanned(end);

        let value = if let Some(value) = unescape(content) {
            value
        } else {
            state.throw_late(PyretErrorKind::InvalidString {
                string: span.into(),
            });

            Box::from(content)
        };

        Ok(Self { span, value })
    }
}

fn unescape(input: &str) -> Option<Box<str>> {
    let mut string = String::with_capacity(input.len());

    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\\' {
            let next = chars.next()?;

            match next {
                'n' => string.push('\n'),
                'r' => string.push('\r'),
                't' => string.push('\t'),
                '\'' => string.push('\''),
                '"' => string.push('"'),
                '\\' => string.push('\\'),
                'x' => {
                    let mut code = 0;

                    if !chars.peek()?.is_ascii_hexdigit() {
                        return None;
                    }

                    for _ in 0..2 {
                        let Some(digit) = chars.next_if(char::is_ascii_hexdigit) else { break };

                        code = code * 16 + digit.to_digit(16).unwrap();
                    }

                    string.push(char::from_u32(code)?);
                }
                'u' => {
                    let mut code = 0;

                    if !chars.peek()?.is_ascii_hexdigit() {
                        return None;
                    }

                    for _ in 0..4 {
                        let Some(digit) = chars.next_if(char::is_ascii_hexdigit) else { break };

                        code = code * 16 + digit.to_digit(16).unwrap();
                    }

                    string.push(char::from_u32(code)?);
                }
                '0'..='7' => {
                    let mut code = next.to_digit(8).unwrap();

                    if let Some(digit) = chars.next_if(char::is_ascii_digit) {
                        code = code * 8 + digit.to_digit(8).unwrap();

                        if let Some(digit) = chars.next_if(char::is_ascii_digit) {
                            code = code * 8 + digit.to_digit(8).unwrap();
                        }
                    }

                    string.push(char::from_u32(code)?);
                }
                _ => return None,
            }
        } else {
            string.push(c);
        }
    }

    Some(string.into_boxed_str())
}
