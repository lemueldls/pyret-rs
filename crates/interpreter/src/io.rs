use crate::PyretValue;

pub enum Output<'a> {
    Display(&'a PyretValue),
    Print(Box<str>),
    Test,
}

#[derive(Default)]
pub struct Io {
    output: Option<Box<dyn Fn(Output)>>,
}

impl Io {
    pub fn read(&mut self, callback: Box<dyn Fn(Output)>) {
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
