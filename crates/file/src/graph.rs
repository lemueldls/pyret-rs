use crate::PyretFile;

pub trait PyretGraph {
    fn register(&mut self, name: &str) -> usize;

    fn get(&self, file_id: usize) -> &PyretFile;
}
