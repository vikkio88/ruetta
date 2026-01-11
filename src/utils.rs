use std::{collections::HashMap, env, path::Path};

use crate::{
    consts::CONFIG_FILE_NAME,
    file::{exists, home_path, read_file},
    models::{Command, Config, Method},
};

pub fn load_config() -> Option<Config> {
    let path = Path::new(CONFIG_FILE_NAME);
    let full_path = home_path().join(path).to_path_buf();
    if !exists(&full_path) {
        return None;
    }

    let content = match read_file(&full_path) {
        Ok(c) => c,
        Err(_) => {
            println!("Could not load config file from '{}'", full_path.display());
            return None;
        }
    };

    let config: Config = match serde_json::from_str(&content) {
        Ok(c) => c,
        Err(err) => {
            println!("Could not parse config file from '{}'", full_path.display());
            return None;
        }
    };

    return Some(config);
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
