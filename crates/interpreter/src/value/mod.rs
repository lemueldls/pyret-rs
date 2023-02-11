pub mod function;
use std::{fmt, ops::Deref, rc::Rc};

pub use function::PyretFunction;
use pyret_number::{One, PyretNumber};

// TODO: Make into struct
pub enum PyretValueScoped {
    Builtin(Rc<PyretValue>),
    Local(Rc<PyretValue>),
}

impl Deref for PyretValueScoped {
    type Target = Rc<PyretValue>;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Builtin(value) | Self::Local(value) => value,
        }
    }
}

pub enum PyretValue {
    Number(PyretNumber),
    String(Box<str>),
    Boolean(bool),
    Function(PyretFunction),
}

impl fmt::Display for PyretValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(number) => match number {
                PyretNumber::Exact(exact_number) => {
                    let numerator = exact_number.numer();
                    let denominator = exact_number.denom();

                    if denominator.is_one() {
                        write!(f, "{numerator}")
                    } else {
                        write!(f, "{numerator}/{denominator}")
                    }
                }
                PyretNumber::Rough(rough_number) => write!(f, "~{rough_number}"),
            },
            Self::String(string) => write!(f, "\"{string}\""),
            Self::Boolean(boolean) => write!(f, "{boolean}"),
            Self::Function(PyretFunction { name, .. }) => write!(f, "<function:{name}>",),
        }
    }
}
