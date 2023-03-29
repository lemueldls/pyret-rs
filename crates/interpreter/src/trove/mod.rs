pub mod constants;
pub mod global;

use std::cell::RefMut;

use pyret_error::PyretResult;

use crate::{context::Context, ty, value::registrar::Registrar};

ty!(Any, |_value, _context| true);

pub trait Trove {
    fn register(context: &mut Registrar) -> PyretResult<()>;
}

pub struct Module {
    pub name: String,
    pub exports: Vec<String>,
}

pub fn import_trove(name: &str, registrar: &mut Registrar) -> PyretResult<()> {
    match name {
        "global" => global::register(registrar),
        "constants" => constants::register(registrar),
        _ => todo!("Handle import of unknown trove."),
    }
}
