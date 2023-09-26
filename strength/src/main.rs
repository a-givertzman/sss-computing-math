mod equivalent_beam;
mod pre_data;
use pre_data::{cross_section_properties::cross_sections::CrossSections, ship::{ship::Ship, loads::lightweight::lightweight::Lightweight}};
use std::env;

fn main() {
    env::set_var("RUST_LOG", "debug");
    //env::set_var("RUST_BACKTRACE", "1");
    env::set_var("RUST_BACKTRACE", "full");
    env_logger::init();   
    let lightweight = Lightweight::from_json_file("./pre_data/lightweight.json".to_string()).unwrap();
    lightweight.lightweight_intensity();

}
