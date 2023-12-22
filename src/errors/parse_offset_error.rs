use std::fmt;

#[derive(Debug)]
pub struct ParseOffsetError {
    pub offset: String,
}
impl std::fmt::Display for ParseOffsetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to parse offset '{}'", self.offset)
    }
}
impl std::error::Error for ParseOffsetError {}
