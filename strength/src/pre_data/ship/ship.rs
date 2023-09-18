use log::{debug, warn};
use serde::Deserialize;
use crate::pre_data::{from_csv::FromCSV, input::CSV};
use std::fs::File;
use csv::Reader;

use super::ship_measurements::ShipMeasurements;


/// Ship
/// - ship_hull_weight - Вес металлического корпуса корабля без учета перевозимого груза, жидкостей, баласта и.т.д.
#[derive(Deserialize, Debug)]
pub struct Ship {
    ship_measurements: ShipMeasurements,
    ship_name: String,
    ship_hull_weight: f64,
}


impl Ship {
    pub fn new(ship_measurements: ShipMeasurements,
        ship_name: String,
        ship_hull_weight: f64,) -> Self {
        Ship { ship_measurements, ship_name, ship_hull_weight }
    }

    fn from_parser(mut parser: Reader<File>) -> Result<Self, String> {
        let mut result = parser.deserialize::<Ship>();
        if let Some(record) = result.next() {
            match record {
                Ok(ship) => {
                    debug!("Ship::new | The ship hase been created successfully.\n Ship:\n{:#?}", ship);
                    return Ok(ship);
                },
                Err(err) => {
                    warn!("Ship::from_data | error: {:?}",err);
                    return Err(err.to_string());
                }
            }
        } else {
            warn!("Ship::from_data | error: Expected at least one record but got none.");
            Err("Expected at least one record but got none.".to_owned())
        }
    }

    fn length_spatium(&self) -> f64 {
        self.ship_measurements.length_spatium()
    }
}

impl FromCSV for Ship {
    fn from_csv(file_path: String) -> Result<Self, String> {
        let input = CSV::new(&file_path);
        match input.parser() {
            Ok(parser) => {
                Ship::from_parser(parser)
            },
            Err(err) => {
                warn!("Ship::from_data | error: {:?}.",err);
                Err(err.to_string())
            }
        }
    }
}