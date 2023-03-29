use crate::{io::Io, value::registrar::Registrar};

#[derive(Default)]
pub struct Context {
    pub io: Io,
    pub registrar: Registrar,
}
