use crate::{
    methods::{clean, help, help_with_error, init},
    models::Config,
    utils::{load_config, parse_args},
};

mod consts;
mod file;
mod methods;
mod models;
mod utils;

fn main() {
    let config = match load_config() {
        Some(f) => f,
        None => Config::default(),
    };
    let parsing = parse_args();
    let cmd = match parsing {
        Ok(c) => c,
        Err(s) => {
            help_with_error(s);
            return;
        }
    };

    match cmd.method {
        utils::Method::Init => init(config, cmd),
        utils::Method::Clean => clean(config, cmd),
        utils::Method::Help => help(),
    }
}
