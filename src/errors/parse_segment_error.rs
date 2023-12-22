use std::fmt;

#[derive(Debug)]
pub struct ParseSegmentError {
    pub segment: String,
}
impl std::fmt::Display for ParseSegmentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to parse segment '{}'", self.segment)
    }
}
impl std::error::Error for ParseSegmentError {}
