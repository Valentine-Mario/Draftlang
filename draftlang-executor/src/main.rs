use color_print::cprintln;
use draftlang_error::Error;

mod util;
fn main() {
    let raw_file = util::read_draftlang_file();
    match raw_file {
        Ok(data) => match util::parse_file(&data) {
            Ok(ast) => {
                println!("{:?}", ast)
            }
            Err(err) => match err {
                Error::ParsingError(message) => {
                    cprintln!("<red>{}<red>", message)
                }
                _ => unreachable!(),
            },
        },
        Err(err) => {
            let error_msg = format!("{:?}", err);
            panic!("{}", error_msg)
        }
    }
}
