use log::{debug, warn};
use serde::Deserialize;

use crate::equivalent_beam::pre_data::{from_csv::FromCSV, input::Input};


#[derive(Deserialize, Debug)]
pub struct Ship {
    
}
impl Ship {
    pub fn new(file_path: String) -> Result<Self, String> {
        match Ship::from_csv(&file_path) {
            Ok(ship) => {
                debug!("Ship::new | The ship hase been created successfully{:#?}", ship);
                Ok(ship)
            }
            Err(err) => {
                warn!("Ship::from_data | error: {:?}",err);
                Err(err.to_string())
            }            
        }
    }
}

impl FromCSV for Ship {
    fn from_csv(file_path: &String) -> Result<Self, String> {
        let input = Input::new(&file_path);
        match input.run() {
            Ok(mut parser) => {               
                let mut result = parser.deserialize::<Ship>();
                if let Some(record) = result.next() {
                    let ship = record.map_err(|err| { err.to_string() })?;                    
                    return Ok(ship);
                } else {
                    warn!("Ship::from_data | error: Expected one record but got none");
                    Err("Expected at least one record but got none".to_owned())
                }
            },
            Err(err) => {
                warn!("Ship::from_data | error: {:?}",err);
                Err(err.to_string())
            }
        }   
    }
}