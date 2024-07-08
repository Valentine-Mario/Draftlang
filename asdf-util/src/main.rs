use asdf_util::{asdf, types::ASDFCommand};

fn main() {
    let a = ASDFCommand::VERSION;
    println!("returned => {}", asdf(&a).unwrap());
}
