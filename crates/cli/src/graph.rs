use std::fs;

use pyret_file::{graph::PyretGraph, PyretFile};

#[derive(Default)]
pub struct FsGraph {
    pub files: Vec<PyretFile>,
}

impl PyretGraph for FsGraph {
    fn register(&mut self, name: &str) -> usize {
        let file_id = self.files.len();

        self.files.push(PyretFile::new(
            format!("file://{name}").into_boxed_str(),
            fs::read_to_string(name).unwrap().into_boxed_str(),
        ));

        file_id
    }

    fn get(&self, file_id: usize) -> &PyretFile {
        self.files.get(file_id).unwrap()
    }
}
