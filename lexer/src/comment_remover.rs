use crate::error::*;

/// Strips out comments and replaces them with spaces.
pub struct CommentRemover {
    location: String,
    full_code: String,
    code: String,
    position: usize,
}

impl CommentRemover {
    pub fn new(location: String, code: String) -> Self {
        Self {
            location,
            full_code: code.clone(),
            code,
            position: 0,
        }
    }

    pub fn remove(&mut self) -> Result<String> {
        let code = self.code.to_owned();

        let rest = self.rest();

        let mut chars = rest.chars();
        if let Some(next) = chars.next() {
            if next == '#' {
                if chars.next() == Some('|') {
                    self.remove_block_comment(rest)?;
                } else {
                    self.remove_line_comment(rest);
                }
            } else {
                // Lazy boolean operations to skip inside strings.
                let _ = self.skip_string("\"") || self.skip_string("'") || self.skip_string("```");
            }

            if self.position < code.chars().count() {
                self.position += 1;

                self.remove()?;
            }
        }

        Ok(self.code.to_owned())
    }

    fn rest(&self) -> String {
        self.code[self.position..].to_owned()
    }

    fn remove_block_comment(&mut self, comment: String) -> Result<()> {
        let length = comment.len();

        // Depth of nested block comments.
        let mut nesting_depth = 1;
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
                self.throw(CompileError::UnmatchedOpeningComment)?
            }
        }

        // Make sure there aren't any unmatched block comments.
        if nesting_depth == 0 {
            let comment = &comment[..position];

            // A list of spaces, used to also keep newlines.
            let spaces: Vec<String> = comment.split('\n').map(|s| " ".repeat(s.len())).collect();

            // Replace block comment with spaces.
            self.code = format!(
                "{}{}{}",
                &self.code[..self.position],
                spaces.join("\n"),
                &self.rest()[comment.len()..]
            );

            Ok(())
        } else {
            self.throw(CompileError::UnmatchedOpeningComment)
        }
    }

    fn remove_line_comment(&mut self, comment: String) {
        // List of the following lines of code.
        let lines: Vec<&str> = comment.split('\n').collect();
        // Length of the current line of code, which is a comment.
        let length = lines[0].len();

        // Replace comment with spaces.
        self.code = format!(
            "{}{}{}",
            &self.code[..self.position],
            " ".repeat(length),
            &self.rest()[length..]
        );
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
