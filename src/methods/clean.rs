use std::path::PathBuf;

use crate::{
    file::rmdir,
    models::{Command, Config},
};

pub fn clean(cfg: Config, _cmd: Command) {
    let template_folder = PathBuf::from(&cfg.folder);
    if rmdir(&template_folder) {
        println!("folder '{}' removed", cfg.folder)
    } else {
        println!("could not remove '{}' folder.", cfg.folder);
    }
}
