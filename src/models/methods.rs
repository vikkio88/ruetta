use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum Method {
    Init,
    Info,
    List,
    Clean,
    Create,
    Make,
    Help,
    Version,
}
impl Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Method::Init => "init",
            Method::Info => "info",
            Method::List => "list",
            Method::Clean => "clean",
            Method::Create => "create",
            Method::Make => "make",
            Method::Help => "help",
            Method::Version => "version",
        };
        write!(f, "{}", name)
    }
}

impl Method {
    pub fn from(method: &str) -> Result<Self, String> {
        Ok(match method {
            "init" => Self::Init,
            "info" | "i" => Self::Info,
            "list" | "ls" | "l" => Self::List,
            "clean" | "cl" => Self::Clean,
            "create" | "c" => Self::Create,
            "make" | "mk" | "m" => Self::Make,
            "help" | "h" | "-h" => Self::Help,
            "version" | "-v" | "--version" | "v" => Self::Version,
            _ => return Err(format!("Command '{}' not recognised", method)),
        })
    }
}
