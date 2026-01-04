use std::path::{Path, PathBuf};

use crate::{
    file::{exists, mkdir, rmdir},
    models::Config,
};

pub fn init(c: Config) {
    let template_folder = PathBuf::from(&c.folder);
    if exists(&template_folder) {
        println!("'{}' exists", c.folder);
        return;
    }

    mkdir(&template_folder);
    println!("created dir: '{}'", c.folder);
}

pub fn clean(c: Config) {
    let template_folder = PathBuf::from(&c.folder);
    if rmdir(&template_folder) {
        println!("folder '{}' removed", c.folder)
    } else {
        println!("could not remove '{}' folder.", c.folder);
    }
}
