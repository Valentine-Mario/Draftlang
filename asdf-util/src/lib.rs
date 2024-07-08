pub mod commands;
pub mod types;
pub mod util;

use draftlang_error::Error;
use types::ASDFCommand;

const BINCOMMAND: &str = "asdf";

pub fn asdf(option: &ASDFCommand) -> Result<String, Error> {
    util::check_asdf()?;
    use ASDFCommand::*;
    match option {
        PLUGIN(cmd) => commands::asdf_cmd("plugin", cmd.trim()),
        CURRENT(cmd) => commands::asdf_cmd("current", cmd.trim()),
        GLOBAL(cmd) => commands::asdf_cmd("global", cmd.trim()),
        HELP(cmd) => commands::asdf_cmd("help", cmd.trim()),
        INSTALL(cmd) => commands::asdf_cmd("install", cmd.trim()),
        LATEST(cmd) => commands::asdf_cmd("latest", cmd.trim()),
        LIST(cmd) => commands::asdf_cmd("list", cmd.trim()),
        LOCAL(cmd) => commands::asdf_cmd("local", cmd.trim()),
        SHELL(cmd) => commands::asdf_cmd("shell", cmd.trim()),
        UNINSTALL(cmd) => commands::asdf_cmd("uninstall", cmd.trim()),
        WHERE(cmd) => commands::asdf_cmd("where", cmd.trim()),
        WHICH(cmd) => commands::asdf_cmd("which", cmd.trim()),
        EXEC(cmd) => commands::asdf_cmd("exec", cmd.trim()),
        ENV(cmd) => commands::asdf_cmd("env", cmd.trim()),
        INFO => commands::asdf_cmd("info", ""),
        VERSION => commands::asdf_cmd("version", ""),
        RESHIM(cmd) => commands::asdf_cmd("reshim", cmd.trim()),
        SHIMVERSION(cmd) => commands::asdf_cmd("shim-versions", cmd.trim()),
        UPDATE => commands::asdf_cmd("update", ""),
        UPDATEHEAD => commands::asdf_cmd("update", "--head"),
    }
}
