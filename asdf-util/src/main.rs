use asdf_util::{asdf_command, types::ASDFCommand};

fn main() {
    let a = ASDFCommand::CURRENT("erlang");
    println!("returned => {}", asdf_command(&a).unwrap());
}
