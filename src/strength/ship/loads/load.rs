use log::{warn, debug};
use serde::Deserialize;
use crate::{core::json::JSON, strength::output::{type_output::TypeOutput, output::Output}};

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
        todo!()
    }

    pub fn spatiums(&self, type_ouput: TypeOutput) -> Output {
        match type_ouput {
            TypeOutput::LightweightIntensity => {
                self.lightweight.lightweight_intensity()
            },
            TypeOutput::DeadweightIntensity => {
                todo!()
            },
            TypeOutput::DisplacementIntensity => { todo!() },
            TypeOutput::BuoyantIntensity => todo!(),
            TypeOutput::TotalLoadIntensity => todo!(),
            TypeOutput::ShearForce => todo!(),
            TypeOutput::BendingMoment => todo!(),
            TypeOutput::Stress => todo!(),
        }
    }


}