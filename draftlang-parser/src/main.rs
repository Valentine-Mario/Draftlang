use draftlang_parser::parse;

fn main() {
    let unparsed_file = std::fs::read_to_string("DRAFTLANG").expect("cannot read elixir file");
    let v = parse(&unparsed_file);
    println!("{:?}", v);
}
