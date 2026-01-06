use crate::models::{Command, Config};

pub fn create(cfg: Config, cmd: Command) {
    println!("Creating {} on '{}'", cmd, cfg.folder)
}
