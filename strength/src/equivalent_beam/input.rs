use csv::Reader;
use csv::Error as CSVError;
use std::collections::HashMap;
use super::cross_section::CrossSection;


pub struct Input {
    file_path: String,
}


impl Input {
    pub fn new(file_path: String) -> Input {
        Input { file_path }
    }

    pub fn run(&self) -> Result<HashMap<i32, CrossSection>, CSVError>{
        let mut cross_sections = HashMap::new();
        let mut reader = Reader::from_path(&self.file_path)?;
        for result in reader.deserialize::<CrossSection>() {
            let cross_section = result?;
            cross_sections.insert(cross_section.id, cross_section);
        }
        Ok(cross_sections)
    }
}