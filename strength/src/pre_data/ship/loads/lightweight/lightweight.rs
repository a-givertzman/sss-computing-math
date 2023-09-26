use log::{warn, debug};
use serde::Deserialize;
use crate::{equivalent_beam::{system_of_units::Newton, spatium::Spatium}, pre_data::{ship::ship::Ship, json::JSON}};


/// Lightweight - the weight of the ship without cargo, fuel, lubricating oil, ballast water,
/// fresh water and feed water in tanks, consumable stores, passengers and crew and their belongings.
#[derive(Deserialize, Debug)]
pub struct Lightweight {
    weight: Newton,
}

impl Lightweight {
    pub fn new(weight: Newton) -> Self {
        Lightweight { weight }
    }

    pub fn from_json_file(file_path: String) -> Result<Self, String> {
        let json = JSON::new(file_path);
        match json.reader() {
            Ok(reader) => {
                match serde_json::from_reader(reader) {
                    Ok(ship) => {
                        debug!("Lightweight::from_json_file | Lightweght have been created from json file successfully.\n Ship:\n {:#?}", ship);
                        return Ok(ship);
                    },
                    Err(err) => {
                        warn!("Lightweight::from_json_file | error: {:?}.",err);
                        return Err(err.to_string());
                    }
                }
            },
            Err(err) => {
                warn!("Lightweight::from_json_file | error: {:?}.",err);
                return Err(err);
            }
        }
    }
    /// Determines the lightweight intensity by spatiums
    pub fn lightweight_intensity(&self) -> Result<Vec<Spatium>, String> {
        match Ship::from_json_file("./pre_data/ship_data.json".to_string()) {
            Ok(ship) => {
                let mut spatiums = vec![];
                let mut current_coord = ship.coord_stern() + ship.length_spatium() / 2.0;
                while current_coord <= (ship.coord_nose() - ship.length_spatium() / 2.0) {
                    let spatium = self.spatium(current_coord, &ship);
                    debug!("{:?}", spatium);
                    spatiums.push(spatium);
                    current_coord += ship.length_spatium();
                }
                Ok(spatiums)
            },
            Err(err) => {
                warn!("Lightweight::from_json_file | error: {:?}.",err);
                return Err(err);
            }
        }
    }

    fn spatium(&self, current_coord: f64, ship: &Ship) -> Spatium {
        let (a, b, c) = self.lightweight_intensity_parameters(&ship);
        let end_coord = current_coord + ship.length_spatium() / 2.0;
        let start_coord = current_coord - ship.length_spatium() / 2.0;
        let intensity_load = |parametr: f64| {
            ((self.weight.0 / ship.number_spatiums() as f64) * parametr) / ship.length_spatium()
        };
        if current_coord > ship.coord_stern() && current_coord < (ship.coord_stern() + ship.length_design_waterline() / 3.0) {
            let parametr = a + ((b - a) * (ship.length_design_waterline() / 2.0 - current_coord.abs()))/(ship.length_design_waterline() / 3.0);
            let intensity_load = intensity_load(parametr);
            Spatium::new(start_coord, end_coord, intensity_load, intensity_load)
        } else if current_coord >= ship.coord_stern() + ship.length_design_waterline() / 3.0 && current_coord < (ship.coord_nose() - ship.length_design_waterline() / 3.0) {
            let intensity_load = intensity_load(b);
            Spatium::new(start_coord, end_coord, intensity_load, intensity_load)
        } else {
            let parametr = c + ((b - c) * (ship.length_design_waterline() / 2.0 - current_coord))/(ship.length_design_waterline() / 3.0);
            let intensity_load = intensity_load(parametr);
            Spatium::new(start_coord, end_coord, intensity_load, intensity_load)
        }
    }


    fn lightweight_intensity_parameters(&self, ship: &Ship) -> (f64, f64, f64) {
        if ship.completeness_coefficient  <= 0.7 {
            (0.65, 1.20, 0.57)
        } else {
            (0.71, 1.17, 0.6)
        }
    }
}

