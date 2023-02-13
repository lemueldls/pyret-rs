use std::cell::RefCell;

use crate::{context::Context, PyretValue, Rc};

pub fn register(context: &Rc<RefCell<Context>>) {
    let mut context = context.as_ref().borrow_mut();

    context.registrar.register_type(
        Box::from("String"),
        Box::new(|value, _context| matches!(value, PyretValue::String(_))),
    );
}
