use pyret_error::{PyretErrorKind, PyretResult};

pub fn remove_comments(source: &str) -> PyretResult<Box<str>> {
    let mut removed = String::with_capacity(source.len());

    let mut chars = source.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '#' {
            let opening = removed.len();

            removed.push(' ');

            if chars.next_if_eq(&'|').is_some() {
                removed.push(' ');

                let mut depth = 1_u8;

                while depth > 0 {
                    if let Some(ch) = chars.next() {
                        removed.push(' ');

                        if ch == '#' {
                            if chars.next_if_eq(&'|').is_some() {
                                removed.push(' ');

                                depth += 1;
                            }
                        } else if ch == '|' && chars.next_if_eq(&'#').is_some() {
                            removed.push(' ');

                            depth -= 1;
                        }
                    } else {
                        return Err(PyretErrorKind::UnmatchedOpeningComment {
                            position: (opening..opening + 2).into(),
                        });
                    }
                }
            } else {
                while chars.next_if(|c| *c != '\n').is_some() {
                    removed.push(' ');
                }
            }
        } else if ch == '|' && chars.next_if_eq(&'#').is_some() {
            return Err(PyretErrorKind::UnmatchedClosingComment {
                position: (removed.len()..removed.len() + 2).into(),
            });
        } else {
            removed.push(ch);
        }
    }

    debug_assert_eq!(removed.len(), source.len());

    Ok(removed.into_boxed_str())
}
