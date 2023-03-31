pub mod global;

use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};

use pyret_error::PyretResult;

use crate::{ty, value::context::Context, Interpreter};

pub trait Trove {
    fn register(context: Rc<RefCell<Context>>) -> PyretResult<()>;
}

pub struct Module {
    pub name: String,
    pub exports: Vec<String>,
}

pub fn import_trove(name: &str, context: Rc<RefCell<Context>>) -> PyretResult<()> {
    match name {
        "global" => global::register(context),
        "constants" => trove!("constants", context),
        _ => todo!("Handle import of unknown trove."),
    }
}
