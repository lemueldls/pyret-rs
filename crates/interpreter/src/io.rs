use crate::PyretValue;

#[derive(Default)]
pub struct Io {
    output: Option<Box<dyn Fn(&PyretValue)>>,
}

impl Io {
    pub fn read_out(&mut self, callback: Box<dyn Fn(&PyretValue)>) {
        if self.output.is_none() {
            self.output = Some(callback);
        } else {
            unimplemented!()
        }
    }

    pub fn write_out(&self, value: &PyretValue) {
        if let Some(output) = &self.output {
            output(value);
        }
    }
}
