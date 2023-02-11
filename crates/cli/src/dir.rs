use std::{fs, path::PathBuf};

pub fn join(path: &str) -> PathBuf {
    let dir = if let Some(cache_dir) = dirs_next::cache_dir() {
        cache_dir.join("pyret")
    } else if let Some(home_dir) = dirs_next::home_dir() {
        home_dir.join(".pyret")
    } else {
        panic!("Could not set the Pyret directory")
    };

    if !dir.exists() {
        fs::create_dir_all(&dir).unwrap();
    }

    dir.join(path)
}
