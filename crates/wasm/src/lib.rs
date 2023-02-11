#![allow(clippy::use_self, clippy::new_without_default, clippy::boxed_local)]

use std::rc::Rc;

use js_sys::{Array, Function};
use pyret_interpreter::{trove, value::PyretValue, Interpreter, PyretGraph};
use pyret_number::{BigRational, PyretNumber};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[cfg(feature = "console_error_panic_hook")]
#[wasm_bindgen(start, skip_typescript)]
#[inline]
pub fn set_panic_hook() {
    console_error_panic_hook::set_once();
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "kind", content = "value")]
#[non_exhaustive]
pub enum PyretValueWrapper {
    ExactNumber(BigRational),
    RoughNumber(f64),
}

#[wasm_bindgen]
pub struct PyretRuntime {
    interpreter: Interpreter,
}

struct PyretGraphWrapper;

impl PyretGraph for PyretGraphWrapper {
    fn register(&mut self, name: &str) -> usize {
        todo!()
    }

    fn get(&self, file_id: usize) -> &pyret_interpreter::PyretFile {
        todo!()
    }
}

#[wasm_bindgen]
impl PyretRuntime {
    #[wasm_bindgen(constructor)]
    #[must_use]
    pub fn new() -> Self {
        let interpreter = Interpreter::new(PyretGraphWrapper);

        interpreter.use_context::<trove::Global>();

        Self { interpreter }
    }

    pub fn eval(&mut self, source: &str) -> Box<[JsValue]> {
        let values = self.interpreter.interpret(0).unwrap();

        Box::from_iter(
            values.iter().map(|value| {
                serde_wasm_bindgen::to_value(&PyretValueWrapper::from(value)).unwrap()
            }),
        )
    }

    pub fn call_function(&mut self, name: &str, args: Box<[JsValue]>) -> JsValue {
        let binding = self.interpreter.context.borrow_mut();

        let function = binding.registrar.get(name).unwrap();

        serde_wasm_bindgen::to_value(&PyretValueWrapper::from(&match &***function {
            PyretValue::Function(function) => function
                .call(
                    &args
                        .iter()
                        .map(|arg| {
                            serde_wasm_bindgen::from_value::<PyretValueWrapper>(arg.to_owned())
                                .unwrap()
                                .into()
                        })
                        .collect::<Vec<Rc<PyretValue>>>(),
                    Rc::clone(&self.interpreter.context),
                )
                .unwrap(),

            _ => todo!(),
        }))
        .unwrap()
    }

    pub fn register_function(&mut self, name: &str, body: Function) {
        self.interpreter
            .context
            .borrow_mut()
            .registrar
            .register_local_function(
                name,
                body.length() as usize,
                Box::new(move |args, _context| {
                    let value = body
                        .apply(
                            &JsValue::NULL,
                            &Array::from_iter(args.iter().map(|arg| {
                                serde_wasm_bindgen::to_value(&PyretValueWrapper::from(arg)).unwrap()
                            })),
                        )
                        .unwrap();

                    if value.is_falsy() {
                        todo!("Must return a value")
                    }

                    let wrapper: PyretValueWrapper = serde_wasm_bindgen::from_value(value).unwrap();

                    Ok(wrapper.into())
                }),
            )
    }

    // pub fn execute(&mut self, wrapper: JsValue) -> PyretValueWrapper {
    //     match wrapper.kind {
    //         _ => wrapper,
    //     }
    // }

    // #[wasm_bindgen(js_name = getValue)]
    // pub fn get_value(&mut self, ident: &str) -> Option<PyretValueWrapper> {
    //     Some(wrap(&**self.interpreter.register.get(ident)?))
    // }
}

impl From<&Rc<PyretValue>> for PyretValueWrapper {
    fn from(value: &Rc<PyretValue>) -> Self {
        match &**value {
            PyretValue::Number(number) => match number {
                PyretNumber::Exact(exact_number) => {
                    PyretValueWrapper::ExactNumber(exact_number.clone())
                }

                PyretNumber::Rough(rough_number) => PyretValueWrapper::RoughNumber(*rough_number),
            },

            _ => todo!(),
        }
    }
}

impl From<PyretValueWrapper> for Rc<PyretValue> {
    fn from(wrapper: PyretValueWrapper) -> Self {
        Rc::new(match wrapper {
            PyretValueWrapper::ExactNumber(exact_number) => {
                PyretValue::Number(PyretNumber::Exact(exact_number))
            }
            PyretValueWrapper::RoughNumber(rough_number) => {
                PyretValue::Number(PyretNumber::Rough(rough_number))
            }
        })
    }
}
