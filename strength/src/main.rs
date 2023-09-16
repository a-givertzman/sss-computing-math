use equivalent_beam::{input::Input, equ_beam::EquBeam};
mod equivalent_beam;
use std::collections::HashMap;

fn main() {
    env::set_var("RUST_LOG", "debug");
    // env::set_var("RUST_BACKTRACE", "1");
    env::set_var("RUST_BACKTRACE", "full");
    env_logger::init();

    let mut equ_beam = EquBeam::new(
        Input::new("./equ_beam/equ_beam.csv".to_string()),
        HashMap::new(),
    );
    equ_beam.run().unwrap();
}
