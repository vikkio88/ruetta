use crate::{
    methods::{clean, init},
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
            println!("{}", s);
            return;
        }
    };

    match cmd.method {
        utils::Method::Init => init(config),
        utils::Method::Clean => clean(config),
        utils::Method::Help => println!("help"),
    }

    // init(config);
    // clean(config);
}
