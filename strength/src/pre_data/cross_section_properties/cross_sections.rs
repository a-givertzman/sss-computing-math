use std::collections::HashMap;
use csv::Reader;
use log::{debug, warn};
use std::fs::File;
use crate::pre_data::csv::{CSV, FromCSV};
use super::cross_section::CrossSection;


pub struct CrossSections {
    cross_sections: HashMap<i32, CrossSection>,
}

impl CrossSections {
    pub fn new(cross_sections: HashMap<i32, CrossSection>) -> Self {
        CrossSections { cross_sections }
    }
    fn from_csv_parser(mut parser: Reader<File>) -> Result<Self, String> {
        let mut cross_sections = HashMap::new();
        for result in parser.deserialize::<CrossSection>() {
            match result {
                Ok(cross_section) => { cross_sections.insert(cross_section.id, cross_section); },
                Err(err) => {
                    warn!("CrossSections::from_csv | error: {:?}",err);
                    return Err(err.to_string());
                }
            }
        }
        if cross_sections.is_empty() {
            warn!("CrossSections::from_csv_parser | error: Expected at least one record but got none.");
            return Err("Expected at least one record but got none.".to_owned());
        }
        debug!("CrossSections::from_csv_parser | Cross sections have been created successfully.\n CrossSections:\n {:#?}", cross_sections);
        Ok(CrossSections::new(cross_sections))
    }
}


impl FromCSV for CrossSections {
    fn from_csv(file_path: String) -> Result<Self, String> {
        let input = CSV::new(&file_path);
        match input.parser() {
            Ok(parser) => {
                CrossSections::from_csv_parser(parser)
            },
            Err(err) => {
                warn!("CrossSections::from_csv | error: {:?}.",err);
                Err(err.to_string())
            }
        }
    }
}


