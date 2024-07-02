use draftlang_error::Error;
use which::which;

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
