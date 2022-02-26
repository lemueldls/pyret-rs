use crate::{bindings, error::Error, inspector::RuntimeInspector};

use std::{cell::RefCell, rc::Rc, sync::Once};

pub struct RuntimeState {
    pub global_context: Option<v8::Global<v8::Context>>,
}

pub struct Runtime {
    isolate: Option<v8::OwnedIsolate>,
    inspector: Option<Box<RuntimeInspector>>,
}

impl Runtime {
    pub fn new() -> Self {
        static PYRET_INIT: Once = Once::new();
        PYRET_INIT.call_once(|| {
            let platform = v8::new_default_platform(0, false).make_shared();

            v8::V8::initialize_platform(platform);
            v8::V8::initialize();
        });

        let mut isolate = v8::Isolate::new(Default::default());

        let global_context = {
            let scope = &mut v8::HandleScope::new(&mut isolate);
            let context = v8::Context::new(scope);

            v8::Global::new(scope, context)
        };

        let inspector = RuntimeInspector::new(&mut isolate, global_context.clone());

        isolate.set_slot(Rc::new(RefCell::new(RuntimeState {
            global_context: Some(global_context),
        })));

        Self {
            isolate: Some(isolate),
            inspector: Some(inspector),
        }
    }

    pub fn global_context(&mut self) -> v8::Global<v8::Context> {
        let state = Self::state(self.isolate());
        let state = state.borrow();

        state.global_context.clone().unwrap()
    }

    pub fn isolate(&mut self) -> &mut v8::OwnedIsolate {
        self.isolate.as_mut().unwrap()
    }

    pub fn handle_scope(&mut self) -> v8::HandleScope {
        let context = self.global_context();

        v8::HandleScope::with_context(self.isolate(), context)
    }

    fn state(isolate: &v8::Isolate) -> Rc<RefCell<RuntimeState>> {
        let state = isolate.get_slot::<Rc<RefCell<RuntimeState>>>().unwrap();

        state.clone()
    }

    pub fn execute_script(&mut self, name: &str, source_code: &str) -> Result<(), Error> {
        let global_context = self.global_context();
        let scope = &mut self.handle_scope();

        let source = v8::String::new(scope, source_code).unwrap();
        let name = v8::String::new(scope, name).unwrap();
        let origin = bindings::script_origin(scope, name);

        let scope = &mut v8::TryCatch::new(scope);

        let script = match v8::Script::compile(scope, source, Some(&origin)) {
            Some(script) => script,
            None => {
                let exception = scope.exception().unwrap();

                return exception_to_err_result(scope, exception);
            }
        };

        match script.run(scope) {
            // Save exported values to the global context.
            Some(value) => {
                let exports = value.to_object(scope).unwrap();

                let property_names: Vec<String> = {
                    let input = exports.get_property_names(scope).unwrap().into();

                    serde_v8::from_v8(scope, input).unwrap()
                };

                let global = global_context.open(scope).global(scope);

                for property in property_names {
                    let key = serde_v8::to_v8(scope, property).unwrap();

                    let input = exports.get(scope, key).unwrap();

                    global.set(scope, key, input);
                }

                Ok(())
            }
            None => {
                let exception = scope.exception().unwrap();

                exception_to_err_result(scope, exception)
            }
        }
    }
}

impl Drop for Runtime {
    fn drop(&mut self) {
        // Drop the inspector before the runtime.
        self.inspector.take();
    }
}

pub(crate) fn exception_to_err_result<'s, T>(
    scope: &mut v8::HandleScope<'s>,
    exception: v8::Local<v8::Value>,
) -> Result<T, Error> {
    let is_terminating_exception = scope.is_execution_terminating();

    let mut exception = exception;

    if is_terminating_exception {
        scope.cancel_terminate_execution();

        if exception.is_null_or_undefined() {
            let message = v8::String::new(scope, "execution terminated").unwrap();
            exception = v8::Exception::error(scope, message);
        }
    }

    let error = Error::from_v8_exception(scope, exception);

    if is_terminating_exception {
        // Re-enable exception termination.
        scope.terminate_execution();
    }

    Err(error)
}
