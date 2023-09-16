mod equivalent_beam;
use equivalent_beam::{equ_beam::EquBeam, pre_data::{ship::ship::Ship, cross_section::cross_sections::CrossSections}};
use std::env;

fn main() {
    env::set_var("RUST_LOG", "debug");
    // env::set_var("RUST_BACKTRACE", "1");
    env::set_var("RUST_BACKTRACE", "full");
    env_logger::init();

    let equ_beam = EquBeam::new(
        CrossSections::new("./pre_data/cross_sections_data.csv".to_string()),
        Ship::new("./pre_data/ship_data.csv".to_string()),
    );
    equ_beam.solve();

}
