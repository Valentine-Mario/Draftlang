pub mod error;


use which::which;
use error::Error;

const BINCOMMAND : &str = "asdf";

pub fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

pub fn check_asdf() -> Result<(), Error> {
    match  which(BINCOMMAND){
        Ok(_)=>{
            return Ok(())
        },
        Err(e)=>{
            let error_msg = format!("{:?}", e);
            return Err(Error::NotFound(string_to_static_str(error_msg)))
        }
    }
}
