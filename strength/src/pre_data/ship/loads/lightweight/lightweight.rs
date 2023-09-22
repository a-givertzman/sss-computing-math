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

    pub fn spatiums(&self, ship: &Ship) -> HashMap<String, Spatium> {
        let spatiums = ship.empty_spatiums();
        todo!()
    }

}

