use draftlang_error::Error;

use crate::util::{exec_stream, split_str};

pub fn asdf_cmd(cmd: &str, args_str: &str) -> Result<String, Error> {
    let mut args = split_str(args_str);
    if args.len() == 1 && args[0].trim() == "" {
        args.pop();
    }
    args.insert(0, cmd);
    exec_stream(&args)
}
