mod core;
mod interpreter;

use color_print::cprintln;
use draftlang_error::Error;

mod util;
fn main() {
    let raw_file = util::read_draftlang_file().expect("error reading DRAFTLANG file.");
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
