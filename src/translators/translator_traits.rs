use crate::errors::translation_error::TranslationError;

pub trait Translate {
    fn convert(&mut self, input: &str) -> Result<String, TranslationError>;
}
