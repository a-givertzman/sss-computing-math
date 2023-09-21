use log::{debug, warn};
use serde::Deserialize;

use crate::{equivalent_beam::force::Force, pre_data::{json::JSON, loads::intensity_load::IntensityLoad}};

#[derive(Deserialize, Debug)]
pub struct Deadweight {
    loads: Vec<Force>,
}


impl Deadweight {
    pub fn new(loads: Vec<Force>) -> Self {
        Deadweight { loads }
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

    pub fn intensity(&self) -> IntensityLoad {
        todo!("Распределить Deadweight по шпациям. Вернуть интенсивность нагрузки H/m")
    }
}