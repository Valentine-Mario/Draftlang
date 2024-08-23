use std::fmt;

pub enum Error {
    ASDFNotFound(&'static str),
    ASDFCmdError(&'static str),
    ParsingError(&'static str),
    ErrorReadingFile(&'static str),
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
            Error::ParsingError(ele) => write!(f, "{}", ele),
            Error::ErrorReadingFile(ele) => write!(f, "Error reading file {}", ele),
            _ => write!(f, "{:?}", self::Error::NullError), // For any variant not previously covered
        }
    }
}
