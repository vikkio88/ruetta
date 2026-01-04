use std::{env, path::Path};

use crate::{
    consts::CONFIG_FILE_NAME,
    file::{exists, home_path},
    models::Config,
};

pub fn load_config() -> Option<Config> {
    let path = Path::new(CONFIG_FILE_NAME);
    let full_path = home_path().join(path);
    if exists(&full_path.to_path_buf()) {
        return Some(Config {
            folder: "loaded from file".into(),
        });
    }

    None
}
#[derive(Debug)]
pub enum Method {
    Init,
    Clean,
    Help,
}

#[derive(Debug)]
pub struct Command {
    pub method: Method,
    pub args: Vec<String>,
}

pub fn parse_args() -> Result<Command, String> {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    if args.len() < 1 {
        return Err("No command was passed".into());
    }

    let command_name = args.remove(0).to_ascii_lowercase();
    let method = match command_name.as_str() {
        "init" | "i" => Method::Init,
        "clean" | "c" => Method::Clean,
        _ => Method::Help,
    };

    Ok(Command { method, args })
}
