use log::{warn, debug};
use serde::Deserialize;
use crate::{core::{json::JSON, output::type_output::TypeOutput}, ship::spatium::Spatium};

use super::{lightweight::lightweight::Lightweight, deadweight::deadweight::Deadweight};


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

    pub fn spatiums(&self, type_ouput: TypeOutput) -> Result<Vec<Spatium>, String> {
        match type_ouput {
            TypeOutput::LightweghtIntensity => {
                self.lightweight.lightweight_intensity()
            },
            TypeOutput::DeadweightIntensity => {
                todo!()
            },
            TypeOutput::DisplacementTonnageIntensity => { todo!() },
            TypeOutput::BuoyantIntensity => todo!(),
            TypeOutput::BendingMoment => todo!(),
            TypeOutput::ShearForce => todo!(),
            TypeOutput::Stress => todo!(),
        }
    }


}