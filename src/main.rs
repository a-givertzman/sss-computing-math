mod strength;
mod core;
mod tests;
use std::env;

use strength::ship::loads::lightweight::lightweight::Lightweight;



fn main() {
    env::set_var("RUST_LOG", "debug");
    //env::set_var("RUST_BACKTRACE", "1");
    env::set_var("RUST_BACKTRACE", "full");
    env_logger::init();
    let mut lightweight = Lightweight::from_json_file("./pre_data/data.json".to_string()).unwrap();
    lightweight.lightweight_intensity();

}

// Solution::new(
//     EquBeam::new(
//         CrossSections::from_csv_file(),
//         BendingMoment::new(
//             SheareForce::new(
//                 TotalShipLoad::new(
//                     BouyanLoad::new(Ship::from_json_file()),
//                     Displacment::new(
//                         Deadweight::from_json_file(),
//                         Lightweight::from_json_file()
//                     )
//                 )
//             )
//         )
//     )
// ).soleve();
