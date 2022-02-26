use std::{cell::RefCell, rc::Rc};

use v8::inspector::*;

// Handles logging messages from the runtime.
pub struct RuntimeInspector {
    client: V8InspectorClientBase,
    inspector: Rc<RefCell<v8::UniquePtr<V8Inspector>>>,
}

impl RuntimeInspector {
    const CONTEXT_GROUP_ID: i32 = 1;

    pub fn new(isolate: &mut v8::OwnedIsolate, context: v8::Global<v8::Context>) -> Box<Self> {
        let scope = &mut v8::HandleScope::new(isolate);

        let client = V8InspectorClientBase::new::<Self>();

        let mut self_ = Box::new(Self {
            client,
            inspector: Default::default(),
        });
        self_.inspector = Rc::new(RefCell::new(V8Inspector::create(scope, &mut *self_).into()));

        let context = v8::Local::new(scope, context);
        let name = StringView::from(&b"global context"[..]);

        self_
            .inspector
            .borrow_mut()
            .as_mut()
            .unwrap()
            .context_created(context, Self::CONTEXT_GROUP_ID, name);

        self_
    }
}

impl V8InspectorClientImpl for RuntimeInspector {
    fn base(&self) -> &V8InspectorClientBase {
        &self.client
    }

    fn base_mut(&mut self) -> &mut V8InspectorClientBase {
        &mut self.client
    }

    fn console_api_message(
        &mut self,
        _context_group_id: i32,
        _level: i32,
        message: &StringView,
        _url: &StringView,
        _line_number: u32,
        _column_number: u32,
        _stack_trace: &mut V8StackTrace,
    ) {
        println!("{message}");
    }
}
