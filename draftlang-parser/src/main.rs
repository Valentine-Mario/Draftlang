use draftlang_parser::parse_file;
fn main() {
    let unparsed_file = std::fs::read_to_string("DRAFTLANG").expect("cannot read elixir file");
    // println!("{:?}", unparsed_file);
    parse_file(&unparsed_file)
}
