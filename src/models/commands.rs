use std::fmt::Display;

#[derive(Debug)]
pub struct Command {
    pub method: super::Method,
    pub args: Vec<String>,
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - [{}]", self.method, self.args.join(", "))
    }
}
