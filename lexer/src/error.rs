#[cfg(feature = "ansi_term")]
use {ansi_term::Color, std::fmt};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum CompileError {
    DidNotUnderstand,
    EmptyBlock,
    UnmatchedOpeningComment,
    UnmatchedClosingComment,
    UnfinishedString,
    MissingComma,
    ExpectedExpression,
}

impl CompileError {
    // TODO: Fancy reasons~
    fn reason(&self) -> String {
        use CompileError::*;

        let message = match *self {
            DidNotUnderstand => "didn't understand your program",
            EmptyBlock => "rejected your program because there is an empty block at",
            UnmatchedOpeningComment => "thinks your program is missing a closing block comment",
            UnmatchedClosingComment => "thinks your program is missing a opening block comment",
            UnfinishedString => "thinks your program has an incomplete string literal",
            MissingComma => "thinks you're missing a comma",
            ExpectedExpression => "expected an expression, but got",
        };

        String::from("Pyret ") + message
    }
}

#[derive(Debug)]
pub struct Error {
    pub error: CompileError,
    pub filename: String,
    pub code: Vec<String>,
    pub from: (usize, usize),
    pub to: (usize, usize),
}

#[cfg(feature = "ansi_term")]
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let red = Color::Red.bold();

        let code = {
            let mut start_line = self.from.0;

            let max = start_line + self.code.len();
            let spacing_length = max.to_string().len();

            let blue = Color::Blue.bold();

            let spaces = " ".repeat(spacing_length);
            let empty = &format!(" {spaces}{}", blue.paint("|"));

            let mut lines = format!(
                "{spaces}{} {}:{start_line}:{}-{}:{}\n{empty}\n",
                blue.paint("-->"),
                self.filename,
                self.from.1,
                self.to.0,
                self.to.1,
            );

            for code in self.code.iter() {
                lines += &format!(
                    "{}{} {code}",
                    " ".repeat(spacing_length - start_line.to_string().len()),
                    blue.paint(format!("{start_line} |")),
                );

                // TODO, properly
                if start_line == self.from.0 {
                    lines += &format!(
                        "\n{} {}",
                        empty,
                        " ".repeat(self.from.1 - 1)
                            + &red
                                .paint(&"^".repeat(self.to.1 - self.from.1 + 1))
                                .to_string()
                    )
                }

                lines += "\n";

                start_line += 1;
            }

            lines
        };

        let reason = red.paint(self.error.reason());

        write!(f, "{reason}\n{code}")
    }
}
