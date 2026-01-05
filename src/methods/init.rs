use std::path::PathBuf;

use crate::{
    commands::Command,
    file::{exists, mkdir},
    models::Config,
};

pub fn init(cfg: Config, _cmd: Command) {
    let template_folder = PathBuf::from(&cfg.folder);
    if exists(&template_folder) {
        println!("'{}' exists", cfg.folder);
        return;
    }

    mkdir(&template_folder);
    println!("created dir: '{}'", cfg.folder);
}
