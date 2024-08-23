use draftlang_error::Error;
use draftlang_parser::{parse, types::DraftLangAst};
use std::fs;

const DRAFTLANG: &str = "DRAFTLANG";

pub fn read_draftlang_file() -> Result<String, Error> {
    match fs::read_to_string(DRAFTLANG) {
        Ok(data) => Ok(data),
        Err(err) => Err(Error::ErrorReadingFile(Box::leak(
            err.to_string().into_boxed_str(),
        ))),
    }
}

pub fn parse_file(content: &str) -> Result<DraftLangAst, Error> {
    parse(content)
}
