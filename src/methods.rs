use std::path::{Path, PathBuf};

use crate::{
    file::{exists, mkdir, rmdir},
    models::Config,
    utils::Command,
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

pub fn clean(cfg: Config, _cmd: Command) {
    let template_folder = PathBuf::from(&cfg.folder);
    if rmdir(&template_folder) {
        println!("folder '{}' removed", cfg.folder)
    } else {
        println!("could not remove '{}' folder.", cfg.folder);
    }
}

pub fn help_with_error(msg: String) {
    println!("error: {}\n", msg);
    help();
}

pub fn help() {
    println!(
        "\
ruetta

USAGE
    ruetta <command>

COMMANDS
    init     create the template folder
    clean    remove the template folder
    help     show this help
"
    );
}
