use std::collections::HashMap;

use log::{debug, warn};

use crate::equivalent_beam::pre_data::{from_csv::FromCSV, input::Input};

use super::cross_section::CrossSection;

pub struct CrossSections {
    cross_sections: HashMap<i32, CrossSection>,
}

impl CrossSections {
    pub fn new(file_path: String) -> Result<Self, String> {
        match CrossSections::from_csv(&file_path) {
            Ok(cross_sections) => {
                debug!("CrossSections::new | CrossSections hase been created successfully from file: {:#?}, CrossSections:\n{:#?}", file_path, cross_sections.cross_sections);
                Ok(cross_sections)
            }
            Err(err) => {
                warn!("CrossSections.new | error: {:?}",err);
                Err(err)

            }
        }
    }
}


impl FromCSV for CrossSections {
    fn from_csv(file_path: &String) -> Result<Self, String> {
        let input = Input::new(file_path);
        match input.run() {
            Ok(mut parser) => {
                let mut cross_sections = HashMap::new();
                for result in parser.deserialize::<CrossSection>() {
                    let cross_section = result?;
                    cross_sections.insert(cross_section.id, cross_section);
                }
                if cross_sections.len() == 0 {
                    todo!("cros_sections не должен быть пустым")
                }
                debug!("CrossSections.from_csv | Cross sections have been applied successfully form file: {:#?} {:#?}", file_path, cross_sections);
                Ok(CrossSections { cross_sections })
            },
            Err(err) => {
                warn!("CrossSections.from_csv | error: {:?}",err);
                Err(err)
            }
        }
        
    }
}


