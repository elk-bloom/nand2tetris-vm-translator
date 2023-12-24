use crate::errors::translation_error::TranslationError;

pub trait Translate {
    fn convert(&mut self, input: &str) -> Result<String, TranslationError>;
    /// if this is Some the caller ought know this ought come last
    fn termination_string(&mut self) -> Option<String>;
}
