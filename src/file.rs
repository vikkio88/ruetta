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

pub fn is_file(path: &PathBuf) -> bool {
    fs::metadata(path).map(|m| m.is_file()).unwrap_or(false)
}

pub fn is_dir(path: &PathBuf) -> bool {
    metadata(path).map(|m| m.is_dir()).unwrap_or(false)
}

pub fn mkdir(path: &PathBuf) -> bool {
    create_dir_all(path).is_ok()
}

pub fn rmdir(path: &PathBuf) -> bool {
    if !exists(path) {
        return true;
    }
    remove_dir_all(path).is_ok()
}

pub fn read_file(path: &PathBuf) -> Result<String, String> {
    let contents = fs::read_to_string(path)
        .map_err(|e| format!("Cannot load file '{}': {}", path.display(), e))?;

    Ok(contents)
}

pub fn write_file(path: &PathBuf, content: &str) -> Result<(), String> {
    fs::write(path, content).map_err(|e| format!("Cannot write file: {}", e))?;

    Ok(())
}

pub fn rm_file(path: &PathBuf) -> bool {
    if !path.exists() {
        return true;
    }

    fs::remove_file(path).is_ok()
}
