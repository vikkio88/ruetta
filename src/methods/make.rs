use std::path::PathBuf;

use crate::{
    file::{is_dir, mkdir},
    methods::args::get_make_args,
    models::{Command, Config},
    templates::{Template, WriteParams},
};

pub fn make(cfg: Config, cmd: Command) {
    let args = match get_make_args(&cmd.args, &cfg) {
        Ok(args) => args,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

    let tpl = match Template::load_from_folder(&args.template_path) {
        Ok(t) => t,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };
    let target_folder = &PathBuf::from(&args.target_folder);

    if !is_dir(target_folder) {
        println!(
            "target folder '{}' does not exist creating it...",
            args.target_folder
        );
        if !mkdir(target_folder) {
            println!("error: could not create folder '{}'", args.target_folder);
            return;
        }
    }

    if args.is_dry_run {
        println!("Dry running '{}':\n\n", tpl.path.display());
        for f in tpl.files() {
            match f.to(&args.name, &args.target_folder) {
                Ok(path) => println!("Would create: {}", path),
                Err(err) => eprintln!("Error rendering path: {}", err),
            }
        }
        return;
    }
    if args.is_force {
        println!("'--force' param was passed, forcing overwrite if files exist.")
    }

    match tpl.write(WriteParams {
        name: &args.name,
        target_folder: &args.target_folder,
        is_force: args.is_force,
    }) {
        Ok(res) => println!("{}", res),
        Err(err) => println!("{}", err),
    };
}
