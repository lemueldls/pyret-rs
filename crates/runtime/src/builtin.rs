use std::{
    collections::HashMap,
    fs::{self, read_dir},
    path::PathBuf,
};

use once_cell::sync::Lazy;

pub static BUILTIN: Lazy<HashMap<String, String>> = Lazy::new(|| {
    let mut builtins = HashMap::new();

    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("builtin");

    for entry in read_dir(dir.clone()).unwrap() {
        let path = entry.unwrap().path();

        if path.is_file() {
            builtins.insert(
                path.file_stem().unwrap().to_string_lossy().to_string(),
                fs::read_to_string(path).unwrap(),
            );
        }
    }

    builtins
});
