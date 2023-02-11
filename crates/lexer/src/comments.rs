#[cfg(feature = "comments")]
use crate::ast::{Comment, CommentKind};
use crate::{ErrorKind, Result};

/// Strips out comments and replaces them with spaces.
pub struct CommentParser {
    input: String,
    position: usize,
    #[cfg(feature = "comments")]
    pub comments: Vec<Comment>,
}

impl CommentParser {
    #[must_use]
    #[inline]
    pub const fn new(input: String) -> Self {
        Self {
            input,
            position: 0,
            #[cfg(feature = "comments")]
            comments: Vec::new(),
        }
    }

    /// # Errors
    ///
    /// Will return an [`Error`] if there is an error parsing the comments.
    #[inline]
    pub fn parse(&mut self) -> PyretResult<String> {
        let code = self.input.clone();

        let rest = self.rest();

        let mut chars = rest.chars();
        if let Some(next) = chars.next() {
            if next == '#' {
                if chars.next() == Some('|') {
                    self.take_block(&rest)?;
                } else {
                    self.take_line(&rest);
                }
            } else {
                // Lazy boolean operations to skip inside strings.
                let _ = self.skip_string("\"") || self.skip_string("'") || self.skip_string("```");
            }

            if self.position < code.len() {
                self.position += 1;

                self.parse()?;
            }
        }

        Ok(self.input.clone())
    }

    fn rest(&self) -> String {
        self.input[self.position..].to_owned()
    }

    fn take_block(&mut self, comment: &str) -> Result<()> {
        let length = comment.len();

        // Count the depth of nested block comments.
        let mut nesting_depth = 1_usize;
        // Used to find where to end the block comment.
        let mut position = 2;

        // While there are block comments, and we haven't reached the end...
        while nesting_depth > 0 && position < length {
            // Check if the block comment doesn't end too soon.
            if let Some(next) = comment.get(position..position + 2) {
                // Increase the position for the next iteration.
                position += match next {
                    // Nested opening block comment.
                    "#|" => {
                        nesting_depth += 1;
                        2
                    }
                    // Closing block comment.
                    "|#" => {
                        nesting_depth -= 1;
                        2
                    }
                    _ => 1,
                }
            } else {
                self.throw()?;
            }
        }

        // Make sure there aren't any unmatched block comments.
        if nesting_depth == 0 {
            let comment = &comment[..position];

            let length = comment.len();

            #[cfg(feature = "comments")]
            self.comments.push(Comment {
                // span: self.position..self.position + length,
                kind: CommentKind::Block,
                text: comment[2..length - 2].to_owned(),
            });

            // A list of spaces, used to also keep newlines.
            let spaces: Box<[str]> = comment.lines().map(|s| " ".repeat(s.len())).collect();

            // Replace block comment with spaces.
            self.input = format!(
                "{}{}{}",
                &self.input[..self.position],
                spaces.join("\n"),
                &self.rest()[length..]
            );

            Ok(())
        } else {
            self.throw()?
        }
    }

    fn take_line(&mut self, comment: &str) {
        let line = comment.lines().next().expect("Could not get line");

        // Length of the current line comment.
        let length = line.len();

        #[cfg(feature = "comments")]
        self.comments.push(Comment {
            // span: self.position..self.position + length,
            kind: CommentKind::Line,
            text: line[1..].to_string(),
        });

        // Replace comment with spaces.
        self.input = format!(
            "{}{}{}",
            &self.input[..self.position],
            " ".repeat(length),
            &self.rest()[length..]
        );
    }

    const fn throw<E>(&self) -> Result<E> {
        Err(ErrorKind::UnmatchedOpeningComment {
            position: self.position,
        })
    }

    // Returns true if a string has been skipped.
    fn skip_string(&mut self, pattern: &str) -> bool {
        let code = self.rest();

        if code.starts_with(pattern) {
            let length = pattern.len();

            if let Some(end) = code[length..].find(pattern) {
                let skip = length + end;

                let is_not_escaped = code.chars().nth(skip - 1).expect("Expected character before") != '\\'
                    // If the escape is escaped.
                    || code.chars().nth(skip - 2).expect("Expected character before") == '\\';

                if is_not_escaped {
                    self.position += skip;

                    return true;
                }
            }
        }

        false
    }
}
