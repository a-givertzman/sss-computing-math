use log::{debug, warn};
use serde::Deserialize;
use crate::equivalent_beam::system_of_units::Tons;
use crate::pre_data::json::JSON;
use super::ship_measurements::ShipMeasurements;


/// Ship
/// - lightweght - это вес судна (в тоннах), когда оно было построено на верфи.
/// Не включает вес любого расходного материала, такого как топливо, вода, масло или другие расходные материалы.
/// - completeness_coefficient - коэффициент полноты,
/// - max_displacement_tonnage - водоизмещение судна (в тоннах) при 100% загрузки,
/// - deadweight - это вес (в тоннах) всего груза, топлива, сухих продуктов, припасов и тд, перевозимых на борту судна.
/// - draft - осадка
#[derive(Deserialize, Debug)]
pub struct Ship {
    pub ship_measurements: ShipMeasurements,
    pub completeness_coefficient: f64,
    pub max_displacement_tonnage: Tons,

}


impl Ship {
    pub fn new(ship_measurements: ShipMeasurements,
        completeness_coefficient: f64,
        max_displacement_tonnage: Tons,) -> Self {
        Ship { ship_measurements, completeness_coefficient, max_displacement_tonnage }
    }

    pub fn from_json_file(file_path: String) -> Result<Self, String> {
        let json = JSON::new(file_path);
        match json.reader() {
            Ok(reader) => {
                match serde_json::from_reader(reader) {
                    Ok(ship) => {
                        debug!("Ship::from_json_file | Ship have been created from json file successfully.\n Ship:\n {:#?}", ship);
                        return Ok(ship)
                    },
                    Err(err) => {
                        warn!("Ship::from_json_file | error: {:?}.",err);
                        return Err(err.to_string());
                    }
                }
            },
            Err(err) => {
                warn!("Ship::from_json_file | error: {:?}.",err);
                return Err(err);
            }
        }
    }

    pub fn coord_nose(&self) -> f64 {
        self.ship_measurements.length_design_waterline / 2.0
    }

    pub fn coord_stern(&self) -> f64 {
        - self.ship_measurements.length_design_waterline / 2.0
    }

    pub fn length_design_waterline(&self) -> f64 {
        self.ship_measurements.length_design_waterline
    }

    pub fn length_spatium(&self) -> f64 {
        self.ship_measurements.length_spatium()

    }

    pub fn number_spatiums(&self) -> i64 {
        self.ship_measurements.number_spatiums
    }
}
