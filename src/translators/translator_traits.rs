use crate::errors::translation_error::TranslationError;

pub trait Translate {
    fn convert(&self, input: &str) -> Result<String, TranslationError>;
}
