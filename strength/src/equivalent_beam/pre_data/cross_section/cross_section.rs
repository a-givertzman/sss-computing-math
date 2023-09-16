use serde::Deserialize;

use crate::equivalent_beam::system_of_units::Meters;


#[derive(Deserialize, Debug)]
pub struct CrossSection {
    pub id: i32,
    I_z: f64,
    I_y: f64,
    z_up: Meters,
    z_down: Meters,
}

impl CrossSection {
    pub fn new(id: i32, I_z: f64, I_y: f64, z_up: Meters, z_down: Meters) -> Self {
        CrossSection { id, I_z, I_y, z_up, z_down }
    }
}