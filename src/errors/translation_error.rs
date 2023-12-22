use std::fmt;

use super::vm_translation_error::VMTranslationError;

#[derive(Debug)]
pub enum TranslationError {
    VMTranslationError(VMTranslationError),
}

impl fmt::Display for TranslationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TranslationError::VMTranslationError(e) => write!(f, "{}", e),
        }
    }
}

impl std::error::Error for TranslationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            TranslationError::VMTranslationError(e) => Some(e),
        }
    }
}

impl From<VMTranslationError> for TranslationError {
    fn from(err: VMTranslationError) -> Self {
        TranslationError::VMTranslationError(err)
    }
}
