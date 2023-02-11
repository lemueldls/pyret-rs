pub mod number;
use std::cell::RefCell;

use crate::{context::Context, Rc, Trove};

pub struct Global;

impl Trove for Global {
    fn register(context: Rc<RefCell<Context>>) {
        context.borrow_mut().registrar.register_builtin_function(
            "display",
            1,
            Box::new(|args, context| {
                let value = &args[0];

                context.borrow().io.write_out(value);

                Ok(Rc::clone(value))
            }),
        );

        number::register(&context);
    }
}