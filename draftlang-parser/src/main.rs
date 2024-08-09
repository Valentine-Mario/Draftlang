use color_print::cprintln;
use draftlang_parser::parse_file;

fn main() {
    let unparsed_file = std::fs::read_to_string("DRAFTLANG").expect("cannot read elixir file");
    // println!("{:?}", unparsed_file);
    let v = parse_file(&unparsed_file);
    match v {
        Ok(v) => {
            println!("parsed values {:#?}", v)
        }
        Err(e) => {
            match e.line_col {
                pest::error::LineColLocation::Pos((line, position)) => {
                    cprintln!(
                        "<red>>>> Error on line {:?} position {:?}<red>",
                        line,
                        position
                    )
                }
                _ => todo!(),
            }
            cprintln!("<red>>>> {:?}<red>", e.line())
        }
    }
}
