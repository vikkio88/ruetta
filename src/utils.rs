use std::{env, path::Path};

use crate::{
    commands::{Command, Method},
    consts::CONFIG_FILE_NAME,
    file::{exists, home_path},
    models::Config,
};

pub fn load_config() -> Option<Config> {
    let path = Path::new(CONFIG_FILE_NAME);
    let full_path = home_path().join(path);
    if exists(&full_path.to_path_buf()) {
        return Some(Config {
            folder: "./examples".into(),
        });
    }

    None
}

pub fn parse_args() -> Result<Command, String> {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    if args.is_empty() {
        return Err("No command was passed".into());
    }

    let command_name = args.remove(0).to_ascii_lowercase();
    let method = match command_name.as_str() {
        "init" | "i" => Method::Init,
        "clean" | "cl" => Method::Clean,
        "create" | "c" => Method::Create,
        "make" | "mk" | "m" => Method::Make,
        "help" | "h" | "-h" => Method::Help,
        _ => return Err(format!("Command '{}' not recognised", command_name)),
    };

    Ok(Command { method, args })
}
