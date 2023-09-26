use log::{warn, debug};
use serde::Deserialize;
use crate::pre_data::{json::JSON, ship::spatium::Spatium};
use super::{lightweight::lightweight::Lightweight, deadweight::deadweight::Deadweight, type_load::TypeLoad};


#[derive(Deserialize, Debug)]
pub struct Load {
    lightweight: Lightweight,
    deadweight: Deadweight
}


impl Load {
    pub fn new(lightweight: Lightweight, deadweight: Deadweight) -> Self {
        Load { lightweight, deadweight }
    }


    pub fn from_json_file(file_path: String) -> Result<Self, String> {
        let json = JSON::new(file_path);
        match json.reader() {
            Ok(reader) => {
                match serde_json::from_reader(reader) {
                    Ok(load) => {
                        debug!("Load::from_json_file | Load have been created from json file successfully.\n Load:\n {:#?}", load);
                        return Ok(load)
                    },
                    Err(err) => {
                        warn!("Load::from_json_file | error: {:?}.",err);
                        return Err(err.to_string());
                    }
                }
            },
            Err(err) => {
                warn!("Load::from_json_file | error: {:?}.",err);
                return Err(err);
            }
        }
    }

    pub fn spatiums(&self, type_load: TypeLoad) -> Result<Vec<Spatium>, String> {
        match type_load {
            TypeLoad::LightweghtIntensity => {
                self.lightweight.lightweight_intensity()
            },
            TypeLoad::DeadweightIntensity => {
                todo!()
            },
            TypeLoad::DisplacementTonnageIntensity => { todo!() },
            TypeLoad::BuoyantIntensity => todo!(),
            TypeLoad::BendingMoment => todo!(),
            TypeLoad::ShearForce => todo!(),
        }
    }


}