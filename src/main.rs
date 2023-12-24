mod errors;
mod models;
mod translators;

mod parser;

use std::fs;
use std::{env, path::PathBuf};

use clap::Parser as ClapParser;
use models::code_writer::CodeWriter;
use parser::Parser;
use translators::vm_translator::VMTranslator;

#[derive(ClapParser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The .vm/.asm file of directory of files to translate/assemble. If no path is provided, the current directory is used.
    input: Option<String>,
    /// Flag to indicate that the VM translator should be run on the input .vm file(s)
    #[arg(short, long, conflicts_with = "assemble")]
    translate: bool,
    /// Flag to indicate that the assembler should be run on the input .asm file(s)
    #[arg(short, long, conflicts_with = "translate")]
    assemble: bool,
}

enum Options {
    Translate,
    Assemble,
    // TODO: Compile?
    // TODO: All three?
}

fn main() {
    let args = Args::parse();

    let input_path: PathBuf = args
        .input
        .map(PathBuf::from)
        .unwrap_or_else(|| env::current_dir().expect("Failed to get current directory"));

    let option: Options;
    let source_extension: &str;
    let target_extension: &str;

    if args.translate {
        option = Options::Translate;
        source_extension = "vm";
        target_extension = "asm";
    } else if args.assemble {
        option = Options::Assemble;
        source_extension = "asm";
        target_extension = "hack";
    } else {
        option = Options::Translate;
        source_extension = "vm";
        target_extension = "asm";
    }

    let files: Vec<PathBuf> = if input_path.is_dir() {
        fs::read_dir(input_path)
            .expect("Invalid directory")
            .filter_map(|dir_entry| {
                let path = dir_entry
                    .expect("Invalid DirEntry in input directory")
                    .path();
                if path.is_file() && path.extension().is_some_and(|s| s.eq(source_extension)) {
                    Some(path)
                } else {
                    None
                }
            })
            .collect()
    } else if input_path.is_file()
        && input_path
            .extension()
            .is_some_and(|s| s.eq(source_extension))
    {
        vec![input_path]
    } else {
        panic!(
            "Input path of {} was not a valid file or directory",
            input_path.to_string_lossy()
        )
    };

    for file in files.iter() {
        let mut parser = match option {
            Options::Translate => Parser::new(file, VMTranslator::new()).unwrap(),
            Options::Assemble => todo!(),
        };
        let out_path = file.with_extension(target_extension);
        let mut writer = CodeWriter::new(out_path).unwrap();
        let translated_line: Option<String> = parser.translate().unwrap();
        match translated_line {
            None => {
                let termination_string = parser.get_termination_string();
                if let Some(s) = termination_string {
                    writer.write(s.as_str()).unwrap()
                }
            }
            Some(s) => writer.write(s.as_str()).unwrap(),
        }
    }
}
