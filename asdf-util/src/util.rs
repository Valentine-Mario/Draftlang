use color_print::cprintln;
use draftlang_error::Error;
use std::io::{BufRead, BufReader};
use std::process::Command;
use std::process::Stdio;
use which::which;

use crate::BINCOMMAND;

pub fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

pub fn split_str(cmd: &str) -> Vec<&str> {
    cmd.split(' ').collect()
}

pub fn check_asdf() -> Result<(), Error> {
    match which(BINCOMMAND) {
        Ok(_) => Ok(()),
        Err(e) => {
            let error_msg = format!("{:?}", e);
            Err(Error::ASDFNotFound(string_to_static_str(error_msg)))
        }
    }
}

pub fn exec_stream(args: &Vec<&str>) -> Result<String, Error> {
    let mut return_string: String = String::from("");
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
                    let str_line = line.expect("error reading line");
                    cprintln!("<green><bold>{:?}<bold><green>", str_line);
                    return_string.push_str(&str_line);
                    return_string.push('\n');
                }
            }

            match cmd.wait() {
                Ok(data) => {
                    cprintln!("<green><bold>{}<bold><green>", data);
                    if data.success() {
                        Ok(return_string)
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
