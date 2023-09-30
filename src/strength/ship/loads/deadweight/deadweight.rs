use log::{debug, warn};
use serde::Deserialize;


use crate::{core::json::JSON, strength::ship::{ship::Ship, spatium::Spatium}};

use super::force::Force;


#[derive(Deserialize, Debug)]
pub struct Deadweight {
    deadweight: Vec<Force>,
}


impl Deadweight {
    pub fn new(deadweight: Vec<Force>) -> Self {
        Deadweight { deadweight }
    }

    pub fn from_json_file(file_path: String) -> Result<Self, String> {
        let json = JSON::new(file_path);
        match json.reader() {
            Ok(reader) => {
                match serde_json::from_reader(reader) {
                    Ok(deadweight) => {
                        debug!("Deadweight::from_json_file | Deadweight have been created from json file successfully.\n Deadweight:\n {:#?}", deadweight);
                        return Ok(deadweight)
                    },
                    Err(err) => {
                        warn!("Deadweight::from_json_file | error: {:?}.",err);
                        return Err(err.to_string());
                    }
                }
            },
            Err(err) => {
                warn!("Deadweight::from_json_file | error: {:?}.",err);
                return Err(err);
            }
        }
    }

    pub fn deadweight_intensity(&self, ship: &Ship) -> Vec<Spatium> {
        todo!()
    }
}
