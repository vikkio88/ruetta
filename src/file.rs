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
    match create_dir(path) {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn rmdir(path: &PathBuf) -> bool {
    match fs::remove_dir_all(path) {
        Ok(_) => true,
        Err(_) => false,
    }
}
