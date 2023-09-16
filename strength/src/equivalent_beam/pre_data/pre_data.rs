use std::collections::HashMap;
use csv::Error as CSVError;
use log::{warn, debug};


use super::{input::Input, cross_section::cross_section::CrossSection};




pub struct PreData {
    cross_section_input_file: String,
    loads_input_file: String,
}

impl PreData {
    pub fn new(cross_section_input_file: String, loads_input_file: String) -> Self {
        PreData { cross_section_input_file, loads_input_file }
    }

    pub fn cross_sections(&self) -> Result<HashMap<i32, CrossSection>, CSVError> {
        let input = Input::new(&self.cross_section_input_file);
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
                debug!("PreData.cross_sections | Cross sections have been applied successfully: {:#?}", cross_sections);
                Ok(cross_sections)
            },
            Err(err) => {
                warn!("PreData.cross_sections | error: {:?}",err);
                Err(err)
            }
        }
    }
}
