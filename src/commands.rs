use std::fmt::Display;

#[derive(Debug)]
pub enum Method {
    Init,
    Clean,
    Create,
    Make,
    Help,
}
impl Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Method::Init => "init",
            Method::Clean => "clean",
            Method::Create => "create",
            Method::Make => "make",
            Method::Help => "help",
        };
        write!(f, "{}", name)
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
