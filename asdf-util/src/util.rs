use color_print::cprintln;
use draftlang_error::Error;
use std::io::{BufRead, BufReader};
use std::process::Command;
use std::process::Stdio;
use which::which;

use crate::types::CommandReturnType;
use crate::BINCOMMAND;

pub fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

pub fn check_asdf() -> Result<CommandReturnType, Error> {
    match which(BINCOMMAND) {
        Ok(_) => Ok(CommandReturnType::Empty),
        Err(e) => {
            let error_msg = format!("{:?}", e);
            Err(Error::ASDFNotFound(string_to_static_str(error_msg)))
        }
    }
}

//use this for short running commands
pub fn run_cmd(args: &Vec<&str>) -> Result<CommandReturnType, Error> {
    match Command::new(BINCOMMAND).args(args).output() {
        Ok(output) => Ok(CommandReturnType::CmdString(
            String::from_utf8_lossy(&output.stdout).to_string(),
        )),
        Err(e) => {
            let error_msg = format!("{:?}", e);
            Err(Error::ASDFCmdError(string_to_static_str(error_msg)))
        }
    }
}

//use this for long running commands eg install, uninstall, etc
pub fn exec_stream(args: &Vec<&str>) -> Result<CommandReturnType, Error> {
    match Command::new(BINCOMMAND)
        .args(args)
        .stdout(Stdio::piped())
        .spawn()
    {
        Ok(mut cmd) => {
            {
                let stdout = cmd.stdout.as_mut().unwrap();
                let stdout_reader = BufReader::new(stdout);
                let stdout_lines = stdout_reader.lines();

                for line in stdout_lines {
                    cprintln!(
                        "<green><bold>{:?}<bold><green>",
                        line.expect("error reading line")
                    );
                }
            }

            match cmd.wait() {
                Ok(data) => {
                    cprintln!("<green><bold>{}<bold><green>", data);
                    if data.success() {
                        Ok(CommandReturnType::Empty)
                    } else {
                        Err(Error::ASDFCmdError(string_to_static_str("".to_string())))
                    }
                }
                Err(e) => {
                    let error_msg = format!("{:?}", e);
                    cprintln!("Error: <red>{:?}<red>", error_msg);
                    Err(Error::ASDFCmdError(string_to_static_str(error_msg)))
                }
            }
        }
        Err(e) => {
            let error_msg = format!("{:?}", e);
            cprintln!("Error: <red>{:?}<red>", error_msg);
            Err(Error::ASDFCmdError(string_to_static_str(error_msg)))
        }
    }
}
