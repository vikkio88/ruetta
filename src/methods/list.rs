use std::path::Path;

use crate::{
    file::{ItemKind, ls},
    models::{Command, Config},
};

pub fn list(cfg: Config, _cmd: Command) {
    let tmpl_folder = Path::new(&cfg.folder).to_path_buf();
    let languages = match ls(&tmpl_folder) {
        Ok(f) => f,
        Err(err) => {
            eprintln!(
                "Could not access template folder at: '{}'\n\terror: {}",
                tmpl_folder.display(),
                err
            );
            return;
        }
    };

    println!(
        "Found {} language in '{}'\n",
        languages.len(),
        tmpl_folder.display()
    );

    for f in languages {
        if f.name == ".git" || f.kind == ItemKind::File {
            continue;
        }
        println!(
            "path: ({})\nlanguage: {}\ntemplates:",
            f.path.display(),
            f.name
        );
        let tmpls = match ls(&f.path) {
            Ok(t) => t,
            Err(err) => {
                eprintln!(
                    "\nerror: could not list items in folder '{}'\nerror:{}",
                    f.path.display(),
                    err
                );
                continue;
            }
        };
        for t in tmpls {
            println!("\t{}", t.name);
        }
        println!("\n");
    }
    println!("to show info about a template 'ruetta info [language] [template]'")
}
