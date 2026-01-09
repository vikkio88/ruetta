use crate::{
    methods::args::get_info_args,
    models::{Command, Config},
    templates::Template,
};

pub fn info(cfg: Config, cmd: Command) {
    let args = match get_info_args(&cmd.args, &cfg) {
        Ok(args) => args,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

    let tpl = match Template::load_from_folder(&args.path) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Failed to load template: {}", e);
            return;
        }
    };

    let files = tpl.files().len();

    match tpl.description() {
        Some(desc) => println!(
            "{}:\n\t{}\n\nthis template will create: {} file(s)",
            args.path.display(),
            desc,
            files
        ),
        None => println!(
            "{}:\n\tNo description available for this template.\n\nthis template will create: {} file(s)",
            args.path.display(),
            files
        ),
    }
}
