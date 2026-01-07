use std::{fs, path::PathBuf};

use crate::{
    file::{is_file, read_file},
    templates::{INDEX_FILE, Template},
};

pub fn load_from_file(path: &PathBuf) -> Result<Template, String> {
    let path = if !is_file(path) {
        path.join(INDEX_FILE)
    } else {
        path.to_path_buf()
    };

    let content = read_file(&path)?;

    Template::from(&content)
}
