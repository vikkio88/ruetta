pub fn help_with_error(msg: String) {
    println!("error: {}\n", msg);
    help();
}

pub fn help() {
    println!(
        "\
ruetta

USAGE
    ruetta <command>

COMMANDS
    init     create the template folder
    clean    remove the template folder
    info     info about a particular template
        ruetta info svelte component
    make     create a templated file
        ruetta make svelte component name /folder/to/dump/in
    help     show this help
"
    );
}
