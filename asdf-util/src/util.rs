use draftlang_error::Error;
use std::process::Command;
use which::which;

use crate::BINCOMMAND;

pub fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

pub fn check_asdf() -> Result<(), Error> {
    match which(BINCOMMAND) {
        Ok(_) => return Ok(()),
        Err(e) => {
            let error_msg = format!("{:?}", e);
            return Err(Error::ASDFNotFound(string_to_static_str(error_msg)));
        }
    }
}

pub fn run_cmd(args: Vec<&str>) -> Result<String, Error> {
    match Command::new(BINCOMMAND).args(args).output() {
        Ok(output) => Ok(String::from_utf8_lossy(&output.stdout).to_string()),
        Err(e) => {
            let error_msg = format!("{:?}", e);
            Err(Error::ASDFCmdError(string_to_static_str(error_msg)))
        }
    }
}
