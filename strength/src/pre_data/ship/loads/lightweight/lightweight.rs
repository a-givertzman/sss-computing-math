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
                        return Ok(ship)
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
                let nose = ship.coord_nose();
                let stern = ship.coord_stern();
                let length_spatium = ship.length_spatium();
                let mut current_coord = stern + length_spatium / 2.0;
                let (a, b, c) = self.lightweight_intensity_parameters(&ship);
                while current_coord < nose {
                    let end_coord = current_coord + length_spatium / 2.0;
                    let start_coord = current_coord - length_spatium / 2.0;
                    
                    let spatium = Spatium::new(start_coord, end_coord, 0.0, 0.0);
                    spatiums.push(spatium);
                    current_coord += length_spatium;
                }
                Ok(spatiums)
            },
            Err(err) => {
                warn!("Lightweight::from_json_file | error: {:?}.",err);
                return Err(err);
            }
        }               
    }

    fn lightweight_intensity_parameters(&self, ship: &Ship) -> (f64, f64, f64) {
        if ship.completeness_coefficient > 0.7 {
            (0.65, 1.20, 0.57)
        } else {
            (0.71, 1.17, 0.6)
        }
    }
}

