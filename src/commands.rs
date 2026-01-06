use std::fmt::Display;

#[derive(Debug)]
pub enum Method {
    Init,
    Info,
    Clean,
    Create,
    Make,
    Help,
}
impl Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Method::Init => "init",
            Method::Info => "info",
            Method::Clean => "clean",
            Method::Create => "create",
            Method::Make => "make",
            Method::Help => "help",
        };
        write!(f, "{}", name)
    }
}

impl Method {
    pub fn from(method: &str) -> Result<Self, String> {
        Ok(match method {
            "init" => Self::Init,
            "info" | "i" => Self::Info,
            "clean" | "cl" => Self::Clean,
            "create" | "c" => Self::Create,
            "make" | "mk" | "m" => Self::Make,
            "help" | "h" | "-h" => Self::Help,
            _ => return Err(format!("Command '{}' not recognised", method)),
        })
    }
}

#[derive(Debug)]
pub struct Command {
    pub method: Method,
    pub args: Vec<String>,
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - [{}]", self.method, self.args.join(", "))
    }
}
