use std::path::PathBuf;

use structopt::StructOpt;

#[derive(Debug, StructOpt, Clone, Default)]
#[structopt(name = "DraftKang ", about = "Draftlang CLI")]
pub struct Args {
    #[structopt(long = "file-path", default_value = "DRAFTLANG", parse(from_os_str))]
    pub file_path: PathBuf,

    #[structopt(long = "func")]
    pub func: String,
}

impl Args {
    pub fn new() -> Self {
        Args::from_args()
    }
}
