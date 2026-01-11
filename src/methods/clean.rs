use std::path::PathBuf;

use crate::{
    consts::CONFIG_FILE_NAME,
    file::{home_path, rm_file, rmdir},
    models::{Command, Config},
};

pub fn clean(cfg: Config, _cmd: Command) {
    let template_folder = PathBuf::from(&cfg.folder);

    let config_path = &home_path().join(CONFIG_FILE_NAME);
    if rm_file(config_path) {
        println!("Removed '{}'", config_path.display())
    }

    if rmdir(&template_folder) {
        println!("Removed folder '{}' ", cfg.folder)
    } else {
        eprintln!("Could not remove '{}' folder.", cfg.folder);
    }
}
