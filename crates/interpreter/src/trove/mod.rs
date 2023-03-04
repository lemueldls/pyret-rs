pub mod constants;
pub mod global;

use std::cell::RefMut;

use pyret_error::PyretResult;

use crate::context::Context;

pub struct Module {
    pub name: String,
    pub exports: Vec<String>,
}

pub fn import_trove(name: &str, context: &mut RefMut<Context>) -> PyretResult<()> {
    match name {
        "global" => global::register(context),
        "constants" => constants::register(context),
        _ => todo!("Handle import of unknown trove."),
    }
}
