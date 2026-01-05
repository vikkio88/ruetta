use std::path::PathBuf;

use crate::{commands::Command, file::exists, models::Config};

struct MakeArgs {
    language: String,
    template: String,
    folder: String,
    name: String,
}

const INDEX: &str = "index.ruetta";

fn parse_args(args: &[String], cfg: &Config) -> Result<MakeArgs, String> {
    match args {
        [language, template, folder, name] => {
            let tmpl_folders = PathBuf::from(&cfg.folder);
            let file = tmpl_folders.join(language).join(template).join(INDEX);

            if !exists(&file) {
                return Err(format!(
                    "file '{}' does not exist in template folder '{}'",
                    file.to_str().unwrap(),
                    cfg.folder,
                ));
            }

            Ok(MakeArgs {
                language: language.clone(),
                template: template.clone(),
                folder: folder.clone(),
                name: name.clone(),
            })
        }
        _ => {
            let mut missing = Vec::new();
            if args.get(0).is_none() {
                missing.push("language");
            }
            if args.get(1).is_none() {
                missing.push("template");
            }
            if args.get(2).is_none() {
                missing.push("folder");
            }
            if args.get(3).is_none() {
                missing.push("name");
            }

            let missing_str = missing.join(", ");
            Err(format!(
                "Missing argument(s): {}. Example usage:\n\
                ruetta make <language> <template> <folder> <name>\n\
                e.g. ruetta make svelte c ciao/ciao Counter",
                missing_str
            ))
        }
    }
}

pub fn make(cfg: Config, cmd: Command) {
    let args = match parse_args(&cmd.args, &cfg) {
        Ok(args) => args,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

    println!("{}", args.folder);
}
