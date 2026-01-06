use std::collections::HashMap;

use crate::{consts::DEFAULT_TEMPLATE_FOLDER, file::home_path};

pub struct Config {
    pub folder: String,
    pub aliases: HashMap<String, String>,
}

impl Config {
    pub fn default() -> Config {
        Config {
            folder: String::from(home_path().join(DEFAULT_TEMPLATE_FOLDER).to_str().unwrap()),
            aliases: HashMap::new(),
        }
    }
}
