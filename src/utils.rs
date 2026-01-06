use std::{collections::HashMap, env, path::Path};

use crate::{
    consts::CONFIG_FILE_NAME,
    file::{exists, home_path},
    models::{Command, Config, Method},
};

pub fn load_config() -> Option<Config> {
    let path = Path::new(CONFIG_FILE_NAME);
    let full_path = home_path().join(path);
    if exists(&full_path.to_path_buf()) {
        return Some(Config {
            folder: "./examples".into(),
            aliases: HashMap::from([("sv".into(), "svelte".into())]),
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
    let method = Method::from(command_name.as_str())?;

    Ok(Command { method, args })
}
