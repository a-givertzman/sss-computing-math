use std::collections::HashMap;
use log::warn;
use crate::pre_data::{cross_section_properties::cross_sections::CrossSections, ship::{ship::Ship, loads::{type_load::TypeLoad, lightweight::lightweight::Lightweght, deadweight::deadweight::Deadweight}}};
use super::spatium::Spatium;





pub struct EquBeam {
    cross_sections: Result<CrossSections, String>,
    ship: Result<Ship, String>,
}

impl EquBeam {
    pub fn new(cross_sections: Result<CrossSections, String>, ship: Result<Ship, String>) -> Self {
        EquBeam { cross_sections, ship }
    }

    pub fn spatiums(&self, type_load: TypeLoad) -> Result<HashMap<i64, Spatium>, String> {
        match type_load {
            TypeLoad::LightweghtIntensity => {
                let lightweight = Lightweght::from_json_file("./pre_data/lightweight.json".to_string());
                match lightweight {
                    Ok(lightweight) => {
                        return lightweight.spatiums();
                    },
                    Err(err) => {
                        warn!("EquBeam.spatiums() | error: {:?}.",err);
                        return Err(err.to_string());
                    }
                }
            },
            TypeLoad::DeadweightIntensity => {
                match Deadweight::from_json_file("./pre_data/deadweight.json".to_string()) {
                    Ok(deadweight) => { return deadweight.spatiums(); },
                    Err(err) => {
                        warn!("EquBeam.spatiums() | error: {:?}.",err);
                        return Err(err.to_string());
                    }
                }
            },
            TypeLoad::DisplacementTonnageIntensity => { todo!() },
            TypeLoad::BuoyantIntensity => todo!(),
            TypeLoad::BendingMoment => todo!(),
            TypeLoad::ShearForce => todo!(),
        }
    }
}