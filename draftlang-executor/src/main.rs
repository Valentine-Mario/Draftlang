mod cli;
mod core;
mod interpreter;

use color_print::cprintln;
use draftlang_error::Error;

mod util;
fn main() {
    let args = cli::Args::new();

    let raw_file = util::read_draftlang_file(&args.file_path).expect("error reading DRAFTLANG file.");

    println!("{:?}", args);
    match util::parse_file(&raw_file) {
        Ok(ast) => {
            println!("{:?}", ast)
        }
        Err(err) => match err {
            Error::ParsingError(message) => {
                cprintln!("<red>{}<red>", message)
            }
            _ => unreachable!(),
        },
    }
}
