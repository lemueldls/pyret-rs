#![allow(clippy::use_self, clippy::new_without_default, clippy::boxed_local)]

use std::{cell::RefCell, rc::Rc, sync::Arc};

use js_sys::{Array, Function, Object, Reflect};
use pyret_error::PyretResult;
use pyret_file::PyretFile;
use pyret_interpreter::{
    trove,
    value::{context::Context, PyretFunction, PyretValue},
    Interpreter, PyretGraph,
};
use pyret_number::{BigRational, PyretNumber};
use wasm_bindgen::prelude::*;

#[cfg(feature = "console_error_panic_hook")]
#[wasm_bindgen(start, skip_typescript)]
#[inline]
pub fn set_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub struct PyretRuntime {
    interpreter: Interpreter,
}

struct PyretGraphWrapper;

impl PyretGraph for PyretGraphWrapper {
    fn register(&mut self, _name: &str) -> usize {
        todo!()
    }

    fn get(&self, _file_id: usize) -> &PyretFile {
        todo!()
    }
}

#[wasm_bindgen]
impl PyretRuntime {
    #[wasm_bindgen(constructor)]
    #[must_use]
    pub fn new() -> Self {
        let mut interpreter = Interpreter::new(PyretGraphWrapper);

        Self { interpreter }
    }

    #[wasm_bindgen(js_name = "useContext")]
    pub fn use_context(&mut self, context: &str) {
        self.interpreter.import_trove(context).unwrap();
    }

    pub fn run(&mut self, file_id: usize) -> Box<[JsValue]> {
        let values = self.interpreter.interpret(file_id).unwrap();

        values.iter().map(pyret_to_js).collect()
    }
}

#[must_use]
fn pyret_to_js(value: &Rc<PyretValue>) -> JsValue {
    let (tag, value) = match &**value {
        PyretValue::Number(number) => match number {
            PyretNumber::Exact(exact) => {
                ("Exactnum", serde_wasm_bindgen::to_value(&exact).unwrap())
            }
            PyretNumber::Rough(rough) => ("Roughnum", JsValue::from_f64(*rough)),
        },
        PyretValue::String(string) => ("String", JsValue::from_str(string)),
        PyretValue::Boolean(boolean) => ("Boolean", JsValue::from_bool(*boolean)),
        PyretValue::Function(function) => {
            let function = function.clone();

            let closure = Closure::wrap(Box::new(move |args: Vec<JsValue>| {
                let args = args
                    .into_iter()
                    .map(|value| Rc::new(js_to_pyret(value)))
                    .collect::<Box<[Rc<PyretValue>]>>();

                let value = function.call(&args, 0).unwrap();

                pyret_to_js(&value)
            }) as Box<dyn FnMut(Vec<JsValue>) -> JsValue>);

            ("Function", closure.into_js_value())
        }
        PyretValue::Nothing => ("Nothing", JsValue::UNDEFINED),
    };

    let object = Object::new();

    Reflect::set(&object, &"tag".into(), &tag.into()).unwrap();
    Reflect::set(&object, &"value".into(), &value).unwrap();

    JsValue::from(object)
}

fn js_to_pyret(value: JsValue) -> PyretValue {
    let object = value.dyn_into::<Object>().unwrap();

    let tag = Reflect::get(&object, &"tag".into())
        .unwrap()
        .as_string()
        .unwrap();

    let value = Reflect::get(&object, &"value".into()).unwrap();

    match tag.as_str() {
        "Exactnum" => PyretValue::Number(PyretNumber::Exact(
            serde_wasm_bindgen::from_value(value).unwrap(),
        )),
        "Roughnum" => PyretValue::Number(PyretNumber::Rough(value.as_f64().unwrap())),
        "String" => PyretValue::String(value.as_string().unwrap().into_boxed_str()),
        "Boolean" => PyretValue::Boolean(value.as_bool().unwrap()),
        "Function" => {
            let function = value.dyn_into::<Function>().unwrap();

            let name = function.name().as_string().unwrap().into_boxed_str();

            let generics = (0..function.length())
                .map(|_| Box::from("Any"))
                .collect::<Vec<_>>()
                .into_boxed_slice();

            let any = trove::global::Any::predicate();

            let param_types = (0..function.length())
                .map(|_| Arc::clone(&any))
                .collect::<Vec<_>>()
                .into_boxed_slice();

            let body = Rc::new(move |args: &[Rc<PyretValue>], _context| {
                let args = args.iter().map(pyret_to_js).collect::<Array>();

                let value = function.apply(&JsValue::UNDEFINED, &args).unwrap();

                Ok(Rc::new(js_to_pyret(value)))
            });

            let context = Rc::new(RefCell::new(Context::default()));

            PyretValue::Function(PyretFunction::new(
                name,
                generics,
                param_types,
                any,
                body,
                context,
            ))
        }
        _ => todo!(),
    }
}
