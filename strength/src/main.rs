use equivalent_beam::{input::Input, equ_beam::EquBeam};
mod equivalent_beam;
use std::collections::HashMap;

fn main() {
    let mut equ_beam = EquBeam::new(
        Input::new("./equ_beam/equ_beam.csv".to_string()),
        HashMap::new(),
    );
    equ_beam.run().unwrap();
}
