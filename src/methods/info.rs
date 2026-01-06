use crate::{
    methods::args::get_info_args,
    models::{Command, Config},
};

pub fn info(cfg: Config, cmd: Command) {
    let args = match get_info_args(&cmd.args, &cfg) {
        Ok(args) => args,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

    println!("{} {}", args.language, args.template);
}
