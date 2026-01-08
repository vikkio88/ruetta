use std::path::PathBuf;

use crate::{
    file::{is_dir, mkdir, write_file},
    methods::args::get_make_args,
    models::{Command, Config},
    templates::Template,
};

//TODO: maybe add dry run option

pub fn make(cfg: Config, cmd: Command) {
    let args = match get_make_args(&cmd.args, &cfg) {
        Ok(args) => args,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

    let tpl = match Template::from_file(&args.template_path) {
        Ok(t) => t,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

    let to = match tpl.to(&args.name, &args.target_folder) {
        Ok(t) => t,
        Err(e) => {
            println!("Error whilst trying to get target name: {}", e);
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

    let body = match tpl.body(&args.name) {
        Ok(b) => b,
        Err(err) => {
            println!(
                "Cannot render body of the template: {}\n error: {}",
                args.template_path.display(),
                err
            );
            return;
        }
    };

    match write_file(&PathBuf::from(&to), &body) {
        Ok(_) => {
            println!("File successfully created at:\n\t{}", to);
        }
        Err(err) => {
            println!("Error whilst writing the file '{}'\n\terror: {}", to, err);
            return;
        }
    };
}
