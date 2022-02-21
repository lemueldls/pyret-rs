// mod builtin;
mod compiler;
mod graph;

// pub use builtins::*;
pub use compiler::{Builtin, COMPILER};
pub use graph::Graph;

use std::{collections::HashMap, fs::read_dir, path::PathBuf};

pub use anyhow::Result;
use once_cell::sync::Lazy;

macro_rules! compile {
    ($dir: expr) => {{
        let mut builtins = HashMap::new();

        let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join($dir);

        for entry in read_dir(dir).unwrap() {
            let path = entry.unwrap().path();

            if path.is_file() {
                let id = path.file_stem().unwrap().to_string_lossy().to_string();

                let builtin = COMPILER.compile(path);

                builtins.insert(id, builtin);
            } else {
                unimplemented!("Folders are not handled.");
            }
        }

        builtins
    }};
}

pub static TROVE: Lazy<HashMap<String, Builtin>> = Lazy::new(|| compile!("ts/trove"));
