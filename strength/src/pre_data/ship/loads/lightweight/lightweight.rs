use std::collections::HashMap;

use log::{warn, debug};
use serde::Deserialize;

use crate::{equivalent_beam::{system_of_units::Tons, spatium::Spatium}, pre_data::{ship::ship::Ship, json::JSON}};

#[derive(Deserialize, Debug)]
pub struct Lightweght {
    massa: Tons,
}

impl Lightweght {
    pub fn new(massa: Tons) -> Self {
        Lightweght { massa }
    }

    pub fn from_f64(massa: f64) -> Self {
        Lightweght::new(Tons(massa))
    }

    pub fn from_json_file(file_path: String) -> Result<Self, String> {
        let json = JSON::new(file_path);
        match json.reader() {
            Ok(reader) => {
                match serde_json::from_reader(reader) {
                    Ok(ship) => {
                        debug!("Lightweght::from_json_file | Lightweght have been created from json file successfully.\n Ship:\n {:#?}", ship);
                        return Ok(ship)
                    },
                    Err(err) => {
                        warn!("Lightweght::from_json_file | error: {:?}.",err);
                        return Err(err.to_string());
                    }
                }
            },
            Err(err) => {
                warn!("Lightweght::from_json_file | error: {:?}.",err);
                return Err(err);
            }
        }
    }

    pub fn spatiums(&self) -> Result<HashMap<i64, Spatium>, String> {
         match Ship::from_json_file("./pre_data/ship_data.json".to_string()) {
            Ok(ship) => {
                let mut spatiums: HashMap<i64, Spatium> = HashMap::new();
                let spatium_noce = Spatium::new(0, 20.0, ship.length_spatium());
                let spatium_stern = Spatium::new(ship.number_spatiums(), 20.0, ship.length_spatium());
                spatiums.insert(0, spatium_noce);
                spatiums.insert(ship.number_spatiums(), spatium_stern);
                let (a, b, c) = self.params(&ship);
                let mut length = ship.length_spatium();
                while length  < ship.length_design_waterline() - ship.length_spatium() {
                    if length > 0.0 && length > ship.length_design_waterline() / 3.0 {

                    }


                    length += ship.length_spatium();
                }

                todo!()
            },
            Err(err) => {
                warn!("Lightweght.spatiums() | error: {:?}.",err);
                return Err(err.to_string());
            }
        }
    }

    

    
    fn params(&self, ship: &Ship) -> (f64, f64, f64) {
        if ship.completeness_coefficient > 0.7 {
            (0.65, 1.20, 0.57)
        } else {
            (0.71, 1.17, 0.6)
        }
    }



}

