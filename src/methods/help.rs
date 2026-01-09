use crate::methods::{consts::HEADER, version::get_version};

pub fn help_with_error(msg: String) {
    println!("error: {}\n", msg);
    help();
}

pub fn help() {
    println!(
        "\
{}

version: {}

USAGE
    ruetta <command> [arguments]

COMMANDS
    init
        create the template folder

        example:
            ruetta init

    clean
        aliases: cl
        remove the template folder

        example:
            ruetta clean

    info
        aliases: i
        show information about a template

        example:
            ruetta info svelte component

    create
        aliases: c
        create a new template definition

        example:
            ruetta create svelte component

    make
        aliases: m, mk
        generate a file from a template

        example:
            ruetta make svelte component Button ./src/components

    help
        aliases: h, -h
        show this help message

        example:
            ruetta help
    version
        aliases: v, -v, --version
        show the app version

        example:
            ruetta help
",
        HEADER,
        get_version()
    );
}
