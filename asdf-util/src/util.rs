use color_print::cprintln;
use draftlang_error::Error;
use std::io::{BufRead, BufReader};
use std::process::Command;
use std::process::Stdio;
use which::which;

use crate::BINCOMMAND;

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
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

pub fn exec_stream(args: Vec<&'static str>) {
    match Command::new(BINCOMMAND)
        .args(&args)
        .stdout(Stdio::piped())
        // .stderr(Stdio::piped())
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
                    cprintln!("<green><bold>{}<bold><green>", data)
                }
                Err(_e) => {}
            }
        }
        Err(e) => {
            let error_msg = format!("{:?}", e);
            cprintln!("Error: <red>{:?}<red>", error_msg)
        }
    }
}
