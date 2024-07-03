pub mod types;


use draftlang_error::Error;
use which::which;
use types::ASDFCommand;

const BINCOMMAND: &str = "asdf";

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


pub fn exec_command(option: &ASDFCommand){
    use ASDFCommand::*;
    match option {
        PLUGIN(_)=>todo!(),
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