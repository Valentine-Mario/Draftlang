use asdf_util::util::exec_stream;
fn main() {
    //run_cmd(vec!["plugin"]).unwrap();
    exec_stream(vec!["plugin", "list", "--urls"]);
}
