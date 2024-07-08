use color_print::cprintln;
use draftlang_error::Error;

use crate::util::{exec_stream, split_str, string_to_static_str};

pub fn plugin_cmd(cmd: &str) -> Result<String, Error> {
    let mut args = split_str(cmd);
    if args.len() > 1 {
        //append plugin to the top of the vector
        args.insert(0, "plugin");
        exec_stream(&args)
    } else {
        cprintln!("<red>Invalid number of command arguments<red>");
        Err(Error::ASDFCmdError(string_to_static_str(
            "Invalid number of args".to_string(),
        )))
    }
}

pub fn current_cmd(cmd: &str) -> Result<String, Error> {
    let mut args = split_str(cmd);
    if args.len() == 1 && args[0].trim() == "" {
        args.pop();
    }
    args.insert(0, "current");
    exec_stream(&args)
}
