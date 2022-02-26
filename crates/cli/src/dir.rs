use std::{fs, path::PathBuf};

pub fn join(path: &str) -> PathBuf {
    let dir = if let Some(cache_dir) = dirs::cache_dir() {
        cache_dir.join("pyret")
    } else if let Some(home_dir) = dirs::home_dir() {
        home_dir.join(".pyret")
    } else {
        panic!("Could not set the Pyret directory")
    };

    if !dir.exists() {
        fs::create_dir(&dir).unwrap();
    }

    dir.join(path)
}
