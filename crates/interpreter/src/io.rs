use std::rc::Rc;

use crate::{PyretValue, TestResult};

pub enum Output {
    Display(Rc<PyretValue>),
    Print(Box<str>),
    Test {
        label: Option<Box<str>>,
        results: Box<[TestResult]>,
    },
}

type OutputFn = Box<dyn Fn(Output)>;

#[derive(Default)]
pub struct Io {
    output: Option<OutputFn>,
}

impl Io {
    pub fn read(&mut self, callback: OutputFn) {
        if self.output.is_none() {
            self.output = Some(callback);
        } else {
            unimplemented!()
        }
    }

    pub fn write(&self, value: Output) {
        if let Some(output) = &self.output {
            output(value);
        }
    }
}
