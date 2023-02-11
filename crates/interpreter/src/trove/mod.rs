pub mod global;
use std::{cell::RefCell, rc::Rc};

pub use global::Global;

use crate::context::Context;

pub trait Trove {
    fn register(context: Rc<RefCell<Context>>);
}
