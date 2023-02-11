use crate::{io::Io, registrar::Registrar};

#[derive(Default)]
pub struct Context {
    pub io: Io,
    pub registrar: Registrar,
}
