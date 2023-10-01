use log::{warn, debug};
use serde::Deserialize;
use crate::{core::json::JSON, strength::output::{type_output::TypeOutput, output::Output}};

use super::{lightweight_tonnage::lightweight_tonnage::LightweightTonnage, deadweight::deadweight::Deadweight};


#[derive(Deserialize, Debug)]
pub struct Load {
    lightweight: LightweightTonnage,
    deadweight: Deadweight
}


impl Load {
    pub fn new(lightweight: LightweightTonnage, deadweight: Deadweight) -> Self {
        Load { lightweight, deadweight }
    }


    pub fn from_json_file(file_path: String) -> Result<Self, String> {
        todo!()
    }

    pub fn spatiums(&self, type_ouput: TypeOutput) -> Output {
        match type_ouput {
            TypeOutput::LightweightTonnageIntensity => {
                self.lightweight.lightweight_tonnage_intensity()
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