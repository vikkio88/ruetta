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
    help     show this help
"
    );
}
