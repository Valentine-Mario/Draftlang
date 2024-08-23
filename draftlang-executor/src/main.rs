mod util;
fn main() {
    let raw_file = util::read_draftlang_file();
    match raw_file {
        Ok(data) => {
            let ast = util::parse_file(&data);
            println!("{:?}", ast)
        }
        Err(err) => {
            let error_msg = format!("{:?}", err);
            panic!("{}", error_msg)
        }
    }
}
