mod equivalent_beam;
mod pre_data;
use equivalent_beam::equ_beam::EquBeam;
use pre_data::{cross_section_properties::cross_sections::CrossSections, ship::ship::Ship};
use std::env;

fn main() {
    env::set_var("RUST_LOG", "debug");
    //env::set_var("RUST_BACKTRACE", "1");
    env::set_var("RUST_BACKTRACE", "full");
    env_logger::init();

    let cross_section = CrossSections::from_csv_file("./pre_data/cross_sections_data.csv".to_string());
    let ship = Ship::from_json_file("./pre_data/ship_data.json".to_string()).unwrap();
    
}


// Solution {
//     EquBeam {
//         ResultantDistributedLoad {
//             DistributedBuoyantLoad {
//                 TheoreticalDrawing::from_file(file_input)}.spatiums(),
//             DistributedLightweight{
//                 Ship::from_file(file_input)
//             }.spatiums(),
//             DistributedDeadweight {
//                 CargoShip::from_file(file_input)
//             }.spatiums()
//         }.spatiums()
//     }.diagrams()
// }.solution()