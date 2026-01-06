use crate::{
    methods::{
        clean::clean,
        create::create,
        help::{help, help_with_error},
        info::info,
        init::init,
        make::make,
    },
    models::{Config, Method},
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
        Method::Init => init(config, cmd),
        Method::Info => info(config, cmd),
        Method::Clean => clean(config, cmd),
        Method::Create => create(config, cmd),
        Method::Make => make(config, cmd),
        Method::Help => help(),
    }
}
