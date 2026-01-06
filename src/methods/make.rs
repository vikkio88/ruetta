use std::path::PathBuf;

use crate::{
    file::{is_dir, mkdir},
    methods::args::get_make_args,
    models::{Command, Config},
};

pub fn make(cfg: Config, cmd: Command) {
    let args = match get_make_args(&cmd.args, &cfg) {
        Ok(args) => args,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

    let target_folder = &PathBuf::from(&args.target_folder);
    if !is_dir(&target_folder) {
        println!(
            "target folder '{}' does not exist creating it...",
            args.target_folder
        );
        if !mkdir(&target_folder) {
            println!("error: could not create folder '{}'", args.target_folder);
            return;
        }
    }

    // for each file in the template folder interpolate and generate
}
