use crate::{commands::Command, models::Config};

pub fn create(cfg: Config, cmd: Command) {
    println!("Creating {} on '{}'", cmd, cfg.folder)
}
