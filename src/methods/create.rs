use crate::{
    // file::home_path,
    file::{exists, mkdir, write_file},
    methods::args::get_create_args,
    models::{Command, Config},
    templates::INDEX_FILE,
};

// maybe add editor command and open template after
// use std::process::Command as StdCmd;
// // StdCmd::new("zed")
// //     .arg(home_path().join("sides"))
// //     .spawn()
// //     .unwrap();

pub fn create(cfg: Config, cmd: Command) {
    let args = match get_create_args(&cmd.args, &cfg) {
        Ok(args) => args,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

    if exists(&args.path) {
        eprintln!(
            "Template {}/{} exists in path '{}'!",
            args.language,
            args.template,
            args.path.display()
        );
        return;
    }

    if !mkdir(&args.path) {
        eprintln!("Could not create folder {}", args.path.display());
        return;
    }

    let index_content = format!(
        "---
description: please add a description
to: <%- folder %>/somefile.{}
---",
        args.language
    );
    let index = &args.path.join(INDEX_FILE);
    match write_file(&index, &index_content) {
        Ok(_) => {
            println!(
                "Generated empty template {}/{} on path '{}'\n\tindex file: {}",
                args.language,
                args.template,
                args.path.display(),
                index.display()
            );
        }
        Err(_) => todo!(),
    }
}
