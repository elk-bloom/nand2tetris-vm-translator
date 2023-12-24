use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

use crate::errors::parser_error::ParserError;
use crate::translators::translator_traits;

pub struct Parser<T: translator_traits::Translate> {
    file_name: String,
    reader: BufReader<File>,
    translator: T,
    current_line: String,
    pub current_line_number: u32,
}

impl<T: translator_traits::Translate> Parser<T> {
    pub fn new<P: AsRef<Path>>(file_path: P, translator: T) -> Result<Parser<T>, ParserError> {
        let file_path_buf = file_path.as_ref().to_path_buf();
        let file_name = file_path_buf
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        let file = File::open(&file_path_buf)
            .map_err(|e| ParserError::from_io_error(file_name.clone(), e))?;
        let reader = BufReader::new(file);
        Ok(Parser {
            file_name,
            reader,
            translator,
            current_line: String::new(),
            current_line_number: 0,
        })
    }

    /// A return value of > 0 indicates that that many lines were read.
    /// A return value of 0 indicates that no lines were read due to EOF, the caller should know to stop.
    fn advance(&mut self) -> io::Result<u32> {
        self.current_line.clear();
        let mut line = String::new();
        let mut lines_read = 0;

        loop {
            let result = self.reader.read_line(&mut line);
            let bytes_read = result?;
            if bytes_read == 0 {
                return Ok(0);
            }
            lines_read += 1;
            self.current_line_number += 1;
            let content_before_comment = line.split("//").next().unwrap_or_default().trim();
            if !content_before_comment.is_empty() {
                self.current_line.push_str(content_before_comment);
                return Ok(lines_read);
            }
            line.clear();
        }
    }

    pub fn translate(&mut self) -> Result<Option<String>, ParserError> {
        let lines_read = self
            .advance()
            .map_err(|e| ParserError::from_io_error(self.file_name.to_string(), e))?;

        if lines_read == 0 {
            return Ok(None);
        }

        self.translator
            .convert(&self.current_line)
            .map_err(|e| {
                ParserError::from_translation_error(
                    self.file_name.to_string(),
                    self.current_line_number,
                    e,
                )
            })
            .map(Some)
    }

    pub fn get_termination_string(&mut self) -> Option<String> {
        self.translator.termination_string()
    }
}
