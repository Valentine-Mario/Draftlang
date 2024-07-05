use color_print::cprintln;
use draftlang_error::Error;

use crate::{
    types::CommandReturnType,
    util::{exec_stream, run_cmd, string_to_static_str},
};

pub fn plugin_cmd(cmd: &str) -> Result<CommandReturnType, Error> {
    let mut args: Vec<&str> = cmd.split(' ').collect();
    if args.len() > 1 {
        //append plugin to the top of the vector
        args.insert(0, "plugin");
        //use exec_stream for add and update since they are long running commands
        if args[1].trim() == "add" || args[1].trim() == "update" {
            exec_stream(&args)
        } else {
            match run_cmd(&args) {
                Ok(cmd_return_string) => {
                    match cmd_return_string {
                        CommandReturnType::CmdString(data) => {
                            cprintln!("<green><bold>{}<bold><green>", data)
                        }
                        CommandReturnType::Empty => {}
                    }
                    Ok(CommandReturnType::Empty)
                }
                Err(e) => Err(e),
            }
        }
    } else {
        cprintln!("<red>Invalid number of command arguments<red>");
        Err(Error::ASDFCmdError(string_to_static_str(
            "Invalid number of args".to_string(),
        )))
    }
}
