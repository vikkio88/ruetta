use std::{
    env,
    fs::{self, create_dir},
    path::PathBuf,
};

pub fn home_path() -> PathBuf {
    env::home_dir().unwrap()
}

pub fn exists(filepath: &PathBuf) -> bool {
    fs::exists(filepath).unwrap_or(false)
}

pub fn mkdir(path: &PathBuf) -> bool {
    create_dir(path).is_ok()
}

pub fn rmdir(path: &PathBuf) -> bool {
    fs::remove_dir_all(path).is_ok()
}
