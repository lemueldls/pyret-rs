pub mod global;

use pyret_error::PyretResult;

use crate::value::context::Context;

pub trait Trove {
    fn register(context: Context) -> PyretResult<()>;
}

pub struct Module {
    pub name: String,
    pub exports: Vec<String>,
}

#[inline]
pub fn import_trove(name: &str, context: Context) -> PyretResult<()> {
    match name {
        "global" => global::register(context),
        "constants" => trove!("constants", context),
        _ => todo!("Handle import of unknown trove."),
    }
}
