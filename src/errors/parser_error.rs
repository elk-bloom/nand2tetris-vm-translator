use core::fmt;
use std::error::Error;

use super::translation_error::TranslationError;

#[derive(Debug)]
pub enum ParserError {
    Io {
        file_name: String,
        e: std::io::Error,
    },
    Translation {
        file_name: String,
        line_number: u32,
        e: TranslationError,
    },
}

impl ParserError {
    pub fn from_io_error(file_name: String, error: std::io::Error) -> Self {
        ParserError::Io {
            file_name,
            e: error,
        }
    }

    pub fn from_translation_error(
        file_name: String,
        line_number: u32,
        error: TranslationError,
    ) -> Self {
        ParserError::Translation {
            file_name,
            line_number,
            e: error,
        }
    }
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io { file_name, e } => write!(f, "IO error for file '{}': {}", file_name, e),
            Self::Translation {
                file_name,
                line_number,
                e,
            } => write!(
                f,
                "Translation error for file '{}' on line {}: {}",
                file_name, line_number, e
            ),
        }
    }
}

impl Error for ParserError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io { e, .. } => Some(e),
            Self::Translation { e, .. } => Some(e),
        }
    }
}
