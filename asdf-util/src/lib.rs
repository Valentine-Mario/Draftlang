pub mod commands;
pub mod types;
pub mod util;

use draftlang_error::Error;
use types::{ASDFCommand, CommandReturnType};

const BINCOMMAND: &str = "asdf";

pub fn asdf_command(option: &ASDFCommand) -> Result<CommandReturnType, Error> {
    util::check_asdf()?;
    use ASDFCommand::*;
    match option {
        PLUGIN(cmd) => commands::plugin_cmd(cmd),
        CURRENT(_) => todo!(),
        GLOBAL(_) => todo!(),
        HELP(_) => todo!(),
        INSTALL(_) => todo!(),
        LATEST(_) => todo!(),
        LIST(_) => todo!(),
        LOCAL(_) => todo!(),
        SHELL(_) => todo!(),
        UNINSTALL(_) => todo!(),
        WHERE(_) => todo!(),
        WHICH(_) => todo!(),
        EXEC(_) => todo!(),
        ENV(_) => todo!(),
        INFO => todo!(),
        VERSION => todo!(),
        RESHIM(_) => todo!(),
        SHIMVERSION(_) => todo!(),
        UPDATE => todo!(),
        UPDATEHEAD => todo!(),
    }
}
