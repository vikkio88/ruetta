use crate::{
    methods::args::get_info_args,
    models::{Command, Config},
    templates::load_from_file,
};

pub fn info(cfg: Config, cmd: Command) {
    let args = match get_info_args(&cmd.args, &cfg) {
        Ok(args) => args,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

    let tpl = match load_from_file(&args.path) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Failed to load template: {}", e);
            return;
        }
    };

    match tpl.description {
        Some(desc) => println!("{}:\n\t{}", args.path.display(), desc),
        None => println!(
            "{}:\n\tNo description available for this template.",
            args.path.display()
        ),
    }
}
