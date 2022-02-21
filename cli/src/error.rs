use std::{
    error,
    fmt::{self, Debug, Display, Formatter},
};

#[derive(Debug, PartialEq, Clone)]
pub struct Error {
    pub message: String,
}

impl Error {
    pub fn from_v8_exception(scope: &mut v8::HandleScope, exception: v8::Local<v8::Value>) -> Self {
        let message = exception.to_rust_string_lossy(scope);

        Self { message }
    }
}

impl error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.message)?;

        Ok(())
    }
}
