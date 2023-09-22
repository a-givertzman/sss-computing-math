use std::collections::HashMap;

use log::{debug, warn};
use serde::Deserialize;

use crate::equivalent_beam::spatium::Spatium;
use crate::equivalent_beam::system_of_units::Tons;
use crate::pre_data::json::JSON;
use crate::pre_data::ship::loads::deadweight::deadweight::Deadweight;
use crate::pre_data::ship::loads::lightweight::lightweight::Lightweght;
use super::loads::load::{Load, TypeLoad};
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
    ship_measurements: ShipMeasurements,
    ship_name: String,
    completeness_coefficient: f64,
    max_displacement_tonnage: Tons,
    draft: f64,
}


impl Ship {
    pub fn new(ship_measurements: ShipMeasurements,
        ship_name: String,
        completeness_coefficient: f64,
        max_displacement_tonnage: Tons,
        draft: f64) -> Self {
        Ship { ship_measurements, ship_name,
            completeness_coefficient, max_displacement_tonnage, draft }
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

    pub fn empty_spatiums(&self) -> HashMap<String, Spatium> {
        let mut spatiums: HashMap<String, Spatium> = HashMap::new();
        for i in 0..self.number_spatiums() - 1 {
            let id = format!("{}-{}", i, i+1);
            let clone_id = id.clone();
            let spatium = Spatium::new(id, 0.0, self.length_spatium());
            spatiums.insert(clone_id, spatium);
        }
        spatiums
    }


    pub fn spatiums(&self, type_load: TypeLoad) -> HashMap<String, Spatium> {
        match type_load {
            TypeLoad::LightweghtIntensity => {
                let lightweight = Lightweght::from_json_file("./pre_data/ship_data.json".to_string());
                todo!()
            },
            TypeLoad::DeadweightIntensity => {
                let deadweight = Deadweight::from_json_file("./pre_data/ship_data.json".to_string());
                todo!()
            },
            TypeLoad::DisplacementTonnageIntensity => { todo!() },
            TypeLoad::BuoyantIntensity => todo!(),
            TypeLoad::BendingMoment => todo!(),
            TypeLoad::ShearForce => todo!(),
        }
    }


    fn length_spatium(&self) -> f64 {
        self.ship_measurements.length_spatium()
    }

    fn number_spatiums(&self) -> i64 {
        self.ship_measurements.number_spatiums()
    }
}
