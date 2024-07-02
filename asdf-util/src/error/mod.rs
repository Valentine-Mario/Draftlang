use std::fmt;

pub enum Error {
    NotFound(&'static str)
}


#[allow(unreachable_patterns)]
impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Error::NotFound(ele) => write!(f, "ASDF installation not found please endure you have ASDF installed and properly configured read the guide here https://asdf-vm.com/guide/getting-started.html: {}", ele),
            _ => write!(f, "{:?}", self), // For any variant not previously covered
        }
    }
}