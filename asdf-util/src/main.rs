use asdf_util::{asdf_command, types::ASDFCommand};

fn main() {
    let a = ASDFCommand::PLUGIN("update --all");
    asdf_command(&a).unwrap();
}
