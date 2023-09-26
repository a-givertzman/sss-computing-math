mod equivalent_beam;
mod ship;
mod core;
mod output;
use std::env;

use ship::loads::lightweight::lightweight::Lightweight;

fn main() {
    env::set_var("RUST_LOG", "debug");
    //env::set_var("RUST_BACKTRACE", "1");
    env::set_var("RUST_BACKTRACE", "full");
    env_logger::init();   
    let lightweight = Lightweight::from_json_file("./pre_data/lightweight.json".to_string()).unwrap();
    lightweight.lightweight_intensity();

}
