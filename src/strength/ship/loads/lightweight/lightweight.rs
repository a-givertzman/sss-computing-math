use log::{warn, debug};
use serde::Deserialize;

use crate::{core::json::JSON, strength::{ship::{ship::Ship, spatium::Spatium}, output::{output::Output, type_output::TypeOutput}}};



/// LightweightTonnage(LWT) - weight of the empty as-built ship without cargo, fuel, lubricating oil, ballast water,
/// fresh water and feed water in tanks, consumable stores, passengers and crew and their belongings. Measured in tons
#[derive(Deserialize, Debug)]
pub struct LightweightTonnage {
    pub lightweight_tonnage: f64,
    pub ship: Ship,
}

impl LightweightTonnage {
    pub fn new(lightweight_tonnage: f64, ship: Ship) -> Self {
        LightweightTonnage { lightweight_tonnage, ship}
    }

    pub fn from_json_file(file_path: String) -> Result<Self, String> {
        let json = JSON::new(file_path);
        match json.reader() {
            Ok(reader) => {
                match serde_json::from_reader(reader) {
                    Ok(ship) => {
                        debug!("LightweightTonnage::from_json_file | Lightweght have been created from json file successfully.\n Ship:\n {:#?}", ship);
                        return Ok(ship);
                    },
                    Err(err) => {
                        warn!("LightweightTonnage::from_json_file | error: {:?}.",err);
                        return Err(err.to_string());
                    }
                }
            },
            Err(err) => {
                warn!("LightweightTonnage::from_json_file | error: {:?}.",err);
                return Err(err);
            }
        }
    }
    /// Computes the lightweight intensity for spatiums
    pub fn lightweight_intensity(&self) -> Output {
        let mut spatiums = vec![];
        let mut current_coord = self.ship.coord_stern() + self.ship.length_spatium() / 2.0;
        while current_coord <= (self.ship.coord_nose() - self.ship.length_spatium() / 2.0) {
            let spatium = self.spatium(current_coord);
            spatiums.push(spatium);
            current_coord += self.ship.length_spatium();
        }
        debug!("LightweightTonnage.lightweight_intensity() | Lightweight Tonnage intensity hase been computed successfully.");
        Output::new(spatiums, TypeOutput::LightweightIntensity)

    }

    fn spatium(&self, current_coord: f64) -> Spatium {
        let (a, b, c) = self.lightweight_intensity_parameters();
        let end_coord = current_coord + self.ship.length_spatium() / 2.0;
        let start_coord = current_coord - self.ship.length_spatium() / 2.0;
        let intensity_load = |parametr: f64| {
            ((self.lightweight_tonnage / self.ship.number_spatiums() as f64) * parametr) / self.ship.length_spatium()
        };
        if current_coord > self.ship.coord_stern() && current_coord < (self.ship.coord_stern() + self.ship.length_design_waterline() / 3.0) {
            let parametr = a + ((b - a) * (self.ship.length_design_waterline() / 2.0 - current_coord.abs()))/(self.ship.length_design_waterline() / 3.0);
            let intensity_load = intensity_load(parametr);
            Spatium::new(start_coord, end_coord, intensity_load, intensity_load)
        } else if current_coord >= self.ship.coord_stern() + self.ship.length_design_waterline() / 3.0 && current_coord < (self.ship.coord_nose() - self.ship.length_design_waterline() / 3.0) {
            let intensity_load = intensity_load(b);
            Spatium::new(start_coord, end_coord, intensity_load, intensity_load)
        } else {
            let parametr = c + ((b - c) * (self.ship.length_design_waterline() / 2.0 - current_coord))/(self.ship.length_design_waterline() / 3.0);
            let intensity_load = intensity_load(parametr);
            Spatium::new(start_coord, end_coord, intensity_load, intensity_load)
        }
    }


    fn lightweight_intensity_parameters(&self) -> (f64, f64, f64) {
        if self.ship.completeness_coefficient  <= 0.7 {
            (0.65, 1.20, 0.57)
        } else {
            (0.71, 1.17, 0.6)
        }
    }
}

