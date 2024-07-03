pub mod types;
pub mod util;

use draftlang_error::Error;
use types::ASDFCommand;

const BINCOMMAND: &str = "asdf";

pub fn exec_command(option: &ASDFCommand) -> Result<(), Error> {
    util::check_asdf()?;
    use ASDFCommand::*;
    match option {
        PLUGIN(_) => todo!(),
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
