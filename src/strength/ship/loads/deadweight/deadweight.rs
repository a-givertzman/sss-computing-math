use log::{debug, warn};
use serde::Deserialize;


use crate::{core::json_file::JsonFile, strength::ship::{ship::Ship, spatium::Spatium}};

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
        todo!()
    }

    pub fn deadweight_intensity(&self, ship: &Ship) -> Vec<Spatium> {
        todo!()
    }
}
