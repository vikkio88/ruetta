use std::path::{Path, PathBuf};

use crate::{
    consts::CONFIG_FILE_NAME,
    file::{exists, home_path, mkdir, write_file},
    models::{Command, Config},
};

pub fn init(cfg: Config, _cmd: Command) {
    let config_path = home_path().join(Path::new(CONFIG_FILE_NAME));
    // TODO: force init by nuking everything
    if !exists(&config_path) {
        println!("writing config file to '{}'", config_path.display());
        let config_json = match serde_json::to_string_pretty(&cfg) {
            Ok(s) => s,
            Err(_) => {
                eprintln!("Error whilst parsing config.");
                return;
            }
        };
        match write_file(&config_path, &config_json) {
            Ok(_) => {}
            Err(_) => {
                eprintln!("Could not write config file to '{}'", config_path.display());
                return;
            }
        }
    }

    let template_folder = PathBuf::from(&cfg.folder);
    if exists(&template_folder) {
        println!("'{}' exists", cfg.folder);
        return;
    }

    mkdir(&template_folder);
    println!("created dir: '{}'", cfg.folder);
}
