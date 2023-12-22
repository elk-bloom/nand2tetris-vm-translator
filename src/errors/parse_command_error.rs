use std::fmt;

#[derive(Debug)]
pub struct ParseCommandError {
    pub command: String,
}
impl std::fmt::Display for ParseCommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to parse command '{}'", self.command)
    }
}
impl std::error::Error for ParseCommandError {}
