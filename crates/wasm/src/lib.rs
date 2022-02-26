mod utils;

use wasm_bindgen::prelude::*;

// use pyret_transpiler::Transpiler;

#[wasm_bindgen]
pub struct Pyret;

#[wasm_bindgen]
impl Pyret {
    pub fn new() -> Self {
        utils::set_panic_hook();

        Self {}
    }
}
