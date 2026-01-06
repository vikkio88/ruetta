use std::{
    env,
    fs::{self, create_dir_all, metadata, remove_dir_all},
    path::PathBuf,
};

pub fn home_path() -> PathBuf {
    env::home_dir().unwrap()
}

pub fn exists(filepath: &PathBuf) -> bool {
    fs::exists(filepath).unwrap_or(false)
}

// pub fn is_file(path: &PathBuf) -> bool {
//     fs::metadata(path).map(|m| m.is_file()).unwrap_or(false)
// }

pub fn is_dir(path: &PathBuf) -> bool {
    metadata(path).map(|m| m.is_dir()).unwrap_or(false)
}

pub fn mkdir(path: &PathBuf) -> bool {
    create_dir_all(path).is_ok()
}

pub fn rmdir(path: &PathBuf) -> bool {
    remove_dir_all(path).is_ok()
}
