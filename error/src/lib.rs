use std::fmt;

pub enum Error {
    ASDFNotFound(&'static str),
    ASDFCmdError(&'static str),
    NullError,
}

#[allow(unreachable_patterns)]
impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Error::ASDFNotFound(ele) => {
                write!(f, "ASDF installation not found please endure you have ASDF installed and properly 
                configured read the guide here https://asdf-vm.com/guide/getting-started.html: {}", 
                ele)
            }
            Error::ASDFCmdError(ele) => write!(f, "Error running ASDF command {}", ele),
            _ => write!(f, "{:?}", self::Error::NullError), // For any variant not previously covered
        }
    }
}
