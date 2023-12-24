use std::{
    fs::File,
    io::{self, BufWriter, Write},
    path::Path,
};

pub struct CodeWriter {
    writer: BufWriter<File>,
}

impl CodeWriter {
    pub fn new<P: AsRef<Path>>(file_path: P) -> io::Result<CodeWriter> {
        let file = File::create(file_path)?;
        let writer = BufWriter::new(file.try_clone()?);
        Ok(CodeWriter { writer })
    }

    pub fn write(&mut self, string: &str) -> io::Result<()> {
        self.writer.write_all(string.as_bytes())
    }
}
