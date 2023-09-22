use std::collections::HashMap;

use serde::Deserialize;

use crate::{equivalent_beam::spatium::Spatium, pre_data::ship::ship::Ship};

use super::{deadweight::deadweight::Deadweight, lightweight::lightweight::Lightweght};

#[derive(Debug, Deserialize)]
pub struct Load {
    deadweight: Deadweight,
    lightweight: Lightweght,
}

impl Load {
    pub fn spatiums(&self, ship: &Ship, type_load: TypeLoad) -> HashMap<String, Spatium> {
        match type_load {
            TypeLoad::LightweghtIntensity => { return self.lightweight.spatiums(&ship); },
            TypeLoad::DeadweightIntensity => { return self.deadweight.spatiums(&ship); },
            _ => { unreachable!() }
        }
    }
}



#[derive(Debug)]
pub enum TypeLoad {
    LightweghtIntensity,
    DeadweightIntensity,
    DisplacementTonnageIntensity,
    BuoyantIntensity,
    BendingMoment,
    ShearForce,
}