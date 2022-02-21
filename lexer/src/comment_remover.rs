use crate::error::*;

/// Strips out comments and replaces them with spaces.
pub struct CommentRemover {
    location: String,
    full_code: String,
    input: String,
    position: usize,
}

impl CommentRemover {
    pub fn new(location: String, input: String) -> Self {
        Self {
            location,
            full_code: input.clone(),
            input,
            position: 0,
        }
    }

    pub fn remove(&mut self) -> Result<String> {
        let code = self.input.to_owned();

        let rest = self.rest();

        let mut chars = rest.chars();
        if let Some(next) = chars.next() {
            if next == '#' {
                if chars.next() == Some('|') {
                    self.remove_block_comment(rest)?;
                } else {
                    self.remove_line_comment(rest);
                }
            } else if rest.starts_with("|#") {
                self.throw(CompileError::UnmatchedClosingComment)?
            } else {
                // Lazy boolean operations to skip inside strings.
                let _ = self.skip_string("\"") || self.skip_string("'") || self.skip_string("```");
            }

            if self.position < code.chars().count() {
                self.position += 1;

                self.remove()?;
            }
        }

        Ok(self.input.to_owned())
    }

    fn rest(&self) -> String {
        self.input[self.position..].to_owned()
    }

    fn remove_block_comment(&mut self, comment: String) -> Result<()> {
        // Find the nearest closing block comment.
        if let Some(closing) = comment.find("|#") {
            let comment = {
                // List the positions of every closing block comment in the code after this opening comment.
                let closing_comments: Vec<(usize, &str)> = comment.match_indices("|#").collect();

                // Get the string from this opening comment to the next closing block comment.
                let to_next_closing = &comment[..closing];

                // Amount of nested block comments by counting the opening block comments inside this current one.
                let nested_comments = to_next_closing.matches("#|").count() - 1;

                // Position of the closing block comment that matches this current comment.
                let closing = if let Some((index, _)) = closing_comments.get(nested_comments) {
                    index
                } else {
                    self.position += to_next_closing.rfind("#|").unwrap();

                    self.throw(CompileError::UnmatchedOpeningComment)?
                };

                // Get the string to the closing block that matches this current one.
                &comment[..closing + 2]
            };

            // A list of spaces, used to also keep newlines.
            let spaces: Vec<String> = comment.split('\n').map(|s| " ".repeat(s.len())).collect();

            // Replace block comment with spaces.
            self.input = format!(
                "{}{}{}",
                &self.input[..self.position],
                spaces.join("\n"),
                &self.rest()[comment.len()..]
            );

            Ok(())
        } else {
            self.throw(CompileError::UnmatchedOpeningComment)
        }
    }

    fn throw<E>(&self, error: CompileError) -> Result<E> {
        let split = self.full_code[..self.position].split('\n');

        let line_number = split.clone().count();

        let code = {
            let lines: Vec<&str> = self.full_code.split('\n').collect();

            lines[line_number - 1]
                .split('\n')
                .map(String::from)
                .collect()
        };

        let opening_comment = split.last().unwrap().len() + 1;

        let from = (line_number, opening_comment);
        let to = (line_number, opening_comment + 1);

        Err(Error {
            error,
            filename: self.location.clone(),
            code,
            from,
            to,
        })
    }

    fn remove_line_comment(&mut self, comment: String) {
        // List of the following lines of code.
        let lines: Vec<&str> = comment.split('\n').collect();
        // Length of the current line of code, which is a comment.
        let length = lines[0].len();

        // Replace comment with spaces.
        self.input = format!(
            "{}{}{}",
            &self.input[..self.position],
            " ".repeat(length),
            &self.rest()[length..]
        );
    }

    // Returns true if a string has been skipped.
    fn skip_string(&mut self, pattern: &str) -> bool {
        let code = self.rest();

        if code.starts_with(pattern) {
            let length = pattern.len();

            if let Some(end) = code[length..].find(pattern) {
                let skip = length + end;

                let is_not_escaped = code.chars().nth(skip - 1).unwrap() != '\\'
                    // If the escape is escaped.
                    || code.chars().nth(skip - 2).unwrap() == '\\';

                if is_not_escaped {
                    self.position += skip;

                    return true;
                }
            }
        }

        false
    }
}
