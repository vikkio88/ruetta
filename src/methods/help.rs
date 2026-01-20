use crate::methods::{consts::HEADER, version::get_version};

pub fn help_with_error(msg: String) {
    println!("error: {}\n", msg);
    help();
}

pub fn help() {
    println!(
        "\
{header}
version {version}

Sourcecode scaffolding utility
https://github.com/vikkio88/ruetta

USAGE
  ruetta <command> [arguments]

COMMANDS
  init
    Create the template folder
    Example: ruetta init

  clean (cl)
    Remove the template folder
    Example: ruetta clean

  list (ls, l)
    Show a list of all of the templates available
    Example: ruetta list

  info (i)
    Show information about a template
    Example: ruetta info svelte component

  create (c)
    Create a new template definition
    Example: ruetta create svelte component

  make (m, mk)
    Generate files from a template
    Example: ruetta make svelte component Button ./src/components

  help (h, -h)
    Show this help message
    Example: ruetta help

  version (v, -v, --version)
    Show the app version
    Example: ruetta version
",
        header = HEADER,
        version = get_version(),
    );
}
