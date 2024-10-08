use draftlang_error::Error;
use draftlang_parser::{parse, types::DraftLangAst};
use std::{fs, path::PathBuf};


pub fn read_draftlang_file(file_path: &PathBuf) -> Result<String, Error> {
    match fs::read_to_string(file_path) {
        Ok(data) => Ok(data),
        Err(err) => Err(Error::ErrorReadingFile(Box::leak(
            err.to_string().into_boxed_str(),
        ))),
    }
}

pub fn parse_file(content: &str) -> Result<DraftLangAst, Error> {
    parse(content)
}
